use crate::ast::arena_ast::{ASTNodeId, AST};
use crate::ast::ast_node::ASTNodeType;
use crate::ast::binary_operator_node::BinaryOperatorNode;
use crate::ast::unary_operator_node::UnaryOperatorNode;
use crate::operators::binary_operators::BinaryOperatorType;
use crate::operators::binary_operators::BinaryOperatorType::Assign;
use crate::operators::unary_operators::UnaryOperatorType;
use crate::semantic::error::SemanticError::{MismatchedBinaryOperatorTypes, MismatchedUnaryOperatorTypes};
use crate::semantic::error::SemanticResult;
use crate::semantic::type_resolution::operator_registry::OperatorRegistry;
use crate::types::data_type::DataType;

pub struct TypeResolver {
    ast: AST,
    unary_op_impl: OperatorRegistry<UnaryOperatorType, DataType>,
    binary_op_impl: OperatorRegistry<BinaryOperatorType, (DataType, DataType)>,
}

impl TypeResolver {
    fn new(ast: AST) -> Self {
        Self {
            ast,
            unary_op_impl: OperatorRegistry::new(),
            binary_op_impl: OperatorRegistry::new(),
        }
    }

    pub fn resolve_type(&self, ast_node_id: ASTNodeId) -> SemanticResult<Option<DataType>> {
        use ASTNodeType::*;
        use DataType::*;

        let node = self.ast.lookup(ast_node_id);

        if let Some(data_type) = &node.data_type {
            return Ok(Some(data_type.clone()));
        }
        
        let data_type = match &node.node_type {
            IntLiteral(_) => Int,
            StringLiteral(_) => String,
            BinaryOperator(op) => self.resolve_binary_operation_type(&op)?,
            _ => unimplemented!("{:?} type resolution unimplemented", node.node_type),
        };

        Ok(Some(data_type))
    }

    fn resolve_unary_operation_type(&self, operator_node: &UnaryOperatorNode) -> SemanticResult<DataType> {
        let operator_type = operator_node.op_type;
        
        let operand_type = match &self.ast.lookup(operator_node.operand).data_type {
            Some(e) => e,
            None => return Err(MismatchedUnaryOperatorTypes(operator_type, None)),
        };

        match self.unary_op_impl.resolve_operation_type(operator_type, operand_type) {
            Some(data_type) => Ok(data_type),
            None => Err(MismatchedUnaryOperatorTypes(operator_type, Some(operand_type.clone()))),
        }
    }

    fn resolve_binary_operation_type(&self, operator_node: &BinaryOperatorNode) -> SemanticResult<DataType> {
        let operator_type = operator_node.op_type;

        let rhs_type_opt = &self.ast.lookup(operator_node.right).data_type;
        let lhs_type_opt = &self.ast.lookup(operator_node.left).data_type;

        let rhs_type = match rhs_type_opt {
            Some(data_type) => data_type.clone(),
            None => {
                return Err(MismatchedBinaryOperatorTypes(operator_type, lhs_type_opt.clone(), None))
            },
        };

        let lhs_type = match lhs_type_opt {
            Some(data_type) => data_type.clone(),
            None => {
                return if operator_type == Assign {
                    Ok(rhs_type)
                } else {
                    Err(MismatchedBinaryOperatorTypes(operator_type, None, rhs_type_opt.clone()))
                }
            }
        };

        match self.binary_op_impl.resolve_operation_type(operator_type, &(lhs_type.clone(), rhs_type.clone())) {
            Some(data_type) => Ok(data_type),
            None => Err(MismatchedBinaryOperatorTypes(operator_type, Some(lhs_type), Some(rhs_type))),
        }
    }

    pub fn resolve_ast_types(ast: AST) -> SemanticResult<AST> {
        let mut resolver = TypeResolver::new(ast);

        for id in resolver.ast.ast_node_id_iter() {
            let t = resolver.resolve_type(id)?;

            resolver.ast.lookup_mut(id).data_type = t;
        }

        Ok(resolver.ast)
    }
}
