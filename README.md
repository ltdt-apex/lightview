# Lightview

A blazing-fast, minimalist image viewer built with Rust, designed for Hyprland user.

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

## Usage

```bash
# View a single image
lightview path/to/image.jpg

# View all images in current directory
lightview .

# View all images in specific directory
lightview path/to/directory
```

### Keyboard Controls

- `Left Arrow`: Previous image
- `Right Arrow`: Next image
- `F`: Toggle fullscreen (gallery mode)
- `Q`: Quit

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](LICENSE)