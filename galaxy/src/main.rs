#[macro_use]
extern crate clap;
extern crate piston_window;
extern crate rand;

use piston_window::*;
use std::cmp;
use std::f64::consts::{self, PI};

struct Star {
    // NOTE: coords are relative to the center of the window
    color: [f32; 4],    // RGBα
    pos: (f64,f64),     // meters
    mass: f64,          // kilograms
    vel: (f64,f64),     // m / s
    force: (f64,f64),   // m / s^2
}

impl Star {
    fn draw<G: Graphics>(g: &mut G) {}
}

//fn draw_star<G: Graphics>(x: f64, y: f64, theta: f64, t: [[f64;3];2], g: &mut G) { }

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Galaxy Animation", [640, 480])
        .exit_on_esc(true).build().unwrap();

    while let Some(e) = window.next() {

        window.draw_2d(&e, |c, g| {
            clear(color::BLACK, g);
            //ellipse(color, size, c.transform, g);
            //draw_legend(LEGEND_X, LEGEND_Y, theta, c.transform, g);
        });
    }
}
