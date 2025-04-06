use iced::{
    executor, Application, Command, Element, Theme, Length,
    widget::{container, row, column},
};

mod message;
mod page;
pub mod screens;
pub mod components;
pub mod utils;

use message::Message;
use page::Page;
use screens::{
    home::home_screen,
    agents::agents_screen,
    agents_marketplace::marketplace_screen,
    settings::settings_screen,
};
use components::sidebar::Sidebar;
use components::breadcrumbs::breadcrumbs;
use crate::app::utils::colors::hex;
use crate::app::utils::styles::SidebarContainer;

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
            Message::ChatInputChanged(_) | Message::SendChatMessage => {
                // Chat-related messages are handled within the chat screen state.
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let bg_color = hex("212529");

        // Build breadcrumbs if we're on a subpage like AgentsMarketplace.
        let breadcrumb = match self.current_page {
            Page::AgentsMarketplace => {
                // "agents" is clickable (navigates back to Agents),
                // "marketplace" is displayed as active.
                breadcrumbs(vec![
                    ("agents", Some(Message::NavigateTo(Page::Agents))),
                    ("marketplace", None),
                ])
            }
            _ => breadcrumbs(vec![]),
        };

        // Choose the content based on the current page.
        let content = match self.current_page {
            Page::Home => home_screen(),
            Page::Agents => agents_screen(),
            Page::AgentsMarketplace => marketplace_screen(),
            Page::Settings => settings_screen(),
        };

        container(
            row![
                Sidebar::new(self.current_page),
                container(column![
                    breadcrumb,
                    container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(SidebarContainer(bg_color))
                ])
                
            ]
            .height(Length::Fill)
        )
        .height(Length::Fill)
        .into()
    }
}
