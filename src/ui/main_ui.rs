//! # The primary Apllication Window.
use crate::action_profile::ActionProfile;
use crate::config::Config;
use crate::file_io;
use crate::file_io::from_file;
use crate::primary_dispatcher;
use crate::ui::modal_dialog::ModalDialog;
use crate::voice_req::VoiceReqCommands;
use iced::Element;
use iced::widget::combo_box::State;
use iced::widget::{button, column, combo_box, container, row, toggler};
use std::fs::read_dir;
use tokio::sync::mpsc;

#[derive(Default)]
pub struct MainUIState {
    active_profile: Option<ActionProfile>,
    is_recording: bool,
    config: Option<Config>,
    modal_dialog: Option<ModalDialog<MainUIMessage>>,
    selected_profile: Option<ActionProfile>,
    combo_profiles: combo_box::State<ActionProfile>,
    voice_command_tx: Option<mpsc::Sender<VoiceReqCommands>>,
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
    pub fn new(config: Option<Config>) -> MainUIState {
        let modal_dialog = ModalDialog::new(
            "Initialization Error",
            "Could not find a configuration. Have you run the installer?",
            MainUIMessage::ModalAffirmative,
            MainUIMessage::ModalNegative,
            false,
        );
        let mut working_state = MainUIState {
            active_profile: None,
            is_recording: false,
            config: config.clone(),
            modal_dialog: Some(modal_dialog),
            selected_profile: None,
            combo_profiles: combo_box::State::new(vec![]),
            voice_command_tx: None,
        };
        if let Some(cfg_data) = config
            && let Ok(cfg) = file_io::from_file(&cfg_data.default_profile)
        {
            working_state.active_profile = Some(cfg);
            working_state.load_profiles();
        }
        working_state
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
            self.selected_profile = selection.cloned();
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
            column![button("Profile Details").on_press(MainUIMessage::EditProfile),].spacing(5)
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

    fn start_listening(&mut self) {
        if let Some(profile) = &self.active_profile {
            let (tx, rx) = mpsc::channel(10);
            self.voice_command_tx = Some(tx);
            if let Err(e) = primary_dispatcher::begin_dispatch(profile.actions.clone(), rx) {
                if let Some(diag) = &mut self.modal_dialog {
                    diag.show_message("Error Listening", &e.to_string());
                }
            } else {
                self.is_recording = true;
            }
        }
    }

    fn stop_listening(&mut self) {
        if let Some(tx) = &self.voice_command_tx {
            if let Err(e) = tx.blocking_send(VoiceReqCommands::Stop)
                && let Some(diag) = &mut self.modal_dialog
            {
                diag.show_message("Error Listening", &e.to_string());
            }
            self.is_recording = false;
            self.voice_command_tx = None;
        }
    }

    pub fn update(&mut self, message: MainUIMessage) -> MainUIAction {
        let mut action = MainUIAction::None;
        match message {
            MainUIMessage::ToggleRecording(_) => {
                if self.active_profile.is_some() {
                    match self.is_recording {
                        true => {
                            self.stop_listening();
                        }
                        false => {
                            self.start_listening();
                        }
                    }
                } else {
                    if let Some(diag) = &mut self.modal_dialog {
                        diag.show_message(
                            "No Profile Selected",
                            "Please select a profile before starting recording.",
                        );
                    }
                    self.is_recording = false;
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
                self.stop_listening();
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
