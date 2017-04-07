// Adapted from http://www.efg2.com/Lab/ScienceAndEngineering/Spectra.htm
// Convert from a wavelength of light to RGB representation
// Surprisingly complicated / interesting

const GAMMA: f64 = 0.80;
//const MAX_INTENSIty: u8 = 255;
const MAX_INTENSITY: f64 = 255.;

pub type Wavelength = f64;
pub struct RGB(f64, f64, f64);

impl RGB {
    /*fn normalize(self) -> (f64, f64, f64) {
        // change from being between 0 and 255 to between 0 and 1
        (self.0 / 255., self.1 / 255., self.2 / 255.)
    }*/
    fn gfx_coloring(self) -> [f32; 4] {
        [(self.0 / 255.) as f32, 
            (self.1 / 255.) as f32, 
            (self.2 / 255.) as f32, 
            1.0]
    }
}

fn adjust(val: f64, factor: f64) -> f64 {
    MAX_INTENSITY * (val*factor).powf(GAMMA)
}

pub fn wavelength_to_rgb(l: Wavelength) -> [f32; 4] {
    let (r,g,b) = match l as u16 {
        380 ... 439 => ((-(l-440.)/(440.-380.)), 0., 1.),
        440 ... 489 => (0., ((l-440.)/(490.-440.)), 1.),
        490 ... 509 => (0., 1., (-(l-510.)/(510.-490.))),
        510 ... 579 => (((l-510.)/(580.-510.)), 1., 0.),
        580 ... 644 => (1., (-(l-645.)/(645.-580.)), 0.),
        645 ... 780 => (1., 0., 0.),
        //_ => panic!("Wavelength {} out of bounds", l),
        _ => (0.,0.,0.),
    };
    let f = match l as u16 {
        380 ... 419 => 0.3 + 0.7*(l-380.) / (320. - 380.),
        420 ... 700 => 1.0,
        701 ... 780 => 0.3 + 0.7*(780. - l) / (780. - 700.),
        //_ => panic!("Wavelength {} out of bounds", l),
        _ => 0.0,
    };
    RGB(adjust(r, f), adjust(g, f), adjust(b, f)).gfx_coloring()
}

