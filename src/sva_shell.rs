use std::default;

use eframe::glow::NONE;
use egui::{containers::Window, widgets::Label, Context};
use egui::{Align, Slider, TextEdit, Ui, Widget};

use simple_virtual_assembler::assembler::parsing_err::ParsingError;
use simple_virtual_assembler::vm::instruction::Instruction;
use simple_virtual_assembler::vm::virtual_machine::VirtualMachine;

use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

use simple_virtual_assembler::assembler::assembler::Assembler;
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SVAShell {
    /// Id
    id: i32,
    /// Tile
    title: String,
    /// Simple virtual machine
    vm: VirtualMachine,
    /// Assembler for simple virtual machine
    assembler: Assembler,
    /// Code before assembly
    code: String,
    /// ( Currently useless ) Program to be executed by vm
    program: Vec<Instruction>,
    /// Error assembling code to program
    parsing_error: Option<ParsingError>,
    /// Parsing error message
    parsing_error_msg: String,
    
}

// impl Default for SVAShell {
//     fn default() -> Self {
//         Self {
//             id: 0,
//             sva: VirtualMachine::new(),
//             code: String::new(),
//             acc_label: String::new(),
//             err_buffer: String::new(),
//             assembler: Assembler::new(),
//         }
//     }
// }

impl SVAShell {
    pub fn new(id: i32, title: String) -> SVAShell {
        SVAShell {
            id,
            title,
            vm: VirtualMachine::new(),
            assembler: Assembler::new(),
            code: String::new(),
            program: Vec::new(),

            parsing_error_msg: String::new(),
            parsing_error: None,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        egui::Window::new( &self.id.to_string()).open(&mut true).show(ctx, |ui| {
            ui.vertical(|ui| {});

            ui.horizontal(|ui| {
                if ui.button("run").clicked() {
                    self.assemble_and_run();
                }

                //assemble_and_run(&mut self.vm, &self.code, &mut self.tex_t);
            });
            egui::ScrollArea::vertical()
                .max_height(600.0)
                .show(ui, |ui| {
                    CodeEditor::default()
                        .id_source("code editor")
                        .with_rows(12)
                        .with_fontsize(12.0)
                        .with_theme(ColorTheme::GRUVBOX)
                        .with_syntax(Syntax::rust())
                        .with_numlines(true)
                        .show(ui, &mut self.code);
                });

            if let Some(parsing_error) = &self.parsing_error {
                // ui.label(parsing_error.to_string());
                ui.label(
                    egui::RichText::new(parsing_error.to_string())
                        .color(egui::Color32::from_rgb(255, 0, 0)),
                );
            }

            ui.horizontal(|ui| {
                ui.label("acc");
                ui.button(self.vm.get_acc().to_string());
                ui.label("pc");
                ui.button(self.vm.get_pc().to_string());
                ui.label("flag");
                ui.button(self.vm.get_flag().to_string());
            });

            ui.horizontal(|ui| {
                ui.label("r 0-3");
                ui.button(format!("{:?}", self.vm.get_registers()));
                ui.label("p 0-3");
                ui.button(format!("{:?}", self.vm.get_ports()));
            });

            //ui.label(self.vm.to_string());
        });
    }

    //TODO:
    fn ui_content(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
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
        });
    }

    /// Assembles code to instructions and loads them to vm
    fn assemble_and_load(&mut self) {
        let res = self.assembler.parse(&self.code);

        match res {
            Ok(program) => {
                //TODO:
                //temp
                self.vm = VirtualMachine::new_with_program(program);
                //self.vm.load_program(program);
                self.parsing_error = None
            }
            Err(err) => self.parsing_error = Some(err),
        }
    }

    /// Run vm's program
    fn run(&mut self) {
        match self.parsing_error {
            Some(_) => todo!(),
            None => {
                //self.vm.load_program(self.program.clone());
                //self.vm.load_program(self.program);
                self.vm.run()
            }
        }
    }

    /// Assembles code to instructions and runs them on  vm
    fn assemble_and_run(&mut self) {
        println!("test");

        self.assemble_and_load();

        if let Some(_parsing_error) = &self.parsing_error {
            self.title = "test".to_owned();
        } else {
            self.title = "ok".to_owned();
            self.run();
        }
    }
}

impl Widget for SVAShell {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        // Use Egui API to create your custom widget UI

        let response = Label::new("ascx").ui(ui);
        // Perform any additional actions or logic for your widget
        response
    }
}

////////////////////////////////////////////////////
