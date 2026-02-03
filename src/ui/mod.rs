use crate::action_profile::ActionProfile;
use iced::widget::{row, Row,column, Column, text, toggler, button};

#[derive(Default)]
pub struct MainUIState {
    active_profile: Option<ActionProfile>,// Define fields here
    is_recording: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum MainUIMessage {
    ToggleRecording,
    SelectProfile,
    EditProfile,
}

impl MainUIState {
    pub fn new() -> Self {
        MainUIState {
            active_profile: None,
            is_recording: false,
        }
    }

    pub fn view(&self) -> Row<'_,MainUIMessage> {
        let profile_name = match &self.active_profile {
            Some(profile) => profile.name.clone(),
            None => "No profile loaded".to_string(),
        };

        row![
            text(profile_name).width(600),
            button("Select profile").on_press(MainUIMessage::SelectProfile),
            toggler(self.is_recording)
                .label("Toggle Listening"),
        ].spacing(20)
    }

    pub fn update(&mut self, message: MainUIMessage) {
        match message {
            MainUIMessage::ToggleRecording => {
                if let Some(_) = self.active_profile {
                    self.is_recording = !self.is_recording
                } else {
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
        }
    }
}
