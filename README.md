# ProjectInertia
This is an open-source project to build a low-cost Commercial-Off-The-Shelf on-board computer for small CubeSat missions. At this point in time it is mostly a playground to test out new technologies and learn from it. The first hardware prototype is currently in production.

[<img src="https://rawgit.com/chrismolli/ProjectInertia/main/figures/banner.png" align="right" width="256">](https://github.com/chrismolli/ProjectInertia)
--

## Table of Contents
* [Commercial-Off-The-Shelf in Space](#commercial-off-the-shelf-in-space)
* [Hardware Development](#hardware-development)
* [Firmware Development](#firmware-development)

## Commercial-Off-The-Shelf in Space
TODO

## Hardware Development
The design is centered around the series of Ferroelectric-RAM based Microcontroller Units from Texas Instruments. The first prototype is using the MSP430FR6972IPMR and features 16 kB of radiation-tolerant non-volatile memory, a circuit-breaker for latch-up protection as well as a current measurement section for power monitoring. The current prototype provides access to two UART ports as well as an I2C interface. The Ferro-electric RAM cell is robust against various Single Event Effects.

## Firmware Development
The firmware is being developed using [Embedded Rust](http://www.rust-embedded.org) to explore the versatility of this language in a high-reliability environment. The following compenents will need to be developed. Useful resources might be obtained from the [awesome embedded Rust list](https://github.com/rust-embedded/awesome-embedded-rust).

### Peripheral Access Crate
The PAC has been succesfully created for the chip used in prototype 1.  
[`msp430fr6972`](https://crates.io/crates/msp430fr6972) (beta) ![Crates.io](https://img.shields.io/crates/v/msp430fr6972)

### Hardware Abstraction Layer
TODO

### Radiation-Hardened Software Library
TODO

### Application Layer
TODO
