use dispatcher::ui::window_manager::window_manager;

/// Currently a testbed
fn main() -> iced::Result {
    iced::application(WindowManager::new, WindowManager::update, WindowManager::view).run()
}
