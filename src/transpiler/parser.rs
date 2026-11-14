// ffp: FFP-JIVA-AST edge-agent rust
//! Deterministic line-oriented recursive parser & static type checker for `.jiva` schemas.

use crate::transpiler::ast::{
    ComparisonOp, CompileError, FieldDecl, InvariantClause, InvariantDecl, JivaModule,
    KenningDecl, PrimitiveType, RecordDecl,
};
use std::collections::HashMap;

fn strip_comment(line: &str) -> &str {
    let mut in_quotes = false;
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'"' && (i == 0 || bytes[i - 1] != b'\\') {
            in_quotes = !in_quotes;
        } else if !in_quotes {
            if bytes[i] == b'#' {
                return &line[..i];
            }
            if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'/' {
                return &line[..i];
            }
        }
        i += 1;
    }
    line
}

fn is_valid_ident(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == ':')
}

fn extract_attr(header: &str, attr: &str) -> Option<String> {
    let needle = format!("@{attr}(");
    let start = header.find(&needle)? + needle.len();
    let rest = &header[start..];
    let end = rest.find(')')?;
    Some(rest[..end].trim().trim_matches('"').to_string())
}

/// Parses and statically validates a `.jiva` module into a BLAKE3-sealed `JivaModule` AST.
pub fn parse_jiva_module(source: &str) -> Result<JivaModule, CompileError> {
    let raw_lines: Vec<(usize, String)> = source
        .lines()
        .enumerate()
        .map(|(idx, ln)| (idx + 1, strip_comment(ln).trim().to_string()))
        .filter(|(_, ln)| !ln.is_empty())
        .collect();

    if raw_lines.is_empty() {
        return Err(CompileError::new(1, "Empty .jiva module source"));
    }

    let (first_line_no, first_line) = &raw_lines[0];
    if !first_line.starts_with("module ") || !first_line.ends_with('{') {
        return Err(CompileError::new(
            *first_line_no,
            "Module must open with `module <Name> @domain(<Domain>) @ffp(<FFP-ID>) {`",
        ));
    }

    let mod_tokens: Vec<&str> = first_line.split_whitespace().collect();
    if mod_tokens.len() < 2 || !is_valid_ident(mod_tokens[1]) {
        return Err(CompileError::new(*first_line_no, "Invalid module identifier"));
    }
    let module_name = mod_tokens[1].to_string();
    let domain = extract_attr(first_line, "domain").unwrap_or_else(|| "Matangi".to_string());
    let ffp_id = extract_attr(first_line, "ffp").unwrap_or_else(|| "FFP-JIVA-MODULE".to_string());

    let mut kennings: Vec<KenningDecl> = Vec::new();
    let mut records: Vec<RecordDecl> = Vec::new();
    let mut record_map: HashMap<String, HashMap<String, PrimitiveType>> = HashMap::new();
    let mut invariants: Vec<InvariantDecl> = Vec::new();

    let mut idx = 1;
    let mut closed_module = false;

    while idx < raw_lines.len() {
        let (line_no, line) = &raw_lines[idx];
        if line == "}" {
            closed_module = true;
            break;
        }

        if let Some(rest) = line.strip_prefix("kenning ") {
            let trimmed = rest.trim_end_matches(';').trim();
            let Some((lhs, rhs)) = trimmed.split_once('=') else {
                return Err(CompileError::new(
                    *line_no,
                    "Expected `kenning <KEY> = \"<expansion>\";`",
                ));
            };
            let key = lhs.trim().to_string();
            let expansion = rhs.trim().trim_matches('"').to_string();
            if !is_valid_ident(&key) || expansion.is_empty() {
                return Err(CompileError::new(*line_no, "Invalid kenning key or empty expansion"));
            }
            kennings.push(KenningDecl {
                key,
                expansion,
                line: *line_no,
            });
            idx += 1;
            continue;
        }

        if let Some(rest) = line.strip_prefix("record ") {
            if !rest.ends_with('{') {
                return Err(CompileError::new(*line_no, "Record header must end with `{`"));
            }
            let rec_name = rest.trim_end_matches('{').trim().to_string();
            if !is_valid_ident(&rec_name) {
                return Err(CompileError::new(*line_no, format!("Invalid record name '{rec_name}'")));
            }
            if record_map.contains_key(&rec_name) {
                return Err(CompileError::new(*line_no, format!("Duplicate record '{rec_name}'")));
            }
            idx += 1;
            let mut fields = Vec::new();
            let mut field_lookup = HashMap::new();
            let mut closed_rec = false;
            while idx < raw_lines.len() {
                let (fl_no, fl) = &raw_lines[idx];
                if fl == "}" {
                    closed_rec = true;
                    idx += 1;
                    break;
                }
                let clean = fl.trim_end_matches([',', ';']).trim();
                let Some((fname_raw, ftype_raw)) = clean.split_once(':') else {
                    return Err(CompileError::new(
                        *fl_no,
                        "Expected record field `<name>: <type>`",
                    ));
                };
                let fname = fname_raw.trim().to_string();
                if !is_valid_ident(&fname) {
                    return Err(CompileError::new(*fl_no, format!("Invalid field name '{fname}'")));
                }
                if field_lookup.contains_key(&fname) {
                    return Err(CompileError::new(*fl_no, format!("Duplicate field '{fname}'")));
                }
                let ty = PrimitiveType::parse(ftype_raw.trim(), *fl_no)?;
                field_lookup.insert(fname.clone(), ty);
                fields.push(FieldDecl {
                    name: fname,
                    ty,
                    line: *fl_no,
                });
                idx += 1;
            }
            if !closed_rec || fields.is_empty() {
                return Err(CompileError::new(
                    *line_no,
                    format!("Record '{rec_name}' must contain >= 1 field and close with `}}`"),
                ));
            }
            record_map.insert(rec_name.clone(), field_lookup);
            records.push(RecordDecl {
                name: rec_name,
                fields,
                line: *line_no,
            });
            continue;
        }

        if let Some(rest) = line.strip_prefix("invariant ") {
            if !rest.ends_with('{') {
                return Err(CompileError::new(*line_no, "Invariant header must end with `{`"));
            }
            let head = rest.trim_end_matches('{').trim();
            let Some((inv_name_raw, rec_part)) = head.split_once('(') else {
                return Err(CompileError::new(
                    *line_no,
                    "Expected `invariant <Name>(<RecordName>) {`",
                ));
            };
            let inv_name = inv_name_raw.trim().to_string();
            let target_record = rec_part.trim_end_matches(')').trim().to_string();
            let Some(rec_fields) = record_map.get(&target_record) else {
                return Err(CompileError::new(
                    *line_no,
                    format!("Invariant '{inv_name}' references undeclared record '{target_record}'"),
                ));
            };
            idx += 1;
            let mut clauses = Vec::new();
            let mut closed_inv = false;
            while idx < raw_lines.len() {
                let (cl_no, cl) = &raw_lines[idx];
                if cl == "}" {
                    closed_inv = true;
                    idx += 1;
                    break;
                }
                let clean = cl.trim_end_matches(';').trim();
                let parts: Vec<&str> = clean.split_whitespace().collect();
                if parts.len() != 3 {
                    return Err(CompileError::new(
                        *cl_no,
                        "Expected invariant clause `<field> <op> <literal>;`",
                    ));
                }
                let field_name = parts[0].to_string();
                let Some(&field_type) = rec_fields.get(&field_name) else {
                    return Err(CompileError::new(
                        *cl_no,
                        format!("Unknown field '{field_name}' on record '{target_record}'"),
                    ));
                };
                let op = ComparisonOp::parse(parts[1], *cl_no)?;
                let rhs = parts[2].to_string();
                validate_literal(field_type, op, &rhs, *cl_no)?;
                clauses.push(InvariantClause {
                    field: field_name,
                    field_type,
                    op,
                    rhs_literal: rhs,
                    line: *cl_no,
                });
                idx += 1;
            }
            if !closed_inv || clauses.is_empty() {
                return Err(CompileError::new(
                    *line_no,
                    format!("Invariant '{inv_name}' must contain >= 1 clause and close with `}}`"),
                ));
            }
            invariants.push(InvariantDecl {
                name: inv_name,
                target_record,
                clauses,
                line: *line_no,
            });
            continue;
        }

        return Err(CompileError::new(
            *line_no,
            format!("Unexpected statement: '{line}'"),
        ));
    }

    if !closed_module {
        return Err(CompileError::new(
            raw_lines.last().map(|x| x.0).unwrap_or(1),
            "Unclosed module block (missing trailing `}`)",
        ));
    }

    let canonical_repr = format!(
        "{module_name}:{domain}:{ffp_id}:k={}:r={}:i={}",
        kennings.len(),
        records.len(),
        invariants.len()
    );
    let ast_blake3 = blake3::hash(canonical_repr.as_bytes()).to_hex().to_string();

    Ok(JivaModule {
        name: module_name,
        domain,
        ffp_id,
        kennings,
        records,
        invariants,
        ast_blake3,
    })
}

fn validate_literal(
    ty: PrimitiveType,
    op: ComparisonOp,
    rhs: &str,
    line: usize,
) -> Result<(), CompileError> {
    match ty {
        PrimitiveType::U64 => {
            rhs.parse::<u64>().map_err(|_| {
                CompileError::new(line, format!("Expected unsigned u64 literal, found '{rhs}'"))
            })?;
        }
        PrimitiveType::I64 => {
            rhs.parse::<i64>().map_err(|_| {
                CompileError::new(line, format!("Expected signed i64 literal, found '{rhs}'"))
            })?;
        }
        PrimitiveType::F64 => {
            rhs.parse::<f64>().map_err(|_| {
                CompileError::new(line, format!("Expected f64 literal, found '{rhs}'"))
            })?;
        }
        PrimitiveType::Bool => {
            if !matches!(op, ComparisonOp::Eq | ComparisonOp::Ne) {
                return Err(CompileError::new(
                    line,
                    "Boolean invariant fields only support == or !=",
                ));
            }
            if rhs != "true" && rhs != "false" {
                return Err(CompileError::new(
                    line,
                    format!("Expected boolean literal true|false, found '{rhs}'"),
                ));
            }
        }
        PrimitiveType::Bytes32 | PrimitiveType::Str => {
            return Err(CompileError::new(
                line,
                "Numeric/boolean Stratum-0 invariants cannot compare variable strings or raw bytes32 directly",
            ));
        }
    }
    if !ty.is_numeric() && !matches!(ty, PrimitiveType::Bool) {
        return Err(CompileError::new(line, "Unsupported invariant field type"));
    }
    Ok(())
}
