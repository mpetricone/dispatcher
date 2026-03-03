use crate::action_profile::ActionProfile;
use crate::file_io;
use serde::{Deserialize, Serialize};

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub profile_path: String,
    pub model_path: String,
    pub default_profile: String,
    pub default_profile_name: String,
    pub default_model: String,
}

impl Config {
    /// Creates a configuration by loading files from the configuration directory.
    /// We use crate [dirs] to determine standard directories.
    pub fn build() -> Result<Config, Box<dyn std::error::Error>> {
        let mut config_path = dirs::config_dir().ok_or("Cannot find a general config directory")?;
        config_path.push("dispatcher");
        if !config_path.exists() {
            return Err("Missing config directory".into());
        }
        let mut profile_path = config_path.clone();
        profile_path.push("profiles");
        if !profile_path.exists() {
            return Err("Missing profile directory".into());
        }
        let mut model_path = dirs::data_local_dir().ok_or("Cannot find a local data directory")?;
        model_path.push("dispatcher");
        model_path.push("model");
        if !model_path.exists() {
            return Err("Missing model directory".into());
        }
        let mut config_file = config_path.clone();
        config_file.push("dispatcher.json");
        if config_file.exists() {
            file_io::from_file(&config_file.to_string_lossy())
        } else {
            let empty_profile = ActionProfile::new(vec![], "default");
            let mut default_profile = profile_path.clone();
            default_profile.push("default.pro");
            file_io::to_file(&default_profile.to_string_lossy(), false, &empty_profile)?;
            let conf = Config {
                profile_path: profile_path.to_string_lossy().to_string(),
                model_path: model_path.to_string_lossy().to_string(),
                default_profile: default_profile.to_string_lossy().to_string(),
                default_profile_name: empty_profile.name.clone(),
                default_model: model_path.to_string_lossy().to_string(),
            };
            file_io::to_file(&config_file.to_string_lossy(), false, &conf)?;
            Ok(conf)
        }
    }
}
