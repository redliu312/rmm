use tray_item::{IconSource, TrayItem};
use tracing::info;
use std::process;
use native_dialog::{MessageDialog, MessageType};

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
    let mut tray = TrayItem::new("RMM - Rust Mouse Monitor", icon).unwrap();

    // Add About menu item with native dialog
    tray.add_menu_item("About", || {
        let _ = MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("About RMM")
            .set_text("RMM - Rust Mouse Monitor\n\nAuthor: Red\n\nCreated with LLM help for learning Rust concepts")
            .show_alert();
    }).unwrap();

    tray.add_label("---").unwrap();

    // Add Stop menu item
    tray.add_menu_item("Stop", || {
        info!("Stopping RMM application...");
        println!("RMM stopped by user");
        process::exit(0);
    }).unwrap();

    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();
}
