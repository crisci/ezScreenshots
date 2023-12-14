use iced::Application;
use iced::window::Position;

mod app;
mod custom_widgets;
mod resize;
mod menu;
mod utils;
mod hotkeys;
mod modals;
mod toast;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init(); // TODO: remove in production
    let settings: iced::Settings<()> = iced::Settings {
        window: iced::window::Settings {
            icon: iced::window::icon::from_file(format!("{}/resources/icon.png", env!("CARGO_MANIFEST_DIR"))).ok(),
            position: Position::Centered,
            size: (800, 600),
            min_size: Some((475, 500)),
            ..iced::window::Settings::default()
        },
        id: Some("ezScreenshots".to_string()),
        ..Default::default()
    };
    app::BootstrapApp::run(settings)
}
