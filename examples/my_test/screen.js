const fs = require("node:fs");

const screen = "screen";
const types = [
  "FRAME",
  "TEXT",
  "ELLIPSE",
  "RECTANGLE",
  "INSTANCE",
  "VECTOR",
  "GROUP",
];
const withBgColor = [
  "FRAME",
  "ELLIPSE",
  "RECTANGLE",
  "INSTANCE",
  "VECTOR",
  "GROUP",
];

const content = fs.readFileSync(`${screen}.json`);
const json = JSON.parse(content);
const document = json.nodes[Object.keys(json.nodes)[0]].document;
const nodes = collectNodes(document, []);

const screenNode = nodes[0];
screenNode.children.reverse();
const res = nodes.map((n) => {
  const data = {
    id: n.id,
    name: n.name,
    type: n.type,
    absoluteBoundingBox: {
      ...n.absoluteBoundingBox,
      x: n.absoluteBoundingBox.x - screenNode.absoluteBoundingBox.x,
      y: n.absoluteBoundingBox.y - screenNode.absoluteBoundingBox.y,
    },
  };

  if (withBgColor.includes(n.type)) {
    const fill = n.fills?.[0];
    const background = n.background?.[0];
    if (fill && fill.visible != false) {
      data.backgroundColor = fill.color;
    } else if (background && background.visible != false) {
      data.backgroundColor = background.color;
    }

    const stroke = n.strokes?.[0];
    if (stroke && stroke.visible != false) {
      data.strokeColor = col(stroke.color);
      data.strokeWeight = n.strokeWeight;
      data.strokeAlign = n.strokeAlign;
    }

    data.cornerRadius = n.cornerRadius;
  } else if (n.type === "TEXT") {
    data.characters = n.characters;
    data.color = col(n.fills?.[0]?.color ?? { r: 0, g: 0, b: 0, a: 1 });
  }

  return data;
});

fs.writeFileSync(`${screen}_res.json`, JSON.stringify(res, null, 2));

const rust = res
  .map((n) => {
    if (withBgColor.includes(n.type)) {
      if ((!n.backgroundColor || n.backgroundColor.a === 0) && !n.strokeColor) {
        return "";
      }
      return `// ${n.type} (${n.id}) ${n.name}\n${drawColor(n)}\n${
        n.strokeColor ? `// STROKE\n${drawStroke(n)}` : ""
      }`;
    }

    if (n.type === "TEXT") {
      return `// ${n.type} (${n.name})\n${drawText(n)}`;
    }

    return "";
  })
  .filter((s) => s.length > 0)
  .join("\n");

fs.writeFileSync(`${screen}_rust.txt`, rust);

/****************************
 ********** Collect *********
 ****************************/

function collectNodes(node, res) {
  if (types.includes(node.type)) {
    res.push(node);

    if (node.children?.length) {
      for (const n of node.children) {
        collectNodes(n, res);
      }
    }
  }

  return res;
}

/****************************
 ********** Utils ***********
 ****************************/

function fls(num) {
  return num
    .toLocaleString(undefined, { minimumFractionDigits: 1 })
    .replace(",", "");
}

function col(color) {
  const { r, g, b, a } = color ?? { r: 0, g: 0, b: 0, a: 0 };
  return `vec4(${[r, g, b, a].map((num) => fls(num)).join()})`;
}

function drawText(n) {
  const { x, y, width, height } = n.absoluteBoundingBox;

  return `text_widget.set_text("${n.characters}");
text_widget.draw_walk_all(
    cx,
    &mut Scope::empty(), 
    Walk { 
        abs_pos: Some(DVec2{x: ${fls(x)}, y: ${fls(y)}}), 
        width: Size::Fixed(${fls(width)}), 
        height: Size::Fixed(${fls(height)}),
        margin: Margin::default()
    }
);`;
}

function drawColor(n) {
  const { x, y, width, height } = n.absoluteBoundingBox;

  return `self.draw_bg.color = ${col(n.backgroundColor)};
self.draw_bg.begin(
    cx,
    Walk {
        width: Size::Fixed(${fls(width)}), 
        height: Size::Fixed(${fls(height)}),
        abs_pos: Some(DVec2 {x: ${fls(x)}, y: ${fls(y)}}),
        margin: Margin::default(),
    },
    self.layout
);
self.draw_bg.end(cx);`;
}

function drawStroke(n) {
  const { x, y, width, height } = n.absoluteBoundingBox;
  const strokeColor = n.strokeColor;
  const strokeWeight = fls(n.strokeWeight);

  return [
    drawLine(
      { x: fls(x), y: fls(y) },
      { x: fls(x + width), y: fls(y) },
      strokeColor,
      strokeWeight
    ),
    drawLine(
      { x: fls(x + width), y: fls(y) },
      { x: fls(x + width), y: fls(y + height) },
      strokeColor,
      strokeWeight
    ),
    drawLine(
      { x: fls(x + width), y: fls(y + height) },
      { x: fls(x), y: fls(y + height) },
      strokeColor,
      strokeWeight
    ),
    drawLine(
      { x: fls(x), y: fls(y + height) },
      { x: fls(x), y: fls(y) },
      strokeColor,
      strokeWeight
    ),
  ].join("\n");
}

function drawLine(start, end, color, weight) {
  return `self.draw_line.draw_line_abs(
    cx,
    DVec2{x: ${start.x},y:${start.y}},
    DVec2{x: ${end.x},y:${end.y}},
    ${color},
    ${weight}
);`;
}
