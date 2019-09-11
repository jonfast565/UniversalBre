use code_generation::visualizer::{GraphvizFormatter, Visualizer};
use semantic_analysis::data_types::DataType;
use semantic_analysis::operation_types::OperationType;
use utilities::utility;

use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprType {
    Binary,
    Literal,
    Variable,
}

impl fmt::Display for ExprType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, PartialEq)]
pub struct ExprNode {
    pub id: String,
    expr_type: ExprType,

    // binary stuff
    left_node: Option<Arc<ExprNode>>,
    right_node: Option<Arc<ExprNode>>,
    operation_type: Option<Arc<OperationType>>,

    // literals and variables
    value: Option<String>,
    data_type: Option<Arc<DataType>>,
}

impl ExprNode {
    pub fn init_binary(
        left_node: ExprNode,
        right_node: ExprNode,
        operation_type: OperationType,
        data_type: DataType
    ) -> ExprNode {
        ExprNode {
            id: utility::get_new_uuid(),
            expr_type: ExprType::Binary,
            left_node: Some(Arc::new(left_node)),
            right_node: Some(Arc::new(right_node)),
            value: None,
            operation_type: Some(Arc::new(operation_type)),
            data_type: Some(Arc::new(data_type)),
        }
    }
    pub fn init_literal(value: String, data_type: DataType) -> ExprNode {
        ExprNode {
            id: utility::get_new_uuid(),
            expr_type: ExprType::Literal,
            left_node: None,
            right_node: None,
            value: Some(value),
            operation_type: None,
            data_type: Some(Arc::new(data_type)),
        }
    }

    pub fn init_variable(value: String, data_type: DataType) -> ExprNode {
        ExprNode {
            id: utility::get_new_uuid(),
            expr_type: ExprType::Variable,
            left_node: None,
            right_node: None,
            value: Some(value),
            operation_type: None,
            data_type: Some(Arc::new(data_type)),
        }
    }

    pub fn get_expression_type(&self) -> &ExprType {
        &self.expr_type
    }

    pub fn get_right_node(&self) -> Option<Arc<ExprNode>> {
        self.right_node
    }

    pub fn get_left_node(&self) -> Option<Arc<ExprNode>> {
        self.left_node
    }

    pub fn is_leaf(&self) -> bool {
        self.expr_type != ExprType::Binary
    }

    pub fn left_child_is_internal(&self) -> bool {
        match self.clone().left_node {
            Some(node) => !node.is_leaf(),
            None => false,
        }
    }

    pub fn right_child_is_internal(&self) -> bool {
        match self.clone().right_node {
            Some(node) => !node.is_leaf(),
            None => false,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_type(&self) -> Arc<DataType> {
        self.data_type.unwrap()
    }

    pub fn get_value(&self) -> String {
        self.value.unwrap().to_string()
    }

    pub fn get_operation_type(&self) -> Arc<OperationType> {
        self.operation_type.unwrap()
    }

    pub fn print(&self) {
        println!("Expr Node {}", self.expr_type.to_string());
    }
}

impl Visualizer for ExprNode {
    fn build_graphviz(&self) -> String {
        let self_copy = self.clone();
        let id = self_copy.id;
        if self_copy.expr_type == ExprType::Literal || self_copy.expr_type == ExprType::Variable {
            let data_type = match self_copy.data_type {
                Some(data_type) => data_type,
                None => Arc::new(DataType::AnyType),
            };
            let no_value = "No Value".to_string();
            let value = match self_copy.value {
                Some(value) => value,
                None => no_value,
            };
            return GraphvizFormatter::build_node_bilabel(&id, &value, &data_type.to_string());
        } else if self_copy.expr_type == ExprType::Binary {
            let op_type = self_copy.operation_type.unwrap();
            let left_node = self_copy.left_node.unwrap();
            let right_node = self_copy.right_node.unwrap();
            let current_nodes = GraphvizFormatter::build_binary_node(
                &id,
                &op_type.to_string(),
                &left_node.build_graphviz(),
                &right_node.build_graphviz(),
            );
            let current_left_connection = GraphvizFormatter::build_edge(&id, &left_node.id);
            let current_right_connection = GraphvizFormatter::build_edge(&id, &right_node.id);
            return GraphvizFormatter::concat_three(
                &current_nodes,
                &current_left_connection,
                &current_right_connection,
            );
        } else {
            panic!("Invalid ExprType: This should never happen");
        }
    }
}
