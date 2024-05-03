use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::llvm::value::{Constant, Value};
use crate::llvm::counters::Counters;
use crate::llvm::LLVM;

pub struct Converter {
    counters: Counters,
    ast: Vec<Statement>,
    llvm: Vec<LLVM>
}

impl Converter {
    pub fn new(ast: Vec<Statement>) -> Self {
        Self {
            counters: Counters::new(),
            ast,
            llvm: vec![],
        }
    }

    pub fn convert(mut self) -> Vec<LLVM> {
        for statement in self.ast.clone().into_iter() {
            let v = self.convert_statement(statement);
            self.llvm.push(v)
        }

        return self.llvm
    }

    fn convert_statement(&mut self, statement: Statement) -> LLVM {
        match statement {
            Statement::VariableDeclaration { .. } => unimplemented!(),
            Statement::FunctionDefinitionStatement(func_def) => unimplemented!(),
            Statement::Assignment { .. } => unimplemented!(),
            Statement::While { .. } => unimplemented!(),
            Statement::Return(e) => unimplemented!(),
            Statement::Expression(_) => unimplemented!(),
        }
    }

    fn convert_expression(&mut self, expression: Expression) -> LLVM {
        match expression {
            Expression::If { .. } => unimplemented!(),
            Expression::BinaryOperation { .. } => unimplemented!(),
            Expression::FunctionCall { .. } => unimplemented!(),
            Expression::UnaryOperation { .. } => unimplemented!(),
            Expression::Int(v) => LLVM::Val(Value::Constant(Constant::Int(v.lexeme().parse().expect("Unable to parse int.")))),
            Expression::Bool(_, _) => unimplemented!(),
            Expression::List(_) => unimplemented!(),
            Expression::Name(_) => unimplemented!(),
        }
    }
}