use crate::config::Config;
use crate::file_io;
use iced::Element;
use iced::widget::{button, column, row, text, text_input};
use std::path::PathBuf;

pub struct ConfigEdit {
    config: Config,
    default_ddelay: u32,
    error_message: String,
    audio_library_path: String,
}

#[derive(Debug, Clone)]
pub enum ConfigEditMessage {
    Save,
    Cancel,
    DDelayChanged(String),
    AudioLibraryPathChanged(String),
}

pub enum ConfigEditAction {
    Close,
    None,
}

impl ConfigEdit {
    pub fn new(config: &Config) -> Self {
        let audio_library_path = config
            .audio_library_path
            .clone()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        Self {
            config: config.clone(),
            default_ddelay: config.default_dispatcher_config.default_command_delay,
            error_message: "".to_string(),
            audio_library_path,
        }
    }

    pub fn update(&mut self, message: ConfigEditMessage) -> ConfigEditAction {
        match message {
            ConfigEditMessage::Save => {
                self.config.default_dispatcher_config.default_command_delay = self.default_ddelay;
                if let Err(e) =
                    file_io::to_file(&self.config.config_path.clone(), true, &mut self.config)
                {
                    self.error_message = e.to_string();
                    return ConfigEditAction::None;
                }
                ConfigEditAction::Close
            }
            ConfigEditMessage::Cancel => ConfigEditAction::Close,
            ConfigEditMessage::DDelayChanged(ddelay) => {
                self.default_ddelay = ddelay.parse().unwrap_or(self.default_ddelay);
                self.config.default_dispatcher_config.default_command_delay = self.default_ddelay;
                ConfigEditAction::None
            }
            ConfigEditMessage::AudioLibraryPathChanged(path) => {
                self.audio_library_path = path;
                let bpath = PathBuf::from(&self.audio_library_path);
                if bpath.is_dir() {
                    self.config.audio_library_path = Some(bpath);
                }
                ConfigEditAction::None
            }
        }
    }

    pub fn view(&self) -> Element<'_, ConfigEditMessage> {
        let choices = column![
            row![
                column![
                    text("Dispatcher Command Delay: "),
                    text_input("", &self.default_ddelay.to_string())
                        .on_input(ConfigEditMessage::DDelayChanged),
                ]
                .spacing(10),
                column![
                    button("Save").on_press(ConfigEditMessage::Save),
                    button("Cancel").on_press(ConfigEditMessage::Cancel),
                ]
                .spacing(10)
            ]
            .spacing(10),
            column![
                text("Audio library path:"),
                text_input("", &self.audio_library_path)
                    .on_input(ConfigEditMessage::AudioLibraryPathChanged)
            ]
            .spacing(10),
            row![text(&self.error_message)].spacing(20)
        ]
        .padding(10)
        .spacing(10);

        choices.into()
    }
}
