//! [`SipState`] — a named condition at a point in time.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::enums::ProvenanceSource;

/// A named condition attached to an entity, group, or artifact at a specific point.
///
/// States appear inline within [`SipTransition`] (as `before`/`after`) and
/// within [`SipParticipantState`].  The `state_type` is profile-defined.
///
/// SIP specification §5.5.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SipState {
    /// What this state describes (entity or unit slug).
    /// Required when the state appears standalone; optional when inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// Profile-defined state category (e.g., `"emotional"`, `"authentication"`).
    pub state_type: String,

    /// The state value.  Type depends on `state_type`; profiles define the schema.
    pub value: Value,

    /// Observable basis for this state assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<String>,

    /// Who or what produced this state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ProvenanceSource>,

    /// Certainty of this state assignment (0.0 = none, 1.0 = absolute).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}
