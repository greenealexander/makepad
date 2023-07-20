use {crate::{Diff, Point, Length}, std::ops::AddAssign};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Text {
    lines: Vec<String>,
}

impl Text {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.length() == Length::default()
    }

    pub fn length(&self) -> Length {
        Length::new(self.lines.len() - 1, self.lines.last().unwrap().len())
    }

    pub fn as_lines(&self) -> &[String] {
        &self.lines
    }

    pub fn slice(&self, range: Range) -> Text {
        Text {
            lines: if range.start.line == range.end.line {
                vec![
                    self.lines[range.start.line][range.start.column..range.end.column]
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>(),
                ]
            } else {
                let mut lines = Vec::with_capacity(range.end.line - range.start.line + 1);
                lines.push(
                    self.lines[range.start.line][range.start.column..]
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>(),
                );
                lines.extend(
                    self.lines[range.start.line + 1..range.end.line]
                        .iter()
                        .cloned(),
                );
                lines.push(
                    self.lines[range.end.line][..range.end.column]
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>(),
                );
                lines
            },
        }
    }


    pub fn take(&mut self, length: Length) -> Self {
        let mut lines = self
            .lines
            .drain(..length.lines as usize)
            .collect::<Vec<_>>();
        lines.push(self.lines.first().unwrap()[..length.bytes].to_string());
        self.lines
            .first_mut()
            .unwrap()
            .replace_range(..length.bytes, "");
        Text { lines }
    }

    pub fn skip(&mut self, length: Length) {
        self.lines.drain(..length.lines);
        self.lines
            .first_mut()
            .unwrap()
            .replace_range(..length.bytes, "");
    }

    pub fn insert(&mut self, point: Point, mut text: Self) {
        if text.length().lines == 0 {
            self.lines[point.line]
                .replace_range(point.byte..point.byte, text.lines.first().unwrap());
        } else {
            text.lines
                .first_mut()
                .unwrap()
                .replace_range(..0, &self.lines[point.line][..point.byte]);
            text.lines
                .last_mut()
                .unwrap()
                .push_str(&self.lines[point.line][point.byte..]);
            self.lines
                .splice(point.line..point.line + 1, text.lines);
        }
    }

    pub fn delete(&mut self, start: Point, length: Length) {
        use std::iter;

        if length.lines == 0 {
            self.lines[start.line]
                .replace_range(start.byte..start.byte + length.bytes, "");
        } else {
            let mut line = self.lines[start.line][..start.byte].to_string();
            line.push_str(&self.lines[start.line + length.lines][length.bytes..]);
            self.lines.splice(
                start.line..start.line + length.lines + 1,
                iter::once(line),
            );
        }
    }

    pub fn apply_diff(&mut self, diff: Diff) {
        use crate::diff::Operation;

        let mut point = Point::default();
        for operation in diff {
            match operation {
                Operation::Delete(length) => self.delete(point, length),
                Operation::Retain(length) => point += length,
                Operation::Insert(text) => {
                    let length = text.length();
                    self.insert(point, text);
                    point += length;
                }
            }
        }
    }

    pub fn into_lines(self) -> Vec<String> {
        self.lines
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            lines: vec![String::new()],
        }
    }
}

impl AddAssign for Text {
    fn add_assign(&mut self, mut other: Self) {
        other
            .lines
            .first_mut()
            .unwrap()
            .replace_range(..0, self.lines.last().unwrap());
        self.lines
            .splice(self.lines.len() - 1..self.lines.len(), other.lines);
    }
}

impl From<&str> for Text {
    fn from(string: &str) -> Self {
        let mut lines: Vec<_> = string.lines().map(|string| string.to_owned()).collect();
        if lines.is_empty() {
            lines.push(String::new());
        }
        Self { lines }
    }
}