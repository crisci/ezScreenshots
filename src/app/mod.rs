use directories::UserDirs;

use iced::subscription::events_with;
use iced::{Application, Command, Element, Renderer, executor, window, Length, alignment, Alignment, ContentFit, Theme, Subscription, font, Font, Point};
use iced::widget::{container, column, row, text, svg, image, Row, responsive, Canvas};
use iced::widget::space::Space;
use iced::window::Mode;
use iced_aw::{floating_element, modal, SelectionList, SelectionListStyles};
use screenshots::image::{RgbaImage, DynamicImage};
use iced_aw::native::Spinner;

use crate::custom_widgets::image_button;
use crate::hotkeys::hotkeys_logic::{Hotkeys, HotkeysMap};
use crate::menu::top_menu;
use crate::resize::Modal;

use crate::app::SaveState::{Nothing, OnGoing};
use crate::modals::save_as_modal::{Formats, save_as_modal};
use crate::utils::utils::*;

use crate::modals::setdefaultpath_modal::setpath_modal;

use iced::keyboard::{self};
use crate::modals::delay_modal::delay_modal;

use iced::event::Event;
use iced::Length::Fill;
use crate::crop::CropArea;
use crate::modals::hotkeys_modal::hotkeys_modal;
use crate::modals::Modals;
use crate::utils::select_path;

use crate::toast::toast_logic::{Toast, Status, Manager, DEFAULT_TIMEOUT};


#[derive(Debug, Default)]
pub struct App {
    screenshot: Option<DynamicImage>,
    resize: bool,
    default_path: String,
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
    temp_hotkeys: Hotkeys,
    hotkeys_modification: HotkeysMap,
    hotkeys_error_message: Option<String>,
    copy_status: CopyState,
    // Modal to be shown
    modal: Modals,
    toasts: Vec<Toast>,
    //Multi monitor
    screens: Vec<String>,
    display_selected: usize,
    manual_display_selection: Option<usize>,
    //Crop mode
    crop_area: (Point, Point),
    crop_mode: bool,
    temp_img: Option<DynamicImage>
}

impl App {
    pub fn new() -> Self {
        let mut vec = Vec::with_capacity(10);

        let hotkeys_saved = match hotkeys_file_read() {
            Ok(hk) => hk,
            _ => Hotkeys::new()
        };

        let path = match default_path_file_read() {
            Ok(dp) => dp,
            _ => format!("{}", UserDirs::new().unwrap().picture_dir().unwrap().to_str().unwrap())
        };


        for i in Formats::ALL.iter() {
            vec.push(format!("{i}"))
        }
        Self {
            screenshot: None,
            resize: false,
            default_path: path.clone(),
            save_path: path,
            save_state: SaveState::Nothing,
            formats: vec,
            export_format: Formats::Png,
            manual_select: Some(0),
            delay_time: 0.,
            temp: 0.0,
            hotkeys: hotkeys_saved.clone(),
            hotkeys_modification: HotkeysMap::None,
            modal: Modals::None,
            hotkeys_error_message: None,
            temp_hotkeys: hotkeys_saved.clone(),
            toasts: vec![],
            display_selected: 0,
            screens: (1..=num_of_screens()).map(|u| u.to_string()).collect(),
            manual_display_selection: Some(0),
            crop_area: (Default::default(), Default::default()),
            copy_status: Default::default(),
            crop_mode: false,
            temp_img: None,
        }
    }


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

    pub(crate) fn temp(&self) -> f32 { self.temp }

    pub(crate) fn temp_hotkeys(&self) -> Hotkeys {
        self.temp_hotkeys.clone()
    }

    pub(crate) fn display_selected(&self) -> usize { self.display_selected }
    pub(crate) fn set_screenshot(&mut self, screenshot: Option<DynamicImage>) {
        self.screenshot = screenshot
    }

    pub(crate) fn get_hotkey_modification(&self) -> HotkeysMap {
        self.hotkeys_modification.clone()
    }

    pub(crate) fn get_hotkeys_error(&self) -> Option<String> {
        self.hotkeys_error_message.clone()
    }

    pub(crate) fn refresh_screens(&mut self) {
        self.screens = (1..=num_of_screens()).map(|u| u.to_string()).collect();
    }
}


#[derive(Default, Debug, Clone, PartialEq)]
pub enum SaveState {
    #[default]
    Nothing,
    OnGoing,
    Done,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum CopyState {
    #[default]
    Nothing,
    OnGoing,
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
    OpenDelayModal,
    OpenSetPathModal,
    SaveAsButtonPressed,
    FormatSelected(usize, String),
    Init,
    DelayChanged(f32),
    DelaySave,
    HotkeysSave,
    KeyboardComb(char),
    OpenHotkeysModal,
    CopyToClipboard,
    CopySuccess(Result<(), CopyError>),
    ChangeHotkey(HotkeysMap),
    Quit,
    PathSelected,
    SetDefaultPath,
    MonitorSelected(usize, String),
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
    AddToast(String, String, Status),
    CloseToast(usize),
    ButtonReleased(Point, Point),
    Crop,
    CropModeSwitch,
    Reset,
    None,
}

#[derive(Debug)]
pub enum BootstrapApp {
    Loading,
    Loaded(App),
}

async fn load() -> Result<(), String> {
    Ok(())
}


impl Application for BootstrapApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            BootstrapApp::Loading,
            Command::batch(vec![
                font::load(iced_aw::graphics::icons::ICON_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("ezScreenshots")
    }


    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match self {
            BootstrapApp::Loading => {
                if let Message::Loaded(_) = message {
                    *self = BootstrapApp::Loaded(App::new()) // TODO: Implement default for app
                }
            }
            BootstrapApp::Loaded(app) => {
                app.refresh_screens();
                return match message {
                    Message::Init => {
                        app.save_path = app.default_path.clone();
                        app.manual_select = Some(0);
                        app.modal = Modals::None;
                        app.export_format = Formats::Png;
                        app.save_state = Nothing;
                        app.temp = app.delay_time;
                        app.temp_hotkeys = app.hotkeys.clone();
                        app.manual_display_selection = Some(app.display_selected);
                        Command::none()
                    }
                    Message::MenuAction(action) => {
                        if app.screenshot.is_none() && action != Modals::DelayTime && action != Modals::Hotkeys && action != Modals::SetPath {
                            return Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::AddToast("Error".into(), "Screenshot not available".into(), Status::Danger));
                        };
                        match action {
                            Modals::Save => {
                                return match app.save_state {
                                    Nothing => {
                                        let path = app.save_path.clone();
                                        let screenshot = app.screenshot.clone().unwrap();
                                        app.save_state = OnGoing;
                                        Command::perform(save_to_png(screenshot, path), Message::ScreenshotSaved)
                                    }
                                    _ => Command::none()
                                };
                            }
                            Modals::SaveAs => Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::OpenSaveAsModal),
                            Modals::DelayTime => {
                                app.temp = app.delay_time;
                                Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::OpenDelayModal)
                            }
                            Modals::Hotkeys => Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::OpenHotkeysModal),
                            Modals::SetPath => Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::OpenSetPathModal),
                            _ => Command::none()
                        }
                    }
                    Message::Screenshot => {
                        let change_mode = window::change_mode(window::Mode::Hidden);
                        let wait = Command::perform(tokio::time::sleep(std::time::Duration::from_millis(20)), |_| Message::WindowHidden);
                        Command::batch(vec![change_mode, wait])
                    }
                    Message::WindowHidden => {
                        let command = match screenshot(app) {
                            Err(_) => {
                                app.screens = (1..=num_of_screens()).map(|u| u.to_string()).collect();
                                app.display_selected = 0;
                                app.manual_display_selection = Some(0);
                                Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::AddToast("Error".into(), "The selected display could be unplugged".into(), Status::Danger))
                            }
                            Ok(_) => {
                                app.manual_display_selection = Some(app.display_selected);
                                app.temp_img = app.screenshot.clone();
                                Command::none()
                            }
                        };
                        let change_mode = window::change_mode(Mode::Windowed);
                        Command::batch(vec![command, change_mode])
                    }
                    Message::Drop => {
                        app.screenshot = None;
                        Command::none()
                    }
                    Message::ScreenshotSaved(res) => {
                        if res.is_err() { panic!("{:?}", res.err()) }
                        let success = Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::AddToast("Success".into(), "Screenshot saved correctly!".into(), Status::Success));
                        let init = Command::perform(tokio::time::sleep(std::time::Duration::from_millis(500)), |_| Message::Init);
                        let commands = vec![success, init];
                        Command::batch(commands)
                    }
                    Message::OpenSaveAsModal => {
                        app.modal = Modals::SaveAs;
                        Command::none()
                    }
                    Message::OpenDelayModal => {
                        app.modal = Modals::DelayTime;
                        Command::none()
                    }
                    Message::OpenHotkeysModal => {
                        app.modal = Modals::Hotkeys;
                        Command::none()
                    }
                    Message::CloseModal => {
                        if app.modal == Modals::SaveAs || app.modal == Modals::SetPath { app.save_path = app.default_path.clone() }
                        app.temp = app.delay_time;
                        app.temp_hotkeys = app.hotkeys.clone();
                        app.modal = Modals::None;
                        app.hotkeys_modification = HotkeysMap::None;
                        Command::none()
                    }
                    Message::OpenSetPathModal => {
                        app.modal = Modals::SetPath;
                        Command::none()
                    }
                    Message::SaveAsButtonPressed => {
                        if app.screenshot.is_none() {
                            return Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::AddToast("Error".into(), "Screenshot not available".into(), Status::Danger));
                        };
                        let screenshot = app.screenshot.clone().unwrap();
                        let path = app.save_path.clone();
                        app.save_state = SaveState::OnGoing;
                        match app.export_format {
                            Formats::Png => { Command::perform(save_to_png(screenshot, path), Message::ScreenshotSaved) }
                            Formats::Gif => { Command::perform(save_to_gif(screenshot, path), Message::ScreenshotSaved) }
                            Formats::Jpeg => { Command::perform(save_to_jpeg(screenshot, path), Message::ScreenshotSaved) }
                        }
                    }
                    Message::FormatSelected(_, format) => {
                        app.export_format = Formats::from(format);
                        app.manual_select = None;
                        Command::none()
                    }
                    Message::DelayChanged(value) => {
                        app.temp = value;
                        Command::none()
                    }
                    Message::DelaySave => {
                        app.delay_time = app.temp;
                        app.modal = Modals::None;
                        Command::none()
                    }
                    Message::KeyboardComb(event) => {
                        return if app.hotkeys_modification == HotkeysMap::None {
                            if app.modal == Modals::None {
                                if let Some(m) = app.hotkeys.to_message(event) {
                                    Command::perform(async {}, |_| { m })
                                } else {
                                    Command::none()
                                }
                            } else {
                                Command::none()
                            }
                        } else {
                            //Change the hotkey
                            //Check that the char inserted is not already used
                            if app.temp_hotkeys.char_already_used(event) {
                                app.hotkeys_error_message = Some("Combination already in use".to_string());
                            } else {
                                //Assign temp structure
                                app.temp_hotkeys.assign_new_value(event, app.hotkeys_modification.clone());
                                app.hotkeys_modification = HotkeysMap::None;
                                app.hotkeys_error_message = None
                            }
                            Command::none()
                        };
                    }
                    Message::ChangeHotkey(hotkey) => {
                        app.hotkeys_modification = hotkey;
                        Command::none()
                    }
                    Message::CopyToClipboard => {
                        if app.copy_status == CopyState::OnGoing { return Command::none(); };
                        app.copy_status = CopyState::OnGoing;
                        Command::perform(copy_to_clipboard(app.screenshot.clone()), Message::CopySuccess)
                    }
                    Message::CopySuccess(res) => {
                        app.copy_status = CopyState::Nothing;
                        return match res {
                            Ok(_) => {
                                Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::AddToast("Success".into(), "Screenshot copied to clipboard!".into(), Status::Success))
                            }/*set copy message*/
                            _ => Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::AddToast("Error".into(), "Error while copying to clipboard".into(), Status::Danger))
                        };
                    }
                    Message::HotkeysSave => {
                        app.hotkeys = app.temp_hotkeys.clone();
                        app.temp_hotkeys = app.hotkeys.clone();
                        match app.hotkeys.save_hotkeys() {
                            _ => ()
                        };
                        app.modal = Modals::None;
                        Command::none()
                    }
                    Message::Quit => {
                        std::process::exit(0)
                    }
                    Message::PathSelected => {
                        app.save_path = select_path().unwrap();
                        Command::none()
                    }
                    Message::SetDefaultPath => {
                        app.default_path = app.save_path();
                        save_default_path(app.default_path.clone()).unwrap();
                        app.modal = Modals::None;
                        Command::none()
                    }
                    Message::MonitorSelected(index, _) => {
                        app.display_selected = index;
                        app.manual_display_selection = None;
                        Command::none()
                    }
                    Message::AddToast(title, body, level) => {
                        let toast = Toast {
                            title,
                            body,
                            status: level,
                        };
                        if !app.toasts.contains(&toast) { app.toasts.push(toast) }
                        Command::none()
                    },
                    Message::CloseToast(index) => {
                        app.toasts.remove(index);
                        Command::none()
                    },
                    Message::ButtonReleased(p1, p2) => {
                        let screenshot = app.screenshot.clone().unwrap();
                        app.crop_area = (
                            Point::new(p1.x * (screenshot.width() as f32), p1.y * (screenshot.height() as f32)),
                            Point::new(p2.x * (screenshot.width() as f32), p2.y * (screenshot.height() as f32)),
                        );
                        Command::none()
                    },
                    Message::Crop => {
                       //TODO: implement with dynamic image
                        let width = app.crop_area.1.x - app.crop_area.0.x;
                        let height = app.crop_area.1.y - app.crop_area.0.y;
                        match (width > 0., height >0.) {
                            (true,true) => app.screenshot = Some(app.temp_img.clone().expect("Temp image not found").crop(app.crop_area.0.x as u32, app.crop_area.0.y as u32, width.abs() as u32, height.abs() as u32)),
                            (true,false) => app.screenshot = Some(app.temp_img.clone().expect("Temp image not found").crop(app.crop_area.0.x as u32, app.crop_area.1.y as u32, width.abs() as u32, height.abs() as u32)),
                            (false,true) => app.screenshot = Some(app.temp_img.clone().expect("Temp image not found").crop(app.crop_area.1.x as u32, app.crop_area.0.y as u32, width.abs() as u32, height.abs() as u32)),
                            (false,false) => app.screenshot = Some(app.temp_img.clone().expect("Temp image not found").crop(app.crop_area.1.x as u32, app.crop_area.1.y as u32, width.abs() as u32, height.abs() as u32))
                        }
                        app.temp_img = app.screenshot.clone();
                        // Reset crop mode after cropping
                        app.crop_mode = false;
                        Command::none()
                    },
                    Message::CropModeSwitch => {

                        if !app.crop_mode {
                            // Enabled
                            app.crop_area = (Default::default(), Default::default());
                        } else {
                            // Disabled when cancel
                            app.temp_img = app.screenshot.clone();
                        }
                        app.crop_mode = !app.crop_mode;
                        Command::none()
                    }
                    _ => Command::none()
                };
            }
        }
        Command::none()
    }


    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        return match self {
            BootstrapApp::Loading => container(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_y()
                .center_x()
                .into(),
            BootstrapApp::Loaded(app) => {
                let menu = top_menu(app);
                let selection_list: SelectionList<_, Message> = SelectionList::new_with(
                    &app.screens,
                    Message::MonitorSelected,
                    14.0,
                    5.0,
                    SelectionListStyles::Default,
                    app.manual_display_selection,
                    Font::default(),
                ).width(Length::Shrink).height(Length::Shrink);

                let sel_column = container(
                    column![
                    text("Monitor"),
                    container(selection_list).width(60).height(55).center_x()
                ]
                );
                let mut image: Element<Message> = if let Some(screenshot) = &app.screenshot
                {
                    image(image::Handle::from_pixels(
                        screenshot.width(),
                        screenshot.height(),
                        screenshot.clone().as_bytes().to_vec(),
                    ))
                        .content_fit(ContentFit::Contain)
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                        .into()
                } else {
                    text("Press the button to take a screenshot!").into()
                };

                if let Some(temp) = &app.temp_img {
                    if app.crop_mode {
                        image = iced::widget::image(image::Handle::from_pixels(
                            temp.width(),
                            temp.height(),
                            temp.clone().as_bytes().to_vec(),
                        ))
                            .content_fit(ContentFit::Contain)
                            .width(Length::Shrink)
                            .height(Length::Shrink)
                            .into()
                    }
                };

                let floating_image = floating_element(
                    image,
                    responsive(move |size| {
                        Canvas::new(CropArea::from_point(size.height, size.width, app.crop_mode))
                            .width(Fill)
                            .height(Fill)
                            .into()
                    }),
                );

                let image_container = container(
                    floating_image
                ).center_x().center_y()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y();

                let screenshot_button = image_button("screenshot", "Screenshot", Message::Screenshot);
                let button_row = if app.screenshot.is_some() {
                    let crop_button = image_button("crop", "Crop", Message::CropModeSwitch);
                    let delete_button = image_button("delete", "Delete", Message::Drop);
                    let save_button = image_button("save", "Save", Message::MenuAction(Modals::Save));
                    if app.screens.len() == 1 {
                        row![crop_button].spacing(10).push(delete_button).spacing(10).push(screenshot_button).spacing(10).push(save_button).align_items(Alignment::Center)
                    } else {
                        row![Space::new(30, 55)].spacing(10).push(crop_button).spacing(10).push(delete_button).spacing(10).push(screenshot_button).spacing(10).push(sel_column).spacing(10).push(save_button).align_items(Alignment::Center)
                    }
                } else {
                    if app.screens.len() == 1 {
                        row![Space::new(55, 55)].spacing(10).push(screenshot_button).align_items(Alignment::Center)
                    } else {
                        row![Space::new(55, 55)].spacing(10).push(Space::new(30, 55)).spacing(10)
                            .push(screenshot_button).spacing(10).push(sel_column).align_items(Alignment::Center)
                    }
                };

                let mut bottom_container = Row::new()
                    .push(match app.save_state {
                        OnGoing => container(Spinner::new())
                            .width(Length::Fill)
                            .center_x()
                            .center_y(),
                        _ => container(button_row)
                    });
                if app.delay_time > 0. && app.save_state != OnGoing {
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

                if app.crop_mode {
                    let crop_confirm_button = image_button("crop_confirm", "Confirm", Message::Crop);
                    let crop_cancel_button = image_button("crop_cancel", "Cancel", Message::CropModeSwitch);
                    bottom_container = row![crop_cancel_button].spacing(10).push(crop_confirm_button).align_items(Alignment::Center);
                    //TODO: implement the reset button
                }


                let body = column![
                    image_container,
                    container(
                        bottom_container
                    ).padding([0,0,8,0])
                        .align_x(alignment::Horizontal::Center)
                        .width(Length::FillPortion(1))
                        .center_x()
                ];

                let overlay = match app.modal {
                    Modals::SetPath => setpath_modal(&app),
                    Modals::Hotkeys => hotkeys_modal(&app),
                    Modals::SaveAs => save_as_modal(&app),
                    Modals::DelayTime => delay_modal(&app),
                    _ => None
                };

                let content = column![
                    menu,
                    container(body).width(Length::Fill)
                    .height(Length::Fill)
                    .padding(5)
                    .center_x()
        
                ];
                let content2 = column![modal(container(content), overlay)
                    .backdrop(Message::CloseModal)
                    .on_esc(Message::CloseModal)
                    .align_y(alignment::Vertical::Center)
                    ];
                if app.crop_mode {
                    Manager::new(content2, &app.toasts, Message::CloseToast)
                        .timeout(DEFAULT_TIMEOUT)
                        .into()
                } else {
                    Manager::new(content2, &app.toasts, Message::CloseToast)
                        .timeout(DEFAULT_TIMEOUT)
                        .into()
                }
            }
        };
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


