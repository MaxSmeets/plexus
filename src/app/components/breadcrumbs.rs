use iced::widget::{button, row, text};
use iced::Element;

use crate::app::message::Message;
use crate::app::page::Page; // Import Page to use Page::Home

/// Builds a breadcrumb row from a vector of segments.
/// The breadcrumbs always begin with a clickable ".." that navigates to Home.
/// Each additional segment is a tuple (&str, Option<Message>).
/// If a Message is provided, that segment is rendered as a clickable button.
pub fn breadcrumbs(segments: Vec<(&str, Option<Message>)>) -> Element<Message> {
    // Start with the home button as clickable ".."
    let mut row_builder = row![
        button(text(".."))
            .on_press(Message::NavigateTo(Page::Home))
            .padding(2)
    ];

    // Add each segment, preceded by a separator.
    for (label, maybe_msg) in segments {
        row_builder = row_builder.push(text(" / "));
        if let Some(msg) = maybe_msg {
            row_builder = row_builder.push(
                button(text(label))
                    .on_press(msg)
                    .padding(2)
            );
        } else {
            row_builder = row_builder.push(text(label));
        }
    }
    row_builder.into()
}
