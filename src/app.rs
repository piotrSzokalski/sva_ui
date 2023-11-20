use egui::Button;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

use simple_virtual_assembler::instruction::Instruction;
use simple_virtual_assembler::virtual_machine::VirtualMachine;

use simple_virtual_assembler::assembler::Assembler;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SvaUI {
    tex_t: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    code: String,

    vm: VirtualMachine,

    acc_label: String,
    err_buffer: String,
}

impl Default for SvaUI {
    fn default() -> Self {
        Self {
            // Example stuff:
            tex_t: "Hello World!".to_string(),
            value: 2.7,

            code: String::new(),
            vm: VirtualMachine::new(vec![]),

            acc_label: String::new(),
            err_buffer: String::new(),
        }
    }
}

impl SvaUI {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for SvaUI {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ////
            //let mut vm = SVAShell::new(0);

            egui::Window::new("vm").show(ctx, |ui| {
                ui.vertical(|ui| {});

                ui.horizontal(|ui| {
                    if ui.button("run").clicked() {
                        //TEMP

                        let res = Assembler::new().parse(&self.code);

                        match res {
                            Ok(program) => {
                                let mut vm2 = VirtualMachine::new(program);
                                vm2.run();
    
                                self.acc_label = vm2.get_acc().to_string();
                                self.err_buffer.clear();
                            }
                            Err(err) => {
                                self.err_buffer = err.to_string();
                            }
                        }
    
                    }
                    
                   
                    //assemble_and_run(&mut self.vm, &self.code, &mut self.tex_t);
                });
                CodeEditor::default()
                    .id_source("code editor")
                    .with_rows(12)
                    .with_fontsize(14.0)
                    .with_theme(ColorTheme::GRUVBOX)
                    .with_syntax(Syntax::rust())
                    .with_numlines(true)
                    .show(ui, &mut self.code);

                ui.label("acc");
                ui.button(self.acc_label.clone());
                ui.button("pc");

                ui.label(&self.err_buffer);

               // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {});
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn assemble_and_run(vm: &mut VirtualMachine, code_text: &String, mut err_output: &mut String) {
    let mut assembler = Assembler::new();

    let result = assembler.parse(&code_text);

    // match result {
    //     Ok(program) => run(vm, program),
    //     Err(err) => err_output = &mut err.to_string(),
    // }
}

fn run(vm: &mut VirtualMachine, instructions: Vec<Instruction>) {
    // vm.load_program(instructions);

    let mut x = VirtualMachine::new(vec![]);
    // x.load_program(instructions);

    // vm.load_program(vec![]);

    // *vm = VirtualMachine::new(instructions);

    // vm.run();
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
