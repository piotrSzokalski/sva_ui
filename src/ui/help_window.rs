use egui::Context;
use egui::{RichText, ScrollArea, Ui};
use serde::{Deserialize, Serialize};
use simple_virtual_assembler::language::Language;

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

    pub fn show(&mut self, ctx: &Context, _ui: &mut Ui) {
        let registers_explanation_en = r#"
        acc         Accumulator, stores results of arithmetic and logic operations
        pc          Program counter, stores index of next instruction to be 
                    executed (staring at 0)
        flag        Stores result of comparison instruction can be eider:
                    Equal, Greater or Lesser
        r 0-3       General purpose register
        p 0-5       Port register, that can be connected with other ports to share data
        "#;

        let registers_explanation_pl = r#"
        acc         Akumulator, przechowuje wyniki  operacji arytmetycznych i logicznych
        pc          Licznik poleceń, przechowuje index następnego plecenia które ma 
                    zostać wykonane  (staring at 0)
        flag        Przechowuje wynik instrukcji porównania 
        r 0-3       Rejestry do przechowywania wartości
        p 0-5       Porty, rejestry które mogą być łączne z innymi portami, 
                    w celu współdzielenia danych
        "#;

        let registers_explanation = match self.language {
            Language::Pl => registers_explanation_pl,
            Language::En => registers_explanation_en,
        };

        let instruction_set_explanation_en = r#"

        Instruction can be either lower or upper case

        Operand:

        v - numeric value in decimal binary or hex
        r - register/port including acc and pc
        l - label 

        Instruction:

        NOP         Does nothing 
        HLT         Ends execution of program

        MOV r/v r   Copies first operand value to register specified by 
                    second operand

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

        CMP r/v r/v Compares operands, stores state of Equal, 
                    Lesser, or Greater in Flag
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

        v - wartość numeryczna zapisana w 
            systemie dziesiętnym, dwójkowym lib szesnastkowym
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

        CMP r/v r/v Porównuje operandy, zachowuje wynik: 
                    Równy, Mniejszy lub Większy w Fladze
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

        let vm_explanation_en = r#"

            Virtual machine executes instructions that modify its state

            It can be add with add button, and manged in component menu
            It can be added with or without stack

            Each virtual machine runs on separate thread, with set amount
            of delay before executing each instruction

            To start it the code in ints editor has to be correct
            It can be started, stopped, halted, reset, and steeped through
            Pressing on registers except flag will change their display format 
            
            Stack stores 32 values, pushing to full stack overrides it's top,
            pop'ing from empty stack pops 0

            Ram stores 512 values, that can be written and read
            Ram refreshes every frame
            Refresh copies values between data port nad ram value 
            designated by the value in index port

            It has 3 ports:
            index       determines index of ram to write to / read from 
            data        depending on mode copies value from ram to itself
                        or from itself to ram
            mode        determines wether ram is in read or wite mode,
                        0 - write mode, anything else - read mode

        "#;
        let vm_explanation_pl = r#"
            Maszyna wirtualna wykonuje instrukcje modyfikujące jej stan

            Można go dodać za pomocą przycisku Dodaj i zarządzać nim w menu komponentu
            Można go dodać ze stosem lub bez

            Każda maszyna wirtualna działa na osobnym wątku, z ustaloną ilością
            opóźnienia przed wykonaniem każdej instrukcji

            Aby go uruchomić, kod w edytorze ints musi być poprawny
            Można go uruchomić, zatrzymać, zatrzymać, zresetować i przesiąknąć
            Naciśnięcie na rejestry z wyjątkiem flagi spowoduje zmianę ich formatu wyświetlania
        
            Stos przechowuje 32 wartości, pchanie do pełnego stosu zastępuje jego szczyt,
            zdejmowanie z pustego stosu zdejmuje 0

            Ram przechowuje 512 wartości, które można zapisywać i odczytywać
            Ram odświeża każdą klatkę
            Odśwież kopiuje wartości pomiędzy portem danych a wartością pamięci RAM
            oznaczony przez wartość w porcie indeksu

            Posiada 3 porty:
            indeks          określa indeks pamięci RAM, do której można zapisywać/odczytywać
            dane            w zależności od trybu kopiują wartość z pamięci RAM do siebie
                            lub od siebie do barana
            tryb określa    czy pamięć RAM jest w trybie odczytu czy zapisu,
                            0 - tryb zapisu, cokolwiek innego - tryb odczytu
        "#;

        let vm_explanation = match self.language {
            Language::Pl => vm_explanation_pl,
            Language::En => vm_explanation_en,
        };

        let interface_explanation_en = r#"
        
        
        "#;
        let interface_explanation_pl = r#""#;

        let _interface_explanation = match self.language {
            Language::Pl => interface_explanation_pl,
            Language::En => interface_explanation_en,
        };

        egui::Window::new(t!("help_window.title"))
            .open(&mut self.is_open)
            .max_height(450.0)
            .min_width(600.0)
            .show(ctx, |ui| {
                ScrollArea::new(true).show(ui, |ui| {
                    ui.collapsing(t!("help_window.collapsing.registers"), |ui| {
                        ui.label(RichText::new(registers_explanation));
                    });
                    ui.collapsing(t!("help_window.collapsing.instructions"), |ui| {
                        ui.label(RichText::new(instruction_explanation).extra_letter_spacing(0.5));
                    });

                    ui.collapsing(t!("help_window.collapsing.vm_explanation"), |ui| {
                        ui.label(RichText::new(vm_explanation).extra_letter_spacing(0.5));
                    });
                    // ui.collapsing(t!("help_window.collapsing.user_interface"), |ui| {
                    //     ui.label(RichText::new(interface_explanation).extra_letter_spacing(0.5));
                    // });
                });
            });
    }
}
