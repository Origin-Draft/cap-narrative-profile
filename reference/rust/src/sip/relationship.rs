//! [`SipRelationship`] — a typed directed link between two objects.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A typed, directed link between two entities, two units, or an entity and
/// a unit.
///
/// SIP specification §5.4.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SipRelationship {
    /// Source entity or unit slug.
    pub source: String,

    /// Target entity or unit slug.
    pub target: String,

    /// Profile-defined relationship type (e.g., `"family_parent_child"`, `"dependency"`).
    pub relationship_type: String,

    /// Observable basis for the relationship.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<String>,

    /// Profile-defined interpretive properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretations: Option<Value>,
}
