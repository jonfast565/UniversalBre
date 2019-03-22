use semantic_analysis::program::Program;

pub struct MirInstructionGenerator {}

pub struct MirInstructions {
    instructions: Vec<MirInstruction>,
}

pub struct MirInstruction {
    pub result_operator: Option<MirOperator>,
    pub operand: MirInstructionOperand,
    pub first_arg_operator: Option<MirOperator>,
    pub second_arg_operator: Option<MirOperator>,
    pub flags: MirFlags
}

impl MirInstruction {

}

pub enum MirInstructionOperand {
    Add,
    Subtract,
    Multiply,
    Divide,
    Concat,
    CallFunction,
    PushStack,
    PopStack,
}

pub struct MirFlags {
    pub requires_cast: bool,
}

pub enum MirOperatorType {
    String,
    Boolean,
    Integer,
    Float,
}

pub struct MirOperator {
    pub operator_type: MirOperatorType
}

impl MirInstructionGenerator {
    pub fn generate_mir(&self, p: &Program) -> MirInstructions {
        let result = Vec::<MirInstruction>::new();
        MirInstructions {
            instructions: result,
        }
    }
}
