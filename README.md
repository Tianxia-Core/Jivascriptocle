<div align="center">
  <img src="assets/brand/rasa-x-machina-lockup-dark.svg" alt="RASA-X-MACHINA · tianxiaOS" width="460" />

  # Jivascriptocle

  **Deterministic Neurosymbolic Polyglot AST Transpiler, `TOON` Wire-Contract Compiler, Artificial Kenning (`AK`) IR & Formal `ARSI` Attribution Engine**

  [![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-10b981?style=flat-square)](LICENSE)
  [![Tier 1: Core Compiler](https://img.shields.io/badge/Stratum-Tier%201%20Core%20Framework-c9a84c?style=flat-square)](https://github.com/Tianxia-Core/TianxiaOS)
  [![Nix Flake](https://img.shields.io/badge/Nix-Flake%20Normalized-5277C3?style=flat-square)](flake.nix)
  [![FFP Bounded](https://img.shields.io/badge/DSS--14.2-8%2F8%20FFPs%20Bounded-0ea5e9?style=flat-square)](#architectural-fitness-decomposition-8-ffps)
</div>

<!-- BUILD_LAW_SHOWCASE_START -->

---

## 🌍 In Plain English — What `Jivascriptocle` Is & Why It Matters (Layman's Guide)

- **The Everyday Problem:** When software teams use multiple programming languages—say Rust for speed, OCaml for safety rules, and TypeScript for web screens—keeping data formats synchronized across them by hand breeds subtle, expensive bugs.
- **How It Works (Without the Jargon):** `Jivascriptocle` lets engineers define a capability contract once in a clean specification (`.jiva`) and deterministically compiles it into synchronized **Rust, Zig, OCaml, TypeScript, and Nix** code with zero AI guesswork.
- **Why It Is Different:** Unlike LLM code translators that produce slightly different output every run, `Jivascriptocle` is a deterministic compiler (`Apache-2.0`) embodying Pillar 2 of **[Build Law](https://github.com/Tianxia-Core/Build-Law)**.
- **How It Benefits You:** Teams gain polyglot performance and type safety across their entire stack without maintaining five duplicate codebases by hand.
- **How to Install & Use It in 60 Seconds:** Clone the repo and run `cargo test && cargo build --release` (or `nix build`) to compile `.jiva` specifications locally.

---

## 🏛️ Governed by [`Build Law`](https://github.com/Tianxia-Core/Build-Law) — Engineering Philosophy & Stakeholder Pathways

This repository is a live production showcase of **[`Build Law`](https://github.com/Tianxia-Core/Build-Law)**—the open-source (`MIT`) **4-Pillar, 14-Gate Constitutional Engineering & `5.0×–8.8×` Token-Efficiency Standard** created by **[Collins Takem](https://rasaxmachina.com/collins)** (**RASA-X-MACHINA**):
- **Developers & AI Builders:** Drop **[`Build Law`](https://github.com/Tianxia-Core/Build-Law)** into Claude Code, Cursor, OpenCode, VS Code (`.vsix`), GitHub Copilot, Windsurf, Cline, Aider, or any chat LLM in 60 seconds.
- **CTOs & B2B Engineering Leaders:** Need similar `Rust`/`Go`/`Nix`/Security/FinOps rigor applied to your infrastructure? Scope a fixed-price **72-Hour `VRTTI` Architecture Sprint** at **[vrtti.rasaxmachina.com](https://vrtti.rasaxmachina.com)**.
- **Investors, Researchers, Press & Executive Hiring Committees:** Inspect our cryptographic release proofs across **[`@Tianxia-Core`](https://github.com/Tianxia-Core)**, press assets at **[rasaxmachina.com/press](https://rasaxmachina.com/press)**, and founder dossier at **[rasaxmachina.com/collins](https://rasaxmachina.com/collins)**.

<!-- BUILD_LAW_SHOWCASE_END -->

---
## Overview

**Jivascriptocle** is the open-source (**Apache-2.0**) neurosymbolic interface compiler and polyglot intermediate representation (IR) engine created by **RASA-X-MACHINA** for **tianxiaOS**.

In multi-language distributed systems, security boundaries fail when wire schemas and invariant predicates drift across Rust daemons, Zig shared-memory ring codecs, OCaml capability proxies, TypeScript operator cockpits, and NixOS declarative modules. **Jivascriptocle** eliminates cross-language schema drift by compiling a single BLAKE3-sealed `.jiva` specification into **five target languages simultaneously**, paired with runtime neural-safety introspection:

1. **Deterministic 5-Target `.jiva` AST Transpiler (`Rust` · `Zig` · `OCaml` · `TypeScript` · `Nix`)**: Compiles typed `record` layouts and Stratum-0 `invariant` gates into `#[repr(C)]` Rust structs, Zig C-ABI `extern struct` verifiers, pure OCaml algebraic predicates, strict `readonly` TypeScript validators, and NixOS `lib.mkOption` schemas.
2. **Artificial Kenning (`AK`) IR Engine**: Implements `AK-Zip` macro token compression (`⟦AK:...⟧`), `AK-Shield` adversarial circumlocution/jailbreak unmasking, and `AK-Bridge` (`Karuna` BLAKE3 shadow-state recovery + `KarXT` peripheral side-effect gating).
3. **Oracular `ARSI` Attribution Fidelity Engine**: Evaluates Stratum-0 symbolic invariants strictly *prior* to neural scoring and computes counterfactual attribution fidelity (`I_Reasonwash`) to reject post-hoc unfaithful LLM explanations.
4. **`Jivaslokan` Spike-Weighted Huber & Optimal Transport Probe**: Sub-millisecond hidden-state outlier spike detector combining Spike-Weighted Huber Loss with 1D Wasserstein Optimal Transport (`W_1`) drift bounds.
5. **Polyglot Hardware & Zero-Allocation Primitives**: Includes the formal OCaml `ARSI` proof checker (`polyglot/ocaml/arsi_proof_checker.ml`), Zig zero-allocation POSIX `/dev/shm` ring buffer (`polyglot/zig/zero_alloc_ring.zig`), and C++ AVX2 ternary `{-1, 0, +1}` SIMD dot-product kernel (`polyglot/cpp/bitnet_simd_kernel.cpp`).

---

## Quick Start

### 1. Run via Cargo
```bash
# Validate a .jiva neurosymbolic contract and print its BLAKE3 AST seal
cargo run -- check examples/sovereign_escrow.jiva

# Transpile into any of the 5 target languages
cargo run -- transpile --target rust  examples/sovereign_escrow.jiva
cargo run -- transpile --target zig   examples/sovereign_escrow.jiva
cargo run -- transpile --target ocaml examples/sovereign_escrow.jiva
cargo run -- transpile --target ts    examples/sovereign_escrow.jiva
cargo run -- transpile --target nix   examples/sovereign_escrow.jiva

# Materialize a full SHA-256 attested 5-language bundle via Kamadhenu
cargo build
python3 python/kamadhenu_materializer.py examples/sovereign_escrow.jiva

# Execute the full unit & integration test suite
cargo test --all-targets
```

### 2. Run via Nix Flakes (Zero Store Duplication)
Downstream repositories pin `Jivascriptocle` with normalized `inputs.nixpkgs.follows = "nixpkgs"`:
```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    jivascriptocle = {
      url = "github:Tianxia-Core/Jivascriptocle";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
}
```

---

## Example `.jiva` Contract (`examples/sovereign_escrow.jiva`)

```hcl
module SovereignEscrow @domain(Mahasarasvati) @ffp(FFP-ESCROW-WIRE) {
    kenning TETRA-SEAL:MERKLE-4WAY = "Four-way double-entry settlement reconciliation anchored by deterministic SHA-256 and BLAKE3 Merkle proofs";

    record SettlementFrame {
        epoch_id: u64,
        hhi_concentration: f64,
        daily_drain_ratio: f64,
        reconciliation_delta: f64,
        kyb_attested: bool,
    }

    invariant Stratum0SettlementGate(SettlementFrame) {
        hhi_concentration <= 0.65;
        daily_drain_ratio <= 0.45;
        reconciliation_delta == 0.0;
        kyb_attested == true;
    }
}
```

---

## Architectural Fitness Decomposition (`8` FFPs)

| FFP Identifier | Fitness Class (`ffp_class`) | Language | Artifact Path | Capability Scope |
| :--- | :--- | :--- | :--- | :--- |
| `FFP-JIVA-AST` | `edge-agent` | **Rust** | `src/transpiler/{ast,parser}.rs` | Recursive `.jiva` lexer, parser, static type checker & BLAKE3 AST seal |
| `FFP-JIVA-CODEGEN` | `edge-agent` | **Rust** | `src/transpiler/codegen.rs`, `src/bin/jivascriptocle.rs` | Deterministic 5-target code generator (`Rust`, `Zig`, `OCaml`, `TS`, `Nix`) & CLI |
| `FFP-JIVA-KENNING` | `edge-agent` | **Rust** | `src/kenning/engine.rs` | `AK-Zip` macro codec, `AK-Shield` circumlocution unmasker & `KarXT` gate |
| `FFP-JIVA-ORACULAR` | `capability-proxy` | **Rust** | `src/oracular/engine.rs` | Stratum-0 symbolic gate & `ARSI` counterfactual attribution fidelity checker |
| `FFP-JIVA-PROBE` | `compute-kernel` | **Rust** | `src/jivaslokan/probe.rs` | Spike-Weighted Huber Loss + 1D Wasserstein Optimal Transport (`JOT`) probe |
| `FFP-JIVA-OCAML-PROOF` | `capability-proxy` | **OCaml** | `polyglot/ocaml/arsi_proof_checker.ml` | Pure functional Stratum-0 & `ARSI` formal proof verifier |
| `FFP-JIVA-ZIG-RING` | `wire-codec` / `compute-kernel` | **Zig** / **C++** | `polyglot/zig/zero_alloc_ring.zig`, `polyglot/cpp/` | Lock-free `/dev/shm` C-ABI ring buffer & AVX2 ternary dot-product kernel |
| `FFP-JIVA-FLAKE` | `declarative` | **Nix** / **Python** | `flake.nix`, `python/kamadhenu_materializer.py` | Hermetic Nix Flake derivation & `Kamadhenu` SHA-256 bundle materializer |

---

## Related `RASA-X-MACHINA` · `tianxiaOS` Repositories

- **[TianxiaOS](https://github.com/Tianxia-Core/TianxiaOS)** — Sovereign polyglot operating substrate & declarative NixOS modules
- **[AetherVault](https://github.com/Tianxia-Core/AetherVault)** — Hardware-bound zero-knowledge cryptographic key & secret enclave (`MIT`)
- **[AegisScan](https://github.com/Tianxia-Core/AegisScan)** — 8-domain DevSecOps & runtime security posture scanner (`MIT`)
- **[Faery Fortress](https://github.com/Tianxia-Core/Faery-Fortress)** — `AF_XDP` perimeter defense, `VirtWL` capability firewall & Rosenpass PQ transit (`MPL-2.0`)
- **[nimbus²](https://github.com/Tianxia-Core/nimbus2)** — 14-core polyglot mesh supervisor & deterministic runtime (`MPL-2.0`)
- **[Ndimmôrwyn](https://github.com/Tianxia-Core/Ndimmorwyn)** — Autonomous B2B commercial intelligence & opportunity realization organism
- **[MURUA](https://github.com/Tianxia-Core/MURUA)** — Universal Postal S10/CN08 state-space logistics & dispute-prevention engine

---

## License

Licensed under the **Apache License, Version 2.0** ([LICENSE](LICENSE) · [NOTICE](NOTICE)).
