use code_generation::mir::MirInstructions;

pub struct FasmGenerator {
    pub debug: bool,
}

pub struct FasmDirective {
    pub directive: String,
}

pub struct FasmComment {
    pub comment: String,
}

pub struct FasmInstruction {
    pub operand: String,
    pub first_operator: String,
    pub second_operator: String,
}

impl FasmInstruction {
    pub fn get_instruction_string(&self) -> String {
        let result = self.operand
            + " "
            + &self.first_operator
            + ", "
            + &self.second_operator;
        result
    }
}

impl FasmGenerator {
    pub fn generate_asm(&self, p: &MirInstructions) -> String {
        let program_asm = self.generate_program_asm(p);
        program_asm.iter().fold(String::new(), |acc, x| {
            let instruction_string = x.get_instruction_string();
            acc + &instruction_string + "\n"
        })
    }

    pub fn generate_program_asm(&self, _p: &MirInstructions) -> Vec<FasmInstruction> {
        vec![
            FasmInstruction {
                operand: "mov".to_string(),
                first_operator: "eax".to_string(),
                second_operator: "ebx".to_string(),
            },
            FasmInstruction {
                operand: "mov".to_string(),
                first_operator: "eax".to_string(),
                second_operator: "5".to_string(),
            },
            FasmInstruction {
                operand: "mov".to_string(),
                first_operator: "5".to_string(),
                second_operator: "[ebp+4]".to_string(),
            },
        ]
    }
}
