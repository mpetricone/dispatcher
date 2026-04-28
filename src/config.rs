use crate::action_profile::ActionProfile;
use crate::file_io;
use crate::normalize::Normalizer;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub profile_path: String,
    pub model_path: String,
    pub default_profile: String,
    pub default_profile_name: String,
    pub default_model: String,
    pub default_dispatcher_config: DispatcherConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatcherConfig {
    pub default_command_delay: u32,
}

impl Default for DispatcherConfig {
    fn default() -> Self {
        Self {
            default_command_delay: 300,
        }
    }
}

/// Trait for types that can be serialized to and deserialized from a file using a [Config].
pub trait FilesFromConfig<T> {
    fn to_file(&mut self, config: &Config) -> Result<(), Box<dyn std::error::Error>>;
    fn from_file(name: &str, config: &Config) -> Result<T, Box<dyn std::error::Error>>;
    /// Returns the expected file extension for this type.
    /// it is meant to be used internally, but there is no harm in other uses.
    fn file_extension() -> &'static str;
}

impl Normalizer for Config {
    /// Nothing needed.
    fn normalize(&mut self) -> &mut Self {
        self
    }
}

// Todo: refactor using trait FilesFromConfig.
impl Config {
    /// Creates a configuration by loading filesfrom the configuration directory.
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
        let mut conf;
        if config_file.exists() {
            conf = file_io::from_file(&config_file.to_string_lossy())?
        } else {
            let mut default_profile = profile_path.clone();
            default_profile.push("default.pro");
            conf = Config {
                profile_path: profile_path.to_string_lossy().to_string(),
                model_path: model_path.to_string_lossy().to_string(),
                default_profile: default_profile.to_string_lossy().to_string(),
                default_profile_name: "default".to_string(),
                default_model: "vosk-model-small-en-us-0.15".to_string(),
                default_dispatcher_config: DispatcherConfig::default(),
            };

            file_io::to_file(&config_file.to_string_lossy(), false, &mut conf)?;
        }
        if !PathBuf::from(&conf.default_profile).exists() {
            file_io::to_file(
                &conf.default_profile,
                false,
                &mut ActionProfile::new(vec![], &conf.default_profile_name),
            )?;
        }
        Ok(conf)
    }

    pub fn model_with_path(&self, model: &str) -> String {
        let mut path = PathBuf::new();
        path.push(&self.model_path);
        path.push(model);
        path.to_string_lossy().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //Pretty simple, make a config, try to save something.
    #[test]
    fn test_config_file_roundtrip() {
        let config =
            Config::build().expect("Test Failed, have you run the install.sh script first?");
        // we use action profile since it is the first fully serializable struct
        let mut profile = ActionProfile::new(vec![], "test");
        profile.to_file(&config).unwrap();
        let profile2 = ActionProfile::from_file("test", &config).unwrap();
        //The vecs are empty, not bothering.
        //action_record has fp members, so Eq is out.
        assert_eq!(profile2.name, profile.name);
        std::fs::remove_file(format!(
            "{}/test{}",
            config.profile_path,
            ActionProfile::file_extension()
        ))
        .unwrap();
    }
}
