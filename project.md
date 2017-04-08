# Doppler Visualization

### Owen Stenson

## Overview 

The cosmos provide an interesting candidate for visualization. It encompasses a wide variety of the phenomena, many of which involve spectacular color or immense scales. Animation is an interesting opportunity to simulate interaction between such objects. I've been interested in exploring both visualization and simulation in the Rust programming language, for which this project is a good opportunity.

The most complete product to come of this endeavor was a visualization tool for the Doppler effect.

## Capabilities

This program displays a star as if it were close by and immobile but renders the color as if it were moving relative to the camera. As the star moves toward the camera (between 12 o'clock and 6 o'clock on the legend) its wavelength is compressed and is blueshifted; as it moves away (6 o'clock to 12 o'clock) the wavelength is rarefied and redshifted.

## Default Values

By default, the star orbit's radius is 2.6 * 10^20 and its duration is 225 million terrestrial years, giving it a speed of about 230 km/s. These values represent our Sun's orbit around Sagittarius A*. However, the effect is somewhat slight; a speedup factor (of 10, by default) is used to increase the speed of revolution, magnifying the Doppler effect. 

## Demo

A demo program (built for 64-bit linux and windows) can be found [here](https://github.com/stensonowen/sgc_visualization/raw/master/binaries.zip).

A demo gif can be found [here](https://raw.githubusercontent.com/stensonowen/sgc_visualization/master/doppler/default.gif).

Full code and more information can be found [on github](https://github.com/stensonowen/sgc_visualization).
