use iced::{widget::column, Element, widget::text};
use crate::app::message::Message;

pub fn home_screen() -> Element<'static, Message> {
    column![
        text("Welcome to Plexus!").size(28)
    ]
    .into()
}
