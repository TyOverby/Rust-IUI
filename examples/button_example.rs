extern crate graphics;
extern crate piston;
extern crate iui;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use graphics::{
    Context,
    BackEnd
};

use piston::{
    Game,
    RenderArgs,
    GameWindowSettings,
    GameIteratorSettings,
    GameEvent,
    Render,
    Update
};

use sdl2_game_window:: GameWindowSDL2;

use iui::{
    UiContext,
    raw_rect
};

use opengl_graphics::Gl;

use iui::components::button::button;
use iui::draw_fns::draw_button;

struct App {
    gl: Gl,
    ctx: UiContext
}

impl Game for App {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.clear_rgba(0.0,0.0,0.0,1.0);

        let c = &Context::abs(args.width as f64, args.height as f64);
        let mut rend_ctx = self.ctx.with_graphics(c, &mut self.gl);

        let btn1_pos = raw_rect(0.0,0.0, 50.0,50.0);
        let btn2_pos = raw_rect(0.0,60.0, 50.0,50.0);

        if rend_ctx.with(button("_btn_1"), btn1_pos, draw_button) {
            println!("Button 1 was pressed.");
        }

        if rend_ctx.with(button("_btn_2"), btn2_pos, draw_button) {
            println!("Button 2 was pressed.");
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

    let mut app = App {
        gl: Gl::new(),
        ctx: UiContext::new()
    };
    app.run(&mut game_window, &GameIteratorSettings {
        updates_per_second: 60,
        max_frames_per_second: 60
    });
}
