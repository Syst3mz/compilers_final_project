use itertools::Itertools;
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

#[derive(Debug, Clone)]
pub enum MemoryValue {
    Temp(String, Type),
    Const(String, Type),
}

impl MemoryValue {
    pub fn to_ir(self, include_type: bool) -> String {
        let (value, type_) = match self {
            Temp(name, type_) => (format!("%{}", name), type_.llvm_type()),
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
    fn store_variable(&mut self, scope: &mut Vec<Element>, type_: Type, var: impl AsRef<str>, from: MemoryValue) -> anyhow::Result<()> {
        let var = var.as_ref();
        let v = Variable::new(var.to_string(), type_.clone());
        let ir = v.store(from);
        scope.push(Elem(ir));
        Ok(())
    }

    fn convert_block(&mut self, block: TypedBlock) -> anyhow::Result<(Vec<Element>, Option<MemoryValue>)> {
        let mut new_scope = vec![];
        let mut final_mv = None;
        for index in 0..block.body.len() {
            let mv = self.convert_statement(block.body[index].clone(), &mut new_scope)?;
            if index == block.body.len() - 1 {
                final_mv = mv
            }
        }

        return Ok((new_scope, final_mv));
    }

    pub fn convert_statement(&mut self, statement: TypedStatement, scope: &mut Vec<Element>) -> anyhow::Result<Option<MemoryValue>> {
        match statement {
            TypedStatement::FunctionDefinitionStatement(func_def) => {
                let header = format!("define {} @{}({}) {{",
                                     func_def.type_.llvm_type(),
                                     func_def.name.lexeme(),
                    func_def.arg_list.iter()
                                         .map(|(token, type_)| format!("{} %_{}", type_.llvm_type(), token.lexeme()))
                                         .join(", ")
                );
                let mut allocs = vec![];
                for (name, type_) in func_def.arg_list.iter() {
                    allocs.push(Elem(format!("{} = alloca {}", format!("%{}", name.lexeme()), type_.clone().llvm_type())));
                    self.store_variable(
                        &mut allocs,
                        type_.clone(),
                        name.lexeme(),
                        Temp(format!("_{}", name.lexeme()), type_.clone())
                    )?;
                }
                let (body, _) = self.convert_block(func_def.body)?;
                let tail = String::from("}");

                scope.push(Elem(header));
                scope.push(Scope(allocs));
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
            TypedStatement::While { condition, body } => {
                let mut while_scope = vec![];
                let while_entry = self.counters.next("while");

                while_scope.push(Elem(format!("br label %{}", &while_entry)));
                self.push_label(&mut while_scope, &while_entry);

                let condition = self.convert_expression(condition, &mut while_scope)?;
                let while_true = self.counters.next("while_true");
                let while_end = self.counters.next("while_end");
                while_scope.push(Elem(format!("br {}, label %{}, label %{}",
                    condition.to_ir(true),
                    &while_true,
                    &while_end
                )));

                self.push_label(&mut while_scope, while_true);
                let (body, _) = self.convert_block(body)?;
                while_scope.push(Scope(body));
                while_scope.push(Elem(format!("br label %{}", while_entry)));

                self.push_label(&mut while_scope, while_end);

                scope.push(Scope(while_scope));
                Ok(None)
            },
            TypedStatement::Return(e) => {
                let v = self.convert_expression(e, scope)?;
                scope.push(Elem(format!("ret {}", v.to_ir(true))));

                Ok(None)
            },
            TypedStatement::Expression(e) => {
                let e = self.convert_expression(e, scope)?;
                Ok(Some(e))
            },
        }
    }

    fn push_label(&self, scope: &mut Vec<Element>, label: impl AsRef<str>) {
        scope.push(Elem(format!("{}:", label.as_ref())));
    }

    fn convert_expression(&mut self, expression: TypedExpression, scope: &mut Vec<Element>) -> anyhow::Result<MemoryValue> {
        type T = TypedExpression;
        match expression {
            T::If { condition, true_block, else_block } => {
                let mut if_scope = vec![];
                let condition = self.convert_expression(*condition, &mut if_scope)?;
                let true_block_type = true_block.type_.clone();

                let ret_var = self.counters.next("if_ret_var");
                let ret_var = Variable::new(ret_var, true_block_type.clone());
                if_scope.push(Elem(
                    format!("%{} = alloca {}",ret_var.name.clone(), ret_var.type_.llvm_type()))
                );

                let if_true = self.counters.next("if_true");
                let if_end = self.counters.next("if_end");
                let if_else = self.counters.next("if_else");

                if_scope.push(Elem(format!("br {}, label %{}, label %{}",
                    condition.to_ir(true),
                    if_true.clone(),
                    if_else.clone(),
                )));
                self.push_label(&mut if_scope, &if_true);
                let (true_scope, final_memory) = self.convert_block(true_block)?;
                if_scope.push(Scope(true_scope));
                if let Some(final_memory) = final_memory {
                    self.store_variable(&mut if_scope, true_block_type.clone(), ret_var.name.clone(), final_memory)?;
                }
                if_scope.push(Elem(format!("br label %{}", &if_end)));

                self.push_label(&mut if_scope, &if_else);

                if let Some(else_block) = else_block {
                    let else_block_type = else_block.type_.clone();
                    let (else_scope, final_memory) = self.convert_block(else_block)?;
                    if_scope.push(Scope(else_scope));
                    if let Some(final_memory) = final_memory{
                        self.store_variable(&mut if_scope, else_block_type, ret_var.name.clone(), final_memory)?;
                    }
                }

                if_scope.push(Elem(format!("br label %{}", &if_end)));

                self.push_label(&mut if_scope, &if_end);

                let ret_var_temp = self.load_variable(&mut if_scope, true_block_type, ret_var.name)?;

                scope.push(Scope(if_scope));
                return Ok(ret_var_temp)
            },
            T::BinaryOperation { lhs, operator, rhs, type_ } => {
                let lhs_type = lhs.get_type();
                let lhs = self.convert_expression(*lhs, scope)?;
                let rhs = self.convert_expression(*rhs, scope)?;
                let (op_string, op_name) = match operator {
                    BinaryOperator::Add => ("add", "add"),
                    BinaryOperator::Equals => ("icmp eq", "eq"),
                    BinaryOperator::GreaterThan => ("icmp sgt", "gt"),
                    BinaryOperator::And => ("and", "and"),
                    BinaryOperator::Or => ("or", "or"),
                };

                let ans_name = self.counters.next(op_name);
                let ans = Temp(ans_name.clone(),type_.clone());


                scope.push(Elem(format!("{} = {} {} {}, {}",
                                        ans.clone().to_ir(false),
                                        op_string,
                                        lhs_type.llvm_type(),
                                        lhs.to_ir(false),
                                        rhs.to_ir(false)
                )));

                return Ok(ans)
            },
            T::FunctionCall { name, arguments, type_ } => {
                let expr_homes: Vec<String> = arguments
                    .into_iter()
                    .map(|x| self.convert_expression(x, scope).map(|x| x.to_ir(true)))
                    .try_collect()?;

                let expr_homes = expr_homes.join(",");


                let ans = self.counters.next(format!("function_{}", name.lexeme()));
                let ans_home = Temp(ans, type_.clone());

                let push = format!("{} = call {} @{}({})",
                                   ans_home.clone().to_ir(false),
                                   type_.llvm_type(),
                                   name.lexeme(),
                                   expr_homes
                );

                scope.push(Elem(push));

                return Ok(ans_home)
            },
            T::UnaryOperation { operator, rhs } => {
                match operator {
                    UnaryOperator::Sub => {
                        let rhs_type = rhs.get_type();
                        let rhs = self.convert_expression(*rhs, scope)?;


                        let ans_name = self.counters.next("sub");
                        let ans = Temp(ans_name.clone(),rhs_type.clone());

                        scope.push(Elem(format!("{} = sub i32 0, {}",
                                                ans.clone().to_ir(false),
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