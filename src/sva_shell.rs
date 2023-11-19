use std::default;

use egui::{containers::Window, widgets::Label, Context};
use egui::{Align, Slider, TextEdit, Ui, Widget};

use simple_virtual_assembler::virtual_machine::VirtualMachine;

use simple_virtual_assembler::assembler::Assembler;
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SVAShell {
    id: i32,
    sva: VirtualMachine,
    //assembler: Assembler,
    code: String,
}

impl SVAShell {
    pub fn new(id: i32) -> SVAShell {
        SVAShell {
            id,
            sva: VirtualMachine::new(vec![]),
            //assembler: Assembler {},
            code: "".to_string(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new(&self.id.to_string()).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_multiline(&mut self.code);
                ui.text_edit_singleline(&mut "&mut self.name");
            });

            if ui.button("Click each year").clicked() {}
            ui.label(format!("Hello '', age "));
        });
    }
}

impl Widget for SVAShell {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        // Use Egui API to create your custom widget UI

        let response = Label::new("avxc").ui(ui);
        // Perform any additional actions or logic for your widge
        response
    }
}

////////////////////////////////////////////////////
