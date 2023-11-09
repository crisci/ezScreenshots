use std::thread;
use std::time::Duration;
use iced::{Length, theme, color, Application, Command, Element, Renderer, executor, Theme, window};
use iced::theme::TextInput;
use iced::widget::{button, container, column, svg, row, horizontal_space, text, text_input, Row, Column};
use iced::window::Mode;
use screenshots::Screen;
use iced_aw::{number_input, style::NumberInputStyles};

#[derive(Default)]
pub struct Capture {
    screens: Vec<Screen>,
    timer: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Screenshot(usize),
    WindowHidden,
}

impl Application for Capture {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self { screens: Screen::all().unwrap(), timer: 0 }, Command::none())
    }

    fn title(&self) -> String {
        String::from("📷 Screenshots")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        return match message {
            Message::Screenshot(n) => {
                if self.timer > 0 {
                    thread::sleep(Duration::from_secs(self.timer));
                }
                let change_mode = window::change_mode(window::Mode::Hidden);
                thread::spawn(move || {
                    screenshot(n);
                });
                let wait = Command::perform(tokio::time::sleep(std::time::Duration::from_secs(1)), |_| Message::WindowHidden);
                Command::batch(vec![change_mode, wait])
            },
            Message::WindowHidden => {
                window::change_mode(Mode::Windowed)
            }
        };
    }

    fn view(&self) ->  Element<'_, Self::Message, Renderer<Self::Theme>> {

        let monitor1 = svg::Handle::from_path(format!(
            "{}/resources/monitor1.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg1 = svg(monitor1).width(Length::Fill).height(Length::Fill)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(0xffffff)),
        }));
        let mut content = Row::new();
        if self.screens.iter().count() == 1 {
            content = Row::new().push(image_button(svg1, 0));
        } else {
            let monitor2 = svg::Handle::from_path(format!(
                "{}/resources/monitor2.svg",
                env!("CARGO_MANIFEST_DIR")
            ));
            let svg2 = svg(monitor2).width(Length::Fill).height(Length::Fill)
                .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                    color: Some(color!(0xffffff)),
                }));
            content = Row::new()
                .push(image_button(svg1, 0))
                .push(horizontal_space(10))
                .push(image_button(svg2, 1));
        }

        container(
            content
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

}

fn screenshot(target_screen: usize) {
    let screens = Screen::all().unwrap();
    let image = screens[target_screen].capture().unwrap();
    image.save(format!("./monitor-{}.png", target_screen)).unwrap();
}

fn image_button<'a>(image_svg: svg::Svg, target_screen: usize) -> iced::widget::Button<'a, Message, > {
    button(
        image_svg
    ).on_press(Message::Screenshot(target_screen))
        .style(theme::Button::Primary)
        .width(65)
        .height(65)
}