//! A library for writing audio to audio devices.
//!
//! You can regenerate the bindings for the library using:
//!
//! ```bash
//! cargo run --bin rotary-device-bindings
//! ```

#[cfg(windows)]
pub mod wasapi;

#[cfg(windows)]
mod bindings;
