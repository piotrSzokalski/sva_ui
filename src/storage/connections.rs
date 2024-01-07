use std::sync::{Arc, Mutex};

use egui::ahash::HashMap;
use simple_virtual_assembler::components::connection::Connection;

use super::custom_logger::CustomLogger;

use once_cell::sync::Lazy;

static CONNECTIONS: Lazy<Arc<Mutex<Vec<Connection>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

#[derive(Clone, PartialEq, Debug)]
pub enum ConnectingState {
    NONE,
    Connecting,
    Disconnecting,
}


pub static CONNECTING_STATE: Mutex<ConnectingState> = Mutex::new(ConnectingState::NONE);

static NEXT_CONN_ID: Mutex<usize> = Mutex::new(0);

static CURRENT_CONN_ID: Mutex<Option<usize>> = Mutex::new(None);

pub struct ConnectionManager {
    //connection_names: HashMap<String, u32>
}

impl ConnectionManager {
    pub fn create_connection() {
        {
            let id = *NEXT_CONN_ID.lock().unwrap();
            let conn = Connection::new_with_id(id);
            CONNECTIONS.lock().unwrap().push(conn);
        }
        *NEXT_CONN_ID.lock().unwrap() += 1;
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

    pub fn set_current_id(id: usize) {
        *CURRENT_CONN_ID.lock().unwrap() = Some(id);
        CustomLogger::log(&format!("Setting current connection id to {}", id))
    }

    pub fn unset_current_id() {
        *CURRENT_CONN_ID.lock().unwrap() = None;
    }

    pub fn can_connect() -> bool {
        CONNECTING_STATE.lock().unwrap().clone() == ConnectingState::Connecting
            && *CURRENT_CONN_ID.lock().unwrap() != None
    }

    pub fn toggle_start_connecting() {
        let state = CONNECTING_STATE.lock().unwrap().clone();
        match state {
            ConnectingState::NONE => {
                *CONNECTING_STATE.lock().unwrap() = ConnectingState::Connecting
            }
            ConnectingState::Connecting => {
                *CONNECTING_STATE.lock().unwrap() = ConnectingState::NONE
            }
            ConnectingState::Disconnecting => {
                *CONNECTING_STATE.lock().unwrap() = ConnectingState::Disconnecting
            }
        }
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
    // pub fn get_connection2(id: usize) -> Option< &'a mut Connection> {
    //     let conns: Vec<Connection> = CONNECTIONS
    //         .lock()
    //         .unwrap()
    //         .iter()
    //         .filter(|c| {
    //             let x = *c;
    //             let c_id = x.clone().get_id();
    //             c_id == Some(id)
    //         })
    //         .cloned()
    //         .collect();

    //     if conns.len() == 1 {
    //         let x = conns.get(0).unwrap().clone();
    //         Some(&mut x)
    //     } else {
    //         None
    //     }
    // }

    // pub fn get_connection<'a>(id: usize) -> Option<&'a mut Connection> {
    //     let conns: Vec<Connection> = CONNECTIONS
    //         .lock()
    //         .unwrap()
    //         .iter()
    //         .filter(|c| {
    //             let x = *c;
    //             let c_id = x.clone().get_id();
    //             c_id == Some(id)
    //         })
    //         .cloned()
    //         .collect();

    //     if conns.len() == 1 {
    //         let mut x = conns.get(0).unwrap().clone();
    //         Some(&mut x)
    //     } else {
    //         None
    //     }
    // }
}
