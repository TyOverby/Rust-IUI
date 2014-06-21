use super::lib::{ClippedRectangle, UiContext, Component, };
use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Fill,
    BackEnd,
    Draw,
    Image
};

pub struct Button {
    id: &'static str
}

impl Component<bool> for Button {
    fn id(&self) -> &'static str { self.id }
    fn draw<'a, I: Image, B: BackEnd<I>> (&self, clip: ClippedRectangle, ctx: &Context, back_end: &mut B) {
        let bound = clip.bounds;
        ctx.rect(bound.x, bound.y, bound.w, bound.h).rgb(1.0, 0.0, 0.0).fill(back_end);
    }

    fn act(&self, context: &mut UiContext) -> bool {
        false
    }
}

impl Button {
    pub fn new(id: &'static str) -> Button {
        Button { id: id }
    }
}

