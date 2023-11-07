use iced::{Settings, window, Application};

mod capture;

pub fn main() -> iced::Result {
    let settings: Settings<()> = Settings {
        window: window::Settings {
            size: (500, 250),
            resizable: false,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    };
    capture::Capture::run(settings)
}
