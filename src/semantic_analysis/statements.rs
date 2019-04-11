use code_generation::visualizer::{GraphvizFormatter, Visualizer};
use semantic_analysis::expressions::ExprNode;
use utilities::utility;

#[derive(Debug, Clone, PartialEq)]
pub enum StatementType {
    AssignmentStatement,
    BreakBlock,
    ReturnBlock,
}

pub trait StatementTypeTrait {
    fn get_statement_type(&self) -> StatementType;
}

#[derive(Clone, PartialEq)]
pub struct AssignmentBlock {
    pub id: String,
    assignment_id: Option<String>,
    expression: Option<ExprNode>,
}

impl AssignmentBlock {
    pub fn init(assignment_id: String, expression: ExprNode) -> AssignmentBlock {
        AssignmentBlock {
            id: utility::get_new_uuid(),
            assignment_id: Some(assignment_id),
            expression: Some(expression),
        }
    }

    pub fn get_expression(&self) -> Option<ExprNode> {
        self.clone().expression
    }
}

impl StatementTypeTrait for AssignmentBlock {
    fn get_statement_type(&self) -> StatementType {
        StatementType::AssignmentStatement
    }
}

impl Visualizer for AssignmentBlock {
    fn build_graphviz(&self) -> String {
        if self.get_statement_type() == StatementType::AssignmentStatement {
            let id = &self.id;
            let assignment_id = &self.assignment_id.unwrap();
            let expression = &self.expression.unwrap();
            return GraphvizFormatter::concat_three(
                &GraphvizFormatter::build_node_label(id, assignment_id),
                &expression.build_graphviz(),
                &GraphvizFormatter::build_edge(id, &expression.id),
            );
        }
        panic!("This should never happen");
    }
}

#[derive(Clone, PartialEq)]
pub struct ReturnBlock {
    pub id: String,
    pub return_expression: Option<ExprNode>,
}

impl ReturnBlock {
    pub fn init(return_expression: ExprNode) -> ReturnBlock {
        ReturnBlock {
            id: utility::get_new_uuid(),
            return_expression: Some(return_expression),
        }
    }
}

impl StatementTypeTrait for ReturnBlock {
    fn get_statement_type(&self) -> StatementType {
        StatementType::ReturnBlock
    }
}

#[derive(Clone, PartialEq)]
pub struct BreakBlock {
    pub id: String,
}

impl BreakBlock {
    pub fn init() -> BreakBlock {
        BreakBlock {
            id: utility::get_new_uuid(),
        }
    }
}

impl StatementTypeTrait for BreakBlock {
    fn get_statement_type(&self) -> StatementType {
        StatementType::BreakBlock
    }
}
