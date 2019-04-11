use code_generation::visualizer::{GraphvizFormatter, Visualizer};
use semantic_analysis::data_types::DataType;
use semantic_analysis::operation_types::OperationType;
use utilities::utility;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprType {
    Binary,
    Literal,
    Variable,
}

#[derive(Clone, PartialEq)]
pub struct ExprNode {
    pub id: String,
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
    pub fn init_binary(
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
    pub fn init_literal(value: String, data_type: DataType) -> ExprNode {
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

    pub fn init_variable(value: String) -> ExprNode {
        ExprNode {
            id: utility::get_new_uuid(),
            expr_type: ExprType::Variable,
            left_node: None,
            right_node: None,
            value: Some(value),
            operation_type: None,
            data_type: None,
        }
    }

    pub fn get_expression_type(&self) -> ExprType {
        self.expr_type.clone()
    }

    pub fn get_right_node(&self) -> Option<Box<ExprNode>> {
        self.clone().right_node
    }

    pub fn get_left_node(&self) -> Option<Box<ExprNode>> {
        self.clone().left_node
    }

    pub fn is_leaf(&self) -> bool {
        self.expr_type != ExprType::Binary
    }

    pub fn left_child_is_internal(&self) -> bool {
        match self.clone().left_node {
            Some(node) => !node.is_leaf(),
            None => true,
        }
    }

    pub fn right_child_is_internal(&self) -> bool {
        match self.clone().right_node {
            Some(node) => !node.is_leaf(),
            None => true,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_type(&self) -> DataType {
        self.clone().data_type.unwrap()
    }

    pub fn get_value(&self) -> String {
        self.clone().value.unwrap()
    }

    pub fn get_operation_type(&self) -> OperationType {
        self.clone().operation_type.unwrap()
    }
}

impl Visualizer for ExprNode {
    fn build_graphviz(&self) -> String {
        let id = self.id;
        if self.expr_type == ExprType::Literal || self.expr_type == ExprType::Variable {
            let data_type = match self.data_type {
                Some(data_type) => data_type,
                None => DataType::NoneType,
            };
            let no_value = "No Value".to_string();
            let value = match self.value {
                Some(value) => value,
                None => no_value,
            };
            return GraphvizFormatter::build_node_bilabel(&id, &value, &data_type.to_string());
        } else if self.expr_type == ExprType::Binary {
            let op_type = self.operation_type.unwrap();
            let left_node = self.left_node.unwrap();
            let right_node = self.right_node.unwrap();
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
