use std::{collections::HashMap, sync::Mutex, usize};

use egui_modal::Modal;
use once_cell::sync::Lazy;

use super::{custom_logger::CustomLogger, toasts::ToastsManager};

static MODALS: Lazy<Mutex<HashMap<usize, Modal>>> = Lazy::new(|| Mutex::new(HashMap::new()));

static CURRENT_MODAL_ID: Mutex<usize> = Mutex::new(0);

pub struct ModalManager {}

impl ModalManager {
    pub fn add_modal(id: usize, modal: Modal) {
        MODALS.lock().unwrap().insert(id, modal);
    }

    pub fn open_modal() {
        let res = CURRENT_MODAL_ID.lock();
        match res {
            Ok(v) => {
                let id = *v;
                let binding = MODALS.lock().unwrap();
                let result = binding.get(&id);
                match result {
                    Some(modal) => modal.open(),
                    None => {
                        ToastsManager::show_err("Couldn't open modal".to_owned(), 10);
                    }
                }
            }
            Err(err) => ToastsManager::show_err("Couldn't open modal".to_owned(), 10),
        }
    }

    pub fn set_modal(id: usize) {
        if MODALS.lock().unwrap().contains_key(&id) {
            *CURRENT_MODAL_ID.lock().unwrap() = id;
        }
      
    }
}
