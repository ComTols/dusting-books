mod tray_service;
mod main_window;

use crate::tray_service::TrayService;

pub fn run() {

    tauri::Builder::default()
        .setup(|app| {
            TrayService::default().build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("Fehler beim Starten der App");
}
