use crate::action_record::ActionRecord;
use iced::widget::{button, column, row, text};
use iced::Element;
use std::time::Duration;

pub struct ProfileEdit {
    pub action: ActionRecord,
    pub idx: Option<usize>,
    pub record_string: String,
}

#[derive(Clone)]
pub enum ProfileEditMessage {
    Save,
    Cancel,
    ToggleRecord,
}

pub enum ProfileEditAction {
    Save(Option<usize>, ActionRecord),
    Close,
    None,
}

impl ProfileEdit {
    pub fn new(idx: Option<usize>, action: ActionRecord) -> Self {
        ProfileEdit {
            action,
            record_string: "".to_string(),
            idx,
        }
    }

    fn toggle_record(&mut self) {
        match self.action.capture_actions(Duration::from_secs(5)) {
            Err(e) => self.record_string = format!("Error recording: {}", e),
            Ok(()) => {
                self.record_string = self.action.to_string();
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
           },
       }
    }

    pub fn view(&self) -> Element<'_, ProfileEditMessage> {
        row![
            column![
                text("Recorded Sequence: "),
                text(self.record_string.clone()),
            ],
            column![
                button(text("Record")).on_press(ProfileEditMessage::ToggleRecord),
                button(text("Cancel")).on_press(ProfileEditMessage::Cancel),
                button(text("Save")).on_press(ProfileEditMessage::Save),
            ],
        ].into()
    }
}
