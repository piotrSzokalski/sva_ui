use std::sync::{Arc, Mutex};

use egui::ahash::{HashMap, HashMapExt};
use simple_virtual_assembler::components::connection::Connection;

use super::custom_logger::CustomLogger;

use once_cell::sync::Lazy;

static CONNECTIONS: Lazy<Arc<Mutex<Vec<Connection>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

static NEXT_CONN_ID: Mutex<usize> = Mutex::new(0);

static CURRENT_CONN_ID: Mutex<Option<usize>> = Mutex::new(None);

pub static CONNECTION_NAMES: Lazy<Mutex<HashMap<usize, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct ConnectionManager {}

impl ConnectionManager {
    pub fn create_connection() {
        {
            let id = *NEXT_CONN_ID.lock().unwrap();
            let conn = Connection::new_with_id(id);
            CONNECTIONS.lock().unwrap().push(conn);

            let conn_name = format!("conn:{}", id);
            CONNECTION_NAMES.lock().unwrap().insert(id, conn_name);
        }
        *NEXT_CONN_ID.lock().unwrap() += 1;
    }

    pub fn get_name(id: usize) -> Option<String> {
        CONNECTION_NAMES.lock().unwrap().get(&id).cloned()
    }

    pub fn set_name(id: usize, new_name: String) {
        *CONNECTION_NAMES.lock().unwrap().get_mut(&id).unwrap() = new_name;
    }

    pub fn get_connections() -> Arc<Mutex<Vec<Connection>>> {
        CONNECTIONS.clone()
    }

    pub fn set_connection(state: Vec<Connection>) {
        CustomLogger::log(&format!("Setting connectinos \n to {:?}", state));
        *CONNECTIONS.lock().unwrap() = state;
    }

    pub fn clear_connections() {
        CONNECTIONS.lock().unwrap().clear();
    }

    pub fn set_current_id(id: Option<usize>) {
        *CURRENT_CONN_ID.lock().unwrap() = id;

        CustomLogger::log(&format!("Setting current connection id to {:?}", id))
    }

    pub fn unset_current_id() {
        *CURRENT_CONN_ID.lock().unwrap() = None;
    }

    pub fn get_current_id_index() -> Option<usize> {
        let id = *CURRENT_CONN_ID.lock().unwrap();
        CONNECTIONS
            .lock()
            .unwrap()
            .iter()
            .position(|c| c.get_id() == id)
    }

    pub fn get_connection_to_current<'a>() -> Option<&'a mut Connection> {
        if let Some(id) = *CURRENT_CONN_ID.lock().unwrap() {
            //return ConnectionManager::get_connection(id);
            return None;
        }
        None
    }
}
