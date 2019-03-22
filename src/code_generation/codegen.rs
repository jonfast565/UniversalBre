use semantic_analysis::program::Program;

pub struct IntelAsmGenerator {
}

pub struct AsmInstruction {
    pub operand: String,
    pub operator1: String,
    pub operator2: String,
}

impl AsmInstruction {
    pub fn get_instruction_string(&self) -> String {
        let result = self.operand.clone() + " " + &self.operator1.clone() + ", " + &self.operator2.clone();
        result
    }
}

impl IntelAsmGenerator {
    pub fn generate_asm(&self, p: &Program) -> String {
        let program_asm = self.generate_program_asm(p);
        program_asm.iter().fold(String::new(), | acc, x | {
            let instruction_string = x.get_instruction_string();
            acc + &instruction_string + "\n" 
        })
    }

    pub fn generate_program_asm(&self, p: &Program) -> Vec<AsmInstruction> {
        vec!(
            AsmInstruction {
                operand: "mov".to_string(),
                operator1: "eax".to_string(),
                operator2: "ebx".to_string(),
            },
            AsmInstruction {
                operand: "mov".to_string(),
                operator1: "eax".to_string(),
                operator2: "5".to_string(),
            },
            AsmInstruction {
                operand: "mov".to_string(),
                operator1: "5".to_string(),
                operator2: "[ebp+4]".to_string(),
            })
    }
}