use native_dialog::{MessageDialog, MessageType};
use std::process;
use tracing::info;
use tray_item::{IconSource, TrayItem};

pub fn create_tray() -> () {
    // Platform-specific icon creation
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    let icon = {
        let png_data = include_bytes!("../resources/mouse.png").to_vec();
        IconSource::Data {
            data: png_data,
            height: 16,
            width: 16,
        }
    };

    #[cfg(target_os = "windows")]
    let icon = IconSource::Resource("mouse-icon");

    // Create tray icon
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
    })
    .unwrap();

    // Platform-specific quit handling
    #[cfg(target_os = "macos")]
    {
        let inner = tray.inner_mut();
        inner.add_quit_item("Quit");
        inner.display();
    }

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    {
        // On Linux (ksni) and Windows, add Quit as a regular menu item
        tray.add_menu_item("Quit", || {
            info!("Quitting RMM application...");
            process::exit(0);
        })
        .unwrap();
    }
}
