use crate::action_record::{ActionRecord, ActionRecordStreamFormatted};
use iced::widget::{button, column, row, text, text_input};
use iced::Element;
use std::time::Duration;

pub struct ProfileEdit {
    pub action: ActionRecord,
    pub idx: Option<usize>,
    record_string: String,
}

#[derive(Clone)]
pub enum ProfileEditMessage {
    Save,
    Cancel,
    ToggleRecord,
    NameChanged(String),
    ActivatorChanged(String),
}

pub enum ProfileEditAction {
    Save(Option<usize>, ActionRecord),
    Close,
    None,
}

impl ProfileEdit {
    pub fn new(idx: Option<usize>, action: ActionRecord) -> Self {
        let record_string = ActionRecordStreamFormatted(&action).to_string();
        ProfileEdit {
            action,
            idx,
            record_string,
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
            ProfileEditMessage::Save => {
                ProfileEditAction::Save(self.idx, self.action.clone())
            }
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
            row![
                text("Recorded Events: "),
                text(&self.record_string),
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
