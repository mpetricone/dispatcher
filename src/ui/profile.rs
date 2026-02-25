use crate::action_profile::ActionProfile;
use iced::widget::{button, column, row, text};
use iced::{Font, Renderer, Theme};
use iced_aw::{selection_list::SelectionList, style::selection_list::primary};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ProfileMessage {
    ProfileSelected(usize, ProfileListEntry),
    Add,
    Edit,
    Move(ProfileMoveDirection),
    Delete,
    Close,
}

#[derive(Clone, Debug)]
pub enum ProfileMoveDirection {
    Up,
    Down,
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
        let slv = profile
            .actions
            .iter()
            .enumerate()
            .map(|(idx, action)| ProfileListEntry {
                name: action.name.clone(),
                idx,
            })
            .collect();
        Profile {
            profile,
            selected: None,
            selected_name: String::new(),
            slv,
        }
    }

    fn move_selected_action(&mut self, direction: ProfileMoveDirection) {
        if let Some(selected) = self.selected {
            match direction {
                ProfileMoveDirection::Up => {
                    if self.selected > Some(0) {
                        self.profile.actions.swap(selected, selected - 1);
                        self.selected = Some(selected - 1);
                        self.selected_name = self.profile.actions[selected - 1].name.clone();
                    }
                }
                ProfileMoveDirection::Down => {
                    if self.selected < Some(self.profile.actions.len() - 2) {
                        self.profile.actions.swap(selected, selected + 1);
                        self.selected = Some(selected + 1);
                        self.selected_name = self.profile.actions[selected + 1].name.clone();
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
                ProfileAction::None
            }
            ProfileMessage::Add => todo!("add a new profile action"),
            ProfileMessage::Edit => todo!("edit a profile action"),
            ProfileMessage::Delete => todo!("delete a profile action"),
            ProfileMessage::Move(direction) => {
                self.move_selected_action(direction);
                ProfileAction::None
            }
            ProfileMessage::Close => ProfileAction::Close,
        }
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
                    button(text("Delete Delete Action")).on_press(ProfileMessage::Delete),
                    button(text("Move Action Up"))
                        .on_press(ProfileMessage::Move(ProfileMoveDirection::Up)),
                    button(text("Move Action Down"))
                        .on_press(ProfileMessage::Move(ProfileMoveDirection::Down)),
                ]
            ]
        ]
        .into()
    }
}
