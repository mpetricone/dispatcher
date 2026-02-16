use dispatcher::ui::window_manager::WindowManager;
/// Currently a testbed
fn main() -> iced::Result {
    iced::application(
        WindowManager::new,
        WindowManager::update,
        WindowManager::view,
    )
    .run()
}
