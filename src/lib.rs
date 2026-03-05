//! # Dispatcher
//! A Voice command dispatcher
//!
//! ## Goals
//! To provide reliable voice commands to keyboard input for Linux
//! with audio feedback if desired.
//!
//! ## Current State
//! Working on GUI, profile management, and serialization.
//!
//! ## Next Up
//! 1. Working voice detection command dispatching.
//! 2. Voice playback on events.
//!
//! ## Current Design Limitations
//! * Currently requires X11
//! * supports keyboard input. Mouse and gamepad may be added later.
pub mod action_profile;
pub mod action_record;
pub mod config;
pub mod file_io;
pub mod input_dispatcher;
pub mod input_recorder;
pub mod primary_dispatcher;
pub mod ui;
pub mod voice_req;
