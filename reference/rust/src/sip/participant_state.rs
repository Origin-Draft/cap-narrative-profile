//! [`SipParticipantState`] and [`SipInformationState`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::state::SipState;

/// What an entity knows, doesn't know, and learns during a unit.
///
/// SIP specification §5.9.1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SipInformationState {
    /// Facts the entity holds before the unit.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub knows: Vec<InformationItem>,

    /// Gaps in the entity's knowledge before the unit.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gaps: Vec<InformationItem>,

    /// Facts the entity gains during the unit.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gained: Vec<InformationItem>,
}

/// A single knowledge item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct InformationItem {
    /// Entity or topic slug this item is about.
    pub subject: String,

    /// Predicate (e.g., `"knows_that"`, `"does_not_know"`, `"learns_that"`).
    pub predicate: String,

    /// What is known/unknown/learned.
    pub about: String,

    /// Optional certainty level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certainty: Option<f64>,
}

/// An entity's state within a specific unit — pre-state, post-state,
/// objective, obstacle, and information state.
///
/// SIP specification §5.9.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SipParticipantState {
    /// Which entity this describes.
    pub entity_ref: String,

    /// Profile-defined role within the unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_in_unit: Option<String>,

    /// Entity state before the unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_state: Option<SipState>,

    /// Entity state after the unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_state: Option<SipState>,

    /// What the entity wants during this unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objective: Option<Objective>,

    /// What prevents the entity from achieving the objective.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obstacle: Option<String>,

    /// Epistemic tracking for this participant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information_state: Option<SipInformationState>,

    /// Observable traits relevant to this unit.  Profile-extensible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observables: Option<Value>,

    /// Structural properties of this participant's role in the unit
    /// (tactics, arc beat, trigger type, etc.).  Profile-extensible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<Value>,

    /// Interpretive claims about this participant.  Profile-extensible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretations: Option<Value>,
}

/// A participant's goal within a unit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Objective {
    /// What the entity is trying to do.
    pub action: String,
    /// What or whom the action is directed at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
