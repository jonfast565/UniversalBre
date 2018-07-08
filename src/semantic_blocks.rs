use visualizer::Visualizer;
use utility;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprType {
	Binary,
	Literal,
	Variable
}

pub struct ExprNode {
	id: String,
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
			id: utility::get_new_uuid(),
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
			id: utility::get_new_uuid(),
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
			id: utility::get_new_uuid(),
			expr_type: ExprType::Variable,
			left_node: None,
			right_node: None,
			value: Some(value),
			operation_type: None,
			data_type: None, //TODO: Not wise
		}
	}
}

impl Visualizer for ExprNode {
	fn build_graphviz(&self) -> String {
		let id = &self.id;
		if self.expr_type == ExprType::Literal || self.expr_type == ExprType::Variable {
			let data_type = match self.data_type.as_ref() {
				Some(data_type) => &data_type,
				None => &DataType::NoneType
			};
			let no_value = "No Value".to_string();
			let value = match self.value.as_ref() {
				Some(value) => &value,
				None => &no_value
			};
			return format!("\"{}\" [label=\"{}: {:?}\"]", id, value, data_type);
		} else if self.expr_type == ExprType::Binary {
			let op_type = self.operation_type.as_ref().unwrap();
			let left_node = self.left_node.as_ref().unwrap();
			let right_node = self.right_node.as_ref().unwrap();
			let current_nodes = format!("\"{}\" [label=\"{:?}\"]\n{}\n{}", id, op_type, 
				left_node.build_graphviz(), 
				right_node.build_graphviz());
			let current_left_connection = format!("\"{}\" -> \"{}\"", id, left_node.id);
			let current_right_connection = format!("\"{}\" -> \"{}\"", id, right_node.id);
			return format!("{}\n{}\n{}", current_nodes, current_left_connection, current_right_connection);
		} else {
			panic!("Invalid EXPR_TYPE: This should never happen");
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementType {
	AssignmentStatement,
	BreakStatement,
}

pub struct StatementBlock {
	id: String,
	statement_type: StatementType,
	assignment_id: Option<String>,
	expression: Option<ExprNode>,
}

impl StatementBlock {
	pub fn init_with_assignment(assignment_id: String, expression: ExprNode) -> StatementBlock {
		StatementBlock {
			id: utility::get_new_uuid(),
			statement_type: StatementType::AssignmentStatement,
			assignment_id: Some(assignment_id),
			expression: Some(expression),
		}
	}
}

impl Visualizer for StatementBlock {
	fn build_graphviz(&self) -> String {
		if self.statement_type == StatementType::AssignmentStatement {
			let assignment_id = self.assignment_id.as_ref().unwrap();
			let expression = self.expression.as_ref().unwrap();
			return format!(
				"{}\n{}\n{}",
				format!("\"{}\" [label=\"Assign {}\"]", &self.id, assignment_id),
				expression.build_graphviz(),
				format!("\"{}\" -> \"{}\"", &self.id, expression.id)
			);
		}
		panic!("Invalid STATEMENT_TYPE: This should never happen");
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
	StringType,
	IntegerType,
	FloatType,
	BooleanType,
	NoneType
}

#[derive(Debug, Clone, PartialEq)]
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

impl Visualizer for FunctionBlock {
	fn build_graphviz(&self) -> String {
		String::new()
	}
}

#[derive(Debug, Clone, PartialEq)]
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

impl Visualizer for LoopBlock {
	fn build_graphviz(&self) -> String {
		String::new()
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

impl Visualizer for SemanticBlock {
	fn build_graphviz(&self) -> String {
		if let Some(statement_block) = self.statement_block.as_ref() {
			return statement_block.build_graphviz();
		} 
		String::new()
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

impl Visualizer for Program {
	fn build_graphviz(&self) -> String {
		let blocks_ref = self.blocks.as_slice();
		let mut result = String::new();
		for block in blocks_ref {
			result = format!("digraph g {{\n {}\n{} \n}}", result, block.build_graphviz());
		}
		result
	}
}
