use itertools::Itertools;
use thiserror::Error;
use crate::ast::binary_operator::BinaryOperator;
use crate::ast::unary_operator::UnaryOperator;
use crate::llvm::counters::Counters;
use crate::llvm::element::Element;
use crate::llvm::element::Element::{Elem, Scope};
use crate::llvm::ir_builder::MemoryValue::{Const, Temp};
use crate::llvm::variable::Variable;
use crate::typed_ast::r#type::Type;
use crate::typed_ast::typed_expression::TypedExpression;
use crate::typed_ast::typed_statement::TypedStatement;
use crate::typed_ast::TypedBlock;

#[derive(Debug, Error)]
pub enum ConverterError {
    #[error("Tried to treat {0} as a value, it is not.")]
    UnableToConvertToValue(String),
}

#[derive(Debug, Clone)]
pub enum MemoryValue {
    Temp(String, Type),
    Const(String, Type),
}

impl MemoryValue {
    pub fn to_ir(self, include_type: bool) -> String {
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

    fn push_expr(&mut self, scope: &mut Vec<Element>, type_: Type, expr: impl AsRef<str>) -> MemoryValue {
        let name = self.counters.next(type_.llvm_type());
        let home = Temp(name.clone(), type_.clone());
        scope.push(Elem(format!("{} = {} {}", name, type_.llvm_type(), expr.as_ref())));
        return home;
    }

    /// Load from a variable into a temp
    fn load_variable(&mut self, scope: &mut Vec<Element>, type_: Type, var: impl AsRef<str>) -> anyhow::Result<MemoryValue> {
        let var = var.as_ref();
        let temp = self.counters.next(var);
        let home = Temp(temp, type_.clone());

        let v = Variable::new(var.to_string(), type_.clone());
        let ir = v.load(home.clone());
        scope.push(Elem(ir));

        return Ok(home)
    }

    /// Store from a temp to a variable
    fn store_variable(&mut self, scope: &mut Vec<Element>, type_: Type, var: impl AsRef<str>, from: MemoryValue) -> Result<(), ConverterError> {
        let var = var.as_ref();
        let v = Variable::new(var.to_string(), type_.clone());
        let ir = v.store(from)?;
        scope.push(Elem(ir));
        Ok(())
    }

    fn convert_block(&mut self, body: TypedBlock) -> anyhow::Result<Vec<Element>> {
        let mut new_scope = vec![];
        for statement in body.body.clone() {
            self.convert_statement(statement, &mut new_scope)?;
        }

        return Ok(new_scope);
    }

    pub fn convert_statement(&mut self, statement: TypedStatement, scope: &mut Vec<Element>) -> anyhow::Result<Option<MemoryValue>> {
        match statement {
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
            TypedStatement::VariableDeclaration { name, type_, value } => {
                let value = self.convert_expression(value, scope)?;
                scope.push(Elem(format!("%{} = alloca {}", name.lexeme(), type_.clone().llvm_type())));
                self.store_variable(scope, type_, name.lexeme(), value)?;
                Ok(None)
            }
            TypedStatement::Assignment { to, value } => {
                let type_ = value.get_type();
                let value = self.convert_expression(value, scope)?;
                self.store_variable(scope, type_, to.lexeme(), value)?;
                Ok(None)
            },
            TypedStatement::While { .. } => unimplemented!(),
            TypedStatement::Return(e) => {
                let v = self.convert_expression(e, scope)?;
                scope.push(Elem(format!("ret {}", v.to_ir(true))));

                Ok(None)
            },
            TypedStatement::Expression(e) => Err(self.convert_expression(e, scope).unwrap_err()),
        }
    }

    fn convert_expression(&mut self, expression: TypedExpression, scope: &mut Vec<Element>) -> anyhow::Result<MemoryValue> {
        type T = TypedExpression;
        match expression {
            T::If { .. } => unimplemented!(),
            T::BinaryOperation { lhs, operator, rhs, type_ } => {
                let lhs_type = lhs.get_type();
                let lhs = self.convert_expression(*lhs, scope)?;
                let rhs = self.convert_expression(*rhs, scope)?;
                let (op_string, op_name) = match operator {
                    BinaryOperator::Add => ("add", "add"),
                    BinaryOperator::Equals => ("icmp eq", "eq"),
                    BinaryOperator::GreaterThan => ("icmp sgt", "gt"),
                    _ => unimplemented!()
                };

                let ans_name = self.counters.next(op_name);
                let ans = Temp(ans_name.clone(),type_.clone());

                scope.push(Elem(format!("{} = {} {} {}, {}",
                                        ans_name,
                                        op_string,
                                        lhs_type.llvm_type(),
                                        lhs.to_ir(false),
                                        rhs.to_ir(false)
                )));

                return Ok(ans)
            },
            T::FunctionCall { .. } => unimplemented!(),
            T::UnaryOperation { operator, rhs } => {
                match operator {
                    UnaryOperator::Sub => {
                        let rhs_type = rhs.get_type();
                        let rhs = self.convert_expression(*rhs, scope)?;


                        let ans_name = self.counters.next("sub");
                        let ans = Temp(ans_name.clone(),rhs_type.clone());

                        scope.push(Elem(format!("{} = sub i32 0, {}",
                                                ans_name,
                                                rhs.to_ir(false)
                        )));

                        return Ok(ans)
                    },
                    UnaryOperator::Not => unimplemented!()
                }
            },
            T::Int(t) => Ok(Const(t.lexeme().to_string(), Type::Int)),
            T::Bool(v, _) => Ok(Const(String::from(if v { "1" } else { "0" }), Type::Bool)),
            T::List(_, _) => unimplemented!(),
            T::Name(t, type_) => self.load_variable(scope, type_, t.lexeme()),
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