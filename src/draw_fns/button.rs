use super::super::components::button::ButtonDrawState;
use super::super::ClippedRectangle;

use graphics::Context;
use graphics::BackEnd;
use graphics::ImageSize;
use graphics::AddRectangle;
use graphics::AddColor;
use graphics::Draw;
use graphics::Fill;
use graphics::RectangleColorContext;

pub fn draw_button<I: ImageSize, B: BackEnd<I>>
    (clip_box: ClippedRectangle,
     draw_state: ButtonDrawState,
     ctx: &Context,
     backend: &mut B) {
    if !clip_box.is_part_visible() {
        return;
    }

    let bound = clip_box.bounds;
    let button_shape = ctx.rect(bound.x, bound.y, bound.w, bound.h);
    let colored = if draw_state.is_clicked {
        button_shape.rgb(1.0, 1.0, 1.0)
    } else if draw_state.is_down {
        button_shape.rgb(0.0, 1.0, 0.0)
    } else if draw_state.is_over {
        button_shape.rgb(1.0, 0.0, 0.0)
    } else {
        button_shape.rgb(0.0, 0.0, 1.0)
    };
    colored.fill(backend);
}
