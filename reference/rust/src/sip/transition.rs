//! [`SipTransition`] — a value change within or between units.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::state::SipState;

/// A transition from one state to another, optionally with a trigger and
/// a human-readable description.
///
/// Transitions appear in [`SipUnit.structure.transition`] and may be embedded
/// in [`SipParticipantState`].
///
/// SIP specification §5.6.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SipTransition {
    /// Entity or unit slug this transition applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// State before the transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<SipState>,

    /// State after the transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<SipState>,

    /// What caused the transition (entity slug, event description, or free text).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,

    /// Human-readable summary of what changed and why.
    pub description: String,

    /// Certainty that this transition occurred as described.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,

    /// Observable or textual grounding for this transition's claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grounding: Option<String>,
}
