use egui::{Context, Ui};
use serde::{Deserialize, Serialize};
use simple_virtual_assembler::components::ram::Ram;

use crate::storage::{ custom_logger::CustomLogger, connections_manager::ConnectionManager};

#[derive(Serialize, Deserialize, Debug)]
pub struct RamWidow {
    id: usize,
    /// Is widow open
    pub is_open: bool,
    ram: Ram,
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
        egui::Window::new(t!("widow.help"))
            .open(&mut true)
            .show(ctx, |ui| {
                let values = self.ram.get_data_ref().clone();
                ui.horizontal(|ui| {
                    ui.label("index");
                    if ui
                        .button(format!("{}", self.ram.get_index_port()))
                        .clicked()
                    {
                        //
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
                            //self.vm.lock().unwrap().disconnect(index);
                        }
                        //
                    }
                    ui.label("data");
                    if ui.button(format!("{}", self.ram.get_data_port())).clicked() {
                        //
                        if let Some(conn_index) = ConnectionManager::get_current_id_index() {
                            if let Some(conn) = ConnectionManager::get_connections()
                                .lock()
                                .unwrap()
                                .get_mut(conn_index)
                            {
                                let id = format!("R{}:index", self.get_id().clone());

                                //self.ram.get_index_port().connect(conn);
                                self.ram.connect_data_port(conn);
                                conn.add_port_id(id);
                            }
                        } else if ConnectionManager::in_disconnect_mode() {
                            //self.vm.lock().unwrap().disconnect(index);
                        }
                        //
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
