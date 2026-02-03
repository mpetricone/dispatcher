//! # Dispatcher
//! A Voice command dispatcher
//! ## Goals
//! To provide reliable voice commands to keyboard input for Linux
//! with audio feedback if desired.
//!
//! ## Start here
//! KB IO, voice detection working seperatly, began work on integrating those parts through [primary_dispatcher] and [action_profile].
//!
//! Please note I use <https://github.com/mpetricone/rdev> which is a fork of <https://github.com/Narsil/rdev>
//! The original did not have a way to end input capture that I could find.
//!
//! I've test vosk to ensure somewhat reliable voice detection.
//! ## Next Up
//! 1. Threading to enable simultaneous voice input/output, key detection and key simulation.
//!     * work started
//! 2. Putting it togeter for functional use from a command line
//!     * work started
//! 3. More input testing
//! 4. GUI (leaning towards iced)
//! 5. Voice playback on events - might swap with with 4.
//!
//! ## Current Design Limitations
//! * Currently requires X11
//! * supports keyboard input. Mouse and gamepad may be added later.
//! * Currently only supports keyboard capture and simulation.
//! * Sorry Dispatcher does not support Wayland at this time. To keep the design simple, I decided to focus only on X11, since I still use it for reliability.
//!
pub mod action_profile;
pub mod action_record;
pub mod file_io;
pub mod input_dispatcher;
pub mod input_recorder;
pub mod primary_dispatcher;
pub mod voice_req;
pub mod ui;
