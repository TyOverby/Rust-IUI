#![crate_id = "iui"]

extern crate graphics;

use std::num::abs;
use std::default::Default;
use std::intrinsics::TypeId;
use std::any::{Any, AnyRefExt};

use std::collections::hashmap::HashMap;

use graphics::{Context, BackEnd};

pub struct Rectangle {
    x: f32,
    y: f32,
    w: f32,
    h: f32
}

impl Rectangle {
    fn contains(&self, x: f32, y: f32) -> bool {
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

pub struct ClippedRectangle {
    bounds: Rectangle,
    clipping_box: Rectangle
}

pub struct UiContext {
    hot: Option<&'static str>,
    active: Option<&'static str>,
    stored: HashMap<(TypeId, &'static str), Box<Any>>
}

pub struct TempUiContext<'a, 'b, 'c, BkEnd> {
    ui_ctx: &'a mut UiContext,
    draw_ctx: &'b Context<'b>,
    backend: &'c mut BkEnd
}

pub trait Component<R, I, BkEnd: BackEnd<I>>  {
    fn id(&self) -> &'static str;
    fn draw(&self, clip_box: ClippedRectangle, ctx: &Context, backend: &mut BkEnd);
    fn act(&self, ui_context: &mut UiContext) -> R;
}

pub trait ComponentFromState<R, I, BkEnd: BackEnd<I>>: Component<R, I, BkEnd> {
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

    pub fn with_graphics<'a, 'b, 'c, I, BkEnd: BackEnd<I>>
        (&'a mut self, draw_ctx: &'b Context, backend: &'c mut BkEnd) -> TempUiContext<'a, 'b, 'c, BkEnd> {
        TempUiContext {
            ui_ctx: self,
            draw_ctx: draw_ctx,
            backend: backend
        }
    }
}

impl <'a, 'b, 'c, I, BkEnd: BackEnd<I>> TempUiContext<'a, 'b, 'c, BkEnd> {
    pub fn with<R, C: Component<R, I, BkEnd>>(&mut self, component: C, clipping: ClippedRectangle) -> R {
        component.draw(clipping, self.draw_ctx, self.backend);
        component.act(self.ui_ctx)
    }
    pub fn with_stored<R: Default + 'static, C: ComponentFromState<R, I, BkEnd> + 'static>
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

fn main(){}
