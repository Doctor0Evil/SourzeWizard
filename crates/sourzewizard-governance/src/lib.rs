use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustProfile {
    pub no_rollbacks: bool,
    pub no_downgrades: bool,
    pub no_hidden_control: bool,
    pub no_malicious_signatures: bool,
    pub blacklist_respected: bool,
    pub supported_languages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KOBinding {
    pub offline_tokenizable: bool,
    pub aln_sourze_ready: bool,
    pub anchoring_target: String,
    pub ecosystem: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceKO {
    pub ko_id: String,
    pub ko_scope: String,
    pub ko_purpose: String,
    pub ko_trust: TrustProfile,
    pub ko_binding: KOBinding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KoDecision {
    Accepted,
    Rejected { reason: String },
}

pub struct GovernanceGuard;

impl GovernanceGuard {
    pub fn validate(ko: &GovernanceKO) -> KoDecision {
        if ko.ko_id != "sourzewizard.ko.governance.profile.v1" {
            return KoDecision::Rejected {
                reason: "unstable ko_id".into(),
            };
        }
        if ko.ko_scope != "sourzewizard.aln.syntax-dev" {
            return KoDecision::Rejected {
                reason: "invalid ko_scope".into(),
            };
        }
        let t = &ko.ko_trust;
        if !(t.no_rollbacks && t.no_downgrades && t.blacklist_respected) {
            return KoDecision::Rejected {
                reason: "trust profile violates no-rollback/no-downgrade/blacklist rules".into(),
            };
        }
        // Enforce supported-language-only constraint
        for lang in &t.supported_languages {
            match lang.as_str() {
                "Rust" | "ALN" | "Lua" | "Kotlin/Android" | "Javascript" | "Mojo" => {}
                other => {
                    return KoDecision::Rejected {
                        reason: format!("unsupported language in trust profile: {other}"),
                    };
                }
            }
        }
        if !(ko.ko_binding.offline_tokenizable && ko.ko_binding.aln_sourze_ready) {
            return KoDecision::Rejected {
                reason: "KO not marked offline-tokenizable and sourze-ready".into(),
            };
        }
        KoDecision::Accepted
    }
}
