extern crate piston_window;
extern crate clap;

use piston_window::*;
use std::cmp;
use std::f64::consts::{self, PI};

mod convert_colors;
use convert_colors::wavelength_to_rgb;

const C: f64 = 299_792_458.;
const SPEED_FACTOR: f64 = 10.;  // slightly adjust things to make the effect more obvious

// the Sun is 10**20 meters from Sagittarius A*
const ORBIT_RADIUS: f64 = 2.6 * 100_000_000_000_000_000_000.; // meters

// 1 galactic year: 225 million terrestrial years (measured here in seconds)
const ORBIT_DURATION: f64 = 225. * 1000000. * 365.25 * 24. * 3600. / SPEED_FACTOR;

const ORBIT_SPEED: f64 = PI * 2.0 * ORBIT_RADIUS / ORBIT_DURATION;  // m/s

const INCREMENT: f64 = 0.01;    // how much each frame changes theta

const TRUE_COLOR: f64 = 590.;   // nanometers

const LEGEND_X: f64 = 75.0;
const LEGEND_Y: f64 = 75.0;

fn doppler(v: f64) -> f64 {
    // assume observer immobile w.r.t. medium
    // positive `v` means object is moving toward observer
    // velocity (input) measured in m/s
    // frequency (output) measured in Hz
    C * TRUE_COLOR / (C + v)
}

fn get_dimensions(s: Size) -> [f64; 4] {
    let Size{ width: w, height: h} = s;
    let radius = cmp::min(w, h) as f64 * 0.5;
    let x = w as f64 / 2.0;
    let y = h as f64 / 2.0;
    [x - radius/2.0, y - radius/2.0, radius, radius]
}

fn draw_legend<G: Graphics>(x: f64, y: f64, theta: f64, t: [[f64;3];2], g: &mut G) {
    let r1 = 100.0;
    let r2 = 110.0;
    let r = (r1 + r2) / 2.0;

    // draw the orbit
    ellipse(color::WHITE, [x-r2/2.,y-r2/2.,r2,r2], t, g);
    ellipse(color::BLACK, [x-r1/2.,y-r1/2.,r1,r1], t, g);

    // draw the star
    let (s,c) = (theta - consts::FRAC_PI_2).sin_cos();
    let (a,b) = (x + r*c/2., y + r*s/2.);
    line(color::WHITE, 2.5, [x,y,a,b], t, g);
}

fn main() {
    let args = clap::App::new("The Doppler Effect of our Sun")
        .author("Owen Stenson")
        .about("Visualize the changing wavelength of the Sun as it orbits")
        .arg(clap::Arg::with_name("orbit_radius")
             .long("orbit-radius")
             .takes_value(true)
             .help("Set a custom orbit radius (in meters)"))
        .arg(clap::Arg::with_name("orbit_duration")
             .long("orbit-duration")
             .takes_value(true)
             .help("Set a custom orbit duration (in seconds)"))
        .arg(clap::Arg::with_name("speedup")
             .long("speedup-factor")
             .takes_value(true)
             .help("Speed up the star's speed by a constant to make the effect more visible"))
        .arg(clap::Arg::with_name("increment")
             .long("theta-increment")
             .takes_value(true)
             .help("Change the granularity at which the visualization (not the star) moves"))
        .get_matches();

    let mut window: PistonWindow =
        WindowSettings::new("Doppler Effect of the Sun", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let size = get_dimensions(window.size());

    // TODO: Clap args to customize parameters
    // TODO: print statements indicating max change, speed, etc.
    // TODO: writeup, how to read, what it is, etc.

    // azimuthal angle
    // 0 is closest to camera, increases clockwise from x to y axis
    // measured in radians
    let mut theta = 0.0;
    // velocity in the x-direction
    // + is toward the camera, - is away from the camera
    // starts at 0, because entire velocity is in the y dimension
    let mut x_vel;
    let mut lambda;
    let mut color;

    while let Some(e) = window.next() {
        theta = (theta + INCREMENT) % (2.0 * PI);

        x_vel = - ORBIT_SPEED * theta.sin();
        lambda = doppler(x_vel);
        color = wavelength_to_rgb(lambda);

        window.draw_2d(&e, |c, g| {
            clear(color::BLACK, g);
            ellipse(color, size, c.transform, g);
            draw_legend(LEGEND_X, LEGEND_Y, theta, c.transform, g);
        });
    }
}

