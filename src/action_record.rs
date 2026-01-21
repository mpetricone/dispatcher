use crate::input_recorder::InputEvent;
use serde::{Serialize, Deserialize};

/// # Everything needed to hold an action
#[derive(Serialize, Deserialize)]
pub struct ActionRecord {
    pub name: String,
    pub activator_text: String,
    pub action_stream: Vec<InputEvent>,
    // Placeholder Audio output
}

impl ActionRecord {
    pub fn new(name: String, activator_text: String, action_stream: Vec<InputEvent>) -> ActionRecord {
        return ActionRecord { name, activator_text, action_stream}
    }
}
