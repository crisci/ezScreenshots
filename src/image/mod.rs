use eframe::IconData;

pub fn load_icon() -> eframe::IconData {
	let (icon_rgba, icon_width, icon_height) = {
		let icon = include_bytes!("../../assets/icon.png");
		let image = IconData::try_from_png_bytes(icon)
			.expect("Failed to open icon path");
		let (width, height) = (image.width, image.height);
		let rgba = image.rgba;
		(rgba, width, height)
	};
	
	eframe::IconData {
		rgba: icon_rgba,
		width: icon_width,
		height: icon_height,
	}
}