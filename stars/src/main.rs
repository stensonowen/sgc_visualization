#[macro_use] 
extern crate clap;
extern crate piston_window;

use piston_window::*;
use std::cmp;
use std::f64::consts::{self, PI};

mod convert_colors;
use convert_colors::wavelength_to_rgb;

const C: f64 = 299_792_458.;
const DEF_SPEED_FACTOR: f64 = 10.;  // slightly adjust things to make the effect more obvious

// the Sun is 10**20 meters from Sagittarius A*
const DEF_ORBIT_RADIUS: f64 = 2.6 * 100_000_000_000_000_000_000.; // meters
// 1 galactic year: 225 million terrestrial years (measured here in seconds)
const DEF_ORBIT_DURATION: f64 = 225. * 1000000. * 365.25 * 24. * 3600.; 

const DEF_INCREMENT: f64 = 0.01;    // how much each frame changes theta
const DEF_TRUE_COLOR: f64 = 590.;   // nanometers

const DEF_WIDTH: u32 = 640;
const DEF_HEIGHT: u32 = 480;
const LEGEND_X: f64 = 75.0;
const LEGEND_Y: f64 = 75.0;

fn doppler(v: f64, true_col: f64) -> f64 {
    // assume observer immobile w.r.t. medium
    // positive `v` means object is moving toward observer
    // velocity (input) measured in m/s
    // frequency (output) measured in Hz
    // original frequency can be measured in nm (conversion cancels)
    (C * (true_col) / (C + v))
}

fn get_dimensions(s: Size) -> [f64; 4] {
    let Size{ width: w, height: h} = s;
    let radius = cmp::min(w, h) as f64 * 0.5;
    let x = w as f64 / 2.0;
    let y = h as f64 / 2.0;
    [x - radius/2.0, y - radius/2.0, radius, radius]
}

fn sci_not(x: f64) -> (u8, u8) {
    let exponent = x.log10().trunc() as u8;
    let mantissa = x / 10f64.powi(exponent as i32);
    (mantissa as u8, exponent)
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
        .arg(clap::Arg::with_name("orbit_velocity")
             .long("orbit-velocity")
             .takes_value(true)
             .conflicts_with("orbit-radius")
             .conflicts_with("orbit-duration")
             .help("Manually set the star's velocity (in meters per second)"))
        .arg(clap::Arg::with_name("increment")
             .long("theta-increment")
             .takes_value(true)
             .help("Change the granularity at which the visualization (not the star) moves"))
        .arg(clap::Arg::with_name("width")
             .short("x")
             .takes_value(true)
             .help("Set the window width"))
        .arg(clap::Arg::with_name("height")
             .short("y")
             .takes_value(true)
             .help("Set the window height"))
        .arg(clap::Arg::with_name("true_color")
             .long("color")
             .takes_value(true)
             .help("Set the true wavelength of the star (in nanometers)"))
        .get_matches();

    println!("Starting visualization...");
    // speed object is moving (meters per second)
    let velocity = value_t!(args, "orbit_velocity", f64).unwrap_or_else(|_| {
        let r = value_t!(args, "orbit_radius", f64).unwrap_or(DEF_ORBIT_RADIUS);
        let t = value_t!(args, "orbit_duration", f64).unwrap_or(DEF_ORBIT_DURATION);
        let s = value_t!(args, "speedup", f64).unwrap_or(DEF_SPEED_FACTOR);
        let (r0,r1) = sci_not(r);
        let (t0,t1) = sci_not(t);
        println!("The star's orbital radius is approximately {}*10^{} m", r0, r1);
        println!("The star's orbit has a duration of approximately {}*10^{} s", t0, t1);
        println!("NOTE: The star's velocity is multiplied by {} to magnify the effect", s);
        2.0 * PI * r / t * s
    });
    let increment = value_t!(args, "increment", f64).unwrap_or(DEF_INCREMENT);
    let width = value_t!(args, "width", u32).unwrap_or(DEF_WIDTH);
    let height = value_t!(args, "height", u32).unwrap_or(DEF_HEIGHT);
    let orig_wl = value_t!(args, "true_wavelength", f64).unwrap_or(DEF_TRUE_COLOR);

    let mut window: PistonWindow =
        WindowSettings::new("Doppler Effect of the Sun", [width, height])
        .exit_on_esc(true).build().unwrap();

    let size = get_dimensions(window.size());

    let v = sci_not(velocity);
    println!("The star is moving at approximately {}*10^{} m/s", v.0, v.1);
    println!("The star's true wavelength is {} nm", orig_wl);
    println!("Animating at {:2.2} frames per orbit revolution", 2.0 * PI / increment);

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

