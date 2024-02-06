use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use simple_virtual_assembler::components::connection::Connection;

use super::custom_logger::CustomLogger;

use once_cell::sync::Lazy;

pub static CONNECTIONS: Lazy<Arc<Mutex<Vec<Connection>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

pub static NEXT_CONN_ID: Mutex<usize> = Mutex::new(1);

static CURRENT_CONN_ID: Mutex<Option<usize>> = Mutex::new(None);

pub static CURRENT_CONN_ID_FOR_RENAME: Mutex<Option<usize>> = Mutex::new(None);

pub static CONNECTION_NAMES: Lazy<Mutex<HashMap<usize, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static DISCONNECT_MODE: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));

pub static NEW_CONNECTION_NAME_BUFFER: Lazy<String> = Lazy::new(|| (String::from("")));

//pub static BUFFER_FOR_CONNECTION_REF

pub static RELOAD_CONNECTION: Mutex<bool> = Mutex::new(false);

pub static ANOTHER_ID_BUFFER: Mutex<Option<usize>> = Mutex::new(None);

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

    pub fn get_names() -> HashMap<usize, String> {
        CONNECTION_NAMES.lock().unwrap().clone()
    }

    pub fn set_name(id: Option<usize>, new_name: String) {
        if let Some(id) = id {
            *CONNECTION_NAMES.lock().unwrap().get_mut(&id).unwrap() = new_name;
        }
    }

    pub fn set_names(names: HashMap<usize, String>) {
        *CONNECTION_NAMES.lock().unwrap() = names;
    }

    pub fn get_connections() -> Arc<Mutex<Vec<Connection>>> {
        CONNECTIONS.clone()
    }

    pub fn remove_connection(id: Option<usize>) {
        CustomLogger::log(&format!("removing connection {:?}", id));

        CONNECTIONS.lock().unwrap().retain(|c| c.get_id() != id);
        *RELOAD_CONNECTION.lock().unwrap() = true;
    }

    pub fn set_connection(state: Vec<Connection>) {
        let id = match state.last() {
            Some(conn) => conn.get_id(),
            None => Some(0),
        };

        CustomLogger::log(&format!("Setting connectinos \n to {:?}", state));
        *CONNECTIONS.lock().unwrap() = state;

        *NEXT_CONN_ID.lock().unwrap() = id.unwrap_or(0) + 1;
    }

    pub fn clear_connections() {
        CONNECTIONS.lock().unwrap().clear();
    }

    pub fn set_current_id(id: Option<usize>) {
        CustomLogger::log(&format!(
            "Setting current connection id to {:?}",
            id.clone()
        ));
        {
            *CURRENT_CONN_ID.lock().unwrap() = id;
        }
        CustomLogger::log(&format!(
            "NOW IT IS SET TO    {:?}",
            CURRENT_CONN_ID.lock().unwrap()
        ));
        *DISCONNECT_MODE.lock().unwrap() = false;
    }

    pub fn unset_current_id() {
        *CURRENT_CONN_ID.lock().unwrap() = None;
    }

    pub fn get_current_id() -> Option<usize> {
        *CURRENT_CONN_ID.lock().unwrap()
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
        if let Some(_id) = *CURRENT_CONN_ID.lock().unwrap() {
            //return ConnectionManager::get_connection(id);
            return None;
        }
        None
    }

    pub fn in_disconnect_mode() -> bool {
        *DISCONNECT_MODE.lock().unwrap()
    }

    pub fn toggle_disconnect_mode() {
        let mut current = false;
        *CURRENT_CONN_ID.lock().unwrap() = None;
        {
            current = !*DISCONNECT_MODE.lock().unwrap();
        }
        *DISCONNECT_MODE.lock().unwrap() = current;
    }

    pub fn get_connection_index_by_id(id: Option<usize>) -> Option<usize> {
        CONNECTIONS
            .lock()
            .unwrap()
            .iter()
            .position(|c| c.get_id() == id)
    }

    pub fn get_connection_by_id(id: Option<usize>) -> Option<&'static mut Connection> {
        let mut index = None;
        {
            index = CONNECTIONS
                .lock()
                .unwrap()
                .iter()
                .position(|c| c.get_id() == id);
        }
        if let Some(_i) = index {
            //  return CONNECTIONS.lock().unwrap().get_mut(i);
            return None;
        }
        None
    }
}
