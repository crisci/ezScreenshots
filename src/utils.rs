pub mod utils {
    use std::thread;
    use std::time::Duration;
    use image as img;
    use chrono::{Datelike, Timelike};
    use directories::UserDirs;
    use iced::Command;
    use image::{ColorType, RgbaImage};
    use screenshots::Screen;
    use crate::app::App;

    pub fn screenshot(target: &mut App) {
        thread::sleep(Duration::from_millis(500));
        let screens = Screen::all().unwrap();
        let image = screens[0].capture().unwrap();
        target.screenshot = Some(image);
    }

    #[derive(Clone, Debug)]
    pub struct ExportError(String);


    pub async fn save_to_png(screenshot: RgbaImage, _path: String) -> Result<String, ExportError> {
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

    pub async fn save_to_jpeg(screenshot: RgbaImage, _path: String) -> Result<String, ExportError> {
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

    pub async fn save_to_gif(screenshot: RgbaImage, _path: String) -> Result<String, ExportError> {
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
}