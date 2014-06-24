use super::lib::{
    ClippedRectangle,
    UiContext,
    Component,
    AddState,
    WithoutState
};
use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Fill,
    BackEnd,
    Draw,
    ImageSize,
};

use piston::{MousePress};

pub struct Button {
    id: &'static str,
}

impl Component<bool> for Button {
    fn id(&self) -> &'static str { self.id }
    fn draw<'a, I: ImageSize, B: BackEnd<I>>
        (&self, clip: ClippedRectangle, ui_context: &UiContext,  ctx: &Context, back_end: &mut B) {
        let bound = clip.bounds;
        let id = Some(self.id);

        let border = ctx.rect(bound.x, bound.y, bound.w, bound.h);
        let colored = if ui_context.mouse_over == id {
            if ui_context.mouse_down == id {
                border.rgb(0.0, 0.0, 1.0)
            } else if ui_context.mouse_was_down == id {
                border.rgb(1.0, 1.0, 1.0)
            } else {
                border.rgb(1.0, 0.0, 0.0)
            }
        } else {
            border.rgb(0.0, 1.0, 0.0)
        };

        colored.fill(back_end);
    }

    fn act(&self, clip: ClippedRectangle, ui_context: &mut UiContext) -> bool {
        clip.contains(ui_context.mouse_pos)
            && ui_context.mouse_down != Some(self.id)
            && ui_context.mouse_was_down == Some(self.id)
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


pub struct ToggleButton {
    id: &'static str,
    pressed_state: bool
}

impl Component<bool> for ToggleButton {
    fn id(&self) -> &'static str { self.id }
    fn draw<'a, I: ImageSize, B: BackEnd<I>>
        (&self, clip: ClippedRectangle, ui_context: &UiContext, ctx: &Context, back_end: &mut B) {

        let bound = clip.bounds;
        let id = Some(self.id);

        let border = ctx.rect(bound.x, bound.y, bound.w, bound.h);
        let colored = if (self.pressed_state) {
            border.rgb(1.0, 0.0, 0.0)
        } else {
            border.rgb(0.0, 1.0, 0.1)
        };

        colored.fill(back_end);
    }

    fn act(&self, clip: ClippedRectangle, ui_context: &mut UiContext) -> bool {
        if(clip.contains(ui_context.mouse_pos)
            && ui_context.mouse_down != Some(self.id)
            && ui_context.mouse_was_down == Some(self.id)) {
            !self.pressed_state
        } else {
            self.pressed_state
        }
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

impl WithoutState<bool> for ToggleButton {
    fn without_state(id: &'static str) -> ToggleButton {
        ToggleButton::new(id, false)
    }
}

impl AddState<bool> for ToggleButton {
    fn add_state(&mut self, state: &bool) {
        self.pressed_state = *state;
    }
}
