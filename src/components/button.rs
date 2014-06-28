use super::super::{
    UiContext,
    Component,
    ClippedRectangle
};

pub struct Button {
    id: &'static str,
}

#[deriving(Default)]
pub struct ButtonDrawState {
    pub is_over: bool,
    pub is_down: bool,
    pub is_clicked: bool
}

impl Component<bool, ButtonDrawState> for Button {
    fn id(&self) -> &'static str { self.id }

    fn act(&self,
           clip: ClippedRectangle,
           ui_context: &mut UiContext,
           draw_state: &mut ButtonDrawState) -> bool {

        let id = Some(self.id);
        draw_state.is_over = clip.contains(ui_context.mouse_pos);
        draw_state.is_down = draw_state.is_over && ui_context.mouse_down == id;
        draw_state.is_clicked = draw_state.is_over
            && !draw_state.is_down
            && ui_context.mouse_was_down == id;

        draw_state.is_clicked
    }
}

impl Button {
    pub fn new(id: &'static str) -> Button {
        Button { id: id }
    }
}

pub fn button(id: &'static str) -> Button {
    Button {id: id}
}
