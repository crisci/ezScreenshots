use nfd::{open_pick_folder, Response};


pub mod utils {
    use std::fs::{File, self};
    use std::io::{BufReader, Write};
    use std::{thread, path::PathBuf};
    use std::borrow::Cow;
    use std::time::Duration;
    use anyhow;
    use image as img;
    use chrono::{Datelike, Timelike};
    use directories::UserDirs;
    use image::{ColorType, EncodableLayout, RgbaImage};
    use screenshots::Screen;
    use crate::{app::App, hotkeys::hotkeys_logic::Hotkeys};
    use gif::{Frame,Encoder};
    use arboard::{Clipboard, ImageData, Error};

    pub fn screenshot(target: &mut App) -> Result<(), anyhow::Error> {
        thread::sleep(Duration::from_millis((target.delay_time() * 1000. + 250.) as u64));
        let screens = Screen::all().expect("Monitor not recognized");
        if target.display_selected() > screens.len() - 1 { return Err(anyhow::Error::msg("Out of range screens"))};
        let image = screens[target.display_selected()].capture()?;
        target.set_screenshot(Some(image));
        Ok(())
    }

    #[derive(Clone, Debug)]
    pub struct ExportError(String);


    pub async fn save_to_png(screenshot: RgbaImage, path: String) -> Result<String, ExportError> {
        let time = chrono::Utc::now();
        let string_time = format!("{}{}{}{}{}", time.year(), time.month(), time.day(), time.hour(), time.second());
        let path_image = format!("{}/SCRN_{}.png", path, string_time);
        tokio::task::spawn_blocking(move || {
            img::save_buffer(
                &path_image,
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

    pub async fn save_to_jpeg(screenshot: RgbaImage, path: String) -> Result<String, ExportError> {
        let time = chrono::Utc::now();
        let string_time = format!("{}{}{}{}{}", time.year(), time.month(), time.day(), time.hour(), time.second());
        let path_image = format!("{}/SCRN_{}.jpeg", path, string_time);
        tokio::task::spawn_blocking(move || {
            img::save_buffer(
                &path_image,
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

    pub async fn save_to_gif(screenshot: RgbaImage, path: String) -> Result<String, ExportError> {
        let frame = Frame::from_rgba_speed(screenshot.width() as u16, screenshot.height() as u16, &mut screenshot.into_raw(),30);
        let time = chrono::Utc::now();
        let string_time = format!("{}{}{}{}{}", time.year(), time.month(), time.day(), time.hour(), time.second());
        let path_image = format!("{}/SCRN_{}.gif", path, string_time);
        let mut file_out = File::create(path_image.clone()).unwrap();
        tokio::task::spawn_blocking(move || {
            let mut encoder = Encoder::new(&mut file_out, frame.width, frame.height, &[]).unwrap();
        encoder.write_frame(&frame)
            .map(|_| path_image)
            .map_err(|err| ExportError(format!("{err:?}")))
    })
    .await
    .expect("Blocking task to finish")

    }

    pub fn hotkeys_file_read() -> Result<Hotkeys, String> {
        let hot = Hotkeys::new();
        let serialized = serde_json::to_string(&hot).map_err(|err| format!("Serialization error: {}", err))?;
    
        let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
        let new_dir = PathBuf::from(format!("{}/{}", dir.data_local_dir().to_str().ok_or("Error getting data local dir")?, "ezScreenshot"));
        let file_path = new_dir.join("hotkey.config");
    
        if !new_dir.exists() {
            fs::create_dir_all(&new_dir).map_err(|err| format!("Error creating directory: {}", err))?;
    
            // First time creation
            let mut file = File::create(&file_path).map_err(|err| format!("Error creating file: {}", err))?;
            file.write_all(serialized.as_bytes()).map_err(|err| format!("Error writing to file: {}", err))?;
        } else {
            // File already exists, so read the file
            let file = File::open(&file_path).map_err(|err| format!("Error opening file: {}", err))?;
            let reader = BufReader::new(file);
            let hotkeys: Result<Hotkeys, _> = serde_json::from_reader(reader).map_err(|err| format!("Deserialization error: {}", err));
            return Ok(hotkeys?);
        }
    
        Ok(hot)
    }

 pub fn default_path_file_read() -> Result<String, String> {
        let df = format!("{}", UserDirs::new().clone().unwrap().picture_dir().unwrap().to_str().unwrap());
        let serialized = serde_json::to_string(&df).map_err(|err| format!("Serialization error: {}", err))?;

        let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
        let new_dir = PathBuf::from(format!("{}/{}", dir.data_local_dir().to_str().ok_or("Error getting data local dir")?, "ezScreenshots"));
        let file_path = new_dir.join("default_path.config");

        if !new_dir.exists() {
            fs::create_dir_all(&new_dir).map_err(|err| format!("Error creating directory: {}", err))?;

            // First time creation
            let mut file = File::create(&file_path).map_err(|err| format!("Error creating file: {}", err))?;
            file.write_all(serialized.as_bytes()).map_err(|err| format!("Error writing to file: {}", err))?;
        } else {
            // File already exists, so read the file
            let file = File::open(&file_path).map_err(|err| format!("Error opening file: {}", err))?;
            let reader = BufReader::new(file);
            let default_path: Result<String, _> = serde_json::from_reader(reader).map_err(|err| format!("Deserialization error: {}", err));
            return Ok(default_path?);
        }

        Ok(df)
    }
    pub fn save_default_path(path: String) -> Result<(), String> {
        let dir = directories::BaseDirs::new().ok_or("Error getting base directories")?;
        let new_dir = PathBuf::from(format!("{}/{}", dir.data_local_dir().to_str().ok_or("Error getting data local dir")?, "ezScreenshots"));
        let file_path = new_dir.join("default_path.config");

        if !new_dir.exists() {
            fs::create_dir_all(&new_dir).map_err(|err| format!("Error creating directory: {}", err))?;
        }
        // File already exists, so save the file
        let mut file = File::create(&file_path).map_err(|err| format!("Error creating file: {}", err))?;
        let serialized = serde_json::to_string(&path).map_err(|err| format!("Serialization error: {}", err))?;
        file.write_all(serialized.as_bytes()).map_err(|err| format!("Error writing to file: {}", err))?;

        Ok(())
    }

pub fn copy_to_clipboard(image: &Option<RgbaImage>) -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = Clipboard::new()?;
        let binding = image.clone();
        return match binding {
            Some(b) => {
                let img = ImageData {
                    width: b.width() as usize,
                    height: b.height() as usize,
                    bytes: Cow::from(b.as_bytes())
                };
                ctx.set_image(img)?;
                Ok(())
            },
            _ => Err(Box::new(Error::ContentNotAvailable))
        };
    }

    pub fn num_of_screens() -> usize {
        let screen = Screen::all().unwrap();
        screen.len()
    }

 }
    pub fn select_path() -> Option<String>{
        let result = open_pick_folder(None);
        match result {
            Ok(Response::Okay(folder_path)) => {
                Some(folder_path)
            },
            Ok(Response::OkayMultiple(_)) => {
                None
            },
            Ok(Response::Cancel) => {
                None
            },
            Err(_) => {
                panic!("Error selection folder");
            }
        }
    }
