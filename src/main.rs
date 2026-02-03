use dispatcher::ui::main_ui::MainUIState;
use dispatcher::file_io;
use dispatcher::action_profile::ActionProfile;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    profile_path: String,
    model_path: String,
    default_profile: String,
    default_model: String,
}

impl Config {
    fn build() -> Result<Config, Box<dyn std::error::Error>> {
        let mut config_path = dirs::config_dir().ok_or("Cannot find a general config directory")?;
        config_path.push("dispatcher");
        if !config_path.exists() {
            eprintln!("Missing config directory. Did you run the install script?");
            return Err("Missing config directory".into());
        }
        let mut profile_path = config_path.clone();
        profile_path.push("profiles");
        if !profile_path.exists() {
            eprintln!("Missing profile directory");
            return Err("Missing profile directory".into());
        }
        let mut model_path = dirs::data_local_dir().ok_or("Cannot find a local data directory")?;
        model_path.push("dispatcher");
        model_path.push("model");
        if !model_path.exists() {
            eprintln!("Missing model directory");
            return Err("Missing model directory".into());
        }
        let mut config_file = config_path.clone();
        config_file.push("dispatcher.json");
        if config_file.exists() {
            return file_io::from_file(&config_file.to_string_lossy().into_owned());
        } else {
            let empty_profile = ActionProfile::new(vec![],"default");
            let mut default_profile = profile_path.clone();
            default_profile.push("default.pro");
            file_io::to_file(&default_profile.to_string_lossy().into_owned(), false, empty_profile)?;
            let conf = Config {
                profile_path: profile_path.to_string_lossy().into_owned(),
                model_path: model_path.to_string_lossy().into_owned(),
                default_profile: default_profile.to_string_lossy().into_owned(),
                default_model: model_path.to_string_lossy().into_owned(),
            };
            file_io::to_file(&config_file.to_string_lossy().into_owned(), false, &conf)?;
            Ok(conf)
        }
    }
}

/// Currently a testbed
fn main()  -> iced::Result {
    let _config = Config::build().expect("Failed to build config");
    iced::run(MainUIState::update, MainUIState::view)
}
