use std::sync::Mutex;

use egui_notify::Toasts;
use once_cell::sync::Lazy;
use std::time::Duration;

pub static TOASTS: Lazy<Mutex<Toasts>> = Lazy::new(|| Mutex::new(Toasts::default()));

pub struct ToastsManager {}

impl ToastsManager {
    pub fn show_err(message: String, duration_seconds: usize) {
        TOASTS
            .lock()
            .unwrap()
            .error(message)
            .set_duration(Some(Duration::from_secs(
                duration_seconds.try_into().unwrap_or(5),
            )));
    }
}
