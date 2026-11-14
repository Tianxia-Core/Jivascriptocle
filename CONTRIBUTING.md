# Contributing to Jivascriptocle

Contributions to **Jivascriptocle** are welcomed under the **Apache License, Version 2.0** (Section 5).

## Engineering Standards (`DSS-14.2` & `FFP-First`)
1. Every source file must carry a valid `# ffp:` / `// ffp:` Function-Feature Pair header comment.
2. Zero stubs, zero `todo!()`/`unimplemented!()` macros, and zero dynamic string `eval`.
3. All `.jiva` parser/codegen modifications must pass `cargo test --all-targets` and preserve deterministic output across all 5 transpilation targets (`rust`, `zig`, `ocaml`, `ts`, `nix`).
