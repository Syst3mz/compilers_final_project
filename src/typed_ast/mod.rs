use std::collections::HashMap;
use crate::ast::statement::Statement;
use crate::typed_ast::r#type::Type;
use crate::typed_ast::typed_statement::TypedStatement;

pub mod r#type;
pub mod typed_statement;
pub mod typed_expression;

pub type TypedBlock = Vec<Statement>;


pub struct Typer {
    environment: HashMap<String, Type>,
    typed_ast: Vec<TypedStatement>
}

impl Typer {
    pub fn type_ast(ast: Vec<Statement>) -> Vec<TypedStatement> {
        let mut typer = Self {
            environment: Default::default(),
            typed_ast: vec![],
        };

        typer.run_typer(ast);
        typer.typed_ast
    }

    fn run_typer(&mut self, on: Vec<Statement>) {
        for statement in on {
            self.type_statement(statement)
        }
    }

    fn type_statement(&mut self, statement: Statement) {

    }

    fn type_expr()
}