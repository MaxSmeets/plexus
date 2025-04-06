use crate::app::page::Page;

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(Page),
    ChatInputChanged(String),
    SendChatMessage,
}
