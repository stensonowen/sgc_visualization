#[macro_use]
extern crate clap;
extern crate piston_window;

// kilograms: 25 * 10 **
const STAR_MASS_MIN: f64 = 25. * 1_000_000_000_000_000_000_000_000_000.0;
const STAR_MASS_MAX: f64 = 315. * 1_000_000_000_000_000_000_000_000_000_000.0;
// window dimensions (pixels)
const WINDOW_W: f64 = 640.0;
const WINDOW_H: f64 = 480.0;
// window scale ??
const SPACE_W: f64 = 1_000_000_000_000_000_000f64;
const SPACE_H: f64 = 1_000_000_000_000_000_000f64;
// multiply [0,640] or [0,480] by these to get [0,SPACE_W]
const SCALE_W: f64 = SPACE_W / WINDOW_W;
const SCALE_H: f64 = SPACE_H / WINDOW_H;

// 8 * 10^36 kilograms
const CENTER_MASS: f64 = 8_570_000_000_000_000_000_000_000_000_000_000_000.0;

const UNIV_GRAV_CONST: f64 = 0.000000000066740831f64;


use piston_window::*;
use std::cmp;
use std::f64::consts::{self, PI};

mod sta;
use sta::Star;

//fn draw_star<G: Graphics>(x: f64, y: f64, theta: f64, t: [[f64;3];2], g: &mut G) { }

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Galaxy Animation", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let mut v: Vec<Star> = vec![];
    for _ in 0 .. 100 {
        v.push(Star::new());
    }

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear(color::BLACK, g);
            for s in &v {
                s.draw(c.transform, g);
            }
            //ellipse(color, size, c.transform, g);
            //draw_legend(LEGEND_X, LEGEND_Y, theta, c.transform, g);
        });
    }
}
