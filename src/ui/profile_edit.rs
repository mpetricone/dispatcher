//! # Profile Editing, namely, Actions
use crate::action_record::{ActionRecord, ActionRecordStreamFormatted};
use crate::config::Config;
use crate::file_io;
use iced::Element;
use iced::widget::{button, column, pick_list, row, text, text_input};
use std::path::PathBuf;
use std::string::String;
use std::time::Duration;

pub struct ProfileEdit {
    pub action: ActionRecord,
    pub idx: Option<usize>,
    audio_sources: Vec<String>,
    record_string: String,
    audio_source_state: Option<String>,
    audio_files: Vec<String>,
    audio_file_state: Option<String>,
    //config: Config,
}

#[derive(Clone)]
pub enum ProfileEditMessage {
    Save,
    Cancel,
    ToggleRecord,
    NameChanged(String),
    ActivatorChanged(String),
    AudioSourcesChanged(String),
    AudioFileChanged(String),
}

pub enum ProfileEditAction {
    Save(Option<usize>, ActionRecord),
    Close,
    None,
}

impl ProfileEdit {
    pub fn new(idx: Option<usize>, action: ActionRecord, config: Config) -> Self {
        let record_string = ActionRecordStreamFormatted(&action).to_string();
        let mut audio_sources: Vec<String> = Vec::new();
        if let Some(path) = &config.audio_library_path
            && let Ok(p) = file_io::get_dir_list(path)
        {
            audio_sources = p
                .into_iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect();
        }
        let mut audio_source_state = None;
        let mut audio_files = Vec::new();
        let mut audio_file_state = None;
        if let Some(state) = action.completion_audio_path.audio_path.clone() {
            audio_source_state = Some(state.to_string_lossy().to_string());
            if let Ok(files) = file_io::get_file_list_recursive(&state) {
                audio_files = files
                    .into_iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect();
            }
        }
        if let Some(state) = action.completion_audio_path.audio_file.clone() {
            audio_file_state = Some(state.to_string_lossy().to_string());
        }
        ProfileEdit {
            action,
            idx,
            record_string,
            audio_sources,
            audio_source_state,
            audio_files,
            audio_file_state,
        }
    }

    fn toggle_record(&mut self) {
        match self.action.capture_actions(Duration::from_secs(5)) {
            Err(e) => self.record_string = format!("Error recording: {}", e),
            Ok(()) => {
                self.record_string = ActionRecordStreamFormatted(&self.action).to_string();
            }
        }
    }

    pub fn update(&mut self, message: ProfileEditMessage) -> ProfileEditAction {
        match message {
            ProfileEditMessage::Save => ProfileEditAction::Save(self.idx, self.action.clone()),
            ProfileEditMessage::Cancel => ProfileEditAction::Close,
            ProfileEditMessage::ToggleRecord => {
                self.toggle_record();
                ProfileEditAction::None
            }
            ProfileEditMessage::NameChanged(name) => {
                self.action.name = name;
                ProfileEditAction::None
            }
            ProfileEditMessage::ActivatorChanged(activator) => {
                self.action.activator_text = activator;
                ProfileEditAction::None
            }
            ProfileEditMessage::AudioSourcesChanged(source) => {
                self.audio_source_state = Some(source.clone());
                self.action.completion_audio_path.audio_path = Some(PathBuf::from(source.clone()));
                ProfileEditAction::None
            }
            ProfileEditMessage::AudioFileChanged(source) => {
                self.audio_file_state = Some(source.clone());
                self.action.completion_audio_path.audio_file = Some(PathBuf::from(source.clone()));
                ProfileEditAction::None
            }
        }
    }

    pub fn view(&self) -> Element<'_, ProfileEditMessage> {
        column![
            row![
                text("Name: "),
                text_input("Enter action name", &self.action.name)
                    .on_input(ProfileEditMessage::NameChanged),
            ]
            .spacing(10),
            row![
                text("Voice Command: "),
                text_input("Enter voice command", &self.action.activator_text)
                    .on_input(ProfileEditMessage::ActivatorChanged),
            ]
            .spacing(10),
            row![text("Recorded Events: "), text(&self.record_string),].spacing(10),
            row![
                pick_list(
                    self.audio_sources.clone(),
                    self.audio_source_state.clone(),
                    ProfileEditMessage::AudioSourcesChanged
                )
                .placeholder("Select audio source")
            ]
            .spacing(10),
            row![
                pick_list(
                    self.audio_files.clone(),
                    self.audio_file_state.clone(),
                    ProfileEditMessage::AudioFileChanged
                )
                .placeholder("Select Audio File.")
            ]
            .spacing(10),
            row![
                button("Record").on_press(ProfileEditMessage::ToggleRecord),
                button("Cancel").on_press(ProfileEditMessage::Cancel),
                button("Save").on_press(ProfileEditMessage::Save),
            ]
            .spacing(10),
        ]
        .padding(20)
        .into()
    }
}
