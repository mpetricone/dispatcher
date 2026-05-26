use crate::config::Config ;
use crate::file_io;
use iced::Element;
use iced::widget::{column, row, button, text, text_input};


pub struct ConfigEdit {
    config: Config,
    default_ddelay: u32,
    error_message: String,
}

#[derive(Debug, Clone)]
pub enum ConfigEditMessage {
   Save,
   Cancel,
   DDelayChanged(String),
}

pub enum ConfigEditAction {
    Close,
    None,
}

impl ConfigEdit {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
            default_ddelay: config.default_dispatcher_config.default_command_delay,
            error_message: "".to_string(),
        }
    }

    pub fn update(&mut self, message: ConfigEditMessage) -> ConfigEditAction {
        match message {
            ConfigEditMessage::Save => {
                self.config.default_dispatcher_config.default_command_delay = self.default_ddelay;
                if let Err(e) = file_io::to_file(&self.config.config_path.clone(), true, &mut self.config) {
                    self.error_message = e.to_string();
                    return ConfigEditAction::None;
                }
                return ConfigEditAction::Close;
            }
            ConfigEditMessage::Cancel => {
                return ConfigEditAction::Close;
            }
            ConfigEditMessage::DDelayChanged(ddelay) => {
                self.default_ddelay = ddelay.parse().unwrap_or(self.default_ddelay);
                self.config.default_dispatcher_config.default_command_delay = self.default_ddelay;
                return ConfigEditAction::None;
            }
        }
    }

    pub fn view(&self) -> Element<'_, ConfigEditMessage> {
        let choices = column![
            row![
                column![
                    text("Dispatcher Command Delay: "),
                    text_input("", &self.default_ddelay.to_string()).on_input(ConfigEditMessage::DDelayChanged),
                ],
                column![
                    button("Save").on_press(ConfigEditMessage::Save),
                    button("Cancel").on_press(ConfigEditMessage::Cancel),
                ]
            ],
            row![ text(&self.error_message)]
        ];

        choices.into()
    }
}
