use std::fmt::format;

use egui::{Context, Ui};
use serde::{Deserialize, Serialize};
use simple_virtual_assembler::components::ram::Ram;

use crate::storage::{connections_manager::{ConnectionManager, CONNECTIONS}, custom_logger::CustomLogger, toasts::ToastsManager};

#[derive(Serialize, Deserialize, Debug)]
pub struct RamWidow {
    id: usize,
    /// Is widow open
    pub is_open: bool,
    pub ram: Ram,
}

impl RamWidow {
    pub fn new(id: usize) -> Self {
        let x = Self {
            is_open: true,
            ram: Ram::new().with_id(id).with_size(512),
            id,
        };
        CustomLogger::log("adding ram window");
        CustomLogger::log(&format!("{:?}", x));
        x
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
    /// Toggle weather help widows is open or closed
    pub fn toggle_open_close(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn refresh(&mut self) {
        self.ram.refresh();
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        egui::Window::new(format!("ram {}", self.id))
            .open(&mut true)
            .show(ctx, |ui| {
                let values = self.ram.get_data_ref().clone();
                ui.horizontal(|ui| {
                    ui.label("index");
                    if ui
                        .button(format!("{}", self.ram.get_index_port()))
                        .clicked()
                    {
                        // {}

                        if let Some(conn_index) = ConnectionManager::get_current_id_index() {
                            if let Some(conn) = ConnectionManager::get_connections()
                                .lock()
                                .unwrap()
                                .get_mut(conn_index)
                            {
                                let id = format!("R{}:index", self.get_id().clone());

                                //self.ram.get_index_port().connect(conn);
                                self.ram.connect_index_port(conn);
                                conn.add_port_id(id);
                            }
                        } else if ConnectionManager::in_disconnect_mode() {
                            let conn_id = self.ram.get_index_port().get_conn_id();
                            if let Some(conn_id) = conn_id {
                                //let conn = ConnectionManager::get
                            }
                        }
                        //
                    }
                    if let Some(id) = self.ram.get_index_port().get_conn_id() {
                        if let Some(conn_name) = ConnectionManager::get_name(id) {
                            ui.label(conn_name);
                        }
                    }
                    ui.separator();
                    ui.label("data");
                    if ui.button(format!("{}", self.ram.get_data_port())).clicked() {
                        //
                        if let Some(conn_index) = ConnectionManager::get_current_id_index() {
                            if let Some(conn) = ConnectionManager::get_connections()
                                .lock()
                                .unwrap()
                                .get_mut(conn_index)
                            {
                                let already_connected = match self.ram.get_data_port() {
                                    simple_virtual_assembler::components::port::Port::Connected(_, _) => true,
                                    simple_virtual_assembler::components::port::Port::Disconnected(_) => false,
                                };
                                if ! already_connected {
                                    let id = format!("R{}:data", self.get_id().clone());

                                    //self.ram.get_index_port().connect(conn);
                                    self.ram.connect_data_port(conn);
                                    conn.add_port_id(id);
                                    
                                } else {
                                    ToastsManager::show_info(
                                        "Can't connect port that is alredy connected".to_owned(),
                                        10,
                                    )
                                }
                            

                            
                            }
                        } else if ConnectionManager::in_disconnect_mode() {
                            
                            let conn_id = self.ram.get_data_port().get_conn_id();
                            let conn_index = ConnectionManager::get_connection_index_by_id(conn_id);
                            if let Some(conn_i) = conn_index { 
                                let mut conns_lock = CONNECTIONS.lock().unwrap();
                                let conn = conns_lock.get_mut(conn_i);
                                if let Some(conn_ref) = conn { 
                                    self.ram.disconnect_data_port();
                                    let id = format!("R{}:data", self.get_id().clone());
                                    conn_ref.remove_port_id(id);
                                }
                            }
                        }
                        //
                    }
                    if let Some(id) = self.ram.get_data_port().get_conn_id() {
                        if let Some(conn_name) = ConnectionManager::get_name(id) {
                            ui.label(conn_name);
                        }
                    }
                    ui.separator();
                    ui.label("mode");
                    if ui.button(format!("{}", self.ram.get_mode_port())).clicked() {
                        //
                        if let Some(conn_index) = ConnectionManager::get_current_id_index() {
                            if let Some(conn) = ConnectionManager::get_connections()
                                .lock()
                                .unwrap()
                                .get_mut(conn_index)
                            {
                                let id = format!("R{}:mode", self.get_id().clone());

                                //self.ram.get_index_port().connect(conn);
                                self.ram.connect_mode_port(conn);
                                conn.add_port_id(id);
                            }
                        } else if ConnectionManager::in_disconnect_mode() {
                            //self.vm.lock().unwrap().disconnect(index);
                        }
                        //
                    }
                    if let Some(id) = self.ram.get_mode_port().get_conn_id() {
                        if let Some(conn_name) = ConnectionManager::get_name(id) {
                            ui.label(conn_name);
                        }
                    }
                });
                //ui.separator();
                //ui.label(format!("{:?}", self.ram));
                ui.separator();
                ui.collapsing("values", |ui| {
                    egui::ScrollArea::new(true).show(ui, |ui| {
                        for i in 0..(256 / 8) {
                            ui.horizontal(|ui| {
                                for j in 0..8 {
                                    ui.button(format!("{}", values[i * 8 + j]));
                                }
                            });
                        }
                    });
                });
            });
    }
}
