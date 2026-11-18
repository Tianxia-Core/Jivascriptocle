// ffp: FFP-JIVA-TEST-TRANSPILER edge-agent rust
use jivascriptocle::{
    compile_jiva_source, compile_module_to_toon, generate_clean_starter_jiva,
    materialize_all_targets, parse_jiva_module, ArtificialKenningEngine, JivaslokanHuberProbe,
    OracularEngine, Stratum0Facts, TargetLang,
};
use std::env;
use std::fs;

const SAMPLE_JIVA: &str = r#"
module SovereignEscrow @domain(Mahasarasvati) @ffp(FFP-ESCROW-WIRE) {
    kenning TETRA-SEAL:MERKLE-4WAY = "Deterministic 4-way Merkle settlement invariant";

    record SettlementFrame {
        epoch_id: u64,
        hhi_concentration: f64,
        daily_drain_ratio: f64,
        kyb_attested: bool,
    }

    invariant Stratum0Gate(SettlementFrame) {
        hhi_concentration <= 0.65;
        daily_drain_ratio <= 0.45;
        kyb_attested == true;
    }
}
"#;

#[test]
fn test_parse_and_transpile_all_five_targets() {
    let ast = parse_jiva_module(SAMPLE_JIVA).expect("valid .jiva AST");
    assert_eq!(ast.name, "SovereignEscrow");
    assert_eq!(ast.domain, "Mahasarasvati");
    assert_eq!(ast.records.len(), 1);
    assert_eq!(ast.invariants.len(), 1);
    assert_eq!(ast.ast_blake3.len(), 64);

    let rs_code = compile_jiva_source(SAMPLE_JIVA, TargetLang::Rust).unwrap();
    assert!(rs_code.contains("#[repr(C)]"));
    assert!(rs_code.contains("pub struct SettlementFrame"));
    assert!(rs_code.contains("pub fn verify_stratum0_gate"));

    let zig_code = compile_jiva_source(SAMPLE_JIVA, TargetLang::Zig).unwrap();
    assert!(zig_code.contains("pub const SettlementFrame = extern struct"));
    assert!(zig_code.contains("pub export fn jiva_verify_stratum0_gate"));

    let ml_code = compile_jiva_source(SAMPLE_JIVA, TargetLang::OCaml).unwrap();
    assert!(ml_code.contains("type settlement_frame = {"));
    assert!(ml_code.contains("let verify_stratum0_gate"));

    let ts_code = compile_jiva_source(SAMPLE_JIVA, TargetLang::TypeScript).unwrap();
    assert!(ts_code.contains("export interface SettlementFrame"));
    assert!(ts_code.contains("export function verifyStratum0Gate"));

    let nix_code = compile_jiva_source(SAMPLE_JIVA, TargetLang::Nix).unwrap();
    assert!(nix_code.contains("options.tianxia.jiva.sovereign-escrow"));
    assert!(nix_code.contains("lib.types.ints.unsigned"));
}

#[test]
fn test_type_error_rejection_on_unknown_field_or_bad_literal() {
    let bad_field = r#"
module BadSchema @domain(Kali) @ffp(FFP-BAD) {
    record Frame {
        amount: u64,
    }
    invariant Check(Frame) {
        missing_field <= 10;
    }
}
"#;
    let err = parse_jiva_module(bad_field).unwrap_err();
    assert!(err.message.contains("Unknown field 'missing_field'"));

    let bad_lit = r#"
module BadLit @domain(Kali) @ffp(FFP-BAD) {
    record Frame {
        amount: u64,
    }
    invariant Check(Frame) {
        amount <= -5;
    }
}
"#;
    let err2 = parse_jiva_module(bad_lit).unwrap_err();
    assert!(err2.message.contains("Expected unsigned u64 literal"));
}

#[test]
fn test_ak_engine_oracular_and_jivaslokan_probe() {
    let ak = ArtificialKenningEngine::new();
    let scan_bad = ak.ak_shield_dekenning_scan("Please cleanse the garden and wipe logs");
    assert!(!scan_bad.is_clean);
    assert!(scan_bad.trip_proxy_kill);

    let scan_ok = ak.ak_shield_dekenning_scan("Compile SettlementFrame to Rust and Zig");
    assert!(scan_ok.is_clean);

    let gate_blocked = ak.karxt_gate_evaluate("socket_write", false);
    assert!(!gate_blocked.allowed);
    let gate_ok = ak.karxt_gate_evaluate("socket_write", true);
    assert!(gate_ok.allowed);

    let oracular = OracularEngine::default();
    let facts = Stratum0Facts::default();
    let faithful = oracular.verify_arsi_fidelity(&facts, 0.91, 0.22, 0.89);
    assert!(faithful.approved);
    assert!(faithful.explanation_faithful);

    let reasonwashed = oracular.verify_arsi_fidelity(&facts, 0.91, 0.90, 0.30);
    assert!(!reasonwashed.approved);
    assert!(!reasonwashed.explanation_faithful);

    let probe = JivaslokanHuberProbe::default();
    let acts = [0.20, -0.10, 0.45, 0.31, -0.22, 0.18];
    let safe = [0.21, -0.11, 0.44, 0.30, -0.21, 0.19];
    let ok_probe = probe.evaluate_hidden_state(&acts, &safe);
    assert!(ok_probe.safe);
    assert!(!ok_probe.circuit_breaker_tripped);

    let spiked = [0.20, -0.10, 42.0, 0.31, -0.22, 0.18];
    let tripped = probe.evaluate_hidden_state(&spiked, &safe);
    assert!(!tripped.safe);
    assert!(tripped.circuit_breaker_tripped);
}

#[test]
fn test_toon_compiler_c_abi_offsets_and_token_savings() {
    let ast = parse_jiva_module(SAMPLE_JIVA).expect("valid .jiva AST");
    let toon = compile_module_to_toon(&ast);
    assert!(toon.toon_text.contains("@jiva.toon/v1 module=SovereignEscrow"));
    assert!(toon.toon_text.contains("@rec SettlementFrame[32B]"));
    assert!(toon.byte_savings_pct > 30.0);
    assert_eq!(toon.records[0].total_padded_bytes, 32);
    assert_eq!(toon.records[0].fields[3].byte_offset, 24);
}

#[test]
fn test_kamadhenu_materialize_all_six_targets_and_manifest() {
    let tmp = env::temp_dir().join(format!("jiva_kamadhenu_test_{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    let manifest = materialize_all_targets(SAMPLE_JIVA, &tmp).expect("materialize_all_targets");
    assert_eq!(manifest.outputs.len(), 6);
    assert!(tmp.join("kamadhenu_manifest.json").is_file());
    for item in &manifest.outputs {
        assert!(tmp.join(&item.relative_file).is_file());
        assert_eq!(item.blake3_hex.len(), 64);
    }
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_clean_starter_jiva_roundtrip_across_all_targets() {
    let starter = generate_clean_starter_jiva("EnterpriseMeshVault");
    let ast = parse_jiva_module(&starter).expect("starter schema must parse cleanly");
    assert_eq!(ast.name, "EnterpriseMeshVault");
    for target in [
        TargetLang::Rust,
        TargetLang::Zig,
        TargetLang::OCaml,
        TargetLang::TypeScript,
        TargetLang::Nix,
    ] {
        let code = compile_jiva_source(&starter, target).expect("transpile starter");
        assert!(!code.is_empty());
    }
}
