use std::f64;
use std::f64::consts::PI;

use crate::{map::Map, player::Player};
use glutin_window::GlutinWindow as Window;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    Button, EventSettings, Events, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent,
    UpdateArgs, UpdateEvent, WindowSettings,
};

pub struct Engine {
    window: Window,
    gl: GlGraphics,
    player: Player,
    key_states: KeyStates,
    map: Map,
}

struct KeyStates {
    w: bool,
    s: bool,
    a: bool,
    d: bool,
}

impl Engine {
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const BLACK: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const WIDTH: u32 = 1080;
    const HEIGHT: u32 = 600;

    pub fn new() -> Engine {
        let opengl = OpenGL::V3_2;
        let window: Window = WindowSettings::new("uaquax", [Self::WIDTH, Self::HEIGHT])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let initial_player_x = (Self::WIDTH as f64 - Player::SIZE) / 2.0;
        let initial_player_y = (Self::HEIGHT as f64 - Player::SIZE) / 2.0;

        Engine {
            window: window,
            gl: GlGraphics::new(opengl),
            player: Player::new([initial_player_x, initial_player_y], [1.0, 1.0]),
            key_states: KeyStates {
                w: false,
                s: false,
                a: false,
                d: false,
            },
            map: Map::new(),
        }
    }

    pub fn run(&mut self) {
        self.player.delta.x = f64::cos(self.player.angle * 5.0);
        self.player.delta.y = f64::sin(self.player.angle * 5.0);

        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(Button::Keyboard(key)) = e.press_args() {
                match key {
                    Key::W => self.key_states.w = true,
                    Key::D => self.key_states.d = true,
                    Key::S => self.key_states.s = true,
                    Key::A => self.key_states.a = true,
                    _ => {}
                }
            }

            if let Some(Button::Keyboard(key)) = e.release_args() {
                match key {
                    Key::W => self.key_states.w = false,
                    Key::D => self.key_states.d = false,
                    Key::S => self.key_states.s = false,
                    Key::A => self.key_states.a = false,
                    _ => {}
                }
            }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let cell_width = args.window_size[0] as f64 / self.map.size[0] as f64;
        let cell_height = args.window_size[1] as f64 / self.map.size[1] as f64;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(Self::BLACK, gl);

            /* ---- MAP ---- */
            for y in 0..self.map.size[1] {
                for x in 0..self.map.size[0] {
                    let index = (y * self.map.size[0] + x) as usize;
                    if self.map.map[index] == 1 {
                        let rect_x = x as f64 * cell_width;
                        let rect_y = y as f64 * cell_height;
                        let rect_width = cell_width;
                        let rect_height = cell_height;

                        rectangle(
                            Self::WHITE,
                            [rect_x, rect_y, rect_width, rect_height],
                            c.transform,
                            gl,
                        );
                    }
                }
            }

            /* ---- PLAYER ---- */
            let square = rectangle::square(0.0, 0.0, Player::SIZE);
            let rotation = c
                .transform
                .trans(
                    self.player.position.x + Player::SIZE / 2.0,
                    self.player.position.y + Player::SIZE / 2.0,
                )
                .rot_rad(self.player.angle)
                .trans(-Player::SIZE / 2.0, -Player::SIZE / 2.0);

            // let line_start_x = self.player.position.x + Player::SIZE / 2.0;
            // let line_start_y = self.player.position.y + Player::SIZE / 2.0;
            // let line_end_x = line_start_x + self.player.delta.x * 20.0;
            // let line_end_y = line_start_y + self.player.delta.y * 20.0;
            // line(
            //     Self::RED,
            //     2.0,
            //     [line_start_x, line_start_y, line_end_x, line_end_y],
            //     c.transform,
            //     gl,
            // );

            rectangle(Self::GREEN, square, rotation, gl);

            /* ---- RAY CASTING ---- */
            let rays_count = 60;
            let fov = 60.0_f64.to_radians();
            let start_angle = self.player.angle - (fov / 2.0);

            for i in 0..rays_count {
                let angle = start_angle + (i as f64 * fov / rays_count as f64);
                let mut ray_end_x = self.player.position.x;
                let mut ray_end_y = self.player.position.y;

                while ray_end_x >= 0.0
                    && ray_end_x <= Self::WIDTH as f64
                    && ray_end_y >= 0.0
                    && ray_end_y <= Self::HEIGHT as f64
                {
                    let map_x = (ray_end_x / cell_width) as usize;
                    let map_y = (ray_end_y / cell_height) as usize;
                    let index = map_y * self.map.size[0] as usize + map_x;

                    if self.map.map[index] == 1 {
                        break;
                    }

                    ray_end_x += f64::cos(angle) * 1.0;
                    ray_end_y += f64::sin(angle) * 1.0;
                }

                let distance = ((ray_end_x - self.player.position.x).powi(2)
                    + (ray_end_y - self.player.position.y).powi(2))
                .sqrt();
                let ray_width = (Self::WIDTH as f64 / 2.0) / distance;

                line(
                    Self::RED,
                    ray_width as f64,
                    [
                        self.player.position.x,
                        self.player.position.y,
                        ray_end_x,
                        ray_end_y,
                    ],
                    c.transform,
                    gl,
                );
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let speed = 200.0;

        if self.key_states.w {
            self.player.position.x += f64::cos(self.player.angle) * speed * args.dt;
            self.player.position.y += f64::sin(self.player.angle) * speed * args.dt;
        }

        if self.key_states.s {
            self.player.position.x -= f64::cos(self.player.angle) * speed * args.dt;
            self.player.position.y -= f64::sin(self.player.angle) * speed * args.dt;
        }

        if self.key_states.d {
            self.player.angle += 0.03;

            if self.player.angle > 2.0 * PI {
                self.player.angle -= 2.0 * PI;
            }

            self.player.delta.x = f64::cos(self.player.angle * 5.0);
            self.player.delta.y = f64::sin(self.player.angle * 5.0);
        }

        if self.key_states.a {
            self.player.angle -= 0.03;

            if self.player.angle < 0.0 {
                self.player.angle += 2.0 * PI;
            }

            self.player.delta.x = f64::cos(self.player.angle * 5.0);
            self.player.delta.y = f64::sin(self.player.angle * 5.0);
        }
    }
}
