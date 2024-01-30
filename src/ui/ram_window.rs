use std::fmt::format;

use egui::{Button, Color32, Context, Stroke, Ui};
use serde::{de::value, Deserialize, Serialize};
use simple_virtual_assembler::components::ram::Ram;

use crate::storage::{
    connections_manager::{ConnectionManager, CONNECTIONS},
    custom_logger::CustomLogger,
    modals_manager::{ModalManager, MODAL_BUFFER_VALUE_I32, MODAL_INDEX_BUFFER, RAM_ID},
    toasts::ToastsManager,
};

use super::indicator_widget::ValueFormat;

#[derive(Serialize, Deserialize, Debug)]
pub struct RamWidow {
    id: usize,
    name: String,
    /// Is widow open
    pub is_open: bool,
    pub ram: Ram,
    format: ValueFormat,
}

impl RamWidow {
    pub fn new(id: usize) -> Self {
        let x = Self {
            is_open: true,
            ram: Ram::new().with_id(id).with_size(512),
            id,
            name: format!("ram:{}", id),
            format: Default::default(),
        };
        x
    }
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn format_value(&mut self, value: i32) -> String {
        match self.format {
            ValueFormat::Dec => format!("{}", value),
            ValueFormat::Hex => format!("0x{:X}", value),
            ValueFormat::Bin => format!("0b{:b}", value),
            ValueFormat::Unicode => {
                if value < 0 {
                    return t!("common.invalid_char");
                } else {
                    if let Some(char) = char::from_u32(value as u32) {
                        return format!("\'{}\'", char);
                    }
                    return t!("common.invalid_char");
                }
            }
        }
    }

    pub fn refresh(&mut self) {
        self.ram.refresh();
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        egui::Window::new(&self.name)
            .id(egui::Id::new(format!("ram:{}", self.id)))
            .show(ctx, |ui| {
                let values = self.ram.get_data_ref().clone();
                ui.horizontal(|ui| {
                    ui.separator();
                    ui.label("index");

                    let already_connected = match self.ram.get_index_port() {
                        simple_virtual_assembler::components::port::Port::Connected(_, _) => true,
                        simple_virtual_assembler::components::port::Port::Disconnected(_) => false,
                    };

                    let mut port_color = Color32::LIGHT_GRAY;

                    if ConnectionManager::get_current_id_index().is_some() && !already_connected {
                        let in_dark_mode = ui.style().visuals.dark_mode;

                        port_color = if in_dark_mode {
                            Color32::YELLOW
                        } else {
                            Color32::BLUE
                        }
                    } else if ConnectionManager::in_disconnect_mode() && already_connected {
                        port_color = Color32::DARK_RED;
                    }

                    let index_port_button = Button::new(format!("{}", self.ram.get_index_port()))
                        .stroke(Stroke::new(1.0, port_color));

                    if ui.add_enabled(true, index_port_button).clicked() {
                        if let Some(conn_index) = ConnectionManager::get_current_id_index() {
                            if let Some(conn) = ConnectionManager::get_connections()
                                .lock()
                                .unwrap()
                                .get_mut(conn_index)
                            {
                                if !already_connected {
                                    let id = format!("R{}:index", self.get_id().clone());

                                    self.ram.connect_index_port(conn);
                                    conn.add_port_id(id);
                                } else {
                                    ToastsManager::show_info(
                                        t!("toast_info.can_connect_connected_port"),
                                        10,
                                    )
                                }
                            }
                        } else if ConnectionManager::in_disconnect_mode() {
                            let conn_id = self.ram.get_index_port().get_conn_id();
                            let conn_index = ConnectionManager::get_connection_index_by_id(conn_id);
                            if let Some(conn_i) = conn_index {
                                let mut conns_lock = CONNECTIONS.lock().unwrap();
                                let conn = conns_lock.get_mut(conn_i);
                                if let Some(conn_ref) = conn {
                                    self.ram.disconnect_index_port();
                                    let id = format!("R{}:index", self.get_id().clone());
                                    conn_ref.remove_port_id(id);
                                }
                            }
                        }
                    }
                    if let Some(id) = self.ram.get_index_port().get_conn_id() {
                        if let Some(conn_name) = ConnectionManager::get_name(id) {
                            ui.label(conn_name);
                        }
                    }

                    // ---------------------------------------------------------------------------------------------------------------------
                    ui.separator();

                    let already_connected = match self.ram.get_data_port() {
                        simple_virtual_assembler::components::port::Port::Connected(_, _) => true,
                        simple_virtual_assembler::components::port::Port::Disconnected(_) => false,
                    };

                    let mut port_color = Color32::GRAY;

                    if ConnectionManager::get_current_id_index().is_some() && !already_connected {
                        let in_dark_mode = ui.style().visuals.dark_mode;

                        port_color = if in_dark_mode {
                            Color32::YELLOW
                        } else {
                            Color32::BLUE
                        }
                    } else if ConnectionManager::in_disconnect_mode() && already_connected {
                        port_color = Color32::DARK_RED;
                    }

                    let data_port_button = Button::new(format!("{}", self.ram.get_data_port()))
                        .stroke(Stroke::new(1.0, port_color));

                    ui.label("data");
                    if ui.add_enabled(true, data_port_button).clicked() {
                        if let Some(conn_index) = ConnectionManager::get_current_id_index() {
                            if let Some(conn) = ConnectionManager::get_connections()
                                .lock()
                                .unwrap()
                                .get_mut(conn_index)
                            {
                                if !already_connected {
                                    let id = format!("R{}:data", self.get_id().clone());

                                    //self.ram.get_index_port().connect(conn);
                                    self.ram.connect_data_port(conn);
                                    conn.add_port_id(id);
                                } else {
                                    ToastsManager::show_info(
                                        t!("toast_info.can_connect_connected_port"),
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
                    }
                    if let Some(id) = self.ram.get_data_port().get_conn_id() {
                        if let Some(conn_name) = ConnectionManager::get_name(id) {
                            ui.label(conn_name);
                        }
                    }
                    // ---------------------------------------------------------------------------------------------------------------------

                    ui.separator();
                    ui.label("mode");

                    let already_connected = match self.ram.get_mode_port() {
                        simple_virtual_assembler::components::port::Port::Connected(_, _) => true,
                        simple_virtual_assembler::components::port::Port::Disconnected(_) => false,
                    };

                    let mut port_color = Color32::LIGHT_GRAY;

                    if ConnectionManager::get_current_id_index().is_some() && !already_connected {
                        let in_dark_mode = ui.style().visuals.dark_mode;

                        port_color = if in_dark_mode {
                            Color32::YELLOW
                        } else {
                            Color32::BLUE
                        };
                    } else if ConnectionManager::in_disconnect_mode() && already_connected {
                        port_color = Color32::DARK_RED;
                    }

                    let data_mode_button = Button::new(format!("{}", self.ram.get_mode_port()))
                        .stroke(Stroke::new(1.0, port_color));

                    if ui.add_enabled(true, data_mode_button).clicked() {
                        if let Some(conn_index) = ConnectionManager::get_current_id_index() {
                            if let Some(conn) = ConnectionManager::get_connections()
                                .lock()
                                .unwrap()
                                .get_mut(conn_index)
                            {
                                if !already_connected {
                                    let id = format!("R{}:mode", self.get_id().clone());

                                    //self.ram.get_index_port().connect(conn);
                                    self.ram.connect_mode_port(conn);
                                    conn.add_port_id(id);
                                } else {
                                    ToastsManager::show_info(
                                        t!("toast_info.can_connect_connected_port"),
                                        10,
                                    )
                                }
                            }
                        } else if ConnectionManager::in_disconnect_mode() {
                            let conn_id = self.ram.get_mode_port().get_conn_id();
                            let conn_index = ConnectionManager::get_connection_index_by_id(conn_id);
                            if let Some(conn_i) = conn_index {
                                let mut conns_lock = CONNECTIONS.lock().unwrap();
                                let conn = conns_lock.get_mut(conn_i);
                                if let Some(conn_ref) = conn {
                                    self.ram.disconnect_mode_port();
                                    let id = format!("R{}:mode", self.get_id().clone());
                                    conn_ref.remove_port_id(id);
                                }
                            }
                        }
                    }
                    if let Some(id) = self.ram.get_mode_port().get_conn_id() {
                        if let Some(conn_name) = ConnectionManager::get_name(id) {
                            ui.label(conn_name);
                        }
                    }
                });
                ui.separator();
                ui.collapsing(t!("ram_window.collapsing.values"), |ui| {
                    ui.horizontal(|ui| {
                        if ui.button(t!("button.zero_values")).clicked() {
                            self.ram.zero_data();
                        }
                        egui::ComboBox::from_label("format")
                            .selected_text(format!("{:?}", self.format))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.format, ValueFormat::Dec, t!("ram_window.selectable_value.format.decimal"));
                                ui.selectable_value(&mut self.format, ValueFormat::Bin, t!("ram_window.selectable_value.format.binary"));
                                ui.selectable_value(&mut self.format, ValueFormat::Hex, t!("ram_window.selectable_value.format.hexadecimal"));
                                ui.selectable_value(&mut self.format, ValueFormat::Unicode, t!("ram_window.selectable_value.format.unicode"));
                            });
                    });
                    ui.separator();
                    egui::ScrollArea::new(true).show(ui, |ui| {
                        for i in 0..(512 / 8) {
                            ui.horizontal(|ui| {
                                for j in 0..8 {
                                    let index = i * 8 + j;
                                    if ui.button(self.format_value(values[index])).clicked() {
                                        ModalManager::set_modal(1);
                                        *RAM_ID.lock().unwrap() = Some(self.get_id());
                                        *MODAL_INDEX_BUFFER.lock().unwrap() = Some(index);
                                    };
                                }
                            });
                        }
                    });
                });
            });
    }

    pub fn set_value_at_index(&mut self, index: usize, value: i32) {
        self.ram.set_value(index, value);
    }
}
