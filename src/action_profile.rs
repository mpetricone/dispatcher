use crate::action_record::ActionRecord;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;

/// ActionProfile is essentially a list of any
/// events dispatcher listens for, and the actions
/// associated with that event. It will also store some profile
/// settings.
#[derive(Serialize, Deserialize, Clone)]
pub struct ActionProfile {
    pub actions: Vec<ActionRecord>,
    pub name: String,
}

impl ActionProfile {
    pub fn new(actions: Vec<ActionRecord>, name: String) -> ActionProfile {
        ActionProfile { actions, name }
    }

    /// Adds an Action record to profile, by recording keyboard and mouse events.
    /// essentially adds an [ActionRecord]
    pub fn add_action(
        &mut self,
        name: String,
        activator_text: String,
    ) -> Result<(), Box<dyn Error>> {
        let new_r = ActionRecord::build(name, activator_text, Duration::from_secs(10))?;
        self.actions.push(new_r);
        Ok(())
    }
}
