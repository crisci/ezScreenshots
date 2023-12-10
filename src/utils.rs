pub mod utils {
    use std::fs::{File, self};
    use std::io::{BufReader, Write};
    use std::{thread, path::PathBuf};
    use std::time::Duration;
    use image as img;
    use chrono::{Datelike, Timelike};
    use directories::UserDirs;
    use image::{ColorType, RgbaImage};
    use screenshots::Screen;
    use crate::{app::App, hotkeys::hotkeys_logic::Hotkeys};

    pub fn screenshot(target: &mut App) {
        thread::sleep(Duration::from_millis((target.delay_time() * 1000. + 250.) as u64));
        let screens = Screen::all().unwrap();
        let image = screens[0].capture().unwrap();
        target.set_screenshot(Some(image));
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

    pub fn hotkeys_file_read() -> Result<Hotkeys, String> {
        let hot = Hotkeys::new();
        let serialized = serde_json::to_string(&hot).map_err(|err| format!("Serialization error: {}", err))?;
    
        let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
        let new_dir = PathBuf::from(format!("{}/{}", dir.data_local_dir().to_str().ok_or("Error getting data local dir")?, "ezScreenshots"));
        let file_path = new_dir.join("hotkey.config");
    
        if !new_dir.exists() {
            fs::create_dir_all(&new_dir).map_err(|err| format!("Error creating directory: {}", err))?;
    
            // First time creation
            let mut file = File::create(&file_path).map_err(|err| format!("Error creating file: {}", err))?;
            file.write_all(serialized.as_bytes()).map_err(|err| format!("Error writing to file: {}", err))?;
            println!("File created and serialized: {:?}", file_path);
        } else {
            // File already exists, so read the file
            println!("File already exists.");
            let file = File::open(&file_path).map_err(|err| format!("Error opening file: {}", err))?;
            let reader = BufReader::new(file);
            let hotkeys: Result<Hotkeys, _> = serde_json::from_reader(reader).map_err(|err| format!("Deserialization error: {}", err));
            return Ok(hotkeys?);
        }
    
        Ok(hot)
    }

}

