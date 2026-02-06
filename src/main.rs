use dispatcher::ui::main_ui::MainUIState;

/// Currently a testbed
fn main() -> iced::Result {
    iced::application(MainUIState::new, MainUIState::update, MainUIState::view).run()
}
