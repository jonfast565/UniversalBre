pub struct BinaryExpr {}

impl BinaryExpr {}

pub enum StatementType {
	AssignmentStatement,
	BreakStatement,
}

pub struct StatementBlock {
	statement_type: StatementType,
	assignment_id: Option<String>,
	expression: Option<BinaryExpr>
}

impl StatementBlock {
	pub fn init_with_assignment(assignment_id: String, expression: BinaryExpr) -> StatementBlock {
		StatementBlock {
			statement_type: StatementType::AssignmentStatement,
			assignment_id: Some(assignment_id),
			expression: Some(expression)
		}
	}
}

pub struct ArgumentBlock {
	name: String
}

impl ArgumentBlock {
	pub fn init(name: String) -> ArgumentBlock {
		ArgumentBlock {
			name: name
		}
	}
}

pub struct FunctionBlock {
	name: String,
	arguments: Vec<ArgumentBlock>,
	body: Vec<SemanticBlock>
}

impl FunctionBlock {
	pub fn init(name: String, arguments: Vec<ArgumentBlock>, body: Vec<SemanticBlock>) -> FunctionBlock {
		FunctionBlock {
			name: name,
			arguments: arguments,
			body: body
		}
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
