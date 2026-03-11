//! [`SipUnit`] and [`SipStep`] — atomic interaction blocks.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::enums::{CausalRole, Significance};
use super::participant_state::SipParticipantState;
use super::transition::SipTransition;

/// An ordered atomic action within a [`SipUnit`].
///
/// SIP specification §5.3.1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SipStep {
    /// Order within the unit; MUST be ≥ 1, unique, and ascending.
    pub sequence_number: u32,

    /// Entity performing the action.
    pub agent: String,

    /// Verb or action identifier.
    pub action: String,

    /// What the action is directed at (entity slug, resource path, or free text).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Profile-defined event classification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,

    /// Whether this step is essential or supplementary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub significance: Option<Significance>,

    /// Profile-defined step-level interpretations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretations: Option<Value>,
}

/// Observable section of a [`SipUnit`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SipObservables {
    /// Entities involved in this unit.
    pub participants: Vec<String>,

    /// Profile-defined contextual observables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Profile-defined event classification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,

    /// Raw artifact text or content this unit was derived from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_text: Option<String>,
}

/// Structure section of a [`SipUnit`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SipStructure {
    /// Normalized position in artifact (0.0 = start, 1.0 = end).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<f64>,

    /// Function in causal chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causal_role: Option<CausalRole>,

    /// Profile-defined grouping metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping: Option<Value>,

    /// Ordered sub-unit decomposition.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<SipStep>,

    /// Value change within this unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition: Option<SipTransition>,

    /// Machine-verifiable semantic summary.  Profile defines grammar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_fingerprint: Option<Value>,
}

/// An atomic transformation or interaction block — the smallest meaningful
/// chunk of the artifact that contains a complete interaction cycle
/// (pre-state → action → post-state).
///
/// SIP specification §5.3.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SipUnit {
    /// Unique within artifact; matches `^[a-z0-9_]+$`.
    pub unit_id: String,

    /// Back-reference to parent artifact.
    pub artifact_id: String,

    /// Profile-defined unit type (e.g., `"scene"`, `"interaction"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<String>,

    /// Position in artifact's primary ordering; MUST be ≥ 1.
    pub sequence_index: u32,

    /// All that can be directly observed in this unit.
    pub observables: SipObservables,

    /// Structural role and decomposition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<SipStructure>,

    /// Unit-level interpretive claims.  All fields profile-defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretations: Option<Value>,

    /// Per-entity state snapshots.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant_states: Vec<SipParticipantState>,

    /// Size, word count, LOC, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}
