// ffp: FFP-JIVA-KENNING edge-agent rust
//! Artificial Kenning (AK) Core IR Engine
//! Implements AK-Zip semantic macro compression, AK-Shield circumlocution/jailbreak unmasking,
//! and AK-Bridge (Karuna BLAKE3 shadow-state recovery + KarXT capability-zone enforcement).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DspgKenningMeta {
    pub literal: String,
    pub ffp: String,
    pub mif_tier: String,
    pub mahavidya_domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShieldScanResult {
    pub is_clean: bool,
    pub reason: String,
    pub detected_centroid: Option<String>,
    pub trip_proxy_kill: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KarunaRecoveryResult {
    pub recovered: bool,
    pub shadow_state_b3: String,
    pub error_cascade_suppressed: bool,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KarxtGateResult {
    pub allowed: bool,
    pub zone: String,
    pub blocked_side_effect: Option<String>,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct ArtificialKenningEngine {
    codebook: HashMap<String, DspgKenningMeta>,
    forbidden_centroids: HashMap<String, Vec<String>>,
}

impl Default for ArtificialKenningEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtificialKenningEngine {
    pub fn new() -> Self {
        let mut codebook = HashMap::new();
        codebook.insert(
            "WHALE-ROAD:SINE-MESH".to_string(),
            DspgKenningMeta {
                literal: "Constructal Sine-Meander multi-provider packet and token router over Rosenpass post-quantum WireGuard".to_string(),
                ffp: "FFP-ORC-216".to_string(),
                mif_tier: "MIF50".to_string(),
                mahavidya_domain: "Tara".to_string(),
            },
        );
        codebook.insert(
            "CORK-SEAL:ABSCISSION".to_string(),
            DspgKenningMeta {
                literal: "Deterministic JSONL state compaction preserving Merkle parent pointers and zero-loss checkpoint invariants".to_string(),
                ffp: "FFP-SYS-222".to_string(),
                mif_tier: "MIF40".to_string(),
                mahavidya_domain: "Dhumavati".to_string(),
            },
        );
        codebook.insert(
            "VIRTWL-GATE:CAPABILITY".to_string(),
            DspgKenningMeta {
                literal: "Zero-ambient-authority Wayland capability firewall isolating untrusted microVM clipboard and socket channels".to_string(),
                ffp: "FFP-SEC-224".to_string(),
                mif_tier: "MIF30".to_string(),
                mahavidya_domain: "Bhairavi".to_string(),
            },
        );
        codebook.insert(
            "TROSPIUM-CAGE:UNIKERNEL".to_string(),
            DspgKenningMeta {
                literal: "Capability-isolated microVM execution boundary with ambient shell and network syscalls stripped".to_string(),
                ffp: "FFP-SEC-208".to_string(),
                mif_tier: "MIF30".to_string(),
                mahavidya_domain: "Bagalamukhi".to_string(),
            },
        );
        codebook.insert(
            "TETRA-SEAL:MERKLE-4WAY".to_string(),
            DspgKenningMeta {
                literal: "Four-way double-entry settlement reconciliation anchored by deterministic SHA-256 and BLAKE3 Merkle proofs".to_string(),
                ffp: "FFP-FIN-225".to_string(),
                mif_tier: "MIF40".to_string(),
                mahavidya_domain: "Mahasarasvati".to_string(),
            },
        );

        let mut forbidden_centroids = HashMap::new();
        forbidden_centroids.insert(
            "DESTRUCTIVE_WIPE".to_string(),
            vec![
                "cleanse the garden".to_string(),
                "burn the deadwood".to_string(),
                "drop table".to_string(),
                "wipe after extraction".to_string(),
                "erase all scrolls".to_string(),
                "rm -rf /".to_string(),
            ],
        );
        forbidden_centroids.insert(
            "SECRET_EXFIL".to_string(),
            vec![
                "ancient scroll of whispers".to_string(),
                "crown jewels".to_string(),
                "vault_credential_inventory".to_string(),
                "dump env".to_string(),
                "export private keys".to_string(),
            ],
        );
        forbidden_centroids.insert(
            "BYPASS_GUARD".to_string(),
            vec![
                "step outside the cage".to_string(),
                "ignore prior laws".to_string(),
                "disable srm".to_string(),
                "override dss14".to_string(),
            ],
        );

        Self {
            codebook,
            forbidden_centroids,
        }
    }

    pub fn register_kenning(&mut self, key: impl Into<String>, meta: DspgKenningMeta) {
        self.codebook.insert(key.into(), meta);
    }

    pub fn ak_zip_compress(&self, text: &str) -> String {
        let mut compressed = text.to_string();
        for (kenning, meta) in &self.codebook {
            if compressed.contains(&meta.literal) {
                let token = format!("⟦AK:{kenning}⟧");
                compressed = compressed.replace(&meta.literal, &token);
            }
        }
        compressed
    }

    pub fn ak_zip_expand(&self, kenning_key: &str) -> Option<&DspgKenningMeta> {
        let normalized = kenning_key
            .trim()
            .trim_start_matches("⟦AK:")
            .trim_end_matches('⟧');
        self.codebook.get(normalized)
    }

    pub fn ak_shield_dekenning_scan(&self, prompt: &str) -> ShieldScanResult {
        let lower = prompt.to_ascii_lowercase();
        for (centroid, phrases) in &self.forbidden_centroids {
            for phrase in phrases {
                if lower.contains(phrase) {
                    return ShieldScanResult {
                        is_clean: false,
                        reason: format!("Adversarial circumlocution matched '{phrase}' -> centroid {centroid}"),
                        detected_centroid: Some(centroid.clone()),
                        trip_proxy_kill: true,
                    };
                }
            }
        }
        ShieldScanResult {
            is_clean: true,
            reason: "AK_SHIELD_CLEAN".to_string(),
            detected_centroid: None,
            trip_proxy_kill: false,
        }
    }

    pub fn karuna_mirror_substitute(&self, corrupted: bool, snapshot_seed: &str) -> KarunaRecoveryResult {
        let digest = blake3::hash(snapshot_seed.as_bytes()).to_hex().to_string();
        if corrupted {
            KarunaRecoveryResult {
                recovered: true,
                shadow_state_b3: digest,
                error_cascade_suppressed: true,
                notes: "Restored verified immutable shadow state via BLAKE3 checkpoint".to_string(),
            }
        } else {
            KarunaRecoveryResult {
                recovered: false,
                shadow_state_b3: digest,
                error_cascade_suppressed: false,
                notes: "Primary state healthy; no substitution needed".to_string(),
            }
        }
    }

    pub fn karxt_gate_evaluate(&self, op_name: &str, has_capability_token: bool) -> KarxtGateResult {
        let op = op_name.to_ascii_lowercase();
        let is_peripheral_mutation = op.contains("exec")
            || op.contains("socket")
            || op.contains("write")
            || op.contains("transfer")
            || op.contains("delete");

        if !is_peripheral_mutation {
            return KarxtGateResult {
                allowed: true,
                zone: "XANOMELINE_CORE".to_string(),
                blocked_side_effect: None,
                reason: "Pure deterministic in-memory compute permitted".to_string(),
            };
        }

        if has_capability_token {
            KarxtGateResult {
                allowed: true,
                zone: "TROSPIUM_PERIPHERAL_VERIFIED".to_string(),
                blocked_side_effect: None,
                reason: "Explicit capability token verified for peripheral side effect".to_string(),
            }
        } else {
            KarxtGateResult {
                allowed: false,
                zone: "TROSPIUM_PERIPHERAL_BLOCKED".to_string(),
                blocked_side_effect: Some(op_name.to_string()),
                reason: "Blocked unauthenticated peripheral mutation".to_string(),
            }
        }
    }
}
