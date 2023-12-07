# ezScreenshot

## Overview

ezScreenshot is a cross-platform screen grabbing utility that simplifies the process of capturing screenshots. It is designed to be compatible with Windows, macOS, and Linux, providing a user-friendly interface and a variety of features to enhance the screen grabbing experience.

## Features

1. **Platform Support**
   - Compatible with Windows, macOS, and Linux.

2. **User Interface (UI)**
   - Intuitive and user-friendly interface for easy navigation.

3. **Selection Options**
   - Allows users to custom-select a screen area with a click and drag motion.
   - Supports subsequent adjustments to the selected area.

4. **Hotkey Support**
   - Customizable hotkeys for quick screen grabbing.
   - Users can set up their preferred shortcut keys.

5. **Output Format**
   - Supports multiple output formats: .png, .jpg, .gif.
   - Option to copy the screen grab to the clipboard.

6. **Bonus Features**

   a. **Delay Timer**
      - Supports a delay timer function for scheduled screen grabs.

   b. **Save Options**
      - Allows users to specify the default save location for screen grabs.
      - Supports automatic saving with predefined naming conventions.

   c. **Multi-monitor Support**
      - Recognizes and handles multiple monitors independently.
      - Users can grab screens from any connected displays.

## Usage

1. Download the latest release for your operating system from the [Releases](https://github.com/crisci/ezScreenshot/releases) page.
2. Install the application following the platform-specific instructions.
3. Launch ezScreenshot and use the intuitive UI to capture screens or customize settings.
4. Explore hotkey configurations, selection options, and bonus features for an enhanced experience.

## Configuration

### Hotkeys

To configure hotkeys, navigate to the settings menu and set your preferred shortcut keys for quick screen grabbing.

### Save Options

Specify the default save location and naming conventions for your screen grabs in the settings menu.

### Delay Timer

Set up a delay timer for scheduled screen grabs. Access this feature in the settings menu and define the desired delay.

## Build Instructions

If you prefer to build ezScreenshot from source, follow these instructions:

```bash
# Clone the repository
git clone https://github.com/crisci/ezScreenshot.git

# Navigate to the project directory
cd ezScreenshot

# Build the application
cargo run
