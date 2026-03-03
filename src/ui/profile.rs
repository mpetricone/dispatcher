use crate::action_profile::ActionProfile;
use crate::action_record::ActionRecord;
use iced::widget::{button, column, row, text};
use iced::{Font, Renderer, Theme};
use iced_aw::{selection_list::SelectionList, style::selection_list::primary};
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum ProfileMessage {
    Add,
    None,
    Edit,
    Close,
    Delete,
    Save,
    Move(ProfileMoveDirection),
    ProfileSelected(usize, ProfileListEntry),
}

/// Used for signalling moving a selected profile action up or down the vec
#[derive(Clone, Debug)]
pub enum ProfileMoveDirection {
    Up,
    Down,
}

pub struct Profile {
    profile: ActionProfile,
    selected: Option<usize>,
    selected_name: String,
    /// iced_aw wont work without Eq, and we've got floats
    slv: Vec<ProfileListEntry>,
}

pub enum ProfileAction {
    Edit(Option<usize>, Vec<ActionRecord>),
    Close,
    Save(ActionProfile),
    None,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
        let slv = Self::create_list_view(&profile);

        Profile {
            profile,
            selected: None,
            selected_name: String::new(),
            slv,
        }
    }

    fn create_list_view(profile: &ActionProfile) -> Vec<ProfileListEntry> {
        profile
            .actions
            .iter()
            .enumerate()
            .map(|(idx, action)| ProfileListEntry {
                name: action.name.clone(),
                idx,
            })
            .collect()
    }

    /// Called by update to move a profile action up or down the vec
    fn move_selected_action(&mut self, direction: ProfileMoveDirection) {
        if let Some(selected) = self.selected {
            match direction {
                ProfileMoveDirection::Up => {
                    if self.selected > Some(0) {
                        self.profile.actions.swap(selected, selected - 1);
                        self.selected = Some(selected - 1);
                        self.selected_name = self.profile.actions[selected - 1].name.clone();
                        self.slv = Self::create_list_view(&self.profile);
                    }
                }
                ProfileMoveDirection::Down => {
                    if self.selected < Some(self.profile.actions.len() - 1) {
                        self.profile.actions.swap(selected, selected + 1);
                        self.selected = Some(selected + 1);
                        self.selected_name = self.profile.actions[selected + 1].name.clone();
                        self.slv = Self::create_list_view(&self.profile);
                    }
                }
            }
        }
    }

    pub fn update(&mut self, message: ProfileMessage) -> ProfileAction {
        match message {
            ProfileMessage::ProfileSelected(index, name) => {
                self.selected = Some(index);
                self.selected_name = name.name.clone();
            }
            ProfileMessage::Add => {
                return ProfileAction::Edit(Some(0), vec![ActionRecord::new("", "", vec![])]);
            }
            ProfileMessage::Edit => {
                return ProfileAction::Edit(self.selected, self.profile.actions.clone());
            }
            ProfileMessage::Delete => todo!("delete a profile action"),
            ProfileMessage::Close => return ProfileAction::Close,
            ProfileMessage::Save => return ProfileAction::Save(self.profile.clone()),
            ProfileMessage::Move(direction) => {
                self.move_selected_action(direction);
            }
            ProfileMessage::None => (),
        }
        ProfileAction::None
    }

    pub fn view(&self) -> iced::Element<'_, ProfileMessage> {
        let selection_list: SelectionList<'_, _, _, Theme, Renderer> = SelectionList::new_with(
            &self.slv,
            ProfileMessage::ProfileSelected,
            12.0,
            5.0,
            primary,
            self.selected,
            Font::default(),
        );
        let top = row![text("Selected Profile:"), text(self.selected_name.clone()),];
        column![
            top,
            row![
                selection_list,
                column![
                    button(text("Add Action")).on_press(ProfileMessage::Add),
                    button(text("Edit Action")).on_press(ProfileMessage::Edit),
                    //button(text("Delete Delete Action")).on_press(ProfileMessage::Delete),
                    button(text("Move Action Up"))
                        .on_press(ProfileMessage::Move(ProfileMoveDirection::Up)),
                    button(text("Move Action Down"))
                        .on_press(ProfileMessage::Move(ProfileMoveDirection::Down)),
                    button(text("Close")).on_press(ProfileMessage::Close),
                    button(text("Save")).on_press(ProfileMessage::Save),
                ]
            ]
        ]
        .into()
    }
}
