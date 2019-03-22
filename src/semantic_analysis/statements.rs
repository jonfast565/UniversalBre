use code_generation::visualizer::Visualizer;
use semantic_analysis::expressions::ExprNode;
use utilities::utility;

#[derive(Debug, Clone, PartialEq)]
pub enum StatementType {
    AssignmentStatement,
    BreakStatement,
    ReturnStatement,
}

pub struct StatementBlock {
    pub id: String,
    return_id: Option<String>,
    statement_type: StatementType,
    assignment_id: Option<String>,
    expression: Option<ExprNode>,
}

impl StatementBlock {
    pub fn init_with_assignment(assignment_id: String, expression: ExprNode) -> StatementBlock {
        StatementBlock {
            id: utility::get_new_uuid(),
            return_id: None,
            statement_type: StatementType::AssignmentStatement,
            assignment_id: Some(assignment_id),
            expression: Some(expression),
        }
    }

    pub fn init_with_break() -> StatementBlock {
        StatementBlock {
            id: utility::get_new_uuid(),
            return_id: None,
            statement_type: StatementType::BreakStatement,
            assignment_id: None,
            expression: None,
        }
    }

    pub fn init_with_return(return_id: String) -> StatementBlock {
        StatementBlock {
            id: utility::get_new_uuid(),
            return_id: Some(return_id),
            statement_type: StatementType::ReturnStatement,
            assignment_id: None,
            expression: None,
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