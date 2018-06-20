pub enum ExprType {
	Binary,
	Literal,
	Variable,
}

pub struct ExprNode {
	expr_type: ExprType,

	// binary stuff
	left_node: Option<Box<ExprNode>>,
	right_node: Option<Box<ExprNode>>,
	operation_type: Option<OperationType>,

	// literals and variables
	value: Option<String>,
	data_type: Option<DataType>,
}

impl ExprNode {
	pub fn init_as_binary(
		left_node: ExprNode,
		right_node: ExprNode,
		operation_type: OperationType,
	) -> ExprNode {
		ExprNode {
			expr_type: ExprType::Binary,
			left_node: Some(Box::new(left_node)),
			right_node: Some(Box::new(right_node)),
			value: None,
			operation_type: Some(operation_type),
			data_type: None,
		}
	}
	pub fn init_as_literal(value: String, data_type: DataType) -> ExprNode {
		ExprNode {
			expr_type: ExprType::Literal,
			left_node: None,
			right_node: None,
			value: Some(value),
			operation_type: None,
			data_type: Some(data_type),
		}
	}

	pub fn init_as_variable(value: String) -> ExprNode {
		ExprNode {
			expr_type: ExprType::Variable,
			left_node: None,
			right_node: None,
			value: Some(value),
			operation_type: None,
			data_type: None, //TODO: Not wise
		}
	}
}

pub enum StatementType {
	AssignmentStatement,
	BreakStatement,
}

pub struct StatementBlock {
	statement_type: StatementType,
	assignment_id: Option<String>,
	expression: Option<ExprNode>,
}

impl StatementBlock {
	pub fn init_with_assignment(assignment_id: String, expression: ExprNode) -> StatementBlock {
		StatementBlock {
			statement_type: StatementType::AssignmentStatement,
			assignment_id: Some(assignment_id),
			expression: Some(expression),
		}
	}
}

pub enum DataType {
	StringType,
	IntegerType,
	FloatType,
	BooleanType,
}

pub enum OperationType {
	BooleanOrOperation,
	BooleanAndOperation,

	BooleanGtOperation,
	BooleanGteOperation,
	BooleanLtOperation,
	BooleanLteOperation,
	BooleanEqOperation,
	BooleanNeOperation,

	ConcatOperation,

	AdditionOperation,
	SubtractionOperation,
	MultiplicationOperation,
	DivisionOperation,
}

pub struct ArgumentBlock {
	name: String,
}

impl ArgumentBlock {
	pub fn init(name: String) -> ArgumentBlock {
		ArgumentBlock { name: name }
	}
}

pub struct FunctionBlock {
	name: String,
	arguments: Vec<ArgumentBlock>,
	body: Vec<SemanticBlock>,
}

impl FunctionBlock {
	pub fn init(
		name: String,
		arguments: Vec<ArgumentBlock>,
		body: Vec<SemanticBlock>,
	) -> FunctionBlock {
		FunctionBlock {
			name: name,
			arguments: arguments,
			body: body,
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
	function_block: Option<FunctionBlock>,
}

impl SemanticBlock {
	pub fn init_with_statement(statement_block: StatementBlock) -> SemanticBlock {
		SemanticBlock {
			statement_block: Some(statement_block),
			loop_block: None,
			function_block: None,
		}
	}
	pub fn init_with_loop(loop_block: LoopBlock) -> SemanticBlock {
		SemanticBlock {
			statement_block: None,
			loop_block: Some(loop_block),
			function_block: None,
		}
	}
	pub fn init_with_function(function_block: FunctionBlock) -> SemanticBlock {
		SemanticBlock {
			statement_block: None,
			loop_block: None,
			function_block: Some(function_block),
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
