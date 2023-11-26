
use std::default;

use egui::{containers::Window, widgets::Label, Context};
use egui::{Align, Slider, TextEdit, Ui, Widget};

use simple_virtual_assembler::vm::virtual_machine::VirtualMachine;

use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

use simple_virtual_assembler::assembler::assembler::Assembler;



#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] 
pub struct TetsWindow {
    title: String
}

impl Default for TetsWindow {
    fn default() -> Self {
        Self {
            title: "kot".to_string()
        }
    }
}

impl TetsWindow {
    pub fn new() -> TetsWindow {
        TetsWindow { title: "ko kot".to_string() }
    }

    pub fn change_label(&mut self ,new_title: String) {
        self.title = new_title;
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.label(&self.title);
    }
}
