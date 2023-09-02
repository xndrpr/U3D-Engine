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
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = 1080;

    pub fn new() -> Engine {
        let opengl = OpenGL::V3_2;
        let window: Window = WindowSettings::new("U3D Engine", [Self::WIDTH, Self::HEIGHT])
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
        let mini_map_size = 200.0;

        let map_scale = mini_map_size / Self::WIDTH as f64;

        let mini_map_x = 10.0;
        let mini_map_y = 10.0;
        let cell_width = args.window_size[0] as f64 / self.map.size[0] as f64;
        let cell_height = args.window_size[1] as f64 / self.map.size[1] as f64;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(Self::BLACK, gl);

            /* ---- MAP ---- */
            for y in 0..self.map.size[1] {
                for x in 0..self.map.size[0] {
                    let index = (y * self.map.size[0] + x) as usize;
                    if self.map.map[index] == 1 {
                        let rect_x = x as f64 * cell_width * map_scale + mini_map_x;
                        let rect_y = y as f64 * cell_height * map_scale + mini_map_y;
                        let rect_width = cell_width * map_scale;
                        let rect_height = cell_height * map_scale;

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
            let mini_player_size = Player::SIZE * map_scale;

            if self.player.position.x >= 0.0
                && self.player.position.x <= Self::WIDTH as f64
                && self.player.position.y >= 0.0
                && self.player.position.y <= Self::HEIGHT as f64
            {
                let player_x = self.player.position.x / self.map.size[0] as f64 * mini_map_size
                    + mini_map_x
                    - mini_player_size / 2.0;
                let player_y = self.player.position.y / self.map.size[1] as f64 * mini_map_size
                    + mini_map_y
                    - mini_player_size / 2.0;
                let square = rectangle::square(0.0, 0.0, mini_player_size);
                let rotation = c
                    .transform
                    .trans(
                        player_x + mini_player_size / 2.0,
                        player_y + mini_player_size / 2.0,
                    )
                    .rot_rad(self.player.angle)
                    .trans(-mini_player_size / 2.0, -mini_player_size / 2.0);

                rectangle(Self::GREEN, square, rotation, gl);
            }

            /* ---- RAY CASTING ---- */
            let rays_count = 300;
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

                let wall_height = (Self::HEIGHT as f64 / distance) * 300.0;
                let wall_width = Self::WIDTH as f64 / rays_count as f64;

                let wall_x = i as f64 * wall_width;
                let wall_top = (Self::HEIGHT as f64 - wall_height) / 2.0;

                let shading = 1.0 - (distance / Self::WIDTH as f64);

                rectangle(
                    [shading as f32, shading as f32, shading as f32, 1.0],
                    [wall_x, wall_top, wall_width, wall_height],
                    c.transform,
                    gl,
                );

                line(
                    Self::GREEN,
                    1.0,
                    [
                        self.player.position.x * map_scale,
                        self.player.position.y * map_scale,
                        ray_end_x * map_scale,
                        ray_end_y * map_scale,
                    ],
                    c.transform,
                    gl,
                );
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let speed = Self::WIDTH as f64 / 2.0;
        let angle_increment = 0.05;

        if self.key_states.w {
            self.player.position.x += f64::cos(self.player.angle) * speed * args.dt;
            self.player.position.y += f64::sin(self.player.angle) * speed * args.dt;
        }

        if self.key_states.s {
            self.player.position.x -= f64::cos(self.player.angle) * speed * args.dt;
            self.player.position.y -= f64::sin(self.player.angle) * speed * args.dt;
        }

        if self.key_states.d {
            self.player.angle += angle_increment;

            if self.player.angle > 2.0 * PI {
                self.player.angle -= 2.0 * PI;
            }

            self.player.delta.x = f64::cos(self.player.angle * 5.0);
            self.player.delta.y = f64::sin(self.player.angle * 5.0);
        }

        if self.key_states.a {
            self.player.angle -= angle_increment;

            if self.player.angle < 0.0 {
                self.player.angle += 2.0 * PI;
            }

            self.player.delta.x = f64::cos(self.player.angle * 5.0);
            self.player.delta.y = f64::sin(self.player.angle * 5.0);
        }
    }
}
