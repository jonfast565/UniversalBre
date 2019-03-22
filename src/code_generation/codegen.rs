use code_generation::mir::MirInstructions;

pub struct FasmGenerator {
}

pub struct FasmDirective {
    pub directive: String
}

pub struct FasmComment {
    pub comment: String,
}

pub struct FasmInstruction {
    pub operand: String,
    pub operator1: String,
    pub operator2: String,
}

impl FasmInstruction {
    pub fn get_instruction_string(&self) -> String {
        let result = self.operand.clone() + " " + &self.operator1.clone() + ", " + &self.operator2.clone();
        result
    }
}

impl FasmGenerator {
    pub fn generate_asm(&self, p: &MirInstructions) -> String {
        let program_asm = self.generate_program_asm(p);
        program_asm.iter().fold(String::new(), | acc, x | {
            let instruction_string = x.get_instruction_string();
            acc + &instruction_string + "\n" 
        })
    }

    pub fn generate_program_asm(&self, p: &MirInstructions) -> Vec<FasmInstruction> {
        vec!(
            FasmInstruction {
                operand: "mov".to_string(),
                operator1: "eax".to_string(),
                operator2: "ebx".to_string(),
            },
            FasmInstruction {
                operand: "mov".to_string(),
                operator1: "eax".to_string(),
                operator2: "5".to_string(),
            },
            FasmInstruction {
                operand: "mov".to_string(),
                operator1: "5".to_string(),
                operator2: "[ebp+4]".to_string(),
            })
    }
}