use crate::ui::main_ui::MainUIMessage;
use crate::ui::profile::{ProfileAction, ProfileMessage};
use crate::ui::profile;
use crate::ui::main_ui;
use iced::{ Element, Task };

enum Window {
    MainUI(main_ui::MainUIState),
    Profile(profile::Profile),
}

enum Message {
    MainUI(main_ui::MainUIMessage),
    Profile(profile::ProfileMessage),
}

struct WindowManager{
    window: Window,
}

impl WindowManager {
    pub fn new() -> Self {
        WindowManager { window: Window::MainUI(main_ui::MainUIState::new()) }
    }

    fn update(&mut self, message: Message) -> Task<Message>  {
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
                            let main  = main_ui::MainUIState::new();
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

    fn view(&self) -> Element<Message> {
        match &self.window {
            Window::MainUI(main) => main.view().map(Message::MainUI),
            Window::Profile(profile) => profile.view().map(Message::Profile),
        }
    }
}
