use crate::error::{Result, RmmError};
use crate::icon_data::{ICON_DATA, ICON_WIDTH, ICON_HEIGHT};
use tray_item::{IconSource, TrayItem};
use tracing::info;

pub fn create_tray() -> () {
    // Create tray icon using binary icon data
    let mut tray = TrayItem::new("Tray Example", IconSource::Resource("")).unwrap();

    tray.add_label("Tray Label").unwrap();

    tray.add_menu_item("Hello", || {
        println!("Hello!");
    }).unwrap();

    let mut inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();

}
