# SGC Visualization

### Owen Stenson
### April 7, 2017

## Overview

The cosmos provide an interesting candidate for visualization. It encompasses a wide variety of the phenomena, many of which involve spectacular color or immense scales. Animation is also an interesting opportunity to simulate interaction between objects (gravitational attraction, collision, EM interference, chemical/nuclear reactions, etc.). I've been interested in exploring both visualization and simulation in the Rust programming language, for which this project is a good opportunity.

## Why Rust
Rust is not commonly used for data crunching or game development, but in the future it might be. Rust makes it easy to cleanly interface with external libraries, build for different architectures, and write high level code that is still very fast. It also provides strong safety guarantees, which help to verify data integrity and security.

# Doppler Effect Visualizer
This program lets you visualize the Doppler effect by showing the color of a star as if you were not part of its orbit but showing it as if it were not moving. The result is a a circle which you can infer is moving because its color changes slightly. By default, the star's orbital radius is about 2.6 * 10<sup>20</sup> meters (can be tweaked with `--orbit-radius`) and its orbit's period is 225 million terrestrial years (`--orbit-duration`); these approximately model our Sun's orbit around Sagittarius A*. 
The effect for this example is a bit hard to notice, but it can be magnified with `--speedup-factor`, which multiplied the velocity of the object by a scalar to make the effect more noticeable.

Due to the Doppler Effect, when the star is moving toward the camera its wavelength is compressed and it is blueshifted; this happens when the dial is between 12 o'clock and 6 o'clock (and peaks at 3 o'clock). Then, from 6 o'clock to 12 o'clock, the star moves away from the camera, to the other end of its orbit, and appears redshifted because its wavelength is rarefied.



