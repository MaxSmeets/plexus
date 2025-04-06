mod app;

use iced::Application;
use app::PlexusApp;

fn main() -> iced::Result {
    PlexusApp::run(iced::Settings::default())
}
