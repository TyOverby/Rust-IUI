#![crate_id = "iui"]

extern crate graphics;
extern crate piston;

use std::mem::swap;
use std::num::abs;
use std::default::Default;
use std::intrinsics::TypeId;
use std::any::{Any, AnyRefExt};
use std::collections::hashmap::HashMap;
use std::f64::NEG_INFINITY;

use graphics::{Context, BackEnd, Draw, ImageSize};
use piston::{GameEvent, MouseMove, MousePress, MouseRelease};

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
    pub mouse_over: Option<&'static str>,
    pub mouse_down: Option<&'static str>,
    pub mouse_was_down: Option<&'static str>,
    pub mouse_up: Option<&'static str>,

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

pub trait AddState<R>: Component<R> {
    fn add_state(&mut self, state: &R);
}
pub trait WithoutState<R>: Component<R> {
    fn without_state(id: &'static str) -> Self;
}

impl UiContext {
    pub fn new() -> UiContext {
        UiContext {
            mouse_over: None,
            mouse_down: None,
            mouse_was_down: None,
            mouse_up: None,

            stored: HashMap::new(),
            event_queue: Vec::new(),
            active_events: Vec::new(),
            mouse_pos: (NEG_INFINITY, NEG_INFINITY)
        }
    }

    pub fn with_graphics<'a, 'b, 'c, I: ImageSize, B: BackEnd<I>>
        (&'a mut self, draw_ctx: &'b Context, backend: &'c mut B) -> FrameUiContext<'a, 'b, 'c, I, B> {
        swap(&mut self.event_queue, &mut self.active_events);

        self.mouse_was_down = self.mouse_down;
        self.mouse_over = None;
        self.mouse_down = None;
        self.mouse_up = None;

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

    pub fn forget(&mut self, id: &'static str) {
        let copy_key = {
            let mut keys = self.stored.keys();
            let found_key = keys.find(|pair| {
                match **pair {
                (_, i_id) if id == i_id => true,
                    _ => false
                }
            });
            match found_key {
                Some(key) => *key,
                None => return
            }
        };

        self.stored.remove(&copy_key);
    }
}

fn contains_pressed(events :&Vec<GameEvent>) -> bool{
    events.iter().any(|e| {
        match e {
            &MousePress(_) => true,
            _ => false
        }
    })
}

fn contains_released(events :&Vec<GameEvent>) -> bool{
    events.iter().any(|e| {
        match e {
            &MouseRelease(_) => true,
            _ => false
        }
    })
}

impl <'a, 'b, 'c, I: ImageSize, B: BackEnd<I>> FrameUiContext<'a, 'b, 'c, I, B> {
    pub fn with<R, C: Component<R>>(&mut self, component: C, clipping: ClippedRectangle) -> R {
        let id = Some(component.id());
        if clipping.contains(self.ui_ctx.mouse_pos) {
            self.ui_ctx.mouse_over = id;
            if self.ui_ctx.mouse_was_down == id ||
               contains_pressed(&self.ui_ctx.active_events) {
                self.ui_ctx.mouse_down = id;
                self.ui_ctx.mouse_was_down = id;
            }
            if contains_released(&self.ui_ctx.active_events) {
                self.ui_ctx.mouse_up = id;
                self.ui_ctx.mouse_down = None;
            }
        }

        let ret = component.act(clipping, self.ui_ctx);
        component.draw(clipping, self.ui_ctx, self.draw_ctx, self.backend);
        ret
    }

    pub fn with_stored<'x, R: Default + 'static, C: AddState<R> + 'static>
        (&'x mut self, component: C, clipping: ClippedRectangle)  -> &'x R {
        let id = component.id();
        let mut component = component;
        let key = (TypeId::of::<C>(), id);

        match self.ui_ctx.stored.find(&key) {
            Some(state) => match state.as_ref::<R>() {
                Some(a) => component.add_state(a),
                None => {
                    let r: &R = &Default::default();
                    component.add_state(r)
                }
            },
            None => {
                let r: &R = &Default::default();
                component.add_state(r)
            }
        };

        let result = self.with(component, clipping);
        self.ui_ctx.stored.insert(key, box result as Box<Any>);
        let value = self.ui_ctx.stored.get_mut(&key);
        value.as_ref::<R>().unwrap()
    }
}
