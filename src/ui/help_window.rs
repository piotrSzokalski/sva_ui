use egui::{containers::Window, widgets::Label, Context};
use egui::{Align, Slider, TextEdit, Ui, Widget};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct HelpWindow {
    /// Is widow open
    pub is_open: bool

}

impl HelpWindow {

    /// Opens help window
    pub fn open(&mut self) {
        self.is_open = true;
    }
    /// Closes help window
    pub fn close (&mut self) {
        self.is_open = false;
    }
    /// Toggle weather help widows is open or closed
    pub fn toggle_open_close(&mut self) {
        self.is_open = ! self.is_open;
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {

        let registers_explanation = r#"
        acc         Accumulator, stores results of arithmetic and logic operations
        pc          Program counter, stores next instruction to be executed (staring at 0)
        flag        Stores result of comparison can be eider: Equal, Greater or Lesser
        r 0-3       General purpose register
        p 0-3       Port register, that can be connected with other ports to share data
        "#;

        let instruction_set_explanation = r#"

        v - numeric value in decimal binary or hex
        r - register/port including acc anc pc
        l - label 


        MOV r/v r   Copies first operand value to register specified by second operand

        ADD r/v     Adds first operand value to acc
        SUB r/v     Subtracts first operand value to acc
        MUL r/v     Multiplyes first operand value to acc
        DIV         DIVS    first operand value to acc
        MOD         MODS    first operand value to acc

        AND         Performs bitwise and on acc and operand
        OR          Performs bitwise or on acc and operand
        XOR         Performs bitwise xor on acc and operand
        NOT         Performs bitwise not on

        CMP r/v r/v Compares operands stores result in flag
        JE  l       Jumps to label if flag is set to equal       
        JL  l       Jumps to label if flag is set to lesser
        JG  l       Jumps to label if flag is set to greater

        HLT         Ends execution of program

        "#;

        egui::Window::new(t!("widow.help"))
        .open(&mut self.is_open)
        .show(ctx, |ui| {
            ui.collapsing("Rejestry", |ui| {
                ui.label(registers_explanation);
            });
            ui.collapsing("Polecania", |ui| {
                ui.label(instruction_set_explanation);
            });
            ui.collapsing("Maszyn wirtualna", |ui| {

            });
            ui.collapsing("Interfejs użytkowania", |ui| {

            });
        });
    }
}