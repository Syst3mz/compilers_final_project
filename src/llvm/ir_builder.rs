use itertools::Itertools;
use thiserror::Error;
use crate::ast::binary_operator::BinaryOperator;
use crate::llvm::counters::Counters;
use crate::llvm::element::Element;
use crate::llvm::element::Element::{Elem, Scope};
use crate::llvm::ir_builder::TempOrConst::{Const, Temp};
use crate::parser::token::Token;
use crate::typed_ast::r#type::Type;
use crate::typed_ast::typed_expression::TypedExpression;
use crate::typed_ast::typed_statement::TypedStatement;
use crate::typed_ast::TypedBlock;

#[derive(Debug, Error)]
pub enum ConverterError {
    #[error("Tried to treat {0} as a value, it is not.")]
    UnableToConvertToValue(String)
}

enum TempOrConst {
    Temp(String, Type),
    Const(String, Type)
}

impl TempOrConst {
    fn to_ir(self, include_type: bool) -> String {
        let (value, type_) = match self {
            Temp(name, type_) => (name, type_.llvm_type()),
            Const(value, type_) => (value, type_.llvm_type())
        };

        if include_type {
            format!("{} {}", type_, value)
        } else {
            format!("{}", value)
        }
    }
}
pub struct IrBuilder {
    counters: Counters,
}

impl IrBuilder {
    pub fn new() -> Self {
        Self {
            counters: Counters::new(),
        }
    }

    fn push_expr(&mut self, scope: &mut Vec<Element>, type_: Type, expr: impl AsRef<str>) -> TempOrConst {
        let name = self.counters.next(type_.llvm_type());
        let home = Temp(name.clone(), type_.clone());
        scope.push(Elem(format!("{} = {} {}", name, type_.llvm_type(), expr.as_ref())));
        return home;
    }

    fn push_int(&mut self, scope: &mut Vec<Element>, int: Token) -> TempOrConst {
        self.push_expr(scope, Type::Int, int.lexeme())
    }

    fn push_bool(&mut self, scope: &mut Vec<Element>, bool: bool) -> TempOrConst {
        self.push_expr(scope, Type::Bool, if bool { "1" } else { "0" })
    }

    fn convert_block(&mut self, body: TypedBlock) -> anyhow::Result<Vec<Element>> {
        let mut new_scope = vec![];
        for statement in body.body.clone() {
            self.convert_statement(statement, &mut new_scope)?;
        }

        return Ok(new_scope);
    }

    pub fn convert_statement(&mut self, statement: TypedStatement, scope: &mut Vec<Element>) -> anyhow::Result<Option<String>> {
        match statement {
            TypedStatement::VariableDeclaration { .. } => unimplemented!(),
            TypedStatement::FunctionDefinitionStatement(func_def) => {
                let header = format!("define {} @{}({}) {{",
                                     func_def.type_.llvm_type(),
                                     func_def.name.lexeme(),
                    func_def.arg_list.iter()
                                         .map(|(token, type_)| format!("{} %{}", type_.llvm_type(), token.lexeme()))
                                         .join(", ")
                );
                let body = self.convert_block(func_def.body)?;
                let tail = String::from("}");

                scope.push(Elem(header));
                scope.push(Scope(body));
                scope.push(Elem(tail));

                Ok(None)
            },
            TypedStatement::Assignment { .. } => unimplemented!(),
            TypedStatement::While { .. } => unimplemented!(),
            TypedStatement::Return(e) => {
                let v = self.convert_expression(e, scope)?;
                scope.push(Elem(format!("ret {}", v.to_ir(true))));

                Ok(None)
            },
            TypedStatement::Expression(e) => unimplemented!(),
        }
    }

    pub fn convert_expression(&mut self, expression: TypedExpression, scope: &mut Vec<Element>) -> anyhow::Result<TempOrConst> {
        type T = TypedExpression;
        match expression {
            T::If { .. } => unimplemented!(),
            T::BinaryOperation { lhs, operator, rhs, type_ } => {
                let lhs = self.convert_expression(*lhs, scope)?;
                let rhs = self.convert_expression(*rhs, scope)?;
                let op_string = match operator {
                    BinaryOperator::Add => "add",
                    _ => unimplemented!()
                };

                let ans_name = self.counters.next(op_string);
                let ans = Temp(ans_name.clone(),type_.clone());

                scope.push(Elem(format!("{} = {} {} {}, {}",
                                        ans_name,
                                        op_string,
                                        type_.llvm_type(),
                                        lhs.to_ir(false),
                                        rhs.to_ir(false)
                )));

                return Ok(ans)
            },
            T::FunctionCall { .. } => unimplemented!(),
            T::UnaryOperation { .. } => unimplemented!(),
            T::Int(t) => Ok(Const(t.lexeme().to_string(), Type::Int)),
            T::Bool(v, _) => Ok(Const(String::from(if v { "1" } else { "0" }), Type::Bool)),
            T::List(_, _) => unimplemented!(),
            T::Name(t, type_) => Ok(self.push_expr(scope, type_, t.lexeme())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::llvm::convert;
    use crate::parser::Parser;
    use crate::typer::Typer;

    #[test]
    fn forty_two() -> anyhow::Result<()> {
        let ast = Parser::new(crate::testing::demo_programs::THE_UNIVERSE).parse().unwrap();
        let typed = Typer::type_ast(ast)?;
        let converted = convert(typed)?;
        assert_eq!(converted.join("\n"), "define i32 @main() {\n\tret i32 42\n}");

        Ok(())
    }

    #[test]
    fn forty_two_add() -> anyhow::Result<()> {
        let ast = Parser::new(crate::testing::demo_programs::THE_UNIVERSE_BY_ADDITION).parse().unwrap();
        let typed = Typer::type_ast(ast)?;
        let converted = convert(typed)?;
        assert_eq!(converted.join("\n"), "define i32 @main() {\n\t%add_1 = add i32 20, 22\n\tret i32 %add_1\n}");

        Ok(())
    }
}