# [Documentation](https://docs.rs/embedded-display-controller)

# embedded-display-controller

[![docs.rs](https://docs.rs/embedded-display-controller/badge.svg)](https://docs.rs/embedded-display-controller)
[![Crates.io](https://img.shields.io/crates/v/embedded-display-controller.svg)](https://crates.io/crates/embedded-display-controller)

Traits for display controllers

This crate contains traits that represent display controllers with a
memory-backed framebuffer. These controllers are typically integrated into a
System on Chip (SoC) or connected via a memory bus. They are used to drive
larger / higher performance displays.

Note that this crate is *not* aimed at smaller displays that are updated by
sending commands over a low-speed databus (SPI, I2C, ..). These typically
have individual driver crates (some examples are listed
[here](https://github.com/embedded-graphics/embedded-graphics#display-drivers)).
Also note that this crate does not cover functionality available through the
panel's own line drivers or management controller. Only the controller in the SoC.

This crate allows libraries for operating on display layers to be written
independently of the exact controller architecture.
