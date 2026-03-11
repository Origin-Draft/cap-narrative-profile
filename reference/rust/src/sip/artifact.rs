//! [`SipArtifact`] — top-level container for a SIP canonical decomposition.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::entity::SipEntity;
use super::interpretation::SipInterpretation;
use super::relationship::SipRelationship;
use super::unit::SipUnit;
use super::view::SipView;

/// Domain-agnostic metadata for an artifact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SipMetadata {
    /// Human-readable title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Author or owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Owner (for software artifacts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,

    /// Size in domain-appropriate units (words, LOC, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,

    /// Additional profile-defined metadata.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// Top-level container: a complete canonical decomposition of one source work.
///
/// SIP specification §5.1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SipArtifact {
    /// MUST be `"semantic-interaction-protocol"`.
    pub protocol: String,

    /// SIP version (semver) this artifact conforms to.
    pub protocol_version: String,

    /// Domain profile identifier (e.g., `"narrative"`, `"software"`).
    pub profile: String,

    /// Profile version (semver).
    pub profile_version: String,

    /// Unique identifier for this artifact; matches `^[a-z0-9_]+$`.
    pub artifact_id: String,

    /// Domain-agnostic metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SipMetadata>,

    /// All persistent entities declared in this artifact.
    pub entities: Vec<SipEntity>,

    /// Ordered decomposition into atomic units.
    pub units: Vec<SipUnit>,

    /// Cross-entity and cross-unit links.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationships: Vec<SipRelationship>,

    /// Named projections over the canonical data.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub views: Vec<SipView>,

    /// Artifact-level inferences (themes, architectural style, etc.).
    /// Accepts either an array of [`SipInterpretation`] or a free-form object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretations: Option<ArtifactInterpretations>,
}

/// Artifact-level interpretations — either a typed array or a free-form object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ArtifactInterpretations {
    Array(Vec<SipInterpretation>),
    Object(Value),
}

impl SipArtifact {
    /// Returns `true` if the `protocol` field has the correct value.
    pub fn is_valid_protocol(&self) -> bool {
        self.protocol == "semantic-interaction-protocol"
    }

    /// Returns an iterator over all entity IDs declared in this artifact.
    pub fn entity_ids(&self) -> impl Iterator<Item = &str> {
        self.entities.iter().map(|e| e.entity_id.as_str())
    }

    /// Returns an iterator over all unit IDs declared in this artifact.
    pub fn unit_ids(&self) -> impl Iterator<Item = &str> {
        self.units.iter().map(|u| u.unit_id.as_str())
    }
}
