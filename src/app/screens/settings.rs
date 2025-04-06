use iced::{
    widget::{column, container, text},
    Element, Length,
};
use crate::app::message::Message;
use crate::app::utils::colors::hex;
use crate::app::utils::styles::SolidContainer;

pub fn settings_screen() -> Element<'static, Message> {
    let bg_color = hex("343A40");
    let text_color = hex("F8F9FA");

    let content = column![
        text("Settings").size(32).style(text_color),
        text("Configuration options go here.").style(text_color)
    ]
    .padding(20)
    .spacing(10);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(SolidContainer(bg_color))
        .into()
}
