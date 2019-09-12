use code_generation::visualizer::{GraphvizFormatter, Visualizer};
use semantic_analysis::expressions::ExprNode;

use utilities::utility;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum StatementType {
    AssignmentStatement,
    BreakBlock,
    ReturnBlock,
}

pub trait StatementTypeTrait {
    fn get_statement_type(&self) -> StatementType;
}

#[derive(Clone, PartialEq, Debug)]
pub struct AssignmentBlock {
    pub id: String,
    assignment_id: Option<String>,
    expression: Option<Arc<ExprNode>>,
}

impl AssignmentBlock {
    pub fn init(assignment_id: String, expression: ExprNode) -> AssignmentBlock {
        AssignmentBlock {
            id: utility::get_new_uuid(),
            assignment_id: Some(assignment_id),
            expression: Some(Arc::new(expression)),
        }
    }

    pub fn get_expression(&self) -> Option<Arc<ExprNode>> {
        self.expression.clone()
    }

    pub fn get_assignment_id(&self) -> String {
        self.assignment_id.clone().unwrap().to_string()
    }
}

impl StatementTypeTrait for AssignmentBlock {
    fn get_statement_type(&self) -> StatementType {
        StatementType::AssignmentStatement
    }
}

impl Visualizer for AssignmentBlock {
    fn build_graphviz(&self) -> String {
        let self_copy = self.clone();
        if self_copy.get_statement_type() != StatementType::AssignmentStatement {
            panic!("This should never happen");
        }
        let id = self_copy.id;
        let assignment_id = self_copy.assignment_id.unwrap();
        let expression = self_copy.expression.unwrap();
        return GraphvizFormatter::concat_three(
            &GraphvizFormatter::build_node_label(&id, &assignment_id),
            &expression.build_graphviz(),
            &GraphvizFormatter::build_edge(&id, &expression.id),
        );
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ReturnBlock {
    pub id: String,
    pub return_expression: Option<Arc<ExprNode>>,
}

impl ReturnBlock {
    pub fn init(return_expression: Arc<ExprNode>) -> ReturnBlock {
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

#[derive(Clone, PartialEq, Debug)]
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
