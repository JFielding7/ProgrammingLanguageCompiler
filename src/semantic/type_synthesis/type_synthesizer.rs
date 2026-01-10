use crate::ast::arena_ast::{ASTNodeId, AST};
use crate::ast::ast_node::ASTNodeType;
use crate::compiler_context::type_arena::DataTypeId;
use crate::compiler_context::CompilerContext;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::binary_operators::BinaryOperator::Assign;
use crate::operators::unary_operators::UnaryOperator;
use crate::semantic::error::SemanticError::*;
use crate::semantic::error::SemanticResult;
use crate::semantic::type_synthesis::operator_registry::OperatorRegistry;
use crate::types::data_type::BuiltinType::{Int, String};
use string_interner::DefaultSymbol;

pub struct TypeSynthesizer<'a> {
    ast: AST,
    unary_op_impl: OperatorRegistry<UnaryOperator>,
    binary_op_impl: OperatorRegistry<BinaryOperator>,
    ctx: &'a mut CompilerContext,
}

impl<'a> TypeSynthesizer<'a> {
    fn new(ast: AST, ctx: &'a mut CompilerContext) -> Self {
        Self {
            ast,
            unary_op_impl: OperatorRegistry::new(),
            binary_op_impl: OperatorRegistry::new(),
            ctx,
        }
    }
    
    fn compute_variable_type(&self, var_name: DefaultSymbol) -> SemanticResult<Option<DataTypeId>> {
        match self.ctx.symbol_table.lookup(var_name) {
            None => Ok(None),
            Some(symbol) => Ok(Some(symbol.data_type))
        }
    }

    fn compute_unary_operation_type(&self, operator_type: UnaryOperator, operand: ASTNodeId) -> SemanticResult<DataTypeId> {

        let operand_type = match &self.ast.lookup(operand).data_type {
            Some(e) => e,
            None => return Err(MismatchedUnaryOperatorTypes(operator_type)),
        };

        match self.unary_op_impl.resolve_operation_type(operator_type, operand_type, &self.ctx.type_arena) {
            Some(data_type) => Ok(data_type),
            None => Err(MismatchedUnaryOperatorTypes(operator_type)),
        }
    }

    fn compute_binary_operation_type(&mut self, operator_type: BinaryOperator, left: ASTNodeId, right: ASTNodeId) -> SemanticResult<DataTypeId> {

        let rhs_type_opt = self.ast.lookup(right).data_type;
        let lhs_type_opt = self.ast.lookup(left).data_type;

        let rhs_type = match rhs_type_opt {
            Some(data_type) => data_type,
            None => {
                return Err(MismatchedBinaryOperatorTypes(operator_type))
            },
        };

        let lhs_type = match lhs_type_opt {
            Some(data_type) => data_type,
            None => {
                return if operator_type == Assign {
                    self.ast.lookup_mut(left).data_type = Some(rhs_type.clone());
                    Ok(rhs_type)
                } else {
                    Err(MismatchedBinaryOperatorTypes(operator_type))
                }
            }
        };

        match self.binary_op_impl.resolve_operation_type(operator_type, &(lhs_type, rhs_type), &self.ctx.type_arena) {
            Some(data_type) => Ok(data_type),
            None => Err(MismatchedBinaryOperatorTypes(operator_type)),
        }
    }

    fn compute_type(&mut self, ast_node_id: ASTNodeId) -> SemanticResult<()> {
        use ASTNodeType::*;

        let node = self.ast.lookup(ast_node_id);

        if let Some(_) = &node.data_type {
            return Ok(());
        }

        let data_type = match &node.node_data_type {
            IntLiteral(_) => Some(self.ctx.type_arena.builtin_type_id(Int)),
            StringLiteral(_) => Some(self.ctx.type_arena.builtin_type_id(String)),

            Variable(var) => self.compute_variable_type(var.name)?,

            UnaryOperator(op) => {
                Some(self.compute_unary_operation_type(op.op_type, op.operand)?)
            },

            BinaryOperator(op) => {
                Some(self.compute_binary_operation_type(op.op_type, op.left, op.right)?)
            },

            _ => unimplemented!("{:?} type resolution unimplemented", node.node_data_type),
        };

        self.ast.lookup_mut(ast_node_id).data_type = data_type;

        Ok(())
    }

    pub fn compute_ast_types(ast: AST, ctx: &mut CompilerContext) -> SemanticResult<AST> {
        let mut synthesizer = TypeSynthesizer::new(ast, ctx);

        for id in synthesizer.ast.ast_node_id_iter() {
            synthesizer.compute_type(id)?;
        }

        Ok(synthesizer.ast)
    }
}
