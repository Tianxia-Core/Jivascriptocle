# Changelog — Jivascriptocle (`Tianxia-Core/Jivascriptocle`)

## [v1.3.0] — 2026-11-18
### Added
- **Native Rust `TOON` (Token-Optimized Object Notation) Schema Compiler (`src/toon.rs`)**: Computes deterministic C-ABI byte offsets/alignments and ~50%+ wire/prompt byte savings (`jivascriptocle toon <schema.jiva>`).
- **Native Rust `Kamadhenu` Multi-Target Materializer (`jivascriptocle build-all`)**: Atomically emits `Rust`, `Zig`, `OCaml`, `TypeScript`, `Nix`, `TOON`, and BLAKE3-sealed `kamadhenu_manifest.json` with zero Python dependency.
- **Clean-Room Starter Scaffolder (`jivascriptocle init`)**: Generates fresh zero-PII `.jiva` contracts for end users in `< 1 ms`.
- **JSON-RPC 2.0 MCP Server (`jivascriptocle mcp-stdio`)**: Exposes `jiva_compile_toon` and `jiva_kenning_scan` to Claude Code, Cursor, OpenCode, and VS Code at `$0.00` (`0` LLM tokens).
- **Universal Multi-OS Signed Release (`v1.3.0`)**: Ed25519-signed `.deb`, `.rpm`, `.AppImage`, `.tar.gz`, `flake.nix`, and `SBOM.cdx.json`.

## [v1.2.0] — 2026-11-10
- Typed `.jiva` parser, 5-target AST codegen (`rust`, `zig`, `ocaml`, `ts`, `nix`), `ArtificialKenningEngine`, `OracularEngine`, and `JivaslokanHuberProbe`.
