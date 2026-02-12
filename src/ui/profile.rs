use crate::action_profile::ActionProfile;
use iced::Task;
use iced::widget::{ column, Text};

pub enum ProfileMessage {
    Record(bool),
    Save,
    Cancel,
}

pub enum ProfileAction {
    None,
    Close,
}


pub struct Profile {
    profile: ActionProfile,
}

impl Profile {
    /// Placeholder for creating a new profile
    pub fn new(profile: ActionProfile) -> Self {
            Profile { profile }
    }

    pub fn update(&mut self, message: ProfileMessage) -> Task<ProfileAction> {
        match message {
            ProfileMessage::Record(value) => {
                Task::done(ProfileAction::None)
            }
            ProfileMessage::Save => {
                Task::done(ProfileAction::Close)
            }
            ProfileMessage::Cancel => {
                Task::done(ProfileAction::Close)
            }
        }
    }
    pub fn view(&self) -> iced::Element<'_, ProfileMessage> {
        column![
            Text::new("Placeholder")
        ].into()
    }
}
