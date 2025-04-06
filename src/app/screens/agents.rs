use iced::{
    widget::{button, column, container, text},
    Element, Length,
};
use crate::app::message::Message;
use crate::app::page::Page; // Ensure you import Page
use crate::app::utils::colors::hex;
use crate::app::utils::styles::SolidContainer;

pub fn agents_screen() -> Element<'static, Message> {
    let bg_color = hex("343A40");
    let text_color = hex("F8F9FA");

    let content = column![
        text("Agents Screen").size(32).style(text_color),
        text("List of agents will be displayed here.").style(text_color),
        // Button to navigate to the marketplace screen.
        button("Go to Marketplace")
            .on_press(Message::NavigateTo(Page::AgentsMarketplace))
            .padding(10)
    ]
    .padding(20)
    .spacing(10);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(SolidContainer(bg_color))
        .into()
}
