use crate::action_profile::ActionProfile;
use crate::action_record::ActionRecord;
use iced::widget::{text, column};
use iced::Font;
use iced_aw::{ selection_list::SelectionList, style::selection_list::primary};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ProfileMessage {
    Record(bool),
    ProfileSelected(usize, ProfileListEntry),
    Save,
    Cancel,
}

pub enum ProfileAction {
    None,
    Close,
}

pub struct Profile {
    profile: ActionProfile,
    selected: Option<usize>,
    selected_name: String,
    slv: Vec<ProfileListEntry>,
}

#[derive(Debug,PartialEq, Eq, Hash, Clone)]
pub struct ProfileListEntry {
    name: String,
    idx: usize,
}

impl Display for ProfileListEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Profile {
    /// Placeholder for creating a new profile
    pub fn new(profile: ActionProfile) -> Self {
        let slv = profile.actions.iter().enumerate().map(|(idx, action)| ProfileListEntry {
            name: action.name.clone(),
            idx,
        }).collect();
        Profile { profile, selected: None, selected_name: String::new(), slv}
    }

    pub fn update(&mut self, message: ProfileMessage) -> ProfileAction {
        match message {
            ProfileMessage::ProfileSelected(index, name) => {
                self.selected = Some(index);
                self.selected_name = name.name.clone();
                ProfileAction::None
            }
            ProfileMessage::Record(value) => ProfileAction::None,
            ProfileMessage::Save => ProfileAction::Close,
            ProfileMessage::Cancel => ProfileAction::Close,
        }
    }


    pub fn view(&self) -> iced::Element<'_, ProfileMessage> {

        let selection_list = SelectionList::new_with(
            &self.slv,
            ProfileMessage::ProfileSelected,
            12.0,
            5.0,
            primary,
            self.selected,
            Font::default(),
        );
        column![
            text(self.selected_name.clone()),
            selection_list].into()
    }
}
