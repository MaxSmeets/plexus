use iced::{widget::column, Element, widget::text};
use crate::app::message::Message;

pub fn settings_screen() -> Element<'static, Message> {
    column![
        text("Settings Page").size(28)
    ]
    .into()
}
