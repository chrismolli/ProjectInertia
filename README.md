# ProjectInertia
This is an open-source project to build a low-cost Commercial-Off-The-Shelf on-board computer for small CubeSat missions providing extremely low power usage and tolerance to radiation effects. At this point in time it is mostly a playground to test out new technologies and learn from it. The first hardware prototype [`iteration 1`](https://github.com/chrismolli/ProjectInertia/tree/main/hardware/iteration%201) is currently in production.

[<img src="https://github.com/chrismolli/ProjectInertia/raw/main/figures/banner.png" align="right" width="256">](https://github.com/chrismolli/ProjectInertia)

## Table of Contents
* [Commercial-Off-The-Shelf in Space](#commercial-off-the-shelf-in-space)
* [Hardware Development](#hardware-development)
  * [Features](#features)
  * [Manufacturing](#manufacturing)
* [Firmware Development](#firmware-development)
  * [Peripheral Access Crate](#peripheral-access-crate)
  * [Toolchain](#toolchain)

## Commercial-Off-The-Shelf in Space
The usage of Commercial-Off-The-Shelf (COTS) components in Space has been a topic of research for many years already due to the benefits of higher performance and lower costs, when compared to their radiation hardened counterparts.

However, COTS components are subject to a manifold of radiation induced failure modes, that need to be considered during system design [[Sandia2018](https://www.osti.gov/servlets/purl/1481565)]. Destructive Latch-Up events are automatically detected and mitigated by an on-board circuit breaker. After a hold-down time of some dozen µs, the board is reconnected to the power source. This project proposes the use of a COTS microcontroller that is based on Ferro-Electric RAM. The Ferro-Electric RAM cell itself is resilient against various Single Event Effects [[Fetahovic2017](https://www.researchgate.net/publication/322940214_Overview_of_radiation_effects_on_emerging_non-volatile_memory_technologies)]. However, the peripherals within the respective integrated circuit might still experience Single Event Effects [[Bosser2018](https://www.osti.gov/servlets/purl/1483658)]. These will be recovered by the internal watchdog circuit, present in the microcontroller hardware.

Developing hard- and software products for Space application do require a lot of careful design choices. Thus, this project aims for the most simple and very low cost design to minimize potential sources for failures.

## Hardware Development
The design is centered around the series of Ferroelectric-RAM based Microcontroller Units from Texas Instruments. The first prototype is using the [MSP430FR6972IPMR](https://www.ti.com/store/ti/en/p/product/?p=MSP430FR6972IPMR).  

### Features
The current hardware design ([`iteration 1`](https://github.com/chrismolli/ProjectInertia/tree/main/hardware/iteration%201)) features the following:
- Microcontroller based on FRAM technology clocked at `16 MHz`
  - `64 kB` On-Chip Memory for Code and Data
  - Internal Hardware Watchdog to recover from Functional Interrupts
  - Availability of Ultra Low-Power Modes
- External `16 kB` Non-Volatile FRAM Memory
- Automatic Latch-Up Detection
  - Circuit Breaker limits at `1500 mA`
  - Current Monitoring
- `12V` Power Input
  -  Optional `5V` Input for Development
- Input/Output
  - 2 x UART
  - 1 x I<sup>2</sup>C
  - JTAG Programming Interface
  - `3.3V` output for peripherals
  - Screw Terminals for all Interfaces
- CubeSat Size
  - `1/4 U` or (`5cm x 5cm`)

<p align="center">
  <img src="https://github.com/chrismolli/ProjectInertia/raw/main/figures/flatview.png" width="500">
</p>

### Manufacturing
You can find the schematics, gerber files and bill of materials in the [hardware folder](https://github.com/chrismolli/ProjectInertia/tree/main/hardware).

## Firmware Development
The firmware is being developed using [Embedded Rust](http://www.rust-embedded.org) to explore the versatility of this language in a environment with extremely limited resources and the need for high reliability. Useful resources may be obtained from the [awesome embedded Rust list](https://github.com/rust-embedded/awesome-embedded-rust).

### Peripheral Access Crate
The PAC has been successfully created for the MSP430FR6972 chip used in prototype [`iteration 1`](https://github.com/chrismolli/ProjectInertia/tree/main/hardware/iteration%201).  
- [`msp430fr6972`](https://crates.io/crates/msp430fr6972) (beta) ![Crates.io](https://img.shields.io/crates/v/msp430fr6972)

### Toolchain
The current toolchain uses:
- Rust `Edition 2018` (nightly)
- [`msp430-gcc-elf`](https://www.ti.com/tool/MSP430-GCC-OPENSOURCE) compiler
- [`msp430-bsl-python`](https://github.com/chrismolli/msp430-bsl-python) programmer
