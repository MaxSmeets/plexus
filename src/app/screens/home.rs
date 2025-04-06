use iced::{
    widget::{button, column, container, row, scrollable, text, text_input},
    Element, Length,
};
use crate::app::message::Message;
use crate::app::utils::colors::hex;
use crate::app::utils::styles::SolidContainer;

#[derive(Debug, Clone)]
pub struct ChatScreen {
    pub messages: Vec<String>,
    pub input_value: String,
}

impl ChatScreen {
    pub fn new() -> Self {
        Self {
            messages: vec!["Welcome to Plexus Chat!".into()],
            input_value: String::new(),
        }
    }

    // Use a 'static lifetime so that the Element is owned.
    pub fn view(&self) -> Element<'static, Message> {
        let bg_color = hex("343A40");   // Chat background color
        let text_color = hex("F8F9FA");   // Text color

        // Build a column with each chat message.
        let messages_column = self.messages.iter().fold(column![], |col, msg| {
            col.push(text(msg).style(text_color))
        });

        // Create a scrollable area for messages.
        let scrollable_messages = scrollable(messages_column)
            .height(Length::Fill);

        // Build a row for text input and a "Send" button.
        let input_row = row![
            text_input("Type your message...", &self.input_value)
                .on_input(Message::ChatInputChanged)
                .padding(10)
                .width(Length::Fill),
            button("Send")
                .on_press(Message::SendChatMessage)
                .padding(10)
        ]
        .spacing(10);

        // Combine messages and input into one column.
        let content = column![scrollable_messages, input_row]
            .spacing(20)
            .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(SolidContainer(bg_color))
            .into()
    }
}

// Exported home_screen is now a thin wrapper.
pub fn home_screen() -> Element<'static, Message> {
    // This function will no longer create a new ChatScreen;
    // the ChatScreen is now stored in the app state.
    // For compatibility with existing code, we return a placeholder.
    ChatScreen::new().view()
}
