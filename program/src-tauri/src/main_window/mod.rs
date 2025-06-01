use std::sync::{LazyLock, Mutex};
use tauri::{AppHandle, Manager, WebviewWindow, WebviewWindowBuilder, WindowEvent};
use tauri::utils::config::WindowConfig;

const MAIN_WINDOW_LABEL: &str = "main_window";
pub static MAIN_WINDOW: LazyLock<Mutex<MainWindow>> =
    LazyLock::new(|| Mutex::new(MainWindow::default()));

pub struct MainWindow {
    window_config: WindowConfig,
    window: Option<WebviewWindow>,
}

impl MainWindow {
    pub fn open(&mut self, app: &AppHandle) -> tauri::Result<()> {
        if self.window.is_none() {
            let main_window = app.get_webview_window(MAIN_WINDOW_LABEL);
            self.window = match main_window {
                Some(m_window) => Some(m_window),
                None => Some(self.build(app)?),
            }
        }

        if let Some(window) = &self.window {
            window.show()?;
        }

        Ok(())
    }

    fn build(&self, app: &AppHandle) -> tauri::Result<WebviewWindow> {
        let window = WebviewWindowBuilder::from_config(app, &self.window_config)?.build()?;
        let cloned_window = window.clone();
        window.on_window_event(move |event| MainWindow::on_window_event(event, &cloned_window));
        Ok(window)
    }

    fn on_window_event(event: &WindowEvent, window: &WebviewWindow) {
        match event {
            WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                window.hide().unwrap();
            }
            _ => {}
        }
    }
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            window_config: WindowConfig {
                create: false,
                width: 800.0,
                height: 600.0,
                maximized: true,
                label: MAIN_WINDOW_LABEL.to_string(),
                ..WindowConfig::default()
            },
            window: None
        }
    }
}
