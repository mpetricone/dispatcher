use iced::Element;
use iced::widget::{Row, button, column, row, text};

pub struct MessageDisplay {
    msg: String,
    buttons: Vec<MessageButton>,
}

/// Display a message, with buttons
pub struct MessageButton {
    label: String,
    event: MessageDisplayMessages,
}

#[derive(Debug, Clone)]
pub enum MessageDisplayMessages {
    Ok,
    Cancel,
    ExitApplication,
}

impl MessageDisplay {
    /// Builds a new MessageDisplay
    pub fn new(msg: &str, buttons: Vec<MessageButton>) -> Self {
        MessageDisplay {
            msg: msg.to_string(),
            buttons,
        }
    }

    // Builds a new [MessageDisplay] with an Ok button
    pub fn new_ok(msg: &str) -> Self {
        MessageDisplay {
            msg: msg.to_string(),
            buttons: vec![MessageButton {
                label: "Ok".to_string(),
                event: MessageDisplayMessages::Ok,
            }],
        }
    }

    /// Builds a new Message display with an abort button
    /// Please note the logic to exit application is not contained within this struct
    pub fn new_panic(msg: &str) -> Self {
        MessageDisplay {
            msg: msg.to_string(),
            buttons: vec![MessageButton {
                label: "Abort".to_string(),
                event: MessageDisplayMessages::ExitApplication,
            }],
        }
    }

    /// Builds a new MessageDisplay with Ok and Cancel buttons
    pub fn new_ok_cancel(msg: &str) -> Self {
        MessageDisplay {
            msg: msg.to_string(),
            buttons: vec![
                MessageButton {
                    label: "Ok".to_string(),
                    event: MessageDisplayMessages::Ok,
                },
                MessageButton {
                    label: "Cancel".to_string(),
                    event: MessageDisplayMessages::Cancel,
                },
            ],
        }
    }

    pub fn update(&mut self, event: MessageDisplayMessages) -> MessageDisplayMessages {
        event
    }

    pub fn view(&self) -> Element<'_, MessageDisplayMessages> {
        let msg_row = row![text(&self.msg)];

        let buttons = self
            .buttons
            .iter()
            .map(|b| -> Element<'_, MessageDisplayMessages> {
                button(text(&b.label)).on_press(b.event.clone()).into()
            });
        let button_row = Row::with_children(buttons);
        column![msg_row, button_row,].into()
    }
}
