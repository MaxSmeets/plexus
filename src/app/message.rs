use super::page::Page;

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(Page),
}
