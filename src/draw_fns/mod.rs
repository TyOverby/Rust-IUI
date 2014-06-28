use graphics::{
    Context,
    BackEnd,
    ImageSize,
    AddRectangle,
    AddColor,
    Draw,
    Fill,
    RectangleColorContext
};

use super::components::button::ButtonDrawState;
use super::components::toggle_button::ToggleButtonDrawState;
use super::ClippedRectangle;

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

    let colored = match (draw_state.is_clicked,
                         draw_state.is_down, draw_state.is_over) {
        (false, false, false)  => button_shape.rgb(0.0, 0.0, 1.0),
        (true, _, _) => button_shape.rgb(1.0, 1.0, 1.0),
        (_, true, _) => button_shape.rgb(1.0, 0.0, 0.0),
        (_, _, true) => button_shape.rgb(0.0, 1.0, 0.0)
    };

    colored.fill(backend);
}

pub fn draw_toggle_button<I: ImageSize, B: BackEnd<I>>
    (clip_box: ClippedRectangle,
     draw_state: ToggleButtonDrawState,
     ctx: &Context,
     backend: &mut B) {

    if !clip_box.is_part_visible() {
        return;
    }

    let bound = clip_box.bounds;
    let shape = ctx.rect(bound.x, bound.y, bound.w, bound.h);

    let colored = match (draw_state.is_down, draw_state.is_over,
                         draw_state.activated, draw_state.is_clicked) {
        (_, _, _, true) => shape.rgb(1.0, 1.0, 1.0),
        (true, _, true,  _) => shape.rgb(0.0, 0.7, 0.0),
        (true, _, false, _) => shape.rgb(0.7, 0.0, 0.0),
        (_, true, true,  _) => shape.rgb(0.0, 0.8, 0.0),
        (_, true, false, _) => shape.rgb(0.8, 0.0, 0.0),
        (_, _, true,  _) => shape.rgb(0.0, 1.0, 0.0),
        (_, _, false, _) => shape.rgb(1.0, 0.0, 0.0)
    };
    colored.fill(backend);
}
