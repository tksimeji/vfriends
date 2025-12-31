use tauri::{AppHandle, Window, WindowEvent};
mod startup;
mod tray;

pub const AUTOSTART_ARG: &str = "--autostart";

pub fn setup(app: &AppHandle) -> tauri::Result<()> {
    #[cfg(desktop)]
    {
        tray::setup(app)?;
        startup::init(app)?;
    }

    Ok(())
}

pub fn handle_window_event(window: &Window, event: &WindowEvent) {
    #[cfg(desktop)]
    tray::handle_window_event(window, event);
}
