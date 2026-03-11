//! [`SipEntity`] — a persistent participant or object of interest.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A persistent participant or object of interest that exists across multiple
/// [`SipUnit`](super::unit::SipUnit)s.
///
/// Entities are declared once in the artifact and referenced by slug.
/// Per-unit state lives in `participant_states` within each unit, not here.
///
/// SIP specification §5.2.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SipEntity {
    /// Unique within the artifact; matches `^[a-z0-9_]+$`.
    pub entity_id: String,

    /// Profile-defined entity type (e.g., `"character"`, `"service"`).
    pub entity_type: String,

    /// Human-readable display name.
    pub display_name: String,

    /// Profile-defined observable properties.
    /// `additionalProperties: true` — profiles extend this freely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observable_descriptors: Option<Value>,

    /// Profile-defined structural properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structural_properties: Option<Value>,

    /// Profile-defined interpretive properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretations: Option<Value>,
}
