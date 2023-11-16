use iced::Application;

mod capture;
mod custom_widgets;
mod resize;
mod menu;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();
    capture::Capture::run(iced::Settings::default())
}
