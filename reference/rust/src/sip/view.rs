//! [`SipView`] — a named projection over a canonical artifact.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A named projection or derived representation over the canonical artifact.
///
/// Views are computed or declared; they do not store canonical data.
///
/// SIP specification §5.7.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SipView {
    /// Unique within the artifact.
    pub view_id: String,

    /// Profile-defined view type (e.g., `"entity_trajectory"`, `"dependency_graph"`).
    pub view_type: String,

    /// Human-readable description of what this view shows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Which entities or units are included (default: all).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scope: Vec<String>,

    /// How items are ordered in this view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordering: Option<String>,

    /// View-specific computed data.  Schema is view-type-specific.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}
