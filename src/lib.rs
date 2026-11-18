// ffp: FFP-JIVA-LIB edge-agent rust
//! # Jivascriptocle
//!
//! Deterministic Neurosymbolic Polyglot AST Transpiler, `TOON` Schema Compiler,
//! Artificial Kenning (`AK`) IR Engine, Formal `ARSI` Attribution Verifier, and
//! `Jivaslokan` Spike-Weighted Huber Activation Probe.
//!
//! Licensed under Apache-2.0. Part of the `RASA-X-MACHINA` · `tianxiaOS` ecosystem.

pub mod jivaslokan;
pub mod kenning;
pub mod oracular;
pub mod toon;
pub mod transpiler;

pub use jivaslokan::{JivaslokanHuberProbe, ProbeEvaluationResult};
pub use kenning::{
    ArtificialKenningEngine, DspgKenningMeta, KarunaRecoveryResult, KarxtGateResult,
    ShieldScanResult,
};
pub use oracular::{OmniLatticeScores, OracularArsiVerdict, OracularEngine, Stratum0Facts};
pub use toon::{
    compile_module_to_toon, generate_clean_starter_jiva, materialize_all_targets,
    KamadhenuManifest, KamadhenuTargetFile, ToonFieldLayout, ToonRecordSpec, ToonSchemaArtifact,
};
pub use transpiler::{
    compile_jiva_source, parse_jiva_module, transpile_module, CompileError, ComparisonOp,
    FieldDecl, InvariantClause, InvariantDecl, JivaModule, KenningDecl, PrimitiveType, RecordDecl,
    TargetLang,
};
