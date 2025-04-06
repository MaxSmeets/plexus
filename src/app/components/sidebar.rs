use iced::{
    widget::{button, column, container, text},
    Length, Element,
};
use crate::app::page::Page;
use crate::app::message::Message;
use crate::app::utils::colors::hex;
use crate::app::utils::styles::{SidebarContainer, SidebarButton};

pub struct Sidebar;

impl Sidebar {
    pub fn new(_current: Page) -> Element<'static, Message> {
        // Sidebar background color.
        let bg_color = hex("212529");
        // Sidebar text color.
        let text_color = hex("E9ECEF");
        // Sidebar button background color (a slight variation).
        let btn_bg_color = hex("2A2A2A");

        container(
            column![
                text("Plexus").size(32).style(text_color),
                button("Home")
                    .on_press(Message::NavigateTo(Page::Home))
                    .style(SidebarButton(btn_bg_color, text_color).into()),
                button("Agents")
                    .on_press(Message::NavigateTo(Page::Agents))
                    .style(SidebarButton(btn_bg_color, text_color).into()),
                button("Settings")
                    .on_press(Message::NavigateTo(Page::Settings))
                    .style(SidebarButton(btn_bg_color, text_color).into()),
            ]
            .spacing(15)
            .padding(20)
            .width(Length::Fixed(150.0))
        )
        .height(Length::Fill)
        .style(SidebarContainer(bg_color))
        .into()
    }
}
