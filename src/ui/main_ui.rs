use crate::action_profile::ActionProfile;
use crate::file_io;
use crate::ui::modal_dialog::ModalDialog;
use iced::Element;
use iced::widget::{button, row, text, toggler};
use serde::{Deserialize, Serialize};

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
            file_io::from_file(&config_file.to_string_lossy())
        } else {
            let empty_profile = ActionProfile::new(vec![], "default");
            let mut default_profile = profile_path.clone();
            default_profile.push("default.pro");
            file_io::to_file(&default_profile.to_string_lossy(), false, empty_profile)?;
            let conf = Config {
                profile_path: profile_path.to_string_lossy().to_string(),
                model_path: model_path.to_string_lossy().to_string(),
                default_profile: default_profile.to_string_lossy().to_string(),
                default_model: model_path.to_string_lossy().to_string(),
            };
            file_io::to_file(&config_file.to_string_lossy(), false, &conf)?;
            Ok(conf)
        }
    }
}

#[derive(Default)]
pub struct MainUIState {
    active_profile: Option<ActionProfile>, // Define fields here
    is_recording: bool,
    config: Option<Config>,
    modal_dialog: Option<ModalDialog<MainUIMessage>>,
}

#[derive(Debug, Clone, Copy)]
pub enum MainUIMessage {
    ToggleRecording(bool),
    SelectProfile,
    EditProfile,
    ModalAffirmative,
    ModalNegative,
}

impl MainUIState {
    pub fn new() -> Self {
        let mut modal_dialog = ModalDialog::new(
            "Initialization Error",
            "Could not find a configuration. Have you run the installer?",
            MainUIMessage::ModalAffirmative,
            MainUIMessage::ModalNegative,
            false,
        );
        if let Ok(config) = Config::build() {
            let active_profile = file_io::from_file(&config.default_profile);
            MainUIState {
                active_profile: active_profile.ok(),
                is_recording: false,
                config: Some(config),
                modal_dialog: Some(modal_dialog),
            }
        } else {
            modal_dialog.show = true;
            MainUIState {
                active_profile: None,
                is_recording: false,
                config: None,
                modal_dialog: Some(modal_dialog),
            }
        }
    }

    pub fn view(&self) -> Element<'_, MainUIMessage> {
        let profile_name = match &self.active_profile {
            Some(profile) => &profile.name,
            None => "No profile loaded",
        };

        let window = row![
            text(profile_name).width(600),
            button("Select profile").on_press(MainUIMessage::SelectProfile),
            toggler(self.is_recording)
                .on_toggle(MainUIMessage::ToggleRecording)
                .label("Toggle Listening"),
        ]
        .spacing(20);
        if let Some(dialog) = &self.modal_dialog {
            dialog.apply(window.into())
        } else {
            window.into()
        }
    }

    pub fn update(&mut self, message: MainUIMessage) {
        match message {
            MainUIMessage::ToggleRecording(_) => {
                if self.active_profile.is_some() {
                    self.is_recording = !self.is_recording;
                } else {
                    if let Some(diag) = &mut self.modal_dialog {
                        diag.show(true);
                    }
                    self.is_recording = false;
                    //TODO some type of popup
                };
            }
            MainUIMessage::SelectProfile => {
                // Implement profile selection logic
            }
            MainUIMessage::EditProfile => {
                // Implement profile editing logic
            }
            MainUIMessage::ModalAffirmative | MainUIMessage::ModalNegative => {
                if let Some(dialog) = &mut self.modal_dialog {
                    dialog.show(false);
                }
            }
        }
    }
}
