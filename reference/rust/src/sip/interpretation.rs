//! [`SipInterpretation`] — a standalone structured inference about any object.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::enums::ProvenanceSource;

/// A standalone interpretive claim about another SIP object.
///
/// Interpretations may also appear *inline* within entities, units, steps,
/// and participant states as free-form objects.  This struct represents the
/// *standalone* form used in artifact-level interpretation arrays.
///
/// SIP specification §5.8.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SipInterpretation {
    /// Slug reference to the object this interpretation is about.
    pub target_ref: String,

    /// Profile-defined type (e.g., `"motivation"`, `"pattern"`).
    pub interpretation_type: String,

    /// The interpretive claim.  Type is profile-defined.
    pub value: Value,

    /// Certainty of this interpretation (0.0–1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,

    /// Why this interpretation is asserted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,

    /// Slug references to objects that provide evidence.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,

    /// Who or what produced this interpretation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ProvenanceSource>,
}
