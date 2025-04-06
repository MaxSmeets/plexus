use iced::{
    widget::{column, container, text},
    Element, Length,
};
use crate::app::message::Message;
use crate::app::utils::colors::hex;
use crate::app::utils::styles::SolidContainer;

pub fn home_screen() -> Element<'static, Message> {
    let bg_color = hex("343A40");   // Home screen background
    let text_color = hex("F8F9FA");   // Text color

    let content = text("Welcome to Plexus!")
        .size(28)
        .style(text_color);

    container(column![content])
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .style(SolidContainer(bg_color))
        .into()
}
