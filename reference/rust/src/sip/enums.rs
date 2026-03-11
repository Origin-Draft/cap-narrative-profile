//! SIP core enumerations.
//!
//! All values defined here are normative for the SIP core protocol.
//! Domain profiles MAY extend `CausalRole` with additional values; all
//! other enums are closed.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Significance of a step or event within a unit.
///
/// Corresponds to the `significance` core enum (SIP §Appendix A).
/// Profiles MUST NOT add values to this enum.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Significance {
    /// Causally or semantically required — removing it would break the unit.
    Essential,
    /// Enriching but not required for the unit's core function.
    Supplementary,
}

/// The function a unit plays in the causal chain.
///
/// Core values from SIP §Appendix A.  Domain profiles MAY extend with
/// additional values (e.g., `catalyst`, `escalation`, `crisis` for narrative).
/// When deserialising, unknown values round-trip as [`CausalRole::Other`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CausalRole {
    /// Introduces context, state, or participants.
    Setup,
    /// Initiates the causal chain — the inciting action.
    Trigger,
    /// Raises stakes or introduces an obstacle.
    Complication,
    /// Closes the causal chain for this unit.
    Resolution,
    /// Profile-defined extension value.
    #[serde(other)]
    Other,
}

/// SIP conformance level.
///
/// Levels are strictly ordered: a document at level N also passes all
/// levels < N.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConformanceLevel {
    /// JSON Schema validation only.
    Schema,
    /// Schema + all entity/unit refs resolve within the artifact.
    Referential,
    /// Referential + semantic-fingerprint round-trip invariant holds.
    RoundTrip,
}

/// Severity of a validation result.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValidationSeverity {
    /// Schema or referential invariant violated; artifact MUST be corrected.
    Error,
    /// Advisory; artifact MAY be used, but quality is degraded.
    Warning,
    /// Informational; no action required.
    Info,
}

/// Provenance of an interpretation or state value.
///
/// Free-text string is also permitted (e.g., `"model:gpt-4o"`, `"annotator:alice"`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ProvenanceSource {
    Known(KnownProvenance),
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum KnownProvenance {
    Human,
    Model,
    Inferred,
    Consensus,
}
