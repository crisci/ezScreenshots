use iced::Application;
use iced::window::Position;

mod app;
mod custom_widgets;
mod resize;
mod menu;
mod utils;
mod hotkeys;
mod modals;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init(); // TODO: remove in production
    let settings: iced::Settings<()> = iced::Settings {
        window: iced::window::Settings {
            icon: iced::window::icon::from_file(format!("{}/resources/icon.png", env!("CARGO_MANIFEST_DIR"))).ok(),
            position: Position::Centered,
            ..iced::window::Settings::default()
        },
        id: Some("ezScreenshots".to_string()),
        ..Default::default()
    };
    app::BootstrapApp::run(settings)
}
