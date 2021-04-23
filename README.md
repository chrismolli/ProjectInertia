# ProjectInertia
This is an open-source project to build a low-cost Commercial-Off-The-Shelf on-board computer for small CubeSat missions. At this point in time it is mostly a playground to test out new technologies and learn from it. The first hardware prototype (`iteration 1`) is currently in production.

[<img src="https://github.com/chrismolli/ProjectInertia/raw/main/figures/banner.png" align="right" width="256">](https://github.com/chrismolli/ProjectInertia)

## Table of Contents
* [Commercial-Off-The-Shelf in Space](#commercial-off-the-shelf-in-space)
* [Hardware Development](#hardware-development)
  * [Features](#features)
* [Firmware Development](#firmware-development)
  * [Peripheral Access Crate](#peripheral-access-crate)

## Commercial-Off-The-Shelf in Space
The Ferro-electric RAM cell is robust against various Single Event Effects.

TODO

## Hardware Development
The design is centered around the series of Ferroelectric-RAM based Microcontroller Units from Texas Instruments. The first prototype is using the MSP430FR6972IPMR. You can find the schematics, gerber files and bill of materials in the hardware folder (TODO).

### Features
The current hardware design (`iteration 1`) features the following:
- A FRAM based MCU clocked at 16 MHz
- 64 kB On-Chip FRAM memory for code and data
- An external 16 kB FRAM non-volatile memory
- Circuit Breaker implementation for automatic latch-up protection
- Current Monitoring
- 5V or 12V input power sources
- 2 x UART
- 1 x I<sup>2</sup>C
- Screw Terminals for all Interfaces
- 1/4 U CubeSat size (5cm x 5cm)

## Firmware Development
The firmware is being developed using [Embedded Rust](http://www.rust-embedded.org) to explore the versatility of this language in a environment with extremely limited resources and the need for high reliability. Useful resources may be obtained from the [awesome embedded Rust list](https://github.com/rust-embedded/awesome-embedded-rust).

### Peripheral Access Crate
The PAC has been succesfully created for the MSP430FR6972 chip used in prototype number 1.  
[`msp430fr6972`](https://crates.io/crates/msp430fr6972) (beta) ![Crates.io](https://img.shields.io/crates/v/msp430fr6972)

### Application layer
In the first version only a few features shall be tested.
- Telemetry & Telecommand handling on UART (TODO)
- Configuration Storage in external Memory (TODO)
- Current Monitoring (TODO)
