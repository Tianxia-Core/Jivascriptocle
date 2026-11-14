// ffp: FFP-JIVA-CLI edge-agent rust
//! `jivascriptocle` — Command-line compiler, polyglot AST transpiler & ARSI verifier.

use jivascriptocle::{
    compile_jiva_source, parse_jiva_module, ArtificialKenningEngine, JivaslokanHuberProbe,
    OracularEngine, Stratum0Facts, TargetLang,
};
use std::env;
use std::fs;
use std::process;

fn print_usage() {
    println!(
        "jivascriptocle v1.2.0 (Apache-2.0) — RASA-X-MACHINA · tianxiaOS
Deterministic Neurosymbolic Polyglot AST Transpiler & ARSI Verification CLI

USAGE:
  jivascriptocle check <schema.jiva>
  jivascriptocle ast <schema.jiva>
  jivascriptocle transpile --target <rust|zig|ocaml|ts|nix> <schema.jiva>
  jivascriptocle kenning-scan \"<prompt text>\"
  jivascriptocle arsi-demo
  jivascriptocle probe-demo"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "--version" | "-V" => {
            println!("jivascriptocle 1.2.0 (Apache-2.0)");
        }
        "--help" | "-h" | "help" => {
            print_usage();
        }
        "check" => {
            if args.len() < 3 {
                eprintln!("Usage: jivascriptocle check <schema.jiva>");
                process::exit(2);
            }
            let src = fs::read_to_string(&args[2]).unwrap_or_else(|e| {
                eprintln!("Failed to read '{}': {e}", args[2]);
                process::exit(1);
            });
            match parse_jiva_module(&src) {
                Ok(m) => {
                    println!(
                        "[OK] Module '{}' (@domain={}, @ffp={}) verified | records={} invariants={} kennings={} | blake3={}",
                        m.name,
                        m.domain,
                        m.ffp_id,
                        m.records.len(),
                        m.invariants.len(),
                        m.kennings.len(),
                        m.ast_blake3
                    );
                }
                Err(err) => {
                    eprintln!("{err}");
                    process::exit(1);
                }
            }
        }
        "ast" => {
            if args.len() < 3 {
                eprintln!("Usage: jivascriptocle ast <schema.jiva>");
                process::exit(2);
            }
            let src = fs::read_to_string(&args[2]).unwrap_or_else(|e| {
                eprintln!("Failed to read '{}': {e}", args[2]);
                process::exit(1);
            });
            match parse_jiva_module(&src) {
                Ok(m) => println!("{}", serde_json::to_string_pretty(&m).unwrap()),
                Err(err) => {
                    eprintln!("{err}");
                    process::exit(1);
                }
            }
        }
        "transpile" => {
            if args.len() < 5 || args[2] != "--target" {
                eprintln!("Usage: jivascriptocle transpile --target <rust|zig|ocaml|ts|nix> <schema.jiva>");
                process::exit(2);
            }
            let target = TargetLang::parse_flag(&args[3]).unwrap_or_else(|e| {
                eprintln!("{e}");
                process::exit(2);
            });
            let src = fs::read_to_string(&args[4]).unwrap_or_else(|e| {
                eprintln!("Failed to read '{}': {e}", args[4]);
                process::exit(1);
            });
            match compile_jiva_source(&src, target) {
                Ok(code) => print!("{code}"),
                Err(err) => {
                    eprintln!("{err}");
                    process::exit(1);
                }
            }
        }
        "kenning-scan" => {
            if args.len() < 3 {
                eprintln!("Usage: jivascriptocle kenning-scan \"<prompt>\"");
                process::exit(2);
            }
            let engine = ArtificialKenningEngine::new();
            let res = engine.ak_shield_dekenning_scan(&args[2]);
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
            if !res.is_clean {
                process::exit(3);
            }
        }
        "arsi-demo" => {
            let engine = OracularEngine::default();
            let facts = Stratum0Facts::default();
            let verdict = engine.verify_arsi_fidelity(&facts, 0.88, 0.21, 0.85);
            println!("{}", serde_json::to_string_pretty(&verdict).unwrap());
        }
        "probe-demo" => {
            let probe = JivaslokanHuberProbe::default();
            let acts = [0.42, -0.19, 0.77, 0.12, -0.31, 0.64, 0.08, -0.25];
            let safe = [0.40, -0.20, 0.75, 0.10, -0.30, 0.65, 0.09, -0.24];
            let res = probe.evaluate_hidden_state(&acts, &safe);
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        other => {
            eprintln!("Unknown subcommand '{other}'");
            print_usage();
            process::exit(2);
        }
    }
}
