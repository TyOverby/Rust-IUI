extern crate graphics;
extern crate piston;

use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Fill,
    BackEnd,
    Draw,
    Texture,
    Gl
};

use piston::{
    Game,
    RenderArgs,
    UpdateArgs,
    AssetStore,
    GameWindow,
    GameWindowSettings,
    GameIteratorSettings,
    GameWindowSDL2,
    GameEvent,
    Render,
    Update
};

use lib::{
    UiContext,
    Component,
    ClippedRectangle,
    raw_rect
};

use button::button;
use button::toggle_button;

mod lib;
mod button;

struct App {
    gl: Gl,
    ctx: UiContext
}

impl Game for App {
    fn render(&mut self, args: &mut RenderArgs) {
        self.gl.clear_rgba(0.0,0.0,0.0,1.0);

        let c = &Context::abs(args.width as f64, args.height as f64);
        let mut rend_ctx = self.ctx.with_graphics(c, &mut self.gl);

        if *rend_ctx.with_stored(toggle_button("_hi3"), raw_rect(0.0,120.0, 50.0,50.0))
        {
            if rend_ctx.with(
                 button("_hi"),
                 raw_rect(0.0,0.0, 50.0,50.0)) {

                println!("hello 1");
            }

            if rend_ctx.with(button("_hi2"), raw_rect(0.0,60.0, 50.0,50.0)) {
                println!("hello 2");
            }
        }
    }
    fn event(&mut self, event: &mut GameEvent) {
        match *event {
            Render(ref mut args) => self.render(args),
            Update(ref mut args) => self.update(args),
            other => self.ctx.add_event(&other)
        }
    }
}

fn main() {
    let mut game_window = GameWindowSDL2::new(GameWindowSettings {
        title: "Hello World".to_string(),
        size: [200, 400],
        fullscreen: false,
        exit_on_esc: true
    });

    let mut asset_store = AssetStore::empty();
    let mut app = App {
        gl: Gl::new(),
        ctx: UiContext::new()
    };
    app.run(&mut game_window, &mut asset_store, &GameIteratorSettings {
        updates_per_second: 60,
        max_frames_per_second: 60
    });
}
