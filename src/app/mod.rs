use iced::{Application, Command, Element, Renderer, executor, window, Length, alignment, Alignment, ContentFit, Theme, theme, color, Color};
use iced::overlay::menu::Menu;
use iced::theme::Container;
use iced::widget::{container, column, row, text, Button, button, svg, image, Text, Row};
use iced::widget::pick_list::mouse_interaction;
use iced::window::Mode;
use iced_aw::{Card, CloseCondition, ItemHeight, ItemWidth, menu_bar, modal, PathHighlight};
use screenshots::image::RgbaImage;
use screenshots::Screen;
use iced::widget::horizontal_space;

use crate::custom_widgets::{CustomTheme, image_button};
use crate::menu::{top_menu};
use crate::resize::Modal;

use ::image as img;
use ::image::ColorType;
use chrono::{Datelike, Timelike};
use directories::UserDirs;
use iced::alignment::Horizontal;
use tracing_subscriber::fmt::format;
use crate::save_as_modal::save_as_modal;
use crate::utils::*;
use crate::utils::utils::*;


#[derive(Default)]
pub struct App {
    pub(crate) screenshot: Option<RgbaImage>,
    resize: bool,
    title: String,
    theme: iced::Theme,
    save_path: String,
    save_as_modal: bool
}

#[derive(Default, Debug)]
struct State {
    show_modal: bool,
    last_message: Option<Message>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Screenshot,
    WindowHidden,
    Drop,
    Resize,
    MenuAction(String),
    ScreenshotSaved(Result<String, ExportError>),
    CloseSaveAsModal,
    OpenSaveAsModal,
    CancelButtonPressed,
    OkButtonPressed
}


impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self { screenshot: None, resize: false, title: "".to_string(), theme: Default::default(), save_path: "./".to_string(), save_as_modal: false }, Command::none())
    }

    fn title(&self) -> String {
        String::from("📷 Screenshots")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.resize = false;
        return match message {
            Message::MenuAction(action) => {
                if self.screenshot.is_none() {println!("Screenshot not available"); return Command::none()};
                let screenshot = self.screenshot.clone().unwrap();
                let path = self.save_path.clone();
                match action.as_str() {
                    "Save" => Command::perform(save_to_png(screenshot, path), Message::ScreenshotSaved),
                    "Save as..." => Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_|Message::OpenSaveAsModal),
                    _ => Command::none()
                }
            }
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
            },
            Message::Resize => {
                self.resize = true;
                Command::none()
            },
            Message::ScreenshotSaved(res) => {
                println!("DONE");
                Command::none()
            },
            Message::OpenSaveAsModal => { self.save_as_modal = true; Command::none() },
            Message::CloseSaveAsModal => { self.save_as_modal = false; Command::none() },
            Message::CancelButtonPressed => { self.save_as_modal = false; Command::none() },
            Message::OkButtonPressed => { self.save_as_modal = false; Command::none() },
            _ => Command::none()
        };

    }

    fn view(&self) ->  Element<'_, Self::Message, Renderer<Self::Theme>> {
        let menu = top_menu(self);
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
        let mut image = row![image];
        if self.resize {
            println!("Resize on");
            let handle = svg::Handle::from_path(format!(
                "{}/resources/{}.svg",
                env!("CARGO_MANIFEST_DIR"),
                "resize"
            ));

            let svg = svg(handle)
                .height(Length::from(self.screenshot.clone().unwrap().height() as u16))
                .width(Length::from(self.screenshot.clone().unwrap().width() as u16));
            image = row![Modal::new(image, svg)];
        }

        let image_container = container(
            image.padding(10)
        ).center_x().center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        let screenshot_button = image_button("screenshot", "Screenshot", Message::Screenshot);
        let mut button_row = row![
                screenshot_button
            ].spacing(10).align_items(Alignment::Center);

        if self.screenshot.is_some() {
            let drag_button = image_button("drag", "Resize", Message::Resize);
            let delete_button = image_button("delete", "Delete", Message::Drop);
                button_row = row![drag_button].push(button_row).push(delete_button).spacing(10);
        }

        let body = column![
            image_container
                .center_x()
                .center_y(),
            container(button_row)
                .align_x(alignment::Horizontal::Center)
                .width(Length::FillPortion(1))
                .center_x()
        ];
        let overlay = if self.save_as_modal {
            save_as_modal()
        } else {
            None
        };

        let content = column![
            menu,
            container(body).width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()

        ];
        modal(container(content), overlay)
            .backdrop(Message::CloseSaveAsModal)
            .on_esc(Message::CloseSaveAsModal)
            .align_y(alignment::Vertical::Center)
            .into()
    }


}


