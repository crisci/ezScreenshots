use iced::Application;

mod app;
mod custom_widgets;
mod resize;
mod menu;
mod utils;
mod save_as_modal;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();
    app::App::run(iced::Settings::default())
}
