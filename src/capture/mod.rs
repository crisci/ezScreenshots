use std::thread;
use std::time::Duration;
use iced::{Application, Command, Element, Renderer, executor, Theme, window, Length, alignment, Alignment, ContentFit};
use iced::widget::{container, column, row, text, image};
use iced::window::Mode;
use screenshots::image::RgbaImage;
use screenshots::Screen;

use crate::custom_widgets::image_button;

#[derive(Default)]
pub struct Capture {
    screenshot: Option<RgbaImage>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Screenshot,
    WindowHidden,
    Drop
}

impl Application for Capture {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self { screenshot: None }, Command::none())
    }

    fn title(&self) -> String {
        String::from("📷 Screenshots")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        return match message {
            Message::Screenshot => {
                let change_mode = window::change_mode(window::Mode::Hidden);
                let wait = Command::perform(tokio::time::sleep(std::time::Duration::from_millis(20)), |_| Message::WindowHidden);
                Command::batch(vec![change_mode, wait])
            },
            Message::WindowHidden => {
                screenshot(self);
                window::change_mode(Mode::Windowed)
            },
            Message::Drop => {
                self.screenshot = None;
                Command::none()
            }
        };
    }

    fn view(&self) ->  Element<'_, Self::Message, Renderer<Self::Theme>> {
        let image: Element<Message> = if let Some(screenshot) = &self.screenshot
        {
            image(image::Handle::from_pixels(
                screenshot.width(),
                screenshot.height(),
                screenshot.clone().into_raw(),
            ))
                .content_fit(ContentFit::Contain)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        } else {
            text("Press the button to take a screenshot!").into()
        };

        let image = container(image)
            .padding(10)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .center_x()
            .center_y();

        let mut button_row = row![
                image_button("screenshot", "Screenshot", Message::Screenshot),
            ].spacing(10).align_items(Alignment::Center);

        if self.screenshot.is_some() {
            let delete_button = image_button("delete", "Delete", Message::Drop);
            button_row = button_row.push(delete_button);
        }


        let content = column![
            image,
            container(button_row)
                .align_x(alignment::Horizontal::Center)
                .width(Length::FillPortion(1))
                .center_x()
        ];
        container(
           content
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .into()
    }

}

fn screenshot(target: &mut Capture) {
    thread::sleep(Duration::from_millis(500));
    let screens = Screen::all().unwrap();
    let image = screens[0].capture().unwrap();
    target.screenshot = Some(image);
}
