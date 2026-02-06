use iced::Element;
use iced::widget::{Container, button, center, column, container, row, text};

pub struct ModalDialog<T> {
    title: String,
    message: String,
    pub show: bool,
    affirmitive_response: T,
    negative_response: T,
}

impl<T: std::clone::Clone> ModalDialog<T> {
    pub fn new(
        title: &str,
        message: &str,
        affirmitive_response: T,
        negative_response: T,
        show: bool,
    ) -> ModalDialog<T> {
        ModalDialog {
            title: title.to_string(),
            message: message.to_string(),
            show,
            affirmitive_response,
            negative_response,
        }
    }

    pub fn show(&mut self, show: bool) {
        self.show = show;
    }

    pub fn apply<'a>(&'a self, core: Element<'a, T>) -> Element<'a, T> {
        let dialog = Container::new(center(column![
            row![text(&self.title)],
            row![text(&self.message)],
            center(row![
                button("Yes").on_press(self.affirmitive_response.clone()),
                button("No").on_press(self.negative_response.clone())
            ]),
        ]))
        .style(container::bordered_box)
        .padding(20)
        .center_x(400)
        .center_y(150);

        if self.show {
            center(dialog).into()
        } else {
            core
        }
    }
}
