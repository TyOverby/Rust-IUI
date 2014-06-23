#![crate_id = "iui"]

extern crate graphics;
extern crate piston;

use std::mem::swap;
use std::num::abs;
use std::default::Default;
use std::intrinsics::TypeId;
use std::any::{Any, AnyRefExt};
use std::collections::hashmap::HashMap;

use graphics::{Context, BackEnd, Draw, ImageSize};
use piston::{GameEvent, MouseMove};

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

impl ClippedRectangle {
    pub fn contains(&self, pos: (f64, f64)) -> bool {
        self.bounds.contains(pos) && self.clipping_box.contains(pos)
    }
}

pub fn raw_rect(x: f64, y: f64, w: f64, h: f64) -> ClippedRectangle {
    ClippedRectangle {
        bounds: Rectangle { x: x, y: y, w: w, h:h },
        clipping_box: Rectangle { x: x, y: y, w: w, h:h }
    }
}

impl Rectangle {
    pub fn contains(&self, pos: (f64, f64)) -> bool {
        let (x, y) = pos;
        if x < self.x || y < self.y {
            return false;
        }
        if x > self.x + self.w || y > self.y + self.h {
            return false;
        }
        return true;
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        (abs(self.x - other.x) * 2.0 < (self.w + other.w)) &&
        (abs(self.y - other.y) * 2.0 < (self.h + other.h))
    }
}


pub struct UiContext {
    pub hot: Option<&'static str>,
    pub active: Option<&'static str>,
    stored: HashMap<(TypeId, &'static str), Box<Any>>,

    pub mouse_pos: (f64, f64),

    event_queue: Vec<GameEvent<'static>>,
    pub active_events: Vec<GameEvent<'static>>
}

pub struct FrameUiContext<'a, 'b, 'c, I, B> {
    ui_ctx: &'a mut UiContext,
    draw_ctx: &'b Context<'b>,
    backend: &'c mut B,
}

pub trait Component<R>  {
    fn id(&self) -> &'static str;

    fn draw<I: ImageSize, B: BackEnd<I>>
        (&self, clip_box: ClippedRectangle, ui_ctx: &UiContext, ctx: &Context, backend: &mut B);
    fn act(&self, clip_box: ClippedRectangle, ui_context: &mut UiContext) -> R;
}

pub trait ComponentFromState<R>: Component<R> {
    fn from_state(&mut self, state: &R);
}

impl UiContext {
    pub fn new() -> UiContext {
        UiContext {
            hot: None,
            active: None,
            stored: HashMap::new(),
            event_queue: Vec::new(),
            active_events: Vec::new(),
            mouse_pos: (0.0, 0.0)
        }
    }

    pub fn with_graphics<'a, 'b, 'c, I: ImageSize, B: BackEnd<I>>
        (&'a mut self, draw_ctx: &'b Context, backend: &'c mut B) -> FrameUiContext<'a, 'b, 'c, I, B> {
        swap(&mut self.event_queue, &mut self.active_events);
        self.event_queue.clear();
        let ret = FrameUiContext {
            ui_ctx: self,
            draw_ctx: draw_ctx,
            backend: backend
        };
        ret
    }

    pub fn add_event(&mut self, event: &GameEvent) {
        match event.to_sendable() {
            Some(MouseMove(args)) => {
                self.mouse_pos = (args.x, args.y);
                self.event_queue.push(MouseMove(args));
            }
            Some(e) => self.event_queue.push(e),
            None => {}
        }
    }
}

impl <'a, 'b, 'c, I: ImageSize, B: BackEnd<I>> FrameUiContext<'a, 'b, 'c, I, B> {
    pub fn with<R, C: Component<R>>(&mut self, component: C, clipping: ClippedRectangle) -> R {
        let ret = component.act(clipping, self.ui_ctx);
        component.draw(clipping, self.ui_ctx, self.draw_ctx, self.backend);
        ret
    }

    pub fn with_stored<R: Default + 'static, C: ComponentFromState<R> + 'static>
        (&'a mut self, component: C, clipping: ClippedRectangle)  -> &'a R {
        let id = component.id();
        let mut component = component;
        let key = (TypeId::of::<C>(), id);

        match self.ui_ctx.stored.find(&key) {
            Some(state) => match state.as_ref::<R>() {
                Some(a) => component.from_state(a),
                None => {
                    let r: &R = &Default::default();
                    component.from_state(r)
                }
            },
            None => {
                let r: &R = &Default::default();
                component.from_state(r)
            }
        };

        let result = self.with(component, clipping);
        self.ui_ctx.stored.insert(key, box result as Box<Any>);
        let value = self.ui_ctx.stored.get_mut(&key);
        value.as_ref::<R>().unwrap()
    }
}
