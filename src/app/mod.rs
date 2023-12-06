use iced::subscription::{events, events_with};
use iced::{Application, Command, Element, Renderer, executor, window, Length, alignment, Alignment, ContentFit, Theme, Subscription};
use iced::widget::{container, column, row, text, svg, image, Row};
use iced::window::Mode;
use iced_aw::{ modal };
use screenshots::image::RgbaImage;
use iced_aw::native::Spinner;

use crate::custom_widgets::{image_button};
use crate::hotkeys;
use crate::hotkeys::hotkeys_logic::Hotkeys;
use crate::menu::{top_menu};
use crate::resize::Modal;

use crate::app::SaveState::{Nothing, OnGoing};
use crate::save_as_modal::{Formats, save_as_modal};
use crate::utils::utils::*;

use iced::keyboard::{self, KeyCode, Modifiers};
use iced_native::subscription;
use crate::app::MenuAction::{Save, SaveAs};
use crate::settings_modal::settings_modal;

use iced::event::{self, Event};


#[derive(Default)]
pub struct App {
    pub(crate) screenshot: Option<RgbaImage>,
    resize: bool,
    save_path: String,
    save_state: SaveState,
    //Needed for save as section
    save_as_modal: bool,
    formats: Vec<String>,
    export_format: Formats,
    manual_select: Option<usize>,
    //Settings
    settings_modal: bool,
    delay_time: f32,
    temp: f32,
    //Hotkeys
    hotkeys: Hotkeys
}

impl App {
    pub(crate) fn formats(&self) -> &Vec<String> {
        &self.formats
    }

    pub(crate) fn manual_select(&self) -> Option<usize> {
        self.manual_select
    }

    pub(crate) fn save_state(&self) -> SaveState {
        self.save_state.clone()
    }

    pub(crate) fn delay_time(&self) -> f32 { self.delay_time }

    pub(crate) fn temp(&self) -> f32  { self.temp }

    pub(crate) fn get_screenshot(&self) -> char {
        self.hotkeys.get_screenshot()
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum MenuAction {
    SaveAs,
    Save,
    Settings,
    ShortKeys
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum SaveState {
    #[default]
    Nothing,
    OnGoing,
    Done
}

#[derive(Debug, Clone)]
pub enum Message {
    Screenshot,
    WindowHidden,
    Drop,
    Resize,
    MenuAction(MenuAction),
    ScreenshotSaved(Result<String, ExportError>),
    CloseModal,
    OpenSaveAsModal,
    OpenSettingsModal,
    CancelButtonPressed,
    SaveAsButtonPressed,
    FormatSelected(usize, String),
    Init,
    DelayChanged(f32),
    SettingSave,
    KeyboardComb(char)
}



impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut vec = Vec::with_capacity(10);


        for i in Formats::ALL.iter() {
            vec.push(format!("{i}"))
        }
                    (Self { screenshot: None, resize: false, save_path: "./".to_string(), save_state: SaveState::Nothing, save_as_modal: false, formats: vec, export_format: Formats::Png, manual_select: Some(0), settings_modal: false, delay_time: 0., temp: 0.0, hotkeys: Hotkeys::new()  },
         Command::none())
    }

    fn title(&self) -> String {
        String::from("📷 Screenshots")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.resize = false;
        return match message {
            Message::Init => {
                self.manual_select = Some(0);
                self.save_as_modal = false;
                self.export_format = Formats::Png;
                self.save_state = Nothing;
                Command::none()
            },
            Message::MenuAction(action) => {
                if self.screenshot.is_none() && action != MenuAction::Settings {println!("Screenshot not available"); return Command::none()};
                match action {
                    MenuAction::Save => {
                        let path = self.save_path.clone();
                        let screenshot = self.screenshot.clone().unwrap();
                        self.save_state = OnGoing;
                        Command::perform(save_to_png(screenshot, path), Message::ScreenshotSaved)
                    },
                    MenuAction::SaveAs => Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_|Message::OpenSaveAsModal ),
                    MenuAction::Settings => {
                        self.temp = self.delay_time;
                        Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::OpenSettingsModal) },
                    _ => Command::none()
                }
            },
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
                if res.is_err() {panic!("{:?}", res.err())}
                println!("DONE");
                self.save_state = SaveState::Done;
                Command::perform(tokio::time::sleep(std::time::Duration::from_millis(500)), |_| Message::Init)
            },
            Message::OpenSaveAsModal => { self.settings_modal = false; self.save_as_modal = true; Command::none() },
            Message::OpenSettingsModal => { self.save_as_modal = false; self.settings_modal = true; Command::none() }
            Message::CloseModal => { self.save_as_modal = false; self.settings_modal = false; Command::none() },
            Message::CancelButtonPressed => { self.save_as_modal = false; self.settings_modal = false; Command::none() },
            Message::SaveAsButtonPressed => {
                if self.screenshot.is_none() {println!("Screenshot not available"); return Command::none()};
                let screenshot = self.screenshot.clone().unwrap();
                let path = self.save_path.clone();
                self.save_state = SaveState::OnGoing;
                match self.export_format {
                    Formats::Png => {Command::perform(save_to_png(screenshot, path), Message::ScreenshotSaved)}
                    Formats::Gif => {Command::perform(save_to_gif(screenshot, path), Message::ScreenshotSaved)}
                    Formats::Jpeg => {Command::perform(save_to_jpeg(screenshot, path), Message::ScreenshotSaved)}
                }
            },
            Message::FormatSelected(_, format) => {self.export_format = Formats::from(format); self.manual_select = None; Command::none()},
            Message::DelayChanged(value) => {self.temp = value; Command::none()}
            Message::SettingSave => { self.delay_time = self.temp; self.settings_modal = false; Command::none() },
            Message::KeyboardComb(event)  => {
                if let Some(c) = self.hotkeys.to_message(event) {
                    println!("{:?}", c);
                }
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
            image.padding(5)
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

        let mut bottom_container = Row::new()
            .push(match self.save_state {
                OnGoing => container(Spinner::new())
                    .width(Length::Fill)
                    .center_x()
                    .center_y(),
                SaveState::Done => container(text("Screenshot saved correctly!")),
                _ => container(button_row)
            });
        if self.delay_time > 0. && self.save_state != OnGoing {
            let delay_handle = svg::Handle::from_path(format!(
                "{}/resources/{}.svg",
                env!("CARGO_MANIFEST_DIR"),
                "delay"
            ));

            let delay_svg = svg(delay_handle)
                .height(30)
                .width(30)
                .content_fit(ContentFit::Contain);
            bottom_container = bottom_container.push(container(delay_svg).height(55).width(55).padding(15).center_x().center_y());
        }

        let body = column![
            image_container
                .center_x()
                .center_y(),
            container(
                bottom_container
            )
                .align_x(alignment::Horizontal::Center)
                .width(Length::FillPortion(1))
                .center_x()
        ];

        let overlay = if self.save_as_modal {
            save_as_modal(&self)
        } else if self.settings_modal {
            settings_modal(&self)
        } else { None };

        let content = column![
            menu,
            container(body).width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_x()

        ];
        modal(container(content), overlay)
            .backdrop(Message::CloseModal)
            .on_esc(Message::CloseModal)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        events_with(move |event, _status| match event {
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::CharacterReceived(c) => Some(Message::KeyboardComb(c)),
                _ => None,
            },
            _ => None,
        })
    }

}


