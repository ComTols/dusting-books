use crate::main_window::MAIN_WINDOW;
use image::GenericImageView;
use tauri::image::Image;
use tauri::menu::{Menu, MenuEvent, MenuItem};
use tauri::tray::TrayIcon;
use tauri::tray::TrayIconBuilder;
use tauri::tray::{MouseButton, TrayIconEvent};
use tauri::{App, AppHandle, Wry};

const TRAY_MENU_ID: &str = "tray_menu";
const TRAY_MENU_QUIT_ID: &str = "tray_menu_quit";

pub struct TrayService {}

impl TrayService {
    pub fn build(&self, app: &mut App) -> tauri::Result<()> {
        let icon = TrayService::load_icon()?;

        let menu = self.menu(app)?;

        TrayIconBuilder::new()
            .icon(icon)
            .menu(&menu)
            .show_menu_on_left_click(false)
            .on_tray_icon_event(|app, event| TrayService::on_tray_icon_event(app, event))
            .on_menu_event(|app, event| TrayService::on_menu_event(app, event))
            .build(app)?;

        Ok(())
    }

    fn load_icon() -> tauri::Result<Image<'static>> {
        let image_bytes = include_bytes!("../../icons/tray-icon.png");
        let image = image::load_from_memory(image_bytes)
            .map_err(|e| tauri::Error::AssetNotFound(e.to_string()))?;
        let (width, height) = image.dimensions();
        let rgba_binary = image.to_rgba8().into_raw();

        Ok(Image::new_owned(rgba_binary, width, height))
    }

    fn menu(&self, app: &mut App) -> tauri::Result<Menu<Wry>> {
        let quit_item = MenuItem::with_id(app, TRAY_MENU_QUIT_ID, "Beenden", true, None::<&str>)?;

        Menu::with_id_and_items(app, TRAY_MENU_ID, &[&quit_item])
    }

    fn on_tray_icon_event(app: &TrayIcon, event: TrayIconEvent) {
        match event {
            TrayIconEvent::DoubleClick {
                button: MouseButton::Left,
                ..
            } => {
                let mut main_window = MAIN_WINDOW.lock().unwrap();
                main_window.open(app.app_handle()).unwrap();
            }
            _ => {}
        }
    }
    fn on_menu_event(_app: &AppHandle, event: MenuEvent) {
        match event.id.as_ref() {
            TRAY_MENU_QUIT_ID => {
                std::process::exit(0);
            }
            _ => {}
        }
    }
}

impl Default for TrayService {
    fn default() -> Self {
        TrayService {}
    }
}
