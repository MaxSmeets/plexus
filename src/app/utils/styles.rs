use iced::widget::{button, container};
use iced::{Background, BorderRadius, Color, Theme};

/// -----------------
/// Container Styles
/// -----------------

/// Custom style for a generic solid container.
#[derive(Debug, Clone, Copy)]
pub struct SolidContainer(pub Color);

impl container::StyleSheet for SolidContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0)),
            border_radius: BorderRadius::from(8.0),
            ..Default::default()
        }
    }
}

impl From<SolidContainer> for iced::theme::Container {
    fn from(style: SolidContainer) -> Self {
        iced::theme::Container::Custom(Box::new(style))
    }
}

/// Custom style for the Agents screen container.
#[derive(Debug, Clone, Copy)]
pub struct AgentsContainer(pub Color);

impl container::StyleSheet for AgentsContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0)),
            border_radius: BorderRadius::from(8.0),
            ..Default::default()
        }
    }
}

impl From<AgentsContainer> for iced::theme::Container {
    fn from(style: AgentsContainer) -> Self {
        iced::theme::Container::Custom(Box::new(style))
    }
}

/// Custom style for the Settings screen container.
#[derive(Debug, Clone, Copy)]
pub struct SettingsContainer(pub Color);

impl container::StyleSheet for SettingsContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0)),
            border_radius: BorderRadius::from(8.0),
            ..Default::default()
        }
    }
}

impl From<SettingsContainer> for iced::theme::Container {
    fn from(style: SettingsContainer) -> Self {
        iced::theme::Container::Custom(Box::new(style))
    }
}

/// Custom style for the Sidebar container.
#[derive(Debug, Clone, Copy)]
pub struct SidebarContainer(pub Color);

impl container::StyleSheet for SidebarContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        // Sidebar typically uses no rounded corners.
        container::Appearance {
            background: Some(Background::Color(self.0)),
            border_radius: BorderRadius::from(0.0),
            ..Default::default()
        }
    }
}

impl From<SidebarContainer> for iced::theme::Container {
    fn from(style: SidebarContainer) -> Self {
        iced::theme::Container::Custom(Box::new(style))
    }
}

/// -----------------
/// Button Style for Sidebar
/// -----------------

/// A custom style for sidebar buttons.
/// Holds a background color and a text color.
#[derive(Debug, Clone, Copy)]
pub struct SidebarButton(pub Color, pub Color);

impl button::StyleSheet for SidebarButton {
    // Set the associated style type to Theme.
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0)),
            text_color: self.1,
            border_radius: BorderRadius::from(4.0),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            shadow_offset: Default::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        // For simplicity, use the same appearance in hovered state.
        self.active(style)
    }
}

impl From<SidebarButton> for iced::theme::Button {
    fn from(style: SidebarButton) -> Self {
        iced::theme::Button::Custom(Box::new(style))
    }
}
