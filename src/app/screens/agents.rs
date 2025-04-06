use iced::{widget::column, Element, widget::text};
use crate::app::message::Message;

pub fn agents_screen() -> Element<'static, Message> {
    column![
        text("Agents Dashboard").size(28)
    ]
    .into()
}
