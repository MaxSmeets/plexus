use iced::{
    widget::{column, container, text},
    Element, Length,
};
use crate::app::message::Message;
use crate::app::utils::colors::hex;
use crate::app::utils::styles::AgentsContainer;

pub fn agents_screen() -> Element<'static, Message> {
    let bg_color = hex("343A40");   // Agents screen background
    let text_color = hex("F8F9FA");

    let content = text("Agents Dashboard")
        .size(28)
        .style(text_color);

    container(column![content])
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .style(AgentsContainer(bg_color))
        .into()
}
