// ffp: FFP-JIVA-TOON wire-codec rust
//! Token-Optimized Object Notation (`TOON`) Schema Compiler, C-ABI Byte-Layout Planner,
//! Clean-Room Starter Scaffolder & Native Rust `Kamadhenu` Multi-Target Materializer.

use crate::transpiler::{
    parse_jiva_module, transpile_module, JivaModule, PrimitiveType, TargetLang,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToonFieldLayout {
    pub name: String,
    pub wire_tag: String,
    pub byte_offset: usize,
    pub byte_size: usize,
    pub alignment: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToonRecordSpec {
    pub record_name: String,
    pub total_padded_bytes: usize,
    pub max_alignment: usize,
    pub compact_header: String,
    pub fields: Vec<ToonFieldLayout>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToonSchemaArtifact {
    pub module_name: String,
    pub domain: String,
    pub ffp_id: String,
    pub ast_blake3: String,
    pub toon_blake3: String,
    pub toon_text: String,
    pub records: Vec<ToonRecordSpec>,
    pub json_bytes: usize,
    pub toon_bytes: usize,
    pub byte_savings_pct: f64,
}

fn type_size_align_tag(ty: PrimitiveType) -> (usize, usize, &'static str) {
    match ty {
        PrimitiveType::U64 => (8, 8, "u64"),
        PrimitiveType::I64 => (8, 8, "i64"),
        PrimitiveType::F64 => (8, 8, "f64"),
        PrimitiveType::Bool => (1, 1, "b1"),
        PrimitiveType::Bytes32 => (32, 1, "b32"),
        PrimitiveType::Str => (16, 8, "str"), // (ptr, len) fat-slice descriptor
    }
}

fn align_up(offset: usize, align: usize) -> usize {
    if align <= 1 {
        return offset;
    }
    (offset + align - 1) & !(align - 1)
}

/// Compiles a verified `JivaModule` into compact `TOON` (Token-Optimized Object Notation)
/// with deterministic C-ABI byte offsets and prompt-compression telemetry.
pub fn compile_module_to_toon(module: &JivaModule) -> ToonSchemaArtifact {
    let mut lines = Vec::new();
    lines.push(format!(
        "# ffp: {} wire-codec toon\n@jiva.toon/v1 module={} domain={} ast={}",
        module.ffp_id,
        module.name,
        module.domain,
        &module.ast_blake3[..16]
    ));

    for k in &module.kennings {
        lines.push(format!("@k {} => {}", k.key, k.expansion));
    }

    let mut rec_specs = Vec::with_capacity(module.records.len());
    for rec in &module.records {
        let mut offset = 0usize;
        let mut max_align = 1usize;
        let mut field_layouts = Vec::with_capacity(rec.fields.len());
        let mut compact_cols = Vec::with_capacity(rec.fields.len());

        for f in &rec.fields {
            let (sz, al, tag) = type_size_align_tag(f.ty);
            if al > max_align {
                max_align = al;
            }
            offset = align_up(offset, al);
            field_layouts.push(ToonFieldLayout{
                name: f.name.clone(),
                wire_tag: tag.to_string(),
                byte_offset: offset,
                byte_size: sz,
                alignment: al,
            });
            compact_cols.push(format!("{}:{}@{}", f.name, tag, offset));
            offset += sz;
        }
        let total = align_up(offset, max_align);
        let header = format!("{}[{}B]{{{}}}", rec.name, total, compact_cols.join(","));
        lines.push(format!("@rec {header}"));
        rec_specs.push(ToonRecordSpec {
            record_name: rec.name.clone(),
            total_padded_bytes: total,
            max_alignment: max_align,
            compact_header: header,
            fields: field_layouts,
        });
    }

    for inv in &module.invariants {
        let clauses: Vec<String> = inv
            .clauses
            .iter()
            .map(|c| format!("{}{}{}", c.field, c.op.as_str(), c.rhs_literal))
            .collect();
        lines.push(format!(
            "@inv {}({}) => {}",
            inv.name,
            inv.target_record,
            clauses.join(" & ")
        ));
    }

    let toon_text = lines.join("\n") + "\n";
    let toon_blake3 = blake3::hash(toon_text.as_bytes()).to_hex().to_string();
    let json_repr = serde_json::to_string_pretty(module).unwrap_or_default();
    let json_bytes = json_repr.len().max(1);
    let toon_bytes = toon_text.len();
    let savings = if json_bytes > toon_bytes {
        ((json_bytes - toon_bytes) as f64 / json_bytes as f64) * 100.0
    } else {
        0.0
    };

    ToonSchemaArtifact {
        module_name: module.name.clone(),
        domain: module.domain.clone(),
        ffp_id: module.ffp_id.clone(),
        ast_blake3: module.ast_blake3.clone(),
        toon_blake3,
        toon_text,
        records: rec_specs,
        json_bytes,
        toon_bytes,
        byte_savings_pct: (savings * 10.0).round() / 10.0,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KamadhenuTargetFile {
    pub target: String,
    pub relative_file: String,
    pub bytes: usize,
    pub blake3_hex: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KamadhenuManifest {
    pub compiler_version: String,
    pub edition: String,
    pub module_name: String,
    pub domain: String,
    pub ffp_id: String,
    pub ast_blake3: String,
    pub toon_byte_savings_pct: f64,
    pub outputs: Vec<KamadhenuTargetFile>,
}

fn write_atomic_file(dest: &Path, data: &[u8]) -> Result<(), String> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir '{}': {e}", parent.display()))?;
    }
    let tmp = dest.with_extension(format!("tmp.{}", std::process::id()));
    fs::write(&tmp, data).map_err(|e| format!("write '{}': {e}", tmp.display()))?;
    fs::rename(&tmp, dest).map_err(|e| format!("rename '{}': {e}", dest.display()))?;
    Ok(())
}

/// Native Rust `Kamadhenu` Polyglot Materializer: compiles one `.jiva` schema into
/// `Rust`, `Zig`, `OCaml`, `TypeScript`, `Nix`, `TOON`, and `kamadhenu_manifest.json`.
pub fn materialize_all_targets(source: &str, out_dir: &Path) -> Result<KamadhenuManifest, String> {
    let module = parse_jiva_module(source).map_err(|e| e.to_string())?;
    fs::create_dir_all(out_dir).map_err(|e| format!("mkdir '{}': {e}", out_dir.display()))?;

    let stem = module.name.to_ascii_lowercase();
    let targets: [(&str, String, String); 5] = [
        ("rust", format!("{stem}.rs"), transpile_module(&module, TargetLang::Rust)),
        ("zig", format!("{stem}.zig"), transpile_module(&module, TargetLang::Zig)),
        ("ocaml", format!("{stem}.ml"), transpile_module(&module, TargetLang::OCaml)),
        ("typescript", format!("{stem}.ts"), transpile_module(&module, TargetLang::TypeScript)),
        ("nix", format!("{stem}.nix"), transpile_module(&module, TargetLang::Nix)),
    ];

    let mut outputs = Vec::with_capacity(6);
    for (tname, rel_name, code) in targets {
        let p = out_dir.join(&rel_name);
        let raw = code.as_bytes();
        write_atomic_file(&p, raw)?;
        outputs.push(KamadhenuTargetFile {
            target: tname.to_string(),
            relative_file: rel_name,
            bytes: raw.len(),
            blake3_hex: blake3::hash(raw).to_hex().to_string(),
        });
    }

    let toon = compile_module_to_toon(&module);
    let toon_rel = format!("{stem}.toon");
    let toon_path = out_dir.join(&toon_rel);
    write_atomic_file(&toon_path, toon.toon_text.as_bytes())?;
    outputs.push(KamadhenuTargetFile {
        target: "toon".to_string(),
        relative_file: toon_rel,
        bytes: toon.toon_bytes,
        blake3_hex: toon.toon_blake3.clone(),
    });

    let manifest = KamadhenuManifest {
        compiler_version: "1.3.0".to_string(),
        edition: "PUBLIC_CLEANROOM_SEED".to_string(),
        module_name: module.name,
        domain: module.domain,
        ffp_id: module.ffp_id,
        ast_blake3: module.ast_blake3,
        toon_byte_savings_pct: toon.byte_savings_pct,
        outputs,
    };
    let manifest_bytes = serde_json::to_vec_pretty(&manifest).map_err(|e| e.to_string())?;
    write_atomic_file(&out_dir.join("kamadhenu_manifest.json"), &manifest_bytes)?;
    Ok(manifest)
}

/// Generates a fresh, zero-PII Clean-Room `.jiva` starter contract so any developer
/// adopting Jivascriptocle can transpile their own schemas in 60 seconds.
pub fn generate_clean_starter_jiva(module_name: &str) -> String {
    let clean_mod: String = module_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();
    let final_mod = if clean_mod.is_empty() {
        "CleanroomContract".to_string()
    } else {
        clean_mod
    };
    format!(
        "// ffp: FFP-STARTER-CONTRACT wire-codec jiva\n\
module {final_mod} @domain(Mahasarasvati) @ffp(FFP-STARTER-WIRE) {{\n\
    kenning CLEAN-SEED:ZERO-PII = \"Deterministic zero-PII polyglot wire contract\";\n\
\n\
    record TelemetryEnvelope {{\n\
        sequence_id: u64,\n\
        risk_ratio: f64,\n\
        signature_verified: bool,\n\
    }}\n\
\n\
    invariant SafetyEnvelopeGate(TelemetryEnvelope) {{\n\
        risk_ratio <= 0.35;\n\
        signature_verified == true;\n\
    }}\n\
}}\n"
    )
}
