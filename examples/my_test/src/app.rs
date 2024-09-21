use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    
    Screen = {{FigmaScreen}} {}
    
    App = {{App}} {
        ui: <Root>{
            main_window = <Window>{
                //window: {inner_size: vec2(800, 1000)}
                show_bg: true
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        return #4;
                    }
                }
                
                <Screen> {
                    width: 500
                    height: 500
                    label_walk: {
                        width: Fit
                        height: Fit
                    }
                                            
                    Text = <Label> {
                        draw_text: {
                            color: #0
                        }
                    }
                }
            }
        }
    }
}

app_main!(App);
 
#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}
 
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App{
    fn handle_actions(&mut self, _cx: &mut Cx, _actions:&Actions){}
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

#[derive(Live, Widget)]
struct FigmaScreen{
    #[redraw] #[live] draw_bg: DrawColor,
    #[live] draw_line: DrawLine,
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    #[live] draw_text: DrawText,
    #[live] label_walk: Walk,
    
    #[rust] templates: ComponentMap<LiveId, LivePtr>,
}

impl LiveHook for FigmaScreen {
    fn before_apply(&mut self, _cx: &mut Cx, apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if let ApplyFrom::UpdateFromDoc {..} = apply.from {
            self.templates.clear();
        }
    }
        
    // hook the apply flow to collect our templates and apply to instanced childnodes
    fn apply_value_instance(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
        if nodes[index].is_instance_prop() {
            if let Some(live_ptr) = apply.from.to_live_ptr(cx, index) {
                let id = nodes[index].id;
                self.templates.insert(id, live_ptr);
            }
        }
        else {
            cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
        }
        nodes.skip_node(index)
    }
}

impl Widget for FigmaScreen {
    fn draw_walk(&mut self, cx:&mut Cx2d, _scope:&mut Scope, _walk:Walk)->DrawStep {
        let text_widget = WidgetRef::new_from_ptr(cx, Some(*self.templates.get(&live_id!(Text)).unwrap()));
        
        // FRAME (52:5) Frame
        self.draw_bg.color = vec4(1.0,1.0,1.0,1.0);
        self.draw_bg.begin(
            cx,
            Walk {
                width: Size::Fixed(120.0), 
                height: Size::Fixed(180.0),
                abs_pos: Some(DVec2 {x: 0.0, y: 0.0}),
                margin: Margin::default(),
            },
            self.layout
        );
        self.draw_bg.end(cx);
        
        // TEXT (Here we go)
        text_widget.set_text("Here we go");
        text_widget.draw_walk_all(
            cx,
            &mut Scope::empty(), 
            Walk { 
                abs_pos: Some(DVec2{x: 35.0, y: 20.0}), 
                width: Size::Fixed(65.0), 
                height: Size::Fixed(15.0),
                margin: Margin::default()
            }
        );
        // RECTANGLE (52:2) Rectangle
        self.draw_bg.color = vec4(1.0,0.667,0.592,1.0);
        self.draw_bg.begin(
            cx,
            Walk {
                width: Size::Fixed(80.0), 
                height: Size::Fixed(40.0),
                abs_pos: Some(DVec2 {x: 20.0, y: 36.667}),
                margin: Margin::default(),
            },
            self.layout
        );
        self.draw_bg.end(cx);
        
        // RECTANGLE (52:3) Rectangle
        self.draw_bg.color = vec4(1.0,0.667,0.592,1.0);
        self.draw_bg.begin(
            cx,
            Walk {
                width: Size::Fixed(80.0), 
                height: Size::Fixed(40.0),
                abs_pos: Some(DVec2 {x: 20.0, y: 78.333}),
                margin: Margin::default(),
            },
            self.layout
        );
        self.draw_bg.end(cx);
        
        // RECTANGLE (52:4) Rectangle
        self.draw_bg.color = vec4(1.0,0.667,0.592,1.0);
        self.draw_bg.begin(
            cx,
            Walk {
                width: Size::Fixed(80.0), 
                height: Size::Fixed(40.0),
                abs_pos: Some(DVec2 {x: 20.0, y: 120.0}),
                margin: Margin::default(),
            },
            self.layout
        );
        self.draw_bg.end(cx);
        
        
        DrawStep::done()
    }
}

