extern crate graphics;
extern crate piston;

use graphics::{Context,
    AddRectangle,
    AddColor,
    Fill
};

use piston::{
    Game,
    RenderArgs,
    UpdateArgs,
    AssetStore,
    GameWindow,
    GameWindowSettings,
    GameWindowSDL2
};

struct Bx{
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    vx: f64,
    vy: f64
}
struct App {
    bx: Bx,
    screen_w: f64,
    screen_h: f64
}

mod lib;

impl Game for App {
    fn render(&self, c: &Context, args: &mut RenderArgs) {
        c.rect(self.bx.x, self.bx.y, self.bx.w, self.bx.h).rgb(1.0, 0.0, 0.0).fill(args.gl);
    }

    fn update(&mut self, args: &mut UpdateArgs) {
        let dt = args.dt;
        self.bx.x += self.bx.vx * dt;
        self.bx.y += self.bx.vy * dt;

        self.bx.vy += 9.8 * dt;

        if self.bx.x < 0.0 {
            self.bx.x = 0.0;
            self.bx.vx = - self.bx.vx;
        } else if self.bx.x + self.bx.w > self.screen_w {
            self.bx.x = self.screen_w - self.bx.w - 0.01;
            self.bx.vx = - self.bx.vx / 2.0;
            // friction
            self.bx.vy = self.bx.vy / 1.01;
        }

        if self.bx.y < 0.0 {
            self.bx.y = 0.0;
            self.bx.vy = - self.bx.vy;
        } else if self.bx.y + self.bx.h > self.screen_h {
            self.bx.y = self.screen_h - self.bx.h - 0.01;
            self.bx.vy = - self.bx.vy / 2.0;
            // friction
            self.bx.vx = self.bx.vx / 1.01;
        }
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
        bx: Bx {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
            vx: 40.0,
            vy: 40.0
        },
        screen_w: 200.0,
        screen_h: 400.0
    };
    app.run(&mut game_window, &mut asset_store);
}
