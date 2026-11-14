// ffp: FFP-JIVA-AST edge-agent rust
//! Typed AST representation for `.jiva` neurosymbolic contract & wire schemas.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetLang {
    Rust,
    Zig,
    OCaml,
    TypeScript,
    Nix,
}

impl TargetLang {
    pub fn parse_flag(raw: &str) -> Result<Self, CompileError> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "rust" | "rs" => Ok(Self::Rust),
            "zig" => Ok(Self::Zig),
            "ocaml" | "ml" => Ok(Self::OCaml),
            "ts" | "typescript" => Ok(Self::TypeScript),
            "nix" => Ok(Self::Nix),
            other => Err(CompileError::new(
                0,
                format!("Unsupported transpilation target '{other}' (expected rust|zig|ocaml|ts|nix)"),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimitiveType {
    U64,
    I64,
    F64,
    Bool,
    Bytes32,
    Str,
}

impl PrimitiveType {
    pub fn parse(token: &str, line: usize) -> Result<Self, CompileError> {
        match token {
            "u64" => Ok(Self::U64),
            "i64" => Ok(Self::I64),
            "f64" => Ok(Self::F64),
            "bool" => Ok(Self::Bool),
            "bytes32" => Ok(Self::Bytes32),
            "str" | "string" => Ok(Self::Str),
            other => Err(CompileError::new(
                line,
                format!("Unknown primitive type '{other}' (expected u64|i64|f64|bool|bytes32|str)"),
            )),
        }
    }

    pub fn is_numeric(self) -> bool {
        matches!(self, Self::U64 | Self::I64 | Self::F64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOp {
    Le,
    Ge,
    Lt,
    Gt,
    Eq,
    Ne,
}

impl ComparisonOp {
    pub fn parse(token: &str, line: usize) -> Result<Self, CompileError> {
        match token {
            "<=" => Ok(Self::Le),
            ">=" => Ok(Self::Ge),
            "<" => Ok(Self::Lt),
            ">" => Ok(Self::Gt),
            "==" => Ok(Self::Eq),
            "!=" => Ok(Self::Ne),
            other => Err(CompileError::new(
                line,
                format!("Unsupported invariant operator '{other}' (expected <=|>=|<|>|==|!=)"),
            )),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Le => "<=",
            Self::Ge => ">=",
            Self::Lt => "<",
            Self::Gt => ">",
            Self::Eq => "==",
            Self::Ne => "!=",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KenningDecl {
    pub key: String,
    pub expansion: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDecl {
    pub name: String,
    pub ty: PrimitiveType,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordDecl {
    pub name: String,
    pub fields: Vec<FieldDecl>,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvariantClause {
    pub field: String,
    pub field_type: PrimitiveType,
    pub op: ComparisonOp,
    pub rhs_literal: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvariantDecl {
    pub name: String,
    pub target_record: String,
    pub clauses: Vec<InvariantClause>,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JivaModule {
    pub name: String,
    pub domain: String,
    pub ffp_id: String,
    pub kennings: Vec<KenningDecl>,
    pub records: Vec<RecordDecl>,
    pub invariants: Vec<InvariantDecl>,
    pub ast_blake3: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileError {
    pub line: usize,
    pub message: String,
}

impl CompileError {
    pub fn new(line: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.line > 0 {
            write!(f, "Jivascriptocle compile error at line {}: {}", self.line, self.message)
        } else {
            write!(f, "Jivascriptocle compile error: {}", self.message)
        }
    }
}

impl std::error::Error for CompileError {}
