use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::clear;
use graphics::*;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use std::cmp::max;
use std::process::exit;

const SCREENX: f64 = 600.0;
const SCREENY: f64 = 600.0;

pub struct App {
    gl: GlGraphics,
    screen: (f64, f64),
    pos_enemy: (f64, f64),
    pos_aim: (f64, f64),
}

fn sigmoid(x: f64) -> f64 {
    (1.0 / (1.0 + std::f64::consts::E.powf(-x)))
}

fn random_point(max_width: f64, max_heigth: f64) -> (f64, f64) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0.0, max_width), rng.gen_range(0.0, max_heigth))
}

#[inline]
fn distancev(point_a: (f64, f64), point_b: (f64, f64)) -> (f64, f64) {
    (point_b.0 - point_a.0, point_b.1 - point_a.1)
}

#[inline]
fn magnitude(point_a: (f64, f64)) -> f64 {
    (point_a.1.powf(2.0) + point_a.0.powf(2.0)).sqrt()
}

#[inline]
fn unit_vector(v: (f64, f64)) -> (f64, f64) {
    let m = magnitude(v);
    (v.0/m, v.1/m)
}

impl App {
    fn render(&mut self, args: &RenderArgs, e: (f64, f64), a: (f64, f64)) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let enemy = ellipse::circle(0.0, 0.0, 2.0);
        let aim = ellipse::circle(0.0, 0.0, 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let t_enemy = c
                .transform
                .trans(e.0, e.1);

            let t_aim = c
                .transform
                .trans(a.0, a.1);

            // Draw the enemy and the aim
            ellipse(RED, enemy, t_enemy, gl);
            ellipse(BLUE, aim, t_aim, gl);
        });
    }

    fn update(&mut self) {
        let v = distancev(self.pos_aim, self.pos_enemy);
        let vn = unit_vector(v);

        println!("{:?} - {:?}", vn.0, vn.1);

        // Add vn to the current position of aim to get closer
        self.pos_aim.0 += vn.0; // * sigmoid(v.0);
        self.pos_aim.1 += vn.1; // * sigmoid(v.1);

        if v.0 < 2.0 && v.1 < 2.0 {
            self.pos_enemy = random_point(self.screen.0, self.screen.1);
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("aime", [SCREENX, SCREENY])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        screen: (SCREENX, SCREENY),
        pos_enemy: random_point(SCREENX, SCREENY),
        pos_aim: random_point(SCREENX, SCREENY),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, app.pos_enemy, app.pos_aim);
        }

        if let Some(args) = e.update_args() {
            app.update();
        }
    }
}