use std::default;

use egui::{containers::Window, widgets::Label, Context};
use egui::{Align, Slider, TextEdit, Ui, Widget};

use simple_virtual_assembler::virtual_machine::VirtualMachine;

use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

use simple_virtual_assembler::assembler::Assembler;
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SVAShell {
    id: i32,
    sva: VirtualMachine,
    //assembler: Assembler,
    code: String,
}

impl Default for SVAShell {
    fn default() -> Self {
        Self {
            id: 0,
            sva: VirtualMachine::new(vec![]),
            //assembler: Assembler {},
            code: "".to_string(),
        }
    }
}

impl SVAShell {
    pub fn new(id: i32) -> SVAShell {
        SVAShell {
            id,
            sva: VirtualMachine::new(vec![]),
            //assembler: Assembler {},
            code: "CODE".to_string(),
        }
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        Window::new(format!("Window {}", self.id)).show(ctx, |ui| self.ui_content(ui));

        // Handle button click outside the window
        if ui.button("X").clicked() {
            // Handle button click action
        }
    }
    

    fn ui_content(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Editable Text:");
            CodeEditor::default()
                .id_source("code editor")
                .with_rows(12)
                .with_fontsize(10.0)
                .with_theme(ColorTheme::GRUVBOX)
                .with_syntax(Syntax::rust())
                .with_numlines(true)
                .show(ui, &mut self.code);
        });
    }
}

impl Widget for SVAShell {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        // Use Egui API to create your custom widget UI

        let response = Label::new("ascx").ui(ui);
        // Perform any additional actions or logic for your widge
        response
    }
}

////////////////////////////////////////////////////
