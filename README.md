# ProjectInertia
This is an open-source project to build a low-cost Commercial-Off-The-Shelf on-board computer for small CubeSat missions providing extremely low power usage and tolerance to radiation effects. At this point in time it is mostly a playground to test out new technologies and learn from it. The first hardware prototype ([iteration 1](https://github.com/chrismolli/ProjectInertia/tree/main/hardware/iteration%201)) is currently in production.

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
The design is centered around the series of Ferroelectric-RAM based Microcontroller Units from Texas Instruments. The first prototype is using the [MSP430FR6972IPMR](https://www.ti.com/store/ti/en/p/product/?p=MSP430FR6972IPMR).  

<b>You can find the schematics, gerber files and bill of materials in the [hardware folder](https://github.com/chrismolli/ProjectInertia/tree/main/hardware).</b>

### Features
The current hardware design (`iteration 1`) features the following:
- A FRAM based Microcontroller clocked at 16 MHz
  - 64 kB On-Chip FRAM memory for code and data
  - An internal watchdog circuit to recover from Funcional Interrupts
- An external 16 kB FRAM non-volatile memory
- Latch-Up Detection
  - Circuit Breaker implementation for <b>automatic Latch-Up Protection</b>
  - Current Monitoring
- 5V or 12V input power sources
- I/Os
  - 2 x UART
  - 1 x I<sup>2</sup>C
  - JTAG Programming Interface
  - Screw Terminals for all Interfaces
- CubeSat size
  - `1/4 U` or (`5cm x 5cm`)

## Firmware Development
The firmware is being developed using [Embedded Rust](http://www.rust-embedded.org) to explore the versatility of this language in a environment with extremely limited resources and the need for high reliability. Useful resources may be obtained from the [awesome embedded Rust list](https://github.com/rust-embedded/awesome-embedded-rust).

### Peripheral Access Crate
The PAC has been succesfully created for the MSP430FR6972 chip used in prototype number 1.  
- [`msp430fr6972`](https://crates.io/crates/msp430fr6972) (beta) ![Crates.io](https://img.shields.io/crates/v/msp430fr6972)

### Application layer
In the first version only a few features shall be tested.
- Telemetry & Telecommand handling on UART (TODO)
- Configuration Storage in external Memory (TODO)
- Current Monitoring (TODO)
