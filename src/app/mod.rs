use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use iced::{Application, Command, Element, Renderer, executor, window, Length, alignment, Alignment, ContentFit, Theme, theme, color, Color};
use iced::overlay::menu::Menu;
use iced::theme::Container;
use iced::widget::{container, column, row, text, Button, button, svg, image};
use iced::widget::pick_list::mouse_interaction;
use iced::window::Mode;
use iced_aw::{CloseCondition, ItemHeight, ItemWidth, menu_bar, PathHighlight};
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
use tracing_subscriber::fmt::format;


#[derive(Default)]
pub struct App {
    screenshot: Option<RgbaImage>,
    resize: bool,
    title: String,
    theme: iced::Theme,
    save_path: String
}

#[derive(Debug, Clone)]
pub enum Message {
    Screenshot,
    WindowHidden,
    Drop,
    Resize,
    MenuAction(String),
    ScreenshotSaved(Result<String, ExportError>),
}


impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self { screenshot: None, resize: false, title: "".to_string(), theme: Default::default(), save_path: "./".to_string() }, Command::none())
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
                    "Save" => Command::perform(save_to_gif(screenshot, path), Message::ScreenshotSaved),
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
            }
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
        let content = column![
            menu,
            container(body).width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()

        ];
        container(
           content
        ).into()
    }


}

fn screenshot(target: &mut App) {
    thread::sleep(Duration::from_millis(500));
    let screens = Screen::all().unwrap();
    let image = screens[0].capture().unwrap();
    target.screenshot = Some(image);
}

#[derive(Clone, Debug)]
struct ExportError(String);

async fn save_to_png(screenshot: RgbaImage, path: String) -> Result<String, ExportError> {
    let user_dir = UserDirs::new();
    let time = chrono::Utc::now();
    let string_time = format!("{}{}{}{}{}", time.year(), time.month(), time.day(), time.hour(), time.second());
    let path = format!("{}/SCRN_{}.png", user_dir.clone().unwrap().picture_dir().unwrap().to_str().unwrap(), string_time);
    tokio::task::spawn_blocking(move || {
        img::save_buffer(
            &path,
            &screenshot.clone().into_raw(),
            screenshot.width(),
            screenshot.height(),
            ColorType::Rgba8,
        )
            .map(|_| path)
            .map_err(|err| ExportError(format!("{err:?}")))
    })
        .await
        .expect("Blocking task to finish")
}

async fn save_to_jpeg(screenshot: RgbaImage, path: String) -> Result<String, ExportError> {
    let user_dir = UserDirs::new();
    let time = chrono::Utc::now();
    let string_time = format!("{}{}{}{}{}", time.year(), time.month(), time.day(), time.hour(), time.second());
    let path = format!("{}/SCRN_{}.jpeg", user_dir.clone().unwrap().picture_dir().unwrap().to_str().unwrap(), string_time);
    tokio::task::spawn_blocking(move || {
        img::save_buffer(
            &path,
            &screenshot.clone().into_raw(),
            screenshot.width(),
            screenshot.height(),
            ColorType::Rgba8,
        )
            .map(|_| path)
            .map_err(|err| ExportError(format!("{err:?}")))
    })
        .await
        .expect("Blocking task to finish")
}

async fn save_to_gif(screenshot: RgbaImage, path: String) -> Result<String, ExportError> {
    let user_dir = UserDirs::new();
    let time = chrono::Utc::now();
    let string_time = format!("{}{}{}{}{}", time.year(), time.month(), time.day(), time.hour(), time.second());
    let path = format!("{}/SCRN_{}.gif", user_dir.clone().unwrap().picture_dir().unwrap().to_str().unwrap(), string_time);
    tokio::task::spawn_blocking(move || {
        img::save_buffer(
            &path,
            &screenshot.clone().into_raw(),
            screenshot.width(),
            screenshot.height(),
            ColorType::Rgba8,
        )
            .map(|_| path)
            .map_err(|err| ExportError(format!("{err:?}")))
    })
        .await
        .expect("Blocking task to finish")
}
