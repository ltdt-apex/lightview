use gtk::prelude::*;
use gtk::{gdk, gio, glib};
use gdk_pixbuf::Pixbuf;
use std::path::{Path, PathBuf};
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

const SUPPORTED_FORMATS: &[&str] = &["jpg", "jpeg", "png", "gif", "webp", "bmp"];
const MAX_WINDOW_SIZE_SCALE: f64 = 0.9;

struct ImageViewer {
    window: gtk::ApplicationWindow,
    picture: gtk::Picture,
    current_path: Option<PathBuf>,
    current_dir: Option<PathBuf>,
    is_fullscreen: bool,
}

impl ImageViewer {
    fn new(app: &gtk::Application) -> Rc<RefCell<Self>> {
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .decorated(false)
            .resizable(false)
            .build();

        // Create an overlay to hold both background and foreground
        let overlay = gtk::Overlay::new();
        
        // Create a transparent background
        let background = gtk::Box::new(gtk::Orientation::Vertical, 0);
        background.set_hexpand(true);
        background.set_vexpand(true);
        
        // Set background to be fully transparent
        background.add_css_class("transparent-bg");

        let css_provider = gtk::CssProvider::new();
        css_provider.load_from_data("
            * { background-color: transparent; }
            picture { 
                border: 2px solid black;
            }
        ");
        
        if let Some(display) = gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &css_provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
        
        // Main picture (original size)
        let picture = gtk::Picture::new();
        picture.set_can_shrink(true); // Keep original size
        picture.set_keep_aspect_ratio(true);
        picture.set_halign(gtk::Align::Center);
        picture.set_valign(gtk::Align::Center);
        picture.add_css_class("bordered-image");

        // Set up the overlay with background and centered picture
        overlay.set_child(Some(&background));
        overlay.add_overlay(&picture);

        window.set_child(Some(&overlay));

        let viewer = Rc::new(RefCell::new(Self {
            window,
            picture,
            current_path: None,
            current_dir: None,
            is_fullscreen: false,
        }));

        viewer.borrow().setup_keyboard_events(Rc::clone(&viewer));
        viewer
    }

    fn setup_keyboard_events(&self, viewer: Rc<RefCell<ImageViewer>>) {
        let key_controller = gtk::EventControllerKey::new();

        key_controller.connect_key_pressed(move |_, keyval, _, _| {
            let mut viewer = viewer.borrow_mut();
            viewer.handle_key_press(keyval);
            glib::Propagation::Stop
        });

        self.window.add_controller(key_controller);
    }

    fn handle_key_press(&mut self, keyval: gdk::Key) {
        match keyval {
            gdk::Key::Left => self.load_adjacent_image(-1),
            gdk::Key::Right => self.load_adjacent_image(1),
            gdk::Key::f => self.toggle_fullscreen(),
            gdk::Key::q | gdk::Key::Q | gdk::Key::Escape => self.window.close(),
            _ => (),
        }
    }

    fn toggle_fullscreen(&mut self) {
        self.is_fullscreen = !self.is_fullscreen;

        if self.is_fullscreen {
            self.window.fullscreen();
        } else {
            self.window.unfullscreen();
        }

        if let Some(ref path) = self.current_path {
            self.resize_window_to_image(path);
        }
    }

    fn resize_window_to_image(&self, path: &Path) {
        if let Ok(pixbuf) = Pixbuf::from_file(path) {
            let mut width = pixbuf.width();
            let mut height = pixbuf.height();

            // Get the default display and its primary monitor
            if let Some(display) = gdk::Display::default() {
                if let Some(surface) = self.window.surface() {
                    if let Some(monitor) = display.monitor_at_surface(&surface) {
                        let geometry = monitor.geometry();
                        let max_width = geometry.width();
                        let max_height = geometry.height();
                        
                        let width_scale = max_width as f64 * MAX_WINDOW_SIZE_SCALE / width as f64;
                        let height_scale = max_height as f64 * MAX_WINDOW_SIZE_SCALE / height as f64;
                        
                        // Use the smaller scale to maintain aspect ratio
                        let scale = width_scale.min(height_scale);

                        if scale < 1.0 {
                            width = (width as f64 * scale) as i32;
                            height = (height as f64 * scale) as i32;
                        }
                    }
                }
            }
            
            // Reset any previous size constraints
            self.window.set_size_request(-1, -1);
            
            // Set the window size to match the image
            self.window.set_default_size(width, height);
        }
    }

    fn get_image_files_in_dir(dir: &Path) -> Vec<PathBuf> {
        match fs::read_dir(dir) {
            Ok(entries) => {
                let mut files: Vec<_> = entries
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        let path = entry.path();
                        if path.is_file() && is_supported_format(&path) {
                            Some(path)
                        } else {
                            None
                        }
                    })
                    .collect();
                files.sort();
                files
            }
            Err(e) => {
                eprintln!("Error reading directory {}: {}", dir.display(), e);
                Vec::new()
            }
        }
    }

    fn load_path<P: AsRef<Path>>(&mut self, path: P) {
        let path = path.as_ref();
        if path.is_dir() {
            self.current_dir = Some(path.to_path_buf());
            let files = Self::get_image_files_in_dir(path);
            if let Some(first_image) = files.first() {
                self.load_image(first_image);
            } else {
                eprintln!("No supported images found in directory: {}", path.display());
            }
        } else if path.is_file() {
            if is_supported_format(path) {
                self.current_dir = path.parent().map(|p| p.to_path_buf());
                self.load_image(path);
            } else {
                eprintln!("Unsupported file format: {}", path.display());
            }
        } else {
            eprintln!("Path does not exist: {}", path.display());
        }
    }

    fn load_image<P: AsRef<Path>>(&mut self, path: P) {
        let path = path.as_ref();
        match Pixbuf::from_file(path) {
            Ok(_) => {
                let file = gio::File::for_path(path);
                self.picture.set_file(Some(&file));
                self.current_path = Some(path.to_path_buf());
                println!("Viewing: {}", path.display());
                
                self.resize_window_to_image(path);
                
                self.window.present();
            }
            Err(e) => {
                eprintln!("Error loading image {}: {}", path.display(), e);
            }
        }
    }

    fn load_adjacent_image(&mut self, offset: i32) {
        if let Some(ref dir) = self.current_dir {
            let files = Self::get_image_files_in_dir(dir);
            if let Some(current_path) = &self.current_path {
                if let Some(current_index) = files.iter().position(|p| p == current_path) {
                    let new_index = (current_index as i32 + offset).rem_euclid(files.len() as i32) as usize;
                    if let Some(new_path) = files.get(new_index) {
                        self.load_image(new_path);
                    }
                }
            }
        }
    }
}

fn is_supported_format(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| SUPPORTED_FORMATS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

fn main() {
    println!("Program starting...");
    
    let application = gtk::Application::builder()
        .application_id("ltdt.lightview")
        .flags(gio::ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    println!("Registering command line handler...");
    
    application.connect_command_line(|app, command_line| {
        println!("Command line handler executing!");
        println!("Arguments received: {:?}", command_line.arguments());
        
        let viewer = ImageViewer::new(app);
        let args = command_line.arguments();

        if args.len() <= 1 {
            println!("No arguments, using current directory");
            let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            viewer.borrow_mut().load_path(current_dir);
        } else {
            println!("Processing argument: {:?}", args.get(1).unwrap());
            let path_str = args.get(1).unwrap().to_str().unwrap();
            let path = if path_str == "." {
                std::env::current_dir().unwrap()
            } else {
                PathBuf::from(path_str)
            };
            viewer.borrow_mut().load_path(path);
        }

        command_line.set_exit_status(0);
        0
    });

    println!("Starting GTK application...");
    application.run();
    println!("Application finished."); // This won't print until the app closes
}
