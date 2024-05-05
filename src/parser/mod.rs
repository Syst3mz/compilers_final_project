use itertools::Itertools;
use crate::ast::binary_operator::BinaryOperator;
use crate::ast::binary_operator::BinaryOperator::{Add, GreaterThan};
use crate::ast::Block;
use crate::ast::expression::Expression;
use crate::ast::expression::Expression::{BinaryOperation, List, UnaryOperation};
use crate::ast::statement::{FunctionDefinition, Statement};
use crate::ast::statement::Statement::{Assignment, FunctionDefinitionStatement};
use crate::ast::unary_operator::UnaryOperator;
use crate::parser::lexer::Lexer;
use crate::parser::parser_error::{ParserError, ParserErrorKind};
use crate::parser::token_holder::TokenHolder;
use crate::parser::token_kind::TokenKind;
use crate::parser::token_kind::TokenKind::*;
use core::ops::Fn;
use anyhow::Context;
use crate::typed_ast::r#type::Type;

pub mod lexer;
mod lex_table;
pub mod location;
pub mod token;
pub mod token_kind;
mod token_holder;
pub mod parser_error;

pub struct Parser {
    tokens: TokenHolder
}

impl Parser {
    pub fn new(text: impl AsRef<str>) -> Self {
        Self {
            tokens: TokenHolder::new(Lexer::new(text).lex())
        }
    }

    fn unexpected_token(&self, expected: Option<TokenKind>) -> ParserError {
        let expected = expected.map(|x| format!("Expected {:?}", x));
        ParserError::new(ParserErrorKind::UnexpectedToken, self.tokens.current(), expected)
    }

    fn unexpected_token_alternates(&self, expected: Vec<TokenKind>) -> ParserError {
        let expected = expected.iter().map(|x| format!("{:?}", x)).join(", ");
        ParserError::new(ParserErrorKind::UnexpectedToken, self.tokens.current(), Some(format!("Expected on of: {}", expected)))
    }

    fn parse_list<T>(
        &mut self,
        parser: impl Fn(&mut Self) -> anyhow::Result<T>,
        ending: TokenKind
    ) -> anyhow::Result<Vec<T>> {
        let mut insides = vec![];
        while self.tokens.t_match(ending).is_none(){
            insides.push(parser(self)?);

            if self.tokens.t_match(Comma).is_none() && self.tokens.expect(ending).is_none() {
                return Err(self.unexpected_token(Some(Comma)))
                    .context("Error parsing list.");
            }
        }
        Ok(insides)
    }

    fn invalid_name(&self) -> ParserError {
        ParserError::new(ParserErrorKind::InvalidName, self.tokens.current(), Some(String::from("Expected a name!")))
    }

    fn parse_type(&mut self) -> anyhow::Result<Type> {
        if self.tokens.t_match(IntType).is_some() {
            return Ok(Type::Int)
        }

        if self.tokens.t_match(BoolType).is_some() {
            return Ok(Type::Bool)
        }

        if self.tokens.t_match(ListType).is_some() {
            return Ok(Type::List(Box::new(self.parse_type()?)))
        }

        return Err(self.unexpected_token_alternates(vec![IntType, BoolType, ListType]))
            .context("Error parsing a type!")
    }

    fn parse_var_declaration(&mut self) -> anyhow::Result<Statement> {
        let name = self.parse_atom()?;
        let name = match name {
            Expression::Name(t) => {t}
            _ => {return Err(self.invalid_name())
                .context("Expected a name to start a variable declaration.")}
        };

        
        if self.tokens.t_match(Colon).is_none() {
            return Err(self.unexpected_token(Some(Colon)))
                .context("Expected a colon in a variable declaration")
        }

        let ast_type = self.parse_type()?;

        if self.tokens.t_match(Equals).is_none() {
            return Err(self.unexpected_token(Some(Equals)))
                .context("Expected an equals in a variable declaration")
        }

        let value = self.parse_expr()?;

        return Ok(Statement::VariableDeclaration {
            name,
            type_: ast_type,
            value,
        })
    }

    fn eat_semicolon(&mut self) -> anyhow::Result<()> {
        let is_end_of_block = self.tokens.expect(RCurlyBrace).is_some();
        let is_semi_colon = self.tokens.t_match(Semicolon).is_some();

        if !(is_semi_colon || is_end_of_block) {
            return Err(self.unexpected_token(Some(Semicolon))).context("Expected a semi-colon or end of block.");
        }

        return Ok(())
    }
    
    fn parse_statement(&mut self) -> anyhow::Result<Statement> {

        if self.tokens.t_match(Let).is_some() {

            let var_decl = self.parse_var_declaration()?;
            self.eat_semicolon()?;
            return Ok(var_decl);
        }
        if self.tokens.t_match(While).is_some() {
            return Ok(Statement::While {
                condition: self.parse_expr()?,
                body: self.parse_block()?,
            })
        }
        if self.tokens.t_match(Fn).is_some() {
            let name = if let Some(name) = self.tokens.t_match(Name) {
                name
            } else {
                return Err(self.invalid_name()).context("Invalid name for a function.")
            };

            if self.tokens.t_match(LParen).is_none() {
                return Err(self.unexpected_token(Some(LParen)))
                    .context("Expected a LParen after a name in function call.")
            }

            let args = self.parse_list(|parser| {
                let name = parser.parse_atom()?;
                let name = match name {
                    Expression::Name(t) => t,
                    _ => {return Err(parser.invalid_name())
                        .context("Expected a name for a function argument.")}
                };

                if parser.tokens.t_match(Colon).is_none() {
                    return Err(parser.unexpected_token(Some(Colon)))
                        .context("Expected a colon in a variable declaration")
                }

                let type_ = parser.parse_type()?;

                return Ok((name, type_))
            }, RParen)?;

            if self.tokens.t_match(Arrow).is_none() {
                return Err(self.unexpected_token(Some(Arrow)))
                    .context("A function declaration needs an arrow to delimit type!")
            }

            let type_ = self.parse_type()?;
            let block = self.parse_block()?;

            return Ok(FunctionDefinitionStatement(FunctionDefinition {
                name,
                type_,
                arg_list: args,
                body: block,
            }))
        }
        if self.tokens.t_match(Return).is_some() {
            let ret = Statement::Return(self.parse_expr()?);
            self.eat_semicolon()?;
            return Ok(ret)
        }
        let expr = self.parse_expr()?;

        match expr {
            Expression::Name(ref t) => {
                if self.tokens.t_match(Equals).is_some() {
                    let value = self.parse_expr()?;
                    self.eat_semicolon()?;

                    return Ok(Assignment {
                        to: t.clone(),
                        value,
                    })
                }
            }
            // rember that you can have an if and allow it not to have a semicolon on it.
            Expression::If { .. } => {
                // match a semicolon if it exists
                self.tokens.t_match(Semicolon);
                return Ok(Statement::Expression(expr))
            }
            _ => {}
        }


        self.eat_semicolon()?;
        return Ok(Statement::Expression(expr))
    }
    
    fn parse_block(&mut self) -> anyhow::Result<Block> {
        if self.tokens.t_match(LCurlyBrace).is_none() { 
            return Err(self.unexpected_token(Some(LCurlyBrace)))
                .context("Blocks must start with a {")
        }

        let mut block = vec![];
        while self.tokens.t_match(RCurlyBrace).is_none() {
            block.push(self.parse_statement()?)
        }
        
        return Ok(block)
    }

    fn parse_expr(&mut self) -> anyhow::Result<Expression> {
        return self.parse_logical_binary_operations()
    }

    fn parse_logical_binary_operations(&mut self) -> anyhow::Result<Expression> {
        let mut lhs = self.parse_logical_negation()?;
        loop {
            let operator = if self.tokens.t_match(PipePie).is_some() {
                BinaryOperator::Or
            } else if self.tokens.t_match(AndAnd).is_some() {
                BinaryOperator::And
            } else {
                break
            };

            let rhs = self.parse_logical_negation()?;
            lhs = BinaryOperation {
                lhs: Box::new(lhs),
                operator,
                rhs: Box::new(rhs),
            }
        }
        return Ok(lhs)
    }

    fn parse_logical_negation(&mut self) -> anyhow::Result<Expression>{
        if self.tokens.t_match(Bang).is_some() {
            return Ok(UnaryOperation {
                operator: UnaryOperator::Not,
                rhs: Box::new(self.parse_equality()?),
            })
        }

        self.parse_equality()
    }

    fn parse_equality(&mut self) -> anyhow::Result<Expression> {
        let mut lhs = self.parse_comparison()?;
        loop {
            if !self.tokens.t_match(EqualsEquals).is_some() {
                break
            }

            let rhs = self.parse_comparison()?;
            lhs = BinaryOperation {
                lhs: Box::new(lhs),
                operator: BinaryOperator::Equals,
                rhs: Box::new(rhs),
            }
        }
        return Ok(lhs)
    }

    fn parse_comparison(&mut self) -> anyhow::Result<Expression> {
        let mut lhs = self.parse_add()?;
        loop {
            if !self.tokens.t_match(RAngleBracket).is_some() {
                break
            }

            let rhs = self.parse_add()?;
            lhs = BinaryOperation {
                lhs: Box::new(lhs),
                operator: GreaterThan,
                rhs: Box::new(rhs),
            }
        }
        return Ok(lhs)
    }

    fn parse_add(&mut self) -> anyhow::Result<Expression> {
        let mut lhs = self.parse_unary_sub()?;
        loop {
            if !self.tokens.t_match(Plus).is_some() {
                break
            }

            let rhs = self.parse_unary_sub()?;
            lhs = BinaryOperation {
                lhs: Box::new(lhs),
                operator: Add,
                rhs: Box::new(rhs),
            }
        }
        return Ok(lhs)
    }

    fn parse_unary_sub(&mut self) -> anyhow::Result<Expression> {
        if self.tokens.t_match(Minus).is_some() {
            return Ok(UnaryOperation {
                operator: UnaryOperator::Sub,
                rhs: Box::new(self.parse_atom()?),
            })
        }

        self.parse_atom()
    }

    fn parse_atom(&mut self) -> anyhow::Result<Expression> {
        let token = self.tokens.next().unwrap();
        return match token.kind() {
            LParen => {
                let inside = self.parse_expr()?;
                if self.tokens.t_match(RParen).is_none() {
                    return Err(self.unexpected_token(Some(RParen)))
                        .context("Expected a closing parentheses to an expression.")
                }
                Ok(inside)
            }
            LBracket => {
                let insides = self.parse_list(Self::parse_expr, RBracket)?;

                Ok(List(insides))
            }
            Int => {
                Ok(Expression::Int(token))
            }
            True => {
                Ok(Expression::Bool(true, token))
            }
            False => {
                Ok(Expression::Bool(false, token))
            }
            Name => {
                let name = token;
                if self.tokens.t_match(LParen).is_some() {
                    let arguments = self.parse_list(Self::parse_expr, RParen)?;
                    return Ok(Expression::FunctionCall { name, arguments })
                }

                Ok(Expression::Name(name))
            },
            If => {
                let condition = self.parse_expr()?;

                let true_block = self.parse_block()?;


                if self.tokens.t_match(Else).is_none() {
                    return Ok(Expression::If {
                        condition: Box::new(condition),
                        true_block,
                        else_block: None,
                    });
                }

                return if self.tokens.expect(If).is_some() {
                    let else_block = self.parse_atom()?;
                    let else_block = vec![Statement::Expression(else_block)];
                    Ok(Expression::If {
                        condition: Box::new(condition),
                        true_block,
                        else_block: Some(else_block),
                    })
                } else {
                    Ok(Expression::If {
                        condition: Box::new(condition),
                        true_block,
                        else_block: Some(self.parse_block()?),
                    })
                }
            }
            _ => {
                Err(self.unexpected_token(None))
                .context(
                    format!(
                        "Attempted to parse {:?}({}) as an expression.",
                        self.tokens.current().kind(),
                        self.tokens.current().lexeme()
                    )
                )
            }
        }
    }

    pub fn parse(mut self) -> anyhow::Result<Vec<Statement>>{
        return Ok(vec![self.parse_statement()?])
    }


}

#[cfg(test)]
mod tests {
    use crate::testing::s_expr::SExpr;
    use crate::testing::to_s_expr::ToSExpr;
    use super::*;

    fn to_s_expr(statements: Vec<Statement>) -> Vec<SExpr<String>> {
        statements.into_iter().map(|x| x.to_s_expr()).collect()
    }

    #[test]
    fn one() {
        let text = "1;";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("1")])
    }

    #[test]
    fn one_element_list() {
        let text = "[1];";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(list 1)")])
    }

    #[test]
    fn two_element_list() {
        let text = "[1, 2];";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(list 1 2)")])
    }

    #[test]
    fn two_element_list_trailing_comma() {
        let text = "[1, 2, ];";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(list 1 2)")])
    }

    #[test]
    fn expr_list() {
        let text = "[1, 2, 3 + 4];";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(list 1 2 (+ 3 4))")])
    }

    #[test]
    fn expr_list_after() {
        let text = "[1, 2, 3 + 4] + 5;";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(+ (list 1 2 (+ 3 4)) 5)")])
    }

    #[test]
    fn neg_one() {
        let text = "-1;";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(- 1)")])
    }

    #[test]
    fn plus_neg() {
        let text = "20 + -1;";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(+ 20 (- 1))")])
    }

    #[test]
    fn equals() {
        let text = "20 == 4;";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(== 20 4)")])
    }


    #[test]
    fn simple_math() {
        let text = "1 + 2;";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(+ 1 2)")])
    }

    #[test]
    fn ordered_math() {
        let text = "1 + 2 + 3;";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(+ (+ 1 2) 3)")])
    }

    #[test]
    fn ordered_math_long() {
        let text = "1 + 2 + 3 + 4;";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(+ (+ (+ 1 2) 3) 4)")])
    }

    #[test]
    fn var_decl() {
        let text = "let x: int = 4;";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(variable_declaration x:int 4)")])
    }

    #[test]
    fn var_assign() {
        let text = "x = 4;";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(= x 4)")])
    }

    #[test]
    fn while_2() {
        let text = "while x > 2 { 2; }";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(while (> x 2) (2))")])
    }

    #[test]
    fn nested_while_2() {
        let text = "while x > 2 { while y > 2 { 2 } }";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(while (> x 2) (while (> y 2) (2)))")])
    }

    #[test]
    fn while_2_semicolon_elided() {
        let text = "while x > 2 { 2 }";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(while (> x 2) (2))")])
    }

    #[test]
    fn return_2() {
        let text = "return 2;";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(return 2)")])
    }

    #[test]
    fn elided_semicolon_if() {
        let text = "if x { y; }";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(if x (y))")])
    }

    #[test]
    fn double_elided_semicolon() {
        let text = "if x { y }";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(if x (y))")])
    }

    #[test]
    fn nested_if() {
        let text = "if x { if y { z } }";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(if x (if y (z)))")])
    }

    #[test]
    fn paren_expr() {
        let text = "(1 + 2 + 3) + (1 + 2);";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(+ (+ (+ 1 2) 3) (+ 1 2))")])
    }

    #[test]
    fn func_call() {
        let text = "cat();";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(cat)")])
    }

    #[test]
    fn func_def_1() {
        let text = "fn func(a:int, b:bool) -> int {}";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(function_define func a:int b:bool (empty_block) ->int)")])
    }

    #[test]
    fn func_def_2() {
        let text = "fn func(a:int, b:bool) -> int { return a + b; }";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(function_define func a:int b:bool (return (+ a b)) ->int)")])
    }
}
