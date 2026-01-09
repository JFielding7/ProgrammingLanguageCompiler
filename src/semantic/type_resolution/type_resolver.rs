use crate::ast::arena_ast::{ASTNodeId, AST};
use crate::ast::ast_node::ASTNodeType;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::binary_operators::BinaryOperator::Assign;
use crate::operators::unary_operators::UnaryOperator;
use crate::semantic::error::SemanticError::{MismatchedBinaryOperatorTypes, MismatchedUnaryOperatorTypes};
use crate::semantic::error::SemanticResult;
use crate::semantic::type_resolution::operator_registry::OperatorRegistry;
use crate::types::data_type::DataType;

pub struct TypeResolver {
    ast: AST,
    unary_op_impl: OperatorRegistry<UnaryOperator, DataType>,
    binary_op_impl: OperatorRegistry<BinaryOperator, (DataType, DataType)>,
}

impl TypeResolver {
    fn new(ast: AST) -> Self {
        Self {
            ast,
            unary_op_impl: OperatorRegistry::new(),
            binary_op_impl: OperatorRegistry::new(),
        }
    }

    fn resolve_type(&mut self, ast_node_id: ASTNodeId) -> SemanticResult<()> {
        use ASTNodeType::*;
        use DataType::*;

        let node = self.ast.lookup(ast_node_id);

        if let Some(_) = &node.data_type {
            return Ok(());
        }
        
        let data_type = match &node.node_type {
            IntLiteral(_) => Some(Int),
            StringLiteral(_) => Some(String),
            Variable(v) => None,
            UnaryOperator(op) => {
                Some(self.resolve_unary_operation_type(op.op_type, op.operand)?)
            },
            BinaryOperator(op) => {
                Some(self.resolve_binary_operation_type(op.op_type, op.left, op.right)?)
            },
            _ => unimplemented!("{:?} type resolution unimplemented", node.node_type),
        };

        self.ast.lookup_mut(ast_node_id).data_type = data_type;

        Ok(())
    }
    
    // fn resolve_variable_type(&self) {
    //
    // }

    fn resolve_unary_operation_type(&self, operator_type: UnaryOperator, operand: ASTNodeId) -> SemanticResult<DataType> {

        let operand_type = match &self.ast.lookup(operand).data_type {
            Some(e) => e,
            None => return Err(MismatchedUnaryOperatorTypes(operator_type, None)),
        };

        match self.unary_op_impl.resolve_operation_type(operator_type, operand_type) {
            Some(data_type) => Ok(data_type),
            None => Err(MismatchedUnaryOperatorTypes(operator_type, Some(operand_type.clone()))),
        }
    }

    fn resolve_binary_operation_type(&mut self, operator_type: BinaryOperator, left: ASTNodeId, right: ASTNodeId) -> SemanticResult<DataType> {

        let rhs_type_opt = &self.ast.lookup(right).data_type;
        let lhs_type_opt = &self.ast.lookup(left).data_type;

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
                    self.ast.lookup_mut(left).data_type = Some(rhs_type.clone());
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
            resolver.resolve_type(id)?;
        }

        Ok(resolver.ast)
    }
}
