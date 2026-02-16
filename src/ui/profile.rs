use crate::action_profile::ActionProfile;
use iced::widget::{Text, column};

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

    pub fn update(&mut self, message: ProfileMessage) -> ProfileAction {
        match message {
            ProfileMessage::Record(value) => ProfileAction::None,
            ProfileMessage::Save => ProfileAction::Close,
            ProfileMessage::Cancel => ProfileAction::Close,
        }
    }
    pub fn view(&self) -> iced::Element<'_, ProfileMessage> {
        column![Text::new("Placeholder")].into()
    }
}
