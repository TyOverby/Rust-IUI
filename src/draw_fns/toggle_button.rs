use super::super::components::toggle_button::ToggleButtonDrawState;
use super::super::ClippedRectangle;

use graphics::Context;
use graphics::BackEnd;
use graphics::ImageSize;

pub fn draw_toggle_button<I: ImageSize, B: BackEnd<I>>
    (clip_box: ClippedRectangle,
     draw_state: ToggleButtonDrawState,
     ctx: &Context,
     backend: &mut B) {

}
