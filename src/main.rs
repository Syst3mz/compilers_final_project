use std::path::Path;
use std::process::Output;
use anyhow::Context;
use crate::llvm::convert;
use crate::parser::Parser;
use crate::typer::Typer;

mod ast;
mod parser;
mod testing;
mod interpreter;
mod llvm;
mod typer;
mod typed_ast;

fn ir_text(text: impl AsRef<str>) -> anyhow::Result<Vec<String>> {
    let text = text.as_ref();
    let parsed = Parser::new(text).parse()?;
    let typed = Typer::type_ast(parsed)?;
    return Ok(convert(typed)?)
}

/// Take a program as text in memory and write it to a .ll
fn write_ll(text: impl AsRef<str>, to: impl AsRef<Path>) -> anyhow::Result<()> {
    let text = ir_text(text)?;
    std::fs::write(to.as_ref(), text.join("\n")).context("Unable to write LL file.")
}

/// Take a path to a .ll file and compile it using clang.
fn compile(path: impl AsRef<Path>) -> anyhow::Result<Output>{
    let filename = path.as_ref().with_extension("exe");
    std::process::Command::new("clang")
        .arg("-o")
        .arg(filename)
        .arg(path.as_ref())
        .output()
        .context("Unable to start clang")
}

fn run(path: impl AsRef<Path>) -> anyhow::Result<Output>{
    let filename = path.as_ref().with_extension("exe");
    std::process::Command::new(filename)
        .output()
        .context("Unable to start a.exe")
}



fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::testing::demo_programs::*;
    use super::*;

    fn run_test(path: impl AsRef<Path>, text: impl AsRef<str>) -> anyhow::Result<i32> {
        let path = path.as_ref();
        write_ll(text, path)?;

        let compile_out = path.with_extension("compile_out");
        let compile_err = path.with_extension("compile_err");


        let compile_output = compile(path)?;
        std::fs::write(compile_out, compile_output.stdout)?;
        std::fs::write(compile_err, compile_output.stderr)?;


        let run_output = run(path)?;
        Ok(run_output.status.code().unwrap())
    }

    #[test]
    fn forty_two() -> anyhow::Result<()> {
        let path = ".\\testing\\universe.ll";
        assert_eq!(run_test(path, THE_UNIVERSE)?, 42);
        Ok(())
    }

    #[test]
    fn forty_two_add() -> anyhow::Result<()> {
        let path = ".\\testing\\universe_addition.ll";
        assert_eq!(run_test(path, THE_UNIVERSE_BY_ADDITION)?, 42);
        Ok(())
    }

    #[test]
    fn truth() -> anyhow::Result<()> {
        let path = ".\\testing\\true.ll";
        assert_ne!(run_test(path, TRUE)?, 0);
        Ok(())
    }

    #[test]
    fn falsehood() -> anyhow::Result<()> {
        let path = ".\\testing\\false.ll";
        assert_eq!(run_test(path, FALSE)?, 0);
        Ok(())
    }

    #[test]
    fn assigned_universe() -> anyhow::Result<()> {
        let path = ".\\testing\\assigned_universe.ll";
        assert_eq!(run_test(path, ASSIGNED_UNIVERSE)?, 42);
        Ok(())
    }

    #[test]
    fn assigned_universe_mutation() -> anyhow::Result<()> {
        let path = ".\\testing\\assigned_universe_mutation.ll";
        assert_eq!(run_test(path, ASSIGNED_UNIVERSE_MUTATION)?, 42);
        Ok(())
    }

    #[test]
    fn universal_negation() -> anyhow::Result<()> {
        let path = ".\\testing\\universal_negation.ll";
        assert_eq!(run_test(path, UNIVERSAL_NEGATION)?, 42);
        Ok(())
    }

    #[test]
    fn universal_eq_neg() -> anyhow::Result<()> {
        let path = ".\\testing\\universal_eq_neg.ll";
        assert_eq!(run_test(path, UNIVERSE_EQ_NEG)?, 0);
        Ok(())
    }

    #[test]
    fn universal_eq() -> anyhow::Result<()> {
        let path = ".\\testing\\universal_eq.ll";
        assert_ne!(run_test(path, UNIVERSE_EQ)?, 0);
        Ok(())
    }

    #[test]
    fn universal_g() -> anyhow::Result<()> {
        let path = ".\\testing\\universal_g.ll";
        assert_ne!(run_test(path, UNIVERSE_G)?, 0);
        Ok(())
    }

    #[test]
    fn universal_question() -> anyhow::Result<()> {
        let path = ".\\testing\\universal_question.ll";
        assert_eq!(run_test(path, UNIVERSAL_QUESTION)?, 42);
        Ok(())
    }
}