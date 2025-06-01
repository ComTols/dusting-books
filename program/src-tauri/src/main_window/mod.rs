use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};

const MAIN_WINDOW: &str = "main_window";

pub struct MainWindow {}

impl MainWindow {
    pub fn open(&self, app: &AppHandle) -> tauri::Result<()> {
        let main_window = app.get_webview_window(MAIN_WINDOW);

        if main_window.is_none() {
            return self.build(app);
        }

        let _main_window = main_window.unwrap();

        Ok(())
    }

    fn build(&self, app: &AppHandle) -> tauri::Result<()> {
        let window = WebviewWindowBuilder::new(app, MAIN_WINDOW, WebviewUrl::default()).build()?;
        window.on_window_event(|event| MainWindow::on_window_event(event));
        Ok(())
    }

    fn on_window_event(event: &WindowEvent) {
        match event {
            WindowEvent::CloseRequested { api, .. } => api.prevent_close(),
            _ => {}
        }
    }
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {}
    }
}
