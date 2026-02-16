use crate::ui::main_ui;
use crate::ui::profile;
use iced::{Element, Task};

enum Window {
    MainUI(main_ui::MainUIState),
    Profile(profile::Profile),
}

pub enum Message {
    MainUI(main_ui::MainUIMessage),
    Profile(profile::ProfileMessage),
}

pub struct WindowManager {
    window: Window,
}

impl Default for WindowManager {
    fn default() -> Self {
        WindowManager::new()
    }
}

impl WindowManager {
    pub fn new() -> Self {
        WindowManager {
            window: Window::MainUI(main_ui::MainUIState::new()),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MainUI(message) => {
                if let Window::MainUI(main) = &mut self.window {
                    let action = main.update(message);
                    match action {
                        main_ui::MainUIAction::None => Task::none(),
                        main_ui::MainUIAction::NewProfile(data) => {
                            let profile = profile::Profile::new(data);
                            self.window = Window::Profile(profile);
                            Task::none()
                        }
                        main_ui::MainUIAction::EditProfile(data) => {
                            let profile = profile::Profile::new(data);
                            self.window = Window::Profile(profile);
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::Profile(message) => {
                if let Window::Profile(profile) = &mut self.window {
                    let action = profile.update(message);
                    match action {
                        profile::ProfileAction::None => Task::none(),
                        profile::ProfileAction::Close => {
                            let main = main_ui::MainUIState::new();
                            self.window = Window::MainUI(main);
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.window {
            Window::MainUI(main) => main.view().map(Message::MainUI),
            Window::Profile(profile) => profile.view().map(Message::Profile),
        }
    }
}
