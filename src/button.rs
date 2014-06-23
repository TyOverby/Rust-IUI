use super::lib::{ClippedRectangle, UiContext, Component};
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

        let border = ctx.rect(bound.x, bound.y, bound.w, bound.h);
        let colored = if bound.contains(ui_context.mouse_pos) {
            border.rgb(1.0, 0.0, 0.0)
        } else {
            border.rgb(0.0, 1.0, 0.0)
        };

        colored.fill(back_end);
    }

    fn act(&self, clip: ClippedRectangle, ui_context: &mut UiContext) -> bool {
        if clip.contains(ui_context.mouse_pos) {
            ui_context.active_events.iter().any(|e| {
                match e {
                    &MousePress(_) => true,
                    _ => false
                }
            })
        } else {
            false
        }
    }
}

impl Button {
    pub fn new(id: &'static str) -> Button {
        Button { id: id }
    }
}

