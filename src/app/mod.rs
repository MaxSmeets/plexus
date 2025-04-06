use iced::{
    executor, Application, Command, Element, Theme,
    widget::{container, row},
};

mod message;
mod page;
pub mod screens;
pub mod components;

use message::Message;
use page::Page;
use screens::{home::home_screen, agents::agents_screen, settings::settings_screen};
use components::sidebar::Sidebar;

pub struct PlexusApp {
    pub current_page: Page,
}

impl Application for PlexusApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                current_page: Page::Home,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Plexus - Enter the Post-Web")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::NavigateTo(page) => self.current_page = page,
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let content = match self.current_page {
            Page::Home => home_screen(),
            Page::Agents => agents_screen(),
            Page::Settings => settings_screen(),
        };

        row![
            Sidebar::new(self.current_page),
            container(content).width(iced::Length::Fill).padding(20)
        ]
        .into()
    }
}
