use egui::{containers::Window, widgets::Label, Context};
use egui::{Align, RichText, ScrollArea, Slider, TextEdit, Ui, Widget};
use serde::{Deserialize, Serialize};
use simple_virtual_assembler::language::{self, Language};
use simple_virtual_assembler::vm::instruction;

#[derive(Serialize, Deserialize)]
pub struct HelpWindow {
    /// Is widow open
    pub is_open: bool,
    pub language: Language,
}

impl HelpWindow {
    /// Opens help window
    pub fn open(&mut self) {
        self.is_open = true;
    }
    /// Closes help window
    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }
    /// Toggle weather help widows is open or closed
    pub fn toggle_open_close(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        let registers_explanation_en = r#"
        acc         Accumulator, stores results of arithmetic and logic operations
        pc          Program counter, stores next instruction to be executed (staring at 0)
        flag        Stores result of comparison can be eider: Equal, Greater or Lesser
        r 0-3       General purpose register
        p 0-3       Port register, that can be connected with other ports to share data
        "#;

        let instruction_set_explanation_en = r#"

        Instruction can be either lower or upper case

        Operand:

        v - numeric value in decimal binary or hex
        r - register/port including acc and pc
        l - label 

        Instruction:

        NOP         Does nothing 
        HLT         Ends execution of program

        MOV r/v r   Copies first operand value to register specified by second operand

        ADD r/v     Adds operand value to acc
        SUB r/v     Subtracts operand value from acc
        MUL r/v     Multiples operand value with acc
        DIV r/v     Divides acc by operand
        MOD r/v     Stores reminder of division of acc by operand
        INC         Increments acc by 1
        DEC         Decrements acc by 1
        
        NOT         Performs bitwise not on acc
        AND r/v     Performs bitwise and on acc and operand,stores result in acc
        OR  r/v     Performs bitwise or on acc and operand, stores result in acc
        XOR r/v     Performs bitwise xor on acc and operand,stores result in acc
        SHL r/v     Shits to the left bits of acc by number specified by operand
        SHR r/v     Shits to the right bits of acc by number specified by operand

        CMP r/v r/v Compares operands, stores state of Equal, Lesser, or Greater in Flag
        JMP l       Jumps to label
        JE  l       Jumps to label if flag is set to equal       
        JNE         Jumps to label if flag is not set to equal 
        JL  l       Jumps to label if flag is set to lesser
        JG  l       Jumps to label if flag is set to greater

        PSH r/v     Pushes operand on stack
        POP r/v     Pops operand on stack

        "#;

        let instruction_set_explanation_pl = r#"

        Instrukcje mogą być w duzych lub małych literach

        Operandy:

        v - wartość numeryczna zapisana w systemie dziesiętnym, dwójkowym lib szesnastkowym
        r - rejestr/port wliczają acc i pc
        l - etykieta 

        Instrukcje:

        NOP         Nic nie robi
        HLT         Zatrzymuje egzekucje programu

        MOV r/v r   Kopiuje wartość pierwszego operandu do do rejestru

        ADD r/v     Dodaje operand do acc
        SUB r/v     Odejmuje od acc operand 
        MUL r/v     Mnoży acc przez operand
        DIV r/v     Dzieli acc przez operand
        MOD r/v     Zapisuje reszte z dzielenia acc przez operand
        INC         Zwiększa  acc o 1
        DEC         Zmniejsza acc o 1
        
        NOT         Odwraca bity acc
        AND r/v     Wykuje operacje and an acc z operandem
        OR  r/v     Wykuje operacje or an acc z operandem
        XOR r/v     Wykuje operacje xor an acc z operandem
        SHL r/v     Przesuwa w lewo bit acc o liść podaną w operandzie 
        SHR r/v     Przesuwa w prawo bit acc o liść podaną w operandzie 

        CMP r/v r/v Porównuje operandy, zachowuje wynik: Równy, Mniejszy lub Większy w Fladze
        JMP l       Skacze do etykiety
        JE  l       Skacze do etykiety jeżeli flaga jest w stanie rownym      
        JNE         Skacze do etykiety jeżeli flaga nie jest w stanie rownym  
        JL  l       Skacze do etykiety jeżeli flaga jest w stanie mniejszym  
        JG  l       JSkacze do etykiety jeżeli flaga jest w stanie większym  

        PSH r/v     Wypycha operand na stos
        POP r/v     Wyciąga operand ze stosu

        "#;

        let instruction_explanation = match self.language {
            Language::Pl => instruction_set_explanation_pl,
            Language::En => instruction_set_explanation_en,
        };

        egui::Window::new(t!("widow.help"))
            .open(&mut self.is_open)
            //.min_width(650.0)
            //.max_width(650.0)
            .max_height(450.0)
            .show(ctx, |ui| {
                ScrollArea::new(true).show(ui, |ui| {
                    ui.collapsing("e", |ui| {
                        ScrollArea::new(true).show(ui, |ui| {
                            ui.label(RichText::new(""));
                        });
                    });
                    ui.collapsing(t!("help_window.collapsing.instructions"), |ui| {
                        ScrollArea::new(true).show(ui, |ui| {
                            ui.label(
                                RichText::new(instruction_explanation)
                                    .extra_letter_spacing(0.5),
                            );
                        });
                    });

                    ui.collapsing("Maszyn wirtualna", |ui| {});
                    ui.collapsing("Interfejs użytkowania", |ui| {});
                });
            });
    }
}
