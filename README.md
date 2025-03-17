# Lightview

A blazing-fast, minimalist image viewer built with Rust.

## Design Philosophy

The idea behind Lightview came from using various image viewers on Wayland, each with their own strengths and limitations:

- Swappy has a clean, borderless display (in fullscreen mode) that looks great, but doesn't support navigating between images
- Sxiv offers a minimal viewing experience, but with distracting white/black borders around images
- Eye of GNOME provides useful features but comes with a heavy window decoration that detracts from the viewing experience

Lightview aims to combine the best aspects of these viewers - Swappy's clean display, Sxiv's minimalism, and useful navigation features - while avoiding their limitations. The result is a fast, minimal image viewer that lets you focus entirely on the images.

## Features

- Borderless window that matches image dimensions
- Keyboard navigation between images in the same directory
- Fullscreen mode
- Supports common image formats (JPG, PNG, GIF, WebP, BMP)

## Requirements

- Rust (latest stable)
- GTK4
- Wayland

<!-- ## Building

1. Install GTK4 development libraries:
   ```bash
   # For Arch Linux
   sudo pacman -S gtk4

   # For Ubuntu/Debian
   sudo apt install libgtk-4-dev
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

The compiled binary will be available at `target/release/lightview`

## Usage

```bash
# View a single image
lightview path/to/image.jpg

# Navigate between images:
- Left Arrow: Previous image in directory
- Right Arrow: Next image in directory
- F: Toggle fullscreen mode
- Escape: Exit fullscreen mode
```

## License

MIT -->