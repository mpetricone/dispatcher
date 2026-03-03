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
//! 1. GUI - [iced]
//! 2. Input recording via GUI
//! 3. Working voice detection command dispatching.
//!    ----- Usable at This point -----
//! 4. Voice playback on events - might swap with with 4.
//!
//! ## Current Design Limitations
//! * Currently requires X11
//! * supports keyboard input. Mouse and gamepad may be added later.
//! * Currently only supports keyboard capture and simulation.
//! * Sorry Dispatcher does not support Wayland at this time.
pub mod action_profile;
pub mod action_record;
pub mod config;
pub mod file_io;
pub mod input_dispatcher;
pub mod input_recorder;
pub mod primary_dispatcher;
pub mod ui;
pub mod voice_req;
