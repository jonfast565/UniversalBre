use code_generation::visualizer::Visualizer;
use semantic_analysis::expressions::ExprNode;
use utilities::utility;

#[derive(Debug, Clone, PartialEq)]
pub enum StatementType {
    AssignmentStatement,
    BreakStatement,
}

pub struct StatementBlock {
    pub id: String,
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