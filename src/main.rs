#[macro_use]
extern crate lazy_static;
mod board;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;

use piston::window::WindowSettings;

use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};

use crate::board::{
    Board, GameboardController, GameboardControllerSettings, GameboardView, GameboardViewSettings,
};
use opengl_graphics::{GlGraphics, OpenGL};

fn main() {
    let opengl = OpenGL::V3_2;

    let settings = WindowSettings::new("Lifestyle", [1920, 1080])
        .graphics_api(opengl)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build().expect("Couldn't create window");
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    let gameboard = Board::new();
    let gameboard_controller_settings = GameboardControllerSettings::new();
    let mut gameboard_controller =
        GameboardController::new(gameboard, gameboard_controller_settings);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(&e);

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0; 4], g);
                gameboard_view.draw(&gameboard_controller, &c, g);
            });
        }
        if let Some(args) = e.update_args() {
            gameboard_controller.update(&args);
        }
    }
    println!("{}", settings.get_exit_on_esc());
}
