use std::{collections::HashMap, sync::Mutex, usize};

use egui_modal::Modal;
use once_cell::sync::Lazy;

use super::toasts::ToastsManager;

static MODALS: Lazy<Mutex<HashMap<usize, Modal>>> = Lazy::new(|| Mutex::new(HashMap::new()));

static CURRENT_MODAL_ID: Mutex<Option<usize>> = Mutex::new(None);

pub static MODAL_TEXT_EDIT_BUFFER: Mutex<String> = Mutex::new(String::new());

pub static MODAL_BUFFER_VALUE_I32: Mutex<Option<i32>> = Mutex::new(None);

pub static MODAL_INDEX_BUFFER: Mutex<Option<usize>> = Mutex::new(None);

// why not?
pub static RAM_ID: Mutex<Option<usize>> = Mutex::new(None);

pub struct ModalManager {}

impl ModalManager {
    pub fn add_modal(id: usize, modal: Modal) {
        MODALS.lock().unwrap().insert(id, modal);
    }

    pub fn should_display_modal() -> bool {
        CURRENT_MODAL_ID.lock().unwrap().is_some()
    }

    /// opens modal if current modal id is NOT None
    pub fn open_modal() {
        if !ModalManager::should_display_modal() {
            return;
        }
        let res = CURRENT_MODAL_ID.lock();
        match res {
            Ok(v) => {
                let id = v.unwrap();
                let binding = MODALS.lock().unwrap();
                let result = binding.get(&id);
                match result {
                    Some(modal) => modal.open(),
                    None => {
                        ToastsManager::show_err("Couldn't open modal".to_owned(), 10);
                    }
                }
            }
            Err(_err) => ToastsManager::show_err("Couldn't open modal(2)".to_owned(), 10),
        }
    }

    pub fn set_modal(id: usize) {
        if MODALS.lock().unwrap().contains_key(&id) {
            *CURRENT_MODAL_ID.lock().unwrap() = Some(id);
        }
    }

    pub fn unset_current_modal() {
        *CURRENT_MODAL_ID.lock().unwrap() = None;
    }
}
