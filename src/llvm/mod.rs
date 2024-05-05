use crate::llvm::ir_builder::IrBuilder;
use crate::typed_ast::typed_statement::TypedStatement;

mod counters;
mod ir_builder;
mod element;

pub fn convert(ast: Vec<TypedStatement>) -> anyhow::Result<Vec<String>> {
    let mut builder = IrBuilder::new();
    let mut elements = vec![];
    for statement in ast {
        builder.convert_statement(statement, &mut elements)?;
    }

    Ok(elements.into_iter().map(|x| x.flatten()).flatten().collect())
}