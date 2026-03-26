use crate::action_record::ActionRecord;
use crate::config::{Config, FilesFromConfig};
use crate::file_io;
use crate::normalize::Normalizer;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

/// ActionProfile is essentially a list of any
/// events dispatcher listens for, and the actions
/// associated with that event. It will also store some profile
/// settings.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct ActionProfile {
    pub actions: Vec<ActionRecord>,
    pub name: String,
}

impl std::fmt::Display for ActionProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}
impl Normalizer for ActionProfile {
    fn normalize(&mut self) -> &mut Self {
        for action in &mut self.actions {
            action.normalize();
        }
        self
    }
}

impl FilesFromConfig<ActionProfile> for ActionProfile {
    fn to_file(&mut self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let path: PathBuf = [
            &config.profile_path,
            &format!("{}{}", self.name, ActionProfile::file_extension()),
        ]
        .iter()
        .collect();
        file_io::to_file(&path.to_string_lossy(), true, self)
    }

    fn from_file(name: &str, config: &Config) -> Result<ActionProfile, Box<dyn std::error::Error>> {
        let path: PathBuf = [
            &config.profile_path,
            &format!("{}{}", name, ActionProfile::file_extension()),
        ]
        .iter()
        .collect();
        file_io::from_file(&path.to_string_lossy())
    }

    fn file_extension() -> &'static str {
        ".pro"
    }
}

impl ActionProfile {
    pub fn new(actions: Vec<ActionRecord>, name: &str) -> ActionProfile {
        ActionProfile {
            actions,
            name: name.to_string(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_io;
    #[test]
    fn test_serialze_profile() -> Result<(), Box<dyn Error>> {
        let mut ap1 = ActionProfile::new(vec![], "Test Profile");
        file_io::to_file("target/debug/testprofile1.pro", true, &mut ap1)?;
        let ap2 = file_io::from_file("target/debug/testprofile1.pro")?;
        assert_eq!(ap1, ap2);
        Ok(())
    }
}
