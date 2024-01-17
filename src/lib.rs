//! Traits for display controllers
//!
//! This crate contains traits that represent display controllers with a
//! memory-backed framebuffer. These controllers are typically integrated into a
//! System on Chip (SoC) and are used to drive larger / higher performance
//! displays.
//!
//! Note that this crate is *not* aimed at smaller displays that are updated by
//! sending commands over a low-speed databus (SPI, I2C, ..). These typically
//! have individual driver crates (some examples are listed
//! [here](https://github.com/embedded-graphics/embedded-graphics#display-drivers)).
//! Also note that this crate does not cover functionality available through the
//! panel's own line drivers or management controller. This crate only covers
//! the controller in the SoC.
//!
//! This crate allows libraries for operating on display layers to be written
//! independently of the exact controller architecture.
//!
//! ## License
//!
//! Licensed under either of
//!
//!  * Apache License, Version 2.0
//!    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//!  * MIT license
//!    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally
//! submitted for inclusion in the work by you, as defined in the Apache-2.0
//! license, shall be dual licensed as above, without any additional terms or
//! conditions.
#![no_std]

pub mod dsi;

/// A word type for the display memory buffer
pub trait PixelWord: Copy {}
impl PixelWord for u8 {}
impl PixelWord for u16 {}
impl PixelWord for u32 {}

/// Pixel memory layouts
///
/// * `L8`: 8-bit luminance or CLUT
/// * `AL44`: 4-bit alpha + 4-bit luminance
/// * `AL88`: 8-bit alpha + 8-bit luminance
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PixelFormat {
    ARGB8888 = 0,
    RGB888,
    RGB565,
    ARGB1555,
    ARGB4444,
    L8,
    AL44,
    AL88,
}

/// Display configuration parameters
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DisplayConfiguration {
    pub active_width: u16,
    pub active_height: u16,

    pub h_back_porch: u16,
    pub h_front_porch: u16,
    pub v_back_porch: u16,
    pub v_front_porch: u16,

    pub h_sync: u16,
    pub v_sync: u16,

    /// horizontal synchronization: `false`: active low, `true`: active high
    pub h_sync_pol: bool,
    /// vertical synchronization: `false`: active low, `true`: active high
    pub v_sync_pol: bool,
    /// data enable: `false`: active low, `true`: active high
    pub not_data_enable_pol: bool,
    /// pixel_clock: `false`: active low, `true`: active high
    pub pixel_clock_pol: bool,
}

/// A microcontroller peripheral that drives a LCD-TFT display
pub trait DisplayController {
    /// Initialize the controller with a given configuration
    fn init(&mut self, config: DisplayConfiguration);

    /// Returns the clock frequency (Hz) of the controller
    fn clock(&self) -> u32;
}

/// A layer of a microcontroller peripheral that drives a LCD-TFT display.
///
/// May be implemented alongside `DisplayController` if the LCD-TFT display
/// peripheral only supports one layer.
pub trait DisplayControllerLayer {
    /// Enable this display layer.
    ///
    /// # Safety
    ///
    /// [To be completed by implementation]
    unsafe fn enable<T: PixelWord>(
        &mut self,
        start_ptr: *const T,
        pixel_format: PixelFormat,
    );

    /// Swap the framebuffer to a new one.
    ///
    /// # Safety
    ///
    /// `start_ptr` must point to a location that can be accessed by the LTDC
    /// peripheral, with sufficient length for the framebuffer.
    unsafe fn swap_framebuffer<T: PixelWord>(&mut self, start_ptr: *const T);

    /// Indicates that a framebuffer swap is pending. In this situation, memory
    /// we previously supplied to
    /// [`swap_framebuffer`](#method.swap_framebuffer), before the most recent
    /// call, is still owned by the display.
    fn is_swap_pending(&self) -> bool;

    /// Resizes the framebuffer pitch. This does not change the output window
    /// size. The shadow registers are reloaded immediately.
    ///
    /// The framebuffer pitch is the increment from the start of one line of
    /// pixels to the start of the next line.
    ///
    /// # Safety
    ///
    /// The caller must ensure that enough memory is allocated for the resulting
    /// framebuffer size
    unsafe fn resize_buffer_pitch(&mut self, width: u32);
}
