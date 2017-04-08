# SGC Visualization

### Owen Stenson
### April 7, 2017

## Overview

The cosmos provide an interesting candidate for visualization. It encompasses a wide variety of the phenomena, many of which involve spectacular color or immense scales. Animation is also an interesting opportunity to simulate interaction between objects (gravitational attraction, collision, EM interference, chemical/nuclear reactions, etc.). I've been interested in exploring both visualization and simulation in the Rust programming language, for which this project is a good opportunity.

## Usage

```
./doppler -h

The Doppler Effect of our Sun 
Owen Stenson
Visualize the changing wavelength of the Sun as it orbits

USAGE:
    doppler [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -x <width>                               Set the window width
    -y <height>                              Set the window height
        --theta-increment <increment>        Change the granularity at which the visualization (not the star) moves
        --orbit-duration <orbit_duration>    Set a custom orbit duration (in seconds)
        --orbit-radius <orbit_radius>        Set a custom orbit radius (in meters)
        --orbit-velocity <orbit_velocity>    Manually set the star's velocity (in meters per second)
        --speedup-factor <speedup>           Speed up the star's speed by a constant to make the effect more visible
        --color <true_color>                 Set the true wavelength of the star (in nanometers)
```

## Building

Rust is required to build this project from source; it can be acquired [here](https://rustup.rs).

From there you can build with `cargo run --release`


## Why Rust

Rust is not commonly used for data crunching or game development, but in the future it might be. Rust makes it easy to cleanly interface with external libraries, build for different architectures, and write high level code that is still very fast. It also provides strong safety guarantees, which help to verify data integrity and security.

# Doppler Effect Visualizer
This program lets you visualize the Doppler effect by showing the color of a star as if you were not part of its orbit but showing it as if it were not moving. The result is a a circle which you can infer is moving because its color changes slightly. By default, the star's orbital radius is about 2.6 * 10<sup>20</sup> meters (can be tweaked with `--orbit-radius`) and its orbit's period is 225 million terrestrial years (`--orbit-duration`); these approximately model our Sun's orbit around Sagittarius A*. 
The effect for this example is a bit hard to notice, but it can be magnified with `--speedup-factor`, which multiplied the velocity of the object by a scalar to make the effect more noticeable.

Due to the Doppler Effect, when the star is moving toward the camera its wavelength is compressed and it is blueshifted; this happens when the dial is between 12 o'clock and 6 o'clock (and peaks at 3 o'clock). Then, from 6 o'clock to 12 o'clock, the star moves away from the camera, to the other end of its orbit, and appears redshifted because its wavelength is rarefied.

The legend in the upper left indicates where the star is in its orbit; the line shows where the star is from the point of view "above" the orbit (orthogonal to its motion). 6 o'clock is closest to the camera, 12 o'clock is farthest. 

## Color Adjustment
One challenge I did not anticipate was color representation: to compute the doppler shift its wavelength must be known, but to print it to the screen its red/green/blue/alpha values must be known. RGB are not realy related to the light's wavelength, and cannot really represent constructive interference properly, so it is non-trivial to convert between them. I implemented the pseudocode [here](http://www.efg2.com/Lab/ScienceAndEngineering/Spectra.htm) to add this functionality.

## Demo

![Default command lines](/doppler/default.gif)

Run with default command line args. Terminal output:
```
Starting visualization...
The star's orbital radius is approximately 2*10^20 m
The star's orbit has a duration of approximately 7*10^15 s
NOTE: The star's velocity is multiplied by 10 to magnify the effect
The star is moving at approximately 2*10^6 m/s
The star's true wavelength is 590 nm
Animating at 628.32 frames per orbit revolution
```


