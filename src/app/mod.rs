use iced::subscription::{events_with};
use iced::{Application, Command, Element, Renderer, executor, window, Length, alignment, Alignment, ContentFit, Theme, Subscription};
use iced::widget::{container, column, row, text, svg, image, Row};
use iced::widget::space::Space;
use iced::window::Mode;
use iced_aw::{ modal };
use screenshots::image::RgbaImage;
use iced_aw::native::Spinner;

use crate::custom_widgets::{image_button};
use crate::hotkeys::hotkeys_logic::{Hotkeys, HotkeysMap};
use crate::menu::{top_menu};
use crate::resize::Modal;

use crate::app::SaveState::{Nothing, OnGoing};
use crate::modals::save_as_modal::{Formats, save_as_modal};
use crate::utils::utils::*;

use iced::keyboard::{self};
use crate::modals::settings_modal::settings_modal;

use iced::event::{Event};
use crate::modals::hotkeys_modal::hotkeys_modal;
use crate::modals::Modals;


#[derive(Default)]
pub struct App {
    pub(crate) screenshot: Option<RgbaImage>,
    resize: bool,
    save_path: String,
    save_state: SaveState,
    //Needed for save as section
    formats: Vec<String>,
    export_format: Formats,
    manual_select: Option<usize>,
    //Settings
    delay_time: f32,
    temp: f32,
    //Hotkeys
    hotkeys: Hotkeys,
    _hotkeys_modification: HotkeysMap,
    // Modal to be shown
    modal: Modals
}

impl App {
    pub(crate) fn formats(&self) -> &Vec<String> {
        &self.formats
    }

    pub(crate) fn manual_select(&self) -> Option<usize> {
        self.manual_select
    }

    pub(crate) fn save_path(&self) -> String {
        self.save_path.clone()
    }

    pub(crate) fn save_state(&self) -> SaveState {
        self.save_state.clone()
    }

    pub(crate) fn delay_time(&self) -> f32 { self.delay_time }

    pub(crate) fn temp(&self) -> f32  { self.temp }

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
    MenuAction(Modals),
    ScreenshotSaved(Result<String, ExportError>),
    CloseModal,
    OpenSaveAsModal,
    OpenSettingsModal,
    SaveAsButtonPressed,
    FormatSelected(usize, String),
    Init,
    DelayChanged(f32),
    SettingSave,
    KeyboardComb(char),
    OpenHotkeysModal
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
                    (Self {
                        screenshot: None,
                        resize: false,
                        save_path: "./".to_string(),
                        save_state: SaveState::Nothing,
                        formats: vec,
                        export_format: Formats::Png,
                        manual_select: Some(0),
                        delay_time: 0.,
                        temp: 0.0,
                        hotkeys: Hotkeys::new(),
                        _hotkeys_modification: HotkeysMap::Save,
                        modal: Modals::None
                    },
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
                self.modal = Modals::None;
                self.export_format = Formats::Png;
                self.save_state = Nothing;
                Command::none()
            },
            Message::MenuAction(action) => {
                if self.screenshot.is_none() && action != Modals::Settings && action != Modals::Hotkeys {println!("Screenshot not available"); return Command::none()};
                match action {
                    Modals::Save => {
                        let path = self.save_path.clone();
                        let screenshot = self.screenshot.clone().unwrap();
                        self.save_state = OnGoing;
                        Command::perform(save_to_png(screenshot, path), Message::ScreenshotSaved)
                    },
                    Modals::SaveAs => Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_|Message::OpenSaveAsModal ),
                    Modals::Settings => {
                        self.temp = self.delay_time;
                        Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::OpenSettingsModal) },
                    Modals::Hotkeys => Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_|Message::OpenHotkeysModal ),
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
            Message::OpenSaveAsModal => { self.modal = Modals::SaveAs; Command::none() },
            Message::OpenSettingsModal => { self.modal = Modals::Settings; Command::none() },
            Message::OpenHotkeysModal => { self.modal = Modals::Hotkeys; Command::none()}
            Message::CloseModal => { self.modal = Modals::None; Command::none() },
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
            Message::SettingSave => { self.delay_time = self.temp; self.modal = Modals::None; Command::none() },
            Message::KeyboardComb(event)  => {
                if let Some(m) = self.hotkeys.to_message(event) {
                    println!("{:?}", m);
                    return Command::perform(async {}, |_| {m});
                } else {
                    return Command::none();
                }
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
        let button_row = if self.screenshot.is_some() {
            let drag_button = image_button("drag", "Resize", Message::Resize);
            let delete_button = image_button("delete", "Delete", Message::Drop);
            let save_button = image_button("save", "Save", Message::MenuAction(Modals::Save));
            row![drag_button].spacing(10).push(delete_button).spacing(10).push(screenshot_button).spacing(10).push(save_button).align_items(Alignment::Center)
        } else {
            row![Space::new(55, 55)].spacing(10).push(screenshot_button).align_items(Alignment::Center)
        };
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
            bottom_container = bottom_container.spacing(10).push(container(delay_svg).height(55).width(55).padding(15).center_x().center_y());
        } else {
            bottom_container = bottom_container.spacing(10).push(Space::new(55, 55)).align_items(Alignment::Center);
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

        let overlay = match self.modal {
            Modals::Hotkeys => hotkeys_modal(&self),
            Modals::SaveAs => save_as_modal(&self),
            Modals::Settings => settings_modal(&self),
            _ => None
        };

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


