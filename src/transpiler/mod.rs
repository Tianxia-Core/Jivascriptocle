// ffp: FFP-JIVA-AST edge-agent rust
pub mod ast;
pub mod codegen;
pub mod parser;

pub use ast::{
    ComparisonOp, CompileError, FieldDecl, InvariantClause, InvariantDecl, JivaModule,
    KenningDecl, PrimitiveType, RecordDecl, TargetLang,
};
pub use codegen::transpile_module;
pub use parser::parse_jiva_module;

/// Parses, type-checks, and transpiles `.jiva` source into the requested target language.
pub fn compile_jiva_source(source: &str, target: TargetLang) -> Result<String, CompileError> {
    let module = parse_jiva_module(source)?;
    Ok(transpile_module(&module, target))
}
