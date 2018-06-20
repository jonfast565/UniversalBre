pub struct BinaryExpr {}

impl BinaryExpr {}

pub enum StatementType {
	AssignmentStatement,
	BreakStatement,
}

pub struct StatementBlock {
	statement_type: StatementType,
}

impl StatementBlock {
	pub fn init(statement_type: StatementType) -> StatementBlock {
		StatementBlock {
			statement_type: statement_type,
		}
	}
}

pub struct FunctionBlock {}

impl FunctionBlock {
	pub fn init() -> FunctionBlock {
		FunctionBlock {}
	}
}

pub enum LoopType {
	InfiniteLoop,
	ConditionalLoop,
	ForLoop,
}

pub struct LoopBlock {
	loop_type: LoopType,
}

impl LoopBlock {
	pub fn init(loop_type: LoopType) -> LoopBlock {
		LoopBlock {
			loop_type: loop_type,
		}
	}
}

pub struct SemanticBlock {
	statement_block: Option<StatementBlock>,
	loop_block: Option<LoopBlock>,
	function_block: Option<FunctionBlock>
}

impl SemanticBlock {
	pub fn init_with_statement(statement_block: StatementBlock) -> SemanticBlock {
		SemanticBlock {
			statement_block: Some(statement_block),
			loop_block: None,
			function_block: None
		}
	}
	pub fn init_with_loop(loop_block: LoopBlock) -> SemanticBlock {
		SemanticBlock {
			statement_block: None,
			loop_block: Some(loop_block),
			function_block: None
		}
	}
	pub fn init_with_function(function_block: FunctionBlock) -> SemanticBlock {
		SemanticBlock {
			statement_block: None,
			loop_block: None,
			function_block: Some(function_block)
		}
	}
}

pub struct Program {
	blocks: Vec<SemanticBlock>,
}

impl Program {
	pub fn init(blocks: Vec<SemanticBlock>) -> Program {
		Program { blocks: blocks }
	}
}
