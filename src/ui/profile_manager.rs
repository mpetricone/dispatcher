use crate::action_profile::ActionProfile;
use crate::action_record::ActionRecord;
use crate::ui::profile;
use crate::ui::profile_edit;
use iced::Element;

enum Window {
    Profile(profile::Profile),
    ProfileEdit(profile_edit::ProfileEdit),
}

pub enum Message {
    Profile(profile::ProfileMessage),
    EditProfile(profile_edit::ProfileEditMessage),
}

pub enum ProfileWindowAction {
    Close,
    Save(ActionProfile),
    None,
}

pub struct ProfileManager {
    window: Window,
    stable_profile: ActionProfile,
}

impl ProfileManager {
    pub fn new(profile: ActionProfile) -> Self {
        ProfileManager {
            window: Window::Profile(profile::Profile::new(profile.clone())),
            stable_profile: profile,
        }
    }

    pub fn update(&mut self, message: Message) -> ProfileWindowAction {
        match message {
            Message::Profile(message) => {
                if let Window::Profile(profile) = &mut self.window {
                    match profile.update(message) {
                        profile::ProfileAction::Edit(idx, data) => {
                            if let Some(index) = idx {
                                self.window = Window::ProfileEdit(profile_edit::ProfileEdit::new(
                                    idx,
                                    data[index].clone(),
                                ));
                            } else {
                                self.window = Window::ProfileEdit(profile_edit::ProfileEdit::new(
                                    None,
                                    ActionRecord::new("", "", vec![]),
                                ));
                            }
                        }
                        profile::ProfileAction::None => (),
                        profile::ProfileAction::Save(data) => {
                            //TODO Save
                            return ProfileWindowAction::None;
                        }
                        profile::ProfileAction::Close => {
                            return ProfileWindowAction::Close;
                        }
                    }
                }
            }
            Message::EditProfile(message) => {
                if let Window::ProfileEdit(edit) = &mut self.window {
                    match edit.update(message) {
                        profile_edit::ProfileEditAction::Save(idx, data) => {
                            if let Some(index) = idx {
                                if index < self.stable_profile.actions.len() {
                                    self.stable_profile.actions[index] = data;
                                } else {
                                    self.stable_profile.actions.push(data);
                                }
                            } else {
                                self.stable_profile.actions.push(data);
                            }
                            self.window =
                                Window::Profile(profile::Profile::new(self.stable_profile.clone()));
                        }
                        profile_edit::ProfileEditAction::Close => {
                            self.window =
                                Window::Profile(profile::Profile::new(self.stable_profile.clone()));
                        }
                        profile_edit::ProfileEditAction::None => (),
                    }
                }
            }
        }
        ProfileWindowAction::None
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.window {
            Window::Profile(profile) => profile.view().map(Message::Profile),
            Window::ProfileEdit(edit) => edit.view().map(Message::EditProfile),
        }
    }
}
