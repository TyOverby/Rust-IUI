extern crate graphics;
extern crate piston;

use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Fill,
    BackEnd,
    Draw,
    Image
};

use piston::{
    Game,
    RenderArgs,
    UpdateArgs,
    AssetStore,
    GameWindow,
    GameWindowSettings,
    GameWindowSDL2,
    Texture,
    Gl
};

use lib::{
    UiContext,
    Component,
    ClippedRectangle,
    raw_rect
};

use button::Button;

mod lib;
mod button;

struct App {
    ctx: UiContext
}

impl Game for App {
    fn render(&mut self, c: &Context, args: &mut RenderArgs) {
        let mut rend_ctx = self.ctx.with_graphics(c, args.gl);
        rend_ctx.with(Button::new("hi"), raw_rect(0.0,0.0, 50.0,50.0));
        rend_ctx.with(Button::new("hi"), raw_rect(0.0,50.0, 50.0,50.0));
    }

    fn update(&mut self, args: &mut UpdateArgs) {

    }
}

fn main() {
    let mut game_window: GameWindowSDL2 = GameWindow::new(GameWindowSettings {
        title: "Hello World".to_string(),
        size: [200, 400],
        fullscreen: false,
        exit_on_esc: true,
        background_color: [0.0, 0.5, 0.0, 0.0]
    });

    let mut asset_store = AssetStore::empty();
    let mut app = App {
        ctx: UiContext::new()
    };
    app.run(&mut game_window, &mut asset_store);
}
