extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

pub mod engine;
pub mod map;
pub mod player;

fn main() {
    let mut engine = engine::Engine::new();
    engine.run();
}
