use iced::{
    widget::{column, container, text},
    Element, Length,
};
use crate::app::message::Message;
use crate::app::utils::colors::hex;
use crate::app::utils::styles::SettingsContainer;

pub fn settings_screen() -> Element<'static, Message> {
    let bg_color = hex("343A40");
    let text_color = hex("F8F9FA");

    let content = text("Settings Page")
        .size(28)
        .style(text_color);

    container(column![content])
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .style(SettingsContainer(bg_color))
        .into()
}
