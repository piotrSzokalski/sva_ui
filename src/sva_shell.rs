use eframe::glow::NONE;
use std::cell::Ref;
use std::cell::RefCell;
use std::default;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use egui::{containers::Window, widgets::Label, Context};
use egui::{Align, Slider, TextEdit, Ui, Widget};

use simple_virtual_assembler::assembler::parsing_err::ParsingError;
use simple_virtual_assembler::components::connection::Connection;
use simple_virtual_assembler::components::port::Port;
use simple_virtual_assembler::vm::instruction::Instruction;
use simple_virtual_assembler::vm::virtual_machine::{VirtualMachine, VmStatus};

use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

use simple_virtual_assembler::assembler::assembler::Assembler;

use simple_virtual_assembler::language::Language;
//#[derive(serde::Deserialize, serde::Serialize)]
pub struct SVAShell {
    /// Id
    id: i32,
    /// Tile
    title: String,

    /// Simple virtual machine
    vm: Arc<Mutex<VirtualMachine>>,
    /// Assembler for simple virtual machine
    assembler: Assembler,
    /// Code before assembly
    code: String,
    /// ( Currently useless ) Program to be executed by vm
    program: Vec<Instruction>,
    /// Error assembling code to program
    parsing_error: Option<ParsingError>,
    /// ( Currently useless ) Parsing error message
    parsing_error_msg: String,
    /// Language
    language: Language,
    /// 'Start' or 'Stop' text for button
    control_button_text: String,

    /// Connecting in progress
    connection_started: Rc<RefCell<bool>>,

    /// Connections
    connections: Rc<RefCell<Vec<Connection>>>,

    ///Delay ms
    delay_ms: u64,
}

impl Default for SVAShell {
    fn default() -> Self {
        Self {
            id: -1,
            title: "BRAK".to_owned(),
            vm: Arc::new(Mutex::new(VirtualMachine::new())),
            assembler: Assembler::new(),
            code: String::new(),
            program: Vec::new(),

            parsing_error_msg: String::new(),
            parsing_error: None,
            language: Language::En,
            control_button_text: "Start".to_owned(),
            connection_started: Rc::new(RefCell::new(false)),
            connections: Rc::new(RefCell::new(Vec::new())),
            delay_ms: 1000,
        }
    }
}

impl SVAShell {
    pub fn new(
        id: i32,
        title: String,
        connection_started: Rc<RefCell<bool>>,
        connections: Rc<RefCell<Vec<Connection>>>,
    ) -> SVAShell {
        let mut s = SVAShell {
            id,
            title,
            vm: Arc::new(Mutex::new(VirtualMachine::new())),
            assembler: Assembler::new(),
            code: String::new(),
            program: Vec::new(),

            parsing_error_msg: String::new(),
            parsing_error: None,
            language: Language::En,
            control_button_text: "Start".to_owned(),
            connection_started,
            connections,
            delay_ms: 1000,
        };
        s.vm.lock().unwrap().set_delay(1000);
        s.set_language(Language::Pl);
        s
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn set_language(&mut self, language: Language) {
        //self.language = language;
        self.assembler.set_language(language);
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        egui::Window::new(&self.id.to_string())
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical(|ui| {});
                if ui.add(egui::Slider::new(&mut self.delay_ms, 0..=5000).text("delay")).changed() {
                    self.vm.lock().unwrap().set_delay(self.delay_ms.try_into().unwrap());
                }
                    
                

                if let Some(parsing_error) = &self.parsing_error {
                    // ui.label(parsing_error.to_string());
                    ui.label(
                        egui::RichText::new(parsing_error.to_string())
                            .color(egui::Color32::from_rgb(255, 0, 0)),
                    );
                } else {
                    ui.horizontal(|ui| {
                        // if ui.button("run").clicked() {
                        //     self.assemble_and_run();
                        // }

                        let vm_status;

                        {
                            vm_status = self.vm.lock().unwrap().get_status();
                        }

                        if vm_status == VmStatus::Running {
                            self.control_button_text = "Stop".to_owned();
                        }

                        match vm_status {
                            VmStatus::Initial => self.control_button_text = "Start".to_owned(),
                            VmStatus::Running => self.control_button_text = "Stop".to_owned(),
                            VmStatus::Stopped => self.control_button_text = "Resume".to_owned(),
                            VmStatus::Finished => self.control_button_text = "Start".to_owned(),
                        }

                        if vm_status == VmStatus::Running || vm_status == VmStatus::Stopped {
                            if ui.button("Halt").clicked() {
                                VirtualMachine::halt(self.vm.clone());
                            }
                        }

                        if ui.button(&self.control_button_text).clicked() {
                            {
                                self.vm
                                    .lock()
                                    .unwrap()
                                    .set_delay(self.delay_ms.try_into().unwrap());
                            }
                            match vm_status {
                                VmStatus::Initial => {
                                    VirtualMachine::start(self.vm.clone());
                                }
                                VmStatus::Running => {
                                    VirtualMachine::stop(self.vm.clone());
                                }
                                VmStatus::Stopped => {
                                    VirtualMachine::resume(self.vm.clone());
                                }
                                VmStatus::Finished => {
                                    VirtualMachine::start(self.vm.clone());
                                }
                            }
                        }

                        //assemble_and_run(&mut self.vm, &self.code, &mut self.tex_t);

                        if ui.button("step").clicked() {
                            self.step();
                        }
                    });
                }

                //
                let id = ui.make_persistent_id("Vm code heder");
                egui::collapsing_header::CollapsingState::load_with_default_open(
                    ui.ctx(),
                    id,
                    false,
                )
                .show_header(ui, |ui| {
                    ui.heading("Code");
                })
                .body(|ui| {
                    egui::ScrollArea::vertical()
                        .max_height(600.0)
                        .show(ui, |ui| {
                            if ui.text_edit_multiline(&mut self.code).highlight().changed() {
                                self.try_assemble_and_load();
                            }
                        });
                });
                //

                // ui.collapsing("code", |ui| {
                //     egui::ScrollArea::vertical()
                //         .max_height(600.0)
                //         .show(ui, |ui| {
                //             if ui.text_edit_multiline(&mut self.code).highlight().changed() {
                //                 self.try_assemble_and_load();
                //             }
                //         });
                // });

                ui.horizontal(|ui| {
                    ui.label("acc");
                    {
                        ui.button(self.vm.lock().unwrap().get_acc().to_string());
                    }
                    ui.label("pc");
                    ui.button(self.vm.lock().unwrap().get_pc().to_string());
                    ui.label("flag");
                    ui.button(self.vm.lock().unwrap().get_flag().to_string());
                });

                ui.horizontal(|ui| {
                    ui.label("r 0-3");
                    {
                        ui.button(format!("{:?}", self.vm.lock().unwrap().get_registers()));
                    }
                });

                ui.vertical(|ui| {
                    ui.label("p 0-3");
                    let ports;

                    {
                        ports = self.vm.lock().unwrap().get_ports();
                    }
                    let mut index = 0;
                    for p in ports {
                        if ui.button(format!("{:?}", p)).clicked() {
                            let mut conn_started = self.connection_started.borrow_mut();
                            println!("{}", conn_started.to_string());
                            if *conn_started {
                                *conn_started = false;

                                let mut conn = Connection::new();

                                {
                                    //TODO:
                                    // Change to apposite  index
                                    self.vm.lock().unwrap().connect(index, &mut conn);
                                }
                                self.connections.borrow_mut().push(conn);
                            } else {
                                *conn_started = true;
                                let connections = &mut self.connections.borrow_mut();
                                if let Some(mut conn) = connections.last_mut() {
                                    {
                                        self.vm.lock().unwrap().connect(index, &mut conn);
                                    }
                                }
                            }
                        }
                        if (index < 4) {
                            index += 1;
                        }
                    }

                    //ui.button(format!("{:?}", self.vm.get_ports()));
                });

                //ui.label(self.vm.to_string());
            });
            if self.delay_ms > 10 {
                ctx.request_repaint_after(Duration::from_millis(self.delay_ms));
            } else {
                ctx.request_repaint_after(Duration::from_millis(10));
            }
            
    }

    /// Draws connection to mouse
    fn draw_connection_to_mouse(&mut self) {}
    /// Handles connecting ports, draws connection
    fn connect_port(&mut self, port: Port) {
        self.draw_connection_to_mouse();
    }
    /// Handles disconnecting ports
    fn disconnect_port(&mut self, port: Port) {}

    /// Assembles code supposed to be used in parent 'screen'
    pub fn assemble(&mut self) {}

    /// Tries Assembles code to instructions and loads them to vm
    pub fn try_assemble_and_load(&mut self) {
        let res = self.assembler.parse(&self.code);

        match res {
            Ok(program) => {
                //TODO:
                //temp
                //self.vm = Arc::new(Mutex::new(VirtualMachine::new_with_program(program)));
                {
                    self.vm.lock().unwrap().load_program(program);
                }
                //self.vm.load_program(program);
                self.parsing_error = None
            }
            Err(err) => self.parsing_error = Some(err),
        }
    }
    /// Execute one instruction FIXME:
    fn step(&mut self) {
        // if self.vm.lock().unwrap().get_program().is_empty() {
        //     self.try_assemble_and_load();
        // }

        {
            let mut vm = self.vm.lock().unwrap();
            if vm.get_pc() >= vm.get_program().len() {
                vm.reset_pc();
            }
        }

        match self.parsing_error {
            Some(_) => todo!(),
            None => {
                self.vm.lock().unwrap().execute();
            }
        }
    }

    /// Run vm's program
    fn run(&mut self) {
        match self.parsing_error {
            Some(_) => todo!(),
            None => {
                //self.vm.load_program(self.program.clone());
                //self.vm.load_program(self.program);
                self.vm.lock().unwrap().run()
            }
        }
    }

    /// Assembles code to instructions and runs them on  vm
    fn assemble_and_run(&mut self) {
        println!("test");

        self.try_assemble_and_load();

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
