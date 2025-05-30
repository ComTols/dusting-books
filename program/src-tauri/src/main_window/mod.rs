use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};

const MAIN_WINDOW: &str = "main_window";

pub struct MainWindow {}

impl MainWindow {
    pub fn build(&self, app: &AppHandle) -> tauri::Result<()> {
        WebviewWindowBuilder::new(app, MAIN_WINDOW, WebviewUrl::default())
            .build()?;
        Ok(())
    }
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {}
    }
}