use egui_code_editor::Syntax;
use std::collections::BTreeSet;

pub fn sva_syntax() -> Syntax {
    Syntax {
        language: "Simple virtual assembler",
        case_sensitive: true,
        comment: "#",
        comment_multiline: [
            ",.,.,.,.,.,.,.,.,.,.,.,.,,.,.",
            ".,.,.,.,.,.,.,.,.,.,.,.,.,.,",
        ], // there are no multiline comments
        keywords: BTreeSet::from([
            "nop", "NOP", "hlt", "HLT", "mov", "MOV", "add", "ADD", "sub", "SUB", "mul", "MUL",
            "div", "DIV", "mod", "MOD", "inc", "INC", "dec", "DEC", "and", "AND", "or", "OR",
            "xor", "XOR", "not", "NOT", "shl", "SHL", "shr", "SHR", "cmp", "CPM", "psh", "PSH",
            "pop", "POP",
        ]),
        types: BTreeSet::from([
            "jmp", "JMP", "je", "JE", "jne", "JNE", "jl", "JL", "jg", "JG",
        ]),
        special: BTreeSet::from([]),
    }
}
