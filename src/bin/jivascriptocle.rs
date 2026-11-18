// ffp: FFP-JIVA-CLI edge-agent rust
//! `jivascriptocle` v1.3.0 — Deterministic Neurosymbolic Polyglot AST Transpiler,
//! `TOON` Schema Compiler, `Kamadhenu` Multi-Target Materializer, ARSI Verifier & MCP Server.

use jivascriptocle::{
    compile_jiva_source, compile_module_to_toon, generate_clean_starter_jiva,
    materialize_all_targets, parse_jiva_module, ArtificialKenningEngine, JivaslokanHuberProbe,
    OracularEngine, Stratum0Facts, TargetLang,
};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process;

const VERSION: &str = "1.3.0";

fn print_usage() {
    println!(
        "jivascriptocle v{VERSION} (Apache-2.0) — RASA-X-MACHINA · tianxiaOS
Deterministic Neurosymbolic Polyglot AST Transpiler, TOON Compiler & ARSI Verification CLI

USAGE:
  jivascriptocle init [dir] [--name CleanroomContract]    Bootstrap fresh zero-PII .jiva starter schema
  jivascriptocle check <schema.jiva>                      Parse, type-check & print BLAKE3 AST seal
  jivascriptocle ast <schema.jiva>                        Emit full typed JSON AST
  jivascriptocle toon <schema.jiva> [--json]              Compile compact TOON wire schema + C-ABI offsets
  jivascriptocle transpile --target <rust|zig|ocaml|ts|nix|toon> <schema.jiva>
  jivascriptocle build-all <schema.jiva> [--out-dir dist/polyglot]
  jivascriptocle kenning-scan \"<prompt text>\"
  jivascriptocle arsi-demo
  jivascriptocle probe-demo
  jivascriptocle mcp-stdio                                Start JSON-RPC 2.0 MCP server ($0.00, 0 tokens)"
    );
}

fn run_mcp_stdio() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map_while(Result::ok) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Ok(req) = serde_json::from_str::<Value>(trimmed) else {
            continue;
        };
        let Some(id) = req.get("id").cloned() else {
            continue;
        };
        let method = req.get("method").and_then(Value::as_str).unwrap_or("");
        let result = match method {
            "initialize" => json!({
                "protocolVersion": "2024-11-05",
                "capabilities": { "tools": {} },
                "serverInfo": { "name": "jivascriptocle", "version": VERSION }
            }),
            "tools/list" => json!({
                "tools": [
                    {
                        "name": "jiva_compile_toon",
                        "description": "Parse, verify, and compile a .jiva schema into Token-Optimized Object Notation (TOON) with C-ABI byte offsets.",
                        "inputSchema": {
                            "type": "object",
                            "properties": { "source": { "type": "string" } },
                            "required": ["source"]
                        }
                    },
                    {
                        "name": "jiva_kenning_scan",
                        "description": "Scan untrusted prompt text for adversarial metaphoric kennings via AK-Shield ($0.00, 0 LLM tokens).",
                        "inputSchema": {
                            "type": "object",
                            "properties": { "prompt": { "type": "string" } },
                            "required": ["prompt"]
                        }
                    }
                ]
            }),
            "tools/call" => {
                let params = req.get("params").cloned().unwrap_or(json!({}));
                let name = params.get("name").and_then(Value::as_str).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));
                if name == "jiva_kenning_scan" {
                    let prompt = args.get("prompt").and_then(Value::as_str).unwrap_or("");
                    let res = ArtificialKenningEngine::new().ak_shield_dekenning_scan(prompt);
                    json!({ "content": [{ "type": "text", "text": serde_json::to_string_pretty(&res).unwrap_or_default() }] })
                } else {
                    let src = args.get("source").and_then(Value::as_str).unwrap_or("");
                    match parse_jiva_module(src) {
                        Ok(m) => {
                            let toon = compile_module_to_toon(&m);
                            json!({ "content": [{ "type": "text", "text": serde_json::to_string_pretty(&toon).unwrap_or_default() }] })
                        }
                        Err(err) => json!({ "isError": true, "content": [{ "type": "text", "text": err.to_string() }] }),
                    }
                }
            }
            _ => json!({}),
        };
        println!("{}", json!({ "jsonrpc": "2.0", "id": id, "result": result }));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "--version" | "-V" | "version" => {
            println!("jivascriptocle {VERSION} (Apache-2.0)");
        }
        "--help" | "-h" | "help" => {
            print_usage();
        }
        "init" | "init-clean" => {
            let mut dir = PathBuf::from(".");
            let mut mod_name = String::from("CleanroomContract");
            let mut i = 2;
            while i < args.len() {
                if args[i] == "--name" && i + 1 < args.len() {
                    mod_name = args[i + 1].clone();
                    i += 2;
                } else if !args[i].starts_with('-') {
                    dir = PathBuf::from(&args[i]);
                    i += 1;
                } else {
                    i += 1;
                }
            }
            if let Err(e) = fs::create_dir_all(&dir) {
                eprintln!("Failed to create '{}': {e}", dir.display());
                process::exit(1);
            }
            let out_file = dir.join(format!("{}.jiva", mod_name.to_ascii_lowercase()));
            let starter = generate_clean_starter_jiva(&mod_name);
            if let Err(e) = fs::write(&out_file, starter) {
                eprintln!("Failed to write '{}': {e}", out_file.display());
                process::exit(1);
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "ok": true,
                    "edition": "PUBLIC_CLEANROOM_SEED",
                    "schema_path": out_file,
                    "module_name": mod_name
                }))
                .unwrap()
            );
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
        "toon" => {
            if args.len() < 3 {
                eprintln!("Usage: jivascriptocle toon <schema.jiva> [--json]");
                process::exit(2);
            }
            let as_json = args.iter().any(|a| a == "--json");
            let src = fs::read_to_string(&args[2]).unwrap_or_else(|e| {
                eprintln!("Failed to read '{}': {e}", args[2]);
                process::exit(1);
            });
            match parse_jiva_module(&src) {
                Ok(m) => {
                    let artifact = compile_module_to_toon(&m);
                    if as_json {
                        println!("{}", serde_json::to_string_pretty(&artifact).unwrap());
                    } else {
                        print!("{}", artifact.toon_text);
                    }
                }
                Err(err) => {
                    eprintln!("{err}");
                    process::exit(1);
                }
            }
        }
        "transpile" => {
            if args.len() < 5 || args[2] != "--target" {
                eprintln!("Usage: jivascriptocle transpile --target <rust|zig|ocaml|ts|nix|toon> <schema.jiva>");
                process::exit(2);
            }
            let src = fs::read_to_string(&args[4]).unwrap_or_else(|e| {
                eprintln!("Failed to read '{}': {e}", args[4]);
                process::exit(1);
            });
            if args[3].eq_ignore_ascii_case("toon") {
                match parse_jiva_module(&src) {
                    Ok(m) => print!("{}", compile_module_to_toon(&m).toon_text),
                    Err(err) => {
                        eprintln!("{err}");
                        process::exit(1);
                    }
                }
                return;
            }
            let target = TargetLang::parse_flag(&args[3]).unwrap_or_else(|e| {
                eprintln!("{e}");
                process::exit(2);
            });
            match compile_jiva_source(&src, target) {
                Ok(code) => print!("{code}"),
                Err(err) => {
                    eprintln!("{err}");
                    process::exit(1);
                }
            }
        }
        "build-all" | "materialize" => {
            if args.len() < 3 {
                eprintln!("Usage: jivascriptocle build-all <schema.jiva> [--out-dir dist/polyglot]");
                process::exit(2);
            }
            let src = fs::read_to_string(&args[2]).unwrap_or_else(|e| {
                eprintln!("Failed to read '{}': {e}", args[2]);
                process::exit(1);
            });
            let mut out_dir = PathBuf::from("dist/polyglot");
            let mut i = 3;
            while i < args.len() {
                if args[i] == "--out-dir" && i + 1 < args.len() {
                    out_dir = PathBuf::from(&args[i + 1]);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            match materialize_all_targets(&src, &out_dir) {
                Ok(manifest) => println!("{}", serde_json::to_string_pretty(&manifest).unwrap()),
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
        "mcp-stdio" | "mcp" => {
            run_mcp_stdio();
        }
        other => {
            eprintln!("Unknown subcommand '{other}'");
            print_usage();
            process::exit(2);
        }
    }
}
