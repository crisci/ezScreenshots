use iced::{Settings, window, Application};

mod capture;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();
    capture::Capture::run(iced::Settings::default())
}
