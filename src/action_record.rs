use crate::input_recorder;
use crate::input_recorder::InputEvent;
use crate::normalize::Normalizer;
use rdev;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

/// # Everything needed to hold an action
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct ActionRecord {
    pub name: String,
    pub activator_text: String,
    pub action_stream: Vec<InputEvent>,
    pub completion_audio_path: AudioPath,
    // Placeholder Audio output
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct AudioPath {
    pub audio_path: Option<PathBuf>,
    pub audio_file: Option<PathBuf>,
}

pub struct ActionRecordStreamFormatted<'a>(pub &'a ActionRecord);

impl Normalizer for ActionRecord {
    fn normalize(&mut self) -> &mut Self {
        self.activator_text = self.activator_text.to_lowercase();
        self
    }
}

impl<'a> Display for ActionRecordStreamFormatted<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .action_stream
                .iter()
                .fold(String::new(), |acc, x| acc + &x.to_string() + " ")
        )
    }
}

impl Display for ActionRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ActionRecord {
    pub fn new(
        name: &str,
        activator_text: &str,
        action_stream: Vec<InputEvent>,
        completion_audio_path: AudioPath,
    ) -> ActionRecord {
        ActionRecord {
            name: name.to_string(),
            activator_text: activator_text.to_string(),
            action_stream,
            completion_audio_path,
        }
    }

    /// Creates a functioning, normalized ActionRecord ready to be used or
    /// returns a string of [rdev::ListenError] .
    pub fn build(
        name: String,
        activator_text: String,
        capture_time: Duration,
        completion_audio_path: AudioPath,
    ) -> Result<ActionRecord, Box<dyn Error>> {
        let empty_vec = Vec::new();
        let mut record = ActionRecord {
            name,
            activator_text,
            action_stream: empty_vec,
            completion_audio_path,
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
