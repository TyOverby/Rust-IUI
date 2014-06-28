use super::super::{
    ClippedRectangle,
    UiContext,
    Component,
    AddState,
    WithoutState
};

pub struct ToggleButton {
    id: &'static str,
    pressed_state: bool
}

#[deriving(Default)]
pub struct ToggleButtonDrawState {
    pub activated: bool,
    pub is_over: bool,
    pub is_down: bool,
    pub is_clicked: bool
}

impl Component<bool, ToggleButtonDrawState> for ToggleButton {
    fn id(&self) -> &'static str { self.id }

    fn act(&self,
           clip: ClippedRectangle,
           ui_context: &mut UiContext,
           draw_state: &mut ToggleButtonDrawState) -> bool {
        let id = Some(self.id);
        draw_state.is_over = clip.contains(ui_context.mouse_pos);
        draw_state.is_down = draw_state.is_over && ui_context.mouse_down == id;
        draw_state.is_clicked = draw_state.is_over
            && !draw_state.is_down
            && ui_context.mouse_was_down == id;
        if draw_state.is_clicked {
            draw_state.activated = !self.pressed_state;
        } else {
            draw_state.activated = self.pressed_state;
        }
        return draw_state.activated;
    }
}

pub fn toggle_button(id: &'static str) -> ToggleButton {
    WithoutState::without_state(id)
}

impl ToggleButton {
    pub fn new(id: &'static str, pressed_state: bool) -> ToggleButton {
        ToggleButton {
            id: id,
            pressed_state: pressed_state
        }
    }
}

impl WithoutState<bool, ToggleButtonDrawState> for ToggleButton {
    fn without_state(id: &'static str) -> ToggleButton {
        ToggleButton::new(id, false)
    }
}

impl AddState<bool, ToggleButtonDrawState> for ToggleButton {
    fn add_state(&mut self, state: &bool) {
        self.pressed_state = *state;
    }
}
