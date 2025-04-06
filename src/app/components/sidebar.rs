use iced::{widget::{button, column, text}, Length, Element};
use crate::app::page::Page;
use crate::app::message::Message;

pub struct Sidebar;

impl Sidebar {
    pub fn new(_current: Page) -> Element<'static, Message> {
        column![
            text("Plexus").size(32),
            button("Home").on_press(Message::NavigateTo(Page::Home)),
            button("Agents").on_press(Message::NavigateTo(Page::Agents)),
            button("Settings").on_press(Message::NavigateTo(Page::Settings)),
        ]
        .spacing(15)
        .padding(20)
        .width(Length::Fixed(150.0))
        .into()
    }
}
