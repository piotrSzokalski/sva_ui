
use std::sync::Mutex;

use egui_notify::{Toasts};
use once_cell::sync::Lazy;



pub static TOASTS: Lazy<Mutex<Toasts>> = Lazy::new(|| Mutex::new(Toasts::default()));