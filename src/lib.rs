#![crate_id = "iui"]

extern crate graphics;

use std::num::abs;
use std::default::Default;
use std::intrinsics::TypeId;
use std::any::{Any, AnyRefExt};
use std::collections::hashmap::HashMap;

use graphics::{Context, BackEnd, Draw, Image};

pub use button;

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64
}

pub struct ClippedRectangle {
    pub bounds: Rectangle,
    pub clipping_box: Rectangle
}

pub fn raw_rect(x: f64, y: f64, w: f64, h: f64) -> ClippedRectangle {
    ClippedRectangle {
        bounds: Rectangle { x: x, y: y, w: w, h:h },
        clipping_box: Rectangle { x: x, y: y, w: w, h:h }
    }
}

impl Rectangle {
    fn contains(&self, x: f64, y: f64) -> bool {
        if x < self.x || y < self.y {
            return false;
        }
        if x > self.x + self.w || y > self.y + self.h {
            return false;
        }
        return true;
    }

    fn intersects(&self, other: &Rectangle) -> bool {
        (abs(self.x - other.x) * 2.0 < (self.w + other.w)) &&
        (abs(self.y - other.y) * 2.0 < (self.h + other.h))
    }
}


pub struct UiContext {
    hot: Option<&'static str>,
    active: Option<&'static str>,
    stored: HashMap<(TypeId, &'static str), Box<Any>>
}

pub struct TempUiContext<'a, 'b, 'c, I, B> {
    ui_ctx: &'a mut UiContext,
    draw_ctx: &'b Context<'b>,
    backend: &'c mut B
}

pub trait Component<R>  {
    fn id(&self) -> &'static str;

    // TODO: `draw()` needs a UiContext.
    fn draw<I: Image, B: BackEnd<I>>(&self, clip_box: ClippedRectangle, ctx: &Context, backend: &mut B);
    fn act(&self, ui_context: &mut UiContext) -> R;
}

pub trait ComponentFromState<R>: Component<R> {
    fn from_state(id: &'static str, state: &R) -> Self;
}

impl UiContext {
    pub fn new() -> UiContext {
        UiContext {
            hot: None,
            active: None,
            stored: HashMap::new()
        }
    }

    pub fn with_graphics<'a, 'b, 'c, I: Image, B: BackEnd<I>>
        (&'a mut self, draw_ctx: &'b Context, backend: &'c mut B) -> TempUiContext<'a, 'b, 'c, I, B> {
        TempUiContext {
            ui_ctx: self,
            draw_ctx: draw_ctx,
            backend: backend
        }
    }
}

impl <'a, 'b, 'c, I: Image, B: BackEnd<I>> TempUiContext<'a, 'b, 'c, I, B> {
    pub fn with<R, C: Component<R>>(&mut self, component: C, clipping: ClippedRectangle) -> R {
        component.draw(clipping, self.draw_ctx, self.backend);
        component.act(self.ui_ctx)
    }
    pub fn with_stored<R: Default + 'static, C: ComponentFromState<R> + 'static>
        (&'a mut self, id: &'static str, clipping: ClippedRectangle)  -> &'a R {
        let key = (TypeId::of::<C>(), id);
        let component: C = match self.ui_ctx.stored.find(&key) {
            Some(state) => match state.as_ref::<R>() {
                Some(a) => ComponentFromState::from_state(id, a),
                None => {
                    let r: &R = &Default::default();
                    ComponentFromState::from_state(id, r)
                }
            },
            None => {
                let r: &R = &Default::default();
                ComponentFromState::from_state(id, r)
            }
        };

        let result = self.with(component, clipping);
        self.ui_ctx.stored.insert(key, box result as Box<Any>);
        let value = self.ui_ctx.stored.get_mut(&key);
        value.as_ref::<R>().unwrap()
    }
}

