use egui::{Color32, Context, RichText, Ui};

use simple_virtual_assembler::components::connection::Connection;

use crate::storage::{
    connections_manager::{ConnectionManager, ANOTHER_ID_BUFFER, CURRENT_CONN_ID_FOR_RENAME},
    custom_logger::CustomLogger,
    modals_manager::ModalManager,
};

pub struct ConnectionWidget<'a> {
    conn: Connection,
    change_conn_name_modal_open: &'a mut bool,
}

impl<'a> ConnectionWidget<'a> {
    pub fn new(conn: Connection, change_conn_name_modal_open: &'a mut bool) -> Self {
        Self {
            conn,
            change_conn_name_modal_open,
        }
    }

    pub fn show(&mut self, _ctx: &Context, ui: &mut Ui) {
        let id = self.conn.get_id();

        let mut name = "".to_owned();
        if let Some(id_c) = id {
            if let Some(res) = ConnectionManager::get_name(id_c) {
                name = res;
            }
        }

        ui.separator();

        let mut button_text = t!("button.connect.connect");
        let mut color = Color32::GRAY;
        if ConnectionManager::get_current_id() == self.conn.get_id() {
            button_text = t!("button.connect.stop_connecting");
            let in_dark_mode = ui.style().visuals.dark_mode;

            color = if in_dark_mode {
                Color32::YELLOW
            } else {
                Color32::BLUE
            }
        }

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                //ui.label(format!("{:?}", id));
                ui.separator();
                ui.heading(RichText::new(name).color(color));
            });

            // it's an index

            let current_index = ConnectionManager::get_current_id();
            if ui.button(button_text).clicked() {
                CustomLogger::log(&format!("CONN IDS: {:?} {:?}", current_index, id));
                if current_index == id {
                    ConnectionManager::set_current_id(None);
                } else {
                    ConnectionManager::set_current_id(id);
                }
            }
            if ui.button(t!("button.rename")).clicked() {
                *CURRENT_CONN_ID_FOR_RENAME.lock().unwrap() = id;
                *self.change_conn_name_modal_open = true;
            }
            if ui.button(t!("button.remove")).clicked() {
                *ANOTHER_ID_BUFFER.lock().unwrap() = id;
                ModalManager::set_modal(3);
                //ConnectionManager::remove_connection(id);
            }
        });
        let _collapsing_id = ui.make_persistent_id(self.conn.get_id());
    }
}
