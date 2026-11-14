#!/usr/bin/env python3
# ffp: FFP-JIVA-FLAKE declarative python
"""
Kamadhenu Deterministic Nix Derivation & Polyglot Binding Materializer.
Invokes `jivascriptocle` (or parses `.jiva` contracts) to synthesize reproducible
multi-language contract artifacts with BLAKE3 / SHA-256 provenance manifests.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path
from typing import Dict


def compute_sha256(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def materialize_bundle(jiva_path: Path, bin_path: Path, out_dir: Path) -> Dict[str, str]:
    out_dir.mkdir(parents=True, exist_ok=True)
    targets = {
        "rust": "generated_contract.rs",
        "zig": "generated_contract.zig",
        "ocaml": "generated_contract.ml",
        "ts": "generated_contract.ts",
        "nix": "generated_module.nix",
    }
    digests: Dict[str, str] = {}
    for target, filename in targets.items():
        proc = subprocess.run(
            [str(bin_path), "transpile", "--target", target, str(jiva_path)],
            check=True,
            capture_output=True,
            text=True,
        )
        dest = out_dir / filename
        tmp = dest.with_suffix(dest.suffix + ".tmp")
        tmp.write_text(proc.stdout, encoding="utf-8")
        tmp.replace(dest)
        digests[filename] = compute_sha256(proc.stdout)

    manifest = {
        "schema": str(jiva_path.name),
        "generator": "jivascriptocle-kamadhenu/1.2.0",
        "license": "Apache-2.0",
        "artifacts_sha256": digests,
    }
    manifest_path = out_dir / "kamadhenu_manifest.json"
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    return digests


def main() -> int:
    parser = argparse.ArgumentParser(description="Kamadhenu Polyglot Contract Materializer")
    parser.add_argument("schema", type=Path, help="Path to input .jiva contract")
    parser.add_argument("--bin", type=Path, default=Path("target/debug/jivascriptocle"))
    parser.add_argument("--out", type=Path, default=Path("dist/polyglot"))
    args = parser.parse_args()
    digests = materialize_bundle(args.schema, args.bin, args.out)
    print(json.dumps({"ok": True, "generated": digests}, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
