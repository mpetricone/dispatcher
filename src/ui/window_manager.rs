use crate::config::Config;
use crate::ui::main_ui;
use crate::ui::message_display;
use crate::ui::profile_manager;
use iced::{Element, Task, exit};
use std::sync::Arc;
use std::sync::Mutex;

enum Window {
    MainUI(main_ui::MainUIState),
    Profile(profile_manager::ProfileManager),
    MessageDisplay(message_display::MessageDisplay),
}

pub enum Message {
    MainUI(main_ui::MainUIMessage),
    Profile(profile_manager::Message),
    MessageDisplay(message_display::MessageDisplayMessages),
}

pub struct WindowManager {
    window: Window,
    config: Arc<Mutex<Option<Config>>>,
}

impl Default for WindowManager {
    fn default() -> Self {
        WindowManager::new()
    }
}

impl WindowManager {
    pub fn new() -> Self {
        let mut config = None;
        if let Ok(ready_config) = Config::build() {
            config = Some(ready_config);
        }
        let arc_config = Arc::new(Mutex::new(config));
        let config_clone = arc_config.lock().unwrap().clone();
        WindowManager {
            window: Window::MainUI(main_ui::MainUIState::new(config_clone)),
            config: arc_config,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MainUI(message) => {
                if let Window::MainUI(main) = &mut self.window {
                    let action = main.update(message);
                    match action {
                        main_ui::MainUIAction::NewProfile(data) => {
                            let config_clone = self.config.lock().unwrap().clone();
                            if let Some(config) = config_clone {
                                let profile = profile_manager::ProfileManager::new(data, config);
                                self.window = Window::Profile(profile);
                            }
                            Task::none()
                        }
                        main_ui::MainUIAction::EditProfile(data) => {
                            let config_clone = self.config.lock().unwrap().clone();
                            if let Some(config) = config_clone {
                                let profile = profile_manager::ProfileManager::new(data, config);
                                self.window = Window::Profile(profile);
                            }
                            Task::none()
                        }
                        main_ui::MainUIAction::None => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
            Message::Profile(message) => {
                if let Window::Profile(profile) = &mut self.window {
                    let action = profile.update(message);
                    match action {
                        profile_manager::ProfileWindowAction::Close => {
                            let config_clone = self.config.lock().unwrap().clone();
                            let main = main_ui::MainUIState::new(config_clone);
                            self.window = Window::MainUI(main);
                            Task::none()
                        }
                        profile_manager::ProfileWindowAction::Error(e) => {
                            self.window =
                                Window::MessageDisplay(message_display::MessageDisplay::new_ok(&e));
                            Task::none()
                        }
                        _ => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
            Message::MessageDisplay(message) => {
                if let Window::MessageDisplay(display) = &mut self.window {
                    let action = display.update(message);
                    match action {
                        message_display::MessageDisplayMessages::ExitApplication => exit(),
                        _ => {
                            self.window = Window::MainUI(main_ui::MainUIState::new(
                                self.config.lock().unwrap().clone(),
                            ));
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
            Window::MessageDisplay(display) => display.view().map(Message::MessageDisplay),
        }
    }
}
