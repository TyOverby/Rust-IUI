#![crate_id = "iui"]

// Crates
extern crate graphics;
extern crate piston;

// STD library
use std::mem::swap;
use std::default::Default;
use std::intrinsics::TypeId;
use std::any::{Any, AnyRefExt};
use std::collections::hashmap::HashMap;
use std::f64::NEG_INFINITY;

// External imports
use graphics::{Context, BackEnd, ImageSize};
use piston::{GameEvent, MouseMove, MousePress, MouseRelease};

// Internal exports
pub use clipping::{ClippedRectangle, Rectangle, raw_rect};

// Modules
mod clipping;
pub mod draw_fns;
pub mod components {
    pub mod button;
    pub mod toggle_button;
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

pub trait Component<R, D: Default>  {
    fn id(&self) -> &'static str;
    fn act(&self, clip_box: ClippedRectangle, ui_context: &mut UiContext, drawState: &mut D) -> R;
}

pub trait AddState<R, D>: Component<R, D> {
    fn add_state(&mut self, state: &R);
}
pub trait WithoutState<R, D>: Component<R, D> {
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
    pub fn with<R, D: Default, C: Component<R, D>>
        (&mut self,
         component: C,
         clipping: ClippedRectangle,
         drawfn: |clip_box: ClippedRectangle, draw_state: D, ctx: &Context, backend: &mut B|) -> R {

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

        let mut draw_state = Default::default();
        let ret = component.act(clipping, self.ui_ctx, &mut draw_state);
        drawfn(clipping, draw_state, self.draw_ctx, self.backend);
        ret
    }

    pub fn with_stored<'x, D: Default, R: Default + 'static, C: AddState<R, D> + 'static>
        (&'x mut self,
         component: C,
         clipping: ClippedRectangle,
         drawfn: |clip_box: ClippedRectangle, draw_state: D, ctx: &Context, backend: &mut B|)  -> &'x R {
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

        let result = self.with(component, clipping, drawfn);
        self.ui_ctx.stored.insert(key, box result as Box<Any>);
        let value = self.ui_ctx.stored.get_mut(&key);
        value.as_ref::<R>().unwrap()
    }
}
