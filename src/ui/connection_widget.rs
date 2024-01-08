use egui::{Context, Ui};
use simple_virtual_assembler::components::connection::Connection;

use crate::storage::{connections::ConnectionManager, custom_logger::CustomLogger};

pub struct ConnectionWidget {
    conn: Connection,
}

impl ConnectionWidget {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        let id = self.conn.get_id();
        let mut name = "".to_owned();
        if let Some(id_c) = id {
            if let Some(res) = ConnectionManager::get_name(id_c) {
                name = res;
            }
        }

        ui.separator();

        ui.horizontal(|ui| {
            ui.label(name);
            if ui.button(format!("connect")).clicked() {
                //ConnectionManager::toggle_start_connecting();
                if ConnectionManager::get_current_id_index() == id {
                    ConnectionManager::set_current_id(None);
                } else {
                    ConnectionManager::set_current_id(id);
                }
            }
            if ui.button("rename").clicked() {}
            if ui.button("remove").clicked() {}
        });
        let collapsing_id = ui.make_persistent_id(self.conn.get_id());
        egui::collapsing_header::CollapsingState::load_with_default_open(
            ui.ctx(),
            collapsing_id,
            false,
        )
        .show_header(ui, |ui| {
            ui.label(t!("ports"));
        })
        .body(|ui| {
            ui.label(format!("{:?}", self.conn));
        });
    }
}
