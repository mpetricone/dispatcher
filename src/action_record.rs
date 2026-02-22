use crate::input_recorder;
use crate::input_recorder::InputEvent;
use rdev;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::thread;
use std::time::Duration;
use std::fmt::Display;

/// # Everything needed to hold an action
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct ActionRecord {
    pub name: String,
    pub activator_text: String,
    pub action_stream: Vec<InputEvent>,
    // Placeholder Audio output
}

impl Display for ActionRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.name)
    }
}

impl ActionRecord {
    pub fn new(
        name: String,
        activator_text: String,
        action_stream: Vec<InputEvent>,
    ) -> ActionRecord {
        ActionRecord {
            name,
            activator_text,
            action_stream,
        }
    }

    /// Creates a functioning, normalized ActionRecord ready to be used or
    /// returns a string of [rdev::ListenError] .
    pub fn build(
        name: String,
        activator_text: String,
        capture_time: Duration,
    ) -> Result<ActionRecord, Box<dyn Error>> {
        let empty_vec = Vec::new();
        let mut record = ActionRecord {
            name,
            activator_text,
            action_stream: empty_vec,
        };
        record.capture_actions(capture_time)?;
        Ok(record)
    }

    /// Capture events from the keyboard
    /// or returns a string of [rdev::ListenError]
    ///
    /// This will overwrite existing events on success or leave them alone on failure
    pub fn capture_actions(&mut self, capture_time: Duration) -> Result<(), Box<dyn Error>> {
        match input_recorder::record_sequence() {
            Ok(raw_sequence) => {
                thread::sleep(capture_time);
                rdev::stop_listening();
                let normalized_events = input_recorder::normalize_sequence(raw_sequence)?;
                self.action_stream = normalized_events;
            }
            Err(e) => return Err(format!("{:?}", e).into()),
        }
        Ok(())
    }
}
