//! # Dispatcher
//! A Voice command dispatcher
//! ## Goals
//! To provide reliable voice commands to keyboard input for Linux
//! with audio feedback if desired.
//!
//! ## Start here
//! [input_dispatcher] and [input_recorder] are the only *mostly* working parts as of now.
//!
//! I've test vosk to ensure somewhat reliable voice detection.
//! ## Next Up
//! 1. More input testing
//! 2. Threading to enable simultaneous voice input/output, key detection and key simulation.
//! 3. Putting it togeter for functional use from a command line
//! 4. GUI
//! 5. Voice playback on events - might swap with with 4.
//!
//! ## Current Design Limitations
//! * Currently requires X11
//! * supports keyboard input. Mouse and gamepad may be added later.
//! * Currently only supports keyboard capture and simulation.
//! * Sorry Dispatcher does not support Wayland at this time. To keep the design simple, I decided to focus only on X11, since I still use it for reliability.
//!
pub mod input_recorder;
pub mod input_dispatcher;
