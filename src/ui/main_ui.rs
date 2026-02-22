/// * Main UI and application configuration.
use crate::action_profile::ActionProfile;
use crate::file_io;
use crate::file_io::from_file;
use crate::ui::modal_dialog::ModalDialog;
use iced::Element;
use iced::widget::combo_box::State;
use iced::widget::{button, column, combo_box, container, row, toggler};
use serde::{Deserialize, Serialize};
use std::fs::read_dir;

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    profile_path: String,
    model_path: String,
    default_profile: String,
    default_profile_name: String,
    default_model: String,
}

impl Config {
    /// Creates a configuration by loading files from the configuration directory.
    /// We use crate [dirs] to determine standard directories.
    fn build() -> Result<Config, Box<dyn std::error::Error>> {
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

#[derive(Default)]
pub struct MainUIState {
    active_profile: Option<ActionProfile>, // Define fields here
    is_recording: bool,
    config: Option<Config>,
    modal_dialog: Option<ModalDialog<MainUIMessage>>,
    //profiles: Vec<ActionProfile>,
    selected_profile: Option<ActionProfile>,
    combo_profiles: combo_box::State<ActionProfile>,
}

#[derive(Debug, Clone)]
pub enum MainUIMessage {
    ToggleRecording(bool),
    SelectProfile(ActionProfile),
    EditProfile,
    ModalAffirmative,
    ModalNegative,
}

pub enum MainUIAction {
    EditProfile(ActionProfile),
    NewProfile(ActionProfile),
    None,
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
            let mut working_state = MainUIState {
                active_profile: active_profile.ok(),
                is_recording: false,
                config: Some(config),
                modal_dialog: Some(modal_dialog),
                selected_profile: None,
                combo_profiles: combo_box::State::new(vec![]),
            };
            working_state.load_profiles();
            working_state
        } else {
            modal_dialog.show = true;
            MainUIState {
                active_profile: None,
                is_recording: false,
                config: None,
                modal_dialog: Some(modal_dialog),
                selected_profile: None,
                combo_profiles: combo_box::State::new(vec![]),
            }
        }
    }

    /// Load profiles from the configuration directory.
    /// Also sets default profile
    fn load_profiles(&mut self) {
        if let Some(config) = &self.config {
            let mut loaded_profiles: Vec<ActionProfile> = vec![];
            if let Ok(prof_dir) = read_dir(&config.profile_path) {
                for file in prof_dir.flatten() {
                    let path = file.path();
                    if path.is_file()
                        && path.extension().unwrap_or_default() == "pro"
                        && let Ok(profile) = from_file(&path.to_string_lossy())
                    {
                        loaded_profiles.push(profile);
                    }
                }
            }
            let selection = &loaded_profiles
                .iter()
                .find(|p| p.name == config.default_profile_name);
            self.active_profile = selection.cloned();
            self.combo_profiles = State::with_selection(loaded_profiles.clone(), *selection);
        }
    }

    pub fn view(&self) -> Element<'_, MainUIMessage> {
        let profile_select = combo_box(
            &self.combo_profiles,
            "ActiveProfile:",
            self.selected_profile.as_ref(),
            MainUIMessage::SelectProfile,
        );
        let content = row![
            profile_select,
            toggler(self.is_recording)
                .on_toggle(MainUIMessage::ToggleRecording)
                .label("Toggle Listening"),
            column![
                button("Profile Details").on_press(MainUIMessage::EditProfile),
            ]
            .spacing(5)
        ]
        .spacing(20)
        .padding(10);
        let window = container(content);
        if let Some(dialog) = &self.modal_dialog {
            dialog.apply(window.into())
        } else {
            window.into()
        }
    }

    pub fn update(&mut self, message: MainUIMessage) -> MainUIAction {
        let mut action = MainUIAction::None;
        match message {
            MainUIMessage::ToggleRecording(_) => {
                if self.active_profile.is_some() {
                    self.is_recording = !self.is_recording;
                } else {
                    if let Some(diag) = &mut self.modal_dialog {
                        diag.show_message(
                            "No Profile Selected",
                            "Please select a profile before starting recording.",
                        );
                    }
                    self.is_recording = false;
                    //TODO some type of popup
                };
            }
            MainUIMessage::SelectProfile(prof) => {
                if !self.is_recording {
                    self.selected_profile = Some(prof);
                } else if let Some(diag) = &mut self.modal_dialog {
                    diag.show_message("Please", "Stop listening before changing profiles.");
                }
            }
            MainUIMessage::EditProfile => {
                if let Some(profile) = &self.selected_profile {
                    action = MainUIAction::EditProfile(profile.clone());
                } else if let Some(diag) = &mut self.modal_dialog {
                    diag.show_message(
                        "No Profile Selected",
                        "Please select a profile before editing, or create a new one.",
                    );
                }
            }
            MainUIMessage::ModalAffirmative | MainUIMessage::ModalNegative => {
                if let Some(dialog) = &mut self.modal_dialog {
                    dialog.show(false);
                }
            }
        }
        action
    }
}
