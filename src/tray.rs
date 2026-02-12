use crate::error::{Result, RmmError};
use tray_item::{IconSource, TrayItem};
use tracing::info;

pub fn create_tray() -> () {
    // Load PNG data directly (macOS NSImage can handle PNG format)
    let png_data = include_bytes!("../resources/mouse.png").to_vec();

    // Create icon source with PNG data
    let icon = IconSource::Data {
        data: png_data,
        height: 16,
        width: 16,
    };

    // Create tray icon using PNG data
    let mut tray = TrayItem::new("Tray Example", icon).unwrap();

    tray.add_label("Tray Label").unwrap();

    tray.add_menu_item("Hello", || {
        println!("Hello!");
    }).unwrap();

    let mut inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();

}
