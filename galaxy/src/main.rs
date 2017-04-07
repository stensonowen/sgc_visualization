#[macro_use]
extern crate clap;
extern crate piston_window;

use piston_window::*;
use std::cmp;
use std::f64::consts::{self, PI};

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Galaxy Animation", [width, height])
        .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        theta = (theta + increment) % (2.0 * PI);

        x_vel = - velocity * theta.sin();
        lambda = doppler(x_vel, orig_wl);
        color = wavelength_to_rgb(lambda);

        window.draw_2d(&e, |c, g| {
            clear(color::BLACK, g);
            ellipse(color, size, c.transform, g);
            draw_legend(LEGEND_X, LEGEND_Y, theta, c.transform, g);
        });
    }
}
