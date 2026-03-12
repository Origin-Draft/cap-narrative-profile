//! Multi-artifact corpus container for the GBR narrative profile.
//!
//! A [`NarrativeCorpus`] bundles multiple SIP narrative artifacts (one per
//! scene/chapter) together with shared entity definitions and cross-artifact
//! relationships, so that the full story can be processed without repeating
//! entity declarations in every artifact.
//!
//! # Design
//!
//! SIP is intentionally a single-artifact format; the corpus is a GBR-layer
//! concern only (see plan decision: "corpus in gbr-protocol only").
//!
//! ```rust,no_run
//! use gbr_types::corpus::{NarrativeCorpus, CrossArtifactRelationship};
//! use gbr_types::sip::SipArtifact;
//!
//! let mut corpus = NarrativeCorpus::new("threshold");
//! // corpus.add_artifact(scene_artifact);
//! // let entity = corpus.shared_entity("nadia");
//! ```

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::sip::{SipArtifact, SipEntity};

// ── Cross-artifact relationship ───────────────────────────────────────────────

/// A typed link between two units or entities in different artifacts.
///
/// Enables queries like "all scenes where this character arc beats occur"
/// or "all transitions involving this location".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CrossArtifactRelationship {
    /// Source: `"<artifact_id>/<unit_id>"` or `"<artifact_id>/<entity_id>"`.
    pub from: String,
    /// Target: same format as `from`.
    pub to: String,
    /// Relationship type (e.g., `"continues"`, `"echoes"`, `"resolves"`).
    pub relationship_type: String,
    /// Optional free-text description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Profile-defined properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,
}

// ── Story architecture summary ────────────────────────────────────────────────

/// High-level narrative architecture for the whole story.
///
/// This is the corpus-level equivalent of unit-level structure metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct StoryArchitecture {
    /// Story title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Primary genre (`horror`, `literary_fiction`, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    /// The controlling idea (Egri / McKee).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlling_idea: Option<String>,
    /// The promise to the reader (genre contract).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre_promise: Option<String>,
    /// Macro-level story structure type (e.g., `"three_act"`, `"five_act"`, `"kishotenketsu"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_type: Option<String>,
    /// Additional profile-defined properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

// ── NarrativeCorpus ───────────────────────────────────────────────────────────

/// A collection of SIP narrative artifacts representing a complete story or book.
///
/// Entities that appear across multiple scenes are declared once in
/// `shared_entities`; individual artifacts may omit or repeat their entry.
/// Tools that consume a corpus should merge entity data from `shared_entities`
/// into each artifact's `entities` list when performing resolution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct NarrativeCorpus {
    /// Machine-readable corpus identifier (e.g., book slug).
    pub corpus_id: String,

    /// Entities shared across the whole story (characters, major locations, …).
    ///
    /// These supplement — not replace — per-artifact entities. Resolvers should
    /// union the two sets with per-artifact data taking precedence.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shared_entities: Vec<SipEntity>,

    /// All scene/chapter artifacts in story order.
    ///
    /// The order of entries in this list defines the canonical reading order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<SipArtifact>,

    /// Story-level structural metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_architecture: Option<StoryArchitecture>,

    /// Typed links between units or entities in different artifacts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cross_artifact_relationships: Vec<CrossArtifactRelationship>,
}

impl NarrativeCorpus {
    /// Create an empty corpus with the given identifier.
    pub fn new(corpus_id: impl Into<String>) -> Self {
        NarrativeCorpus {
            corpus_id: corpus_id.into(),
            shared_entities: Vec::new(),
            artifacts: Vec::new(),
            story_architecture: None,
            cross_artifact_relationships: Vec::new(),
        }
    }

    /// Append an artifact to the corpus (maintains reading order).
    pub fn add_artifact(&mut self, artifact: SipArtifact) {
        self.artifacts.push(artifact);
    }

    /// Declare a shared entity.  If an entity with the same `entity_id` already
    /// exists, the existing entry is replaced.
    pub fn declare_shared_entity(&mut self, entity: SipEntity) {
        if let Some(pos) = self
            .shared_entities
            .iter()
            .position(|e| e.entity_id == entity.entity_id)
        {
            self.shared_entities[pos] = entity;
        } else {
            self.shared_entities.push(entity);
        }
    }

    /// Look up a shared entity by ID.
    pub fn shared_entity(&self, entity_id: &str) -> Option<&SipEntity> {
        self.shared_entities
            .iter()
            .find(|e| e.entity_id == entity_id)
    }

    /// Return artifacts in corpus order (reading order).
    pub fn artifacts_in_order(&self) -> impl Iterator<Item = &SipArtifact> {
        self.artifacts.iter()
    }

    /// Return the total unit count across all artifacts.
    pub fn unit_count(&self) -> usize {
        self.artifacts.iter().map(|a| a.units.len()).sum()
    }

    /// Return all entity IDs referenced across all artifacts (deduplicated).
    pub fn all_entity_ids(&self) -> impl Iterator<Item = &str> {
        self.artifacts
            .iter()
            .flat_map(|a| a.entity_ids())
            .chain(self.shared_entities.iter().map(|e| e.entity_id.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entity(id: &str, entity_type: &str) -> SipEntity {
        SipEntity {
            entity_id: id.to_owned(),
            entity_type: entity_type.to_owned(),
            display_name: id.to_owned(),
            observable_descriptors: None,
            structural_properties: None,
            interpretations: None,
        }
    }

    #[test]
    fn corpus_new_is_empty() {
        let corpus = NarrativeCorpus::new("my_book");
        assert_eq!(corpus.corpus_id, "my_book");
        assert!(corpus.artifacts.is_empty());
        assert!(corpus.shared_entities.is_empty());
    }

    #[test]
    fn declare_shared_entity_replaces_existing() {
        let mut corpus = NarrativeCorpus::new("book");
        corpus.declare_shared_entity(make_entity("nadia", "character"));
        corpus.declare_shared_entity(make_entity("nadia", "character")); // duplicate
        assert_eq!(corpus.shared_entities.len(), 1);
    }

    #[test]
    fn shared_entity_lookup() {
        let mut corpus = NarrativeCorpus::new("book");
        corpus.declare_shared_entity(make_entity("nadia", "character"));
        assert!(corpus.shared_entity("nadia").is_some());
        assert!(corpus.shared_entity("ghost").is_none());
    }

    #[test]
    fn unit_count_sums_across_artifacts() {
        let mut corpus = NarrativeCorpus::new("book");
        let raw1 = r#"{
            "protocol": "semantic-interaction-protocol",
            "protocol_version": "0.1.0",
            "profile": "narrative",
            "profile_version": "0.1.0",
            "artifact_id": "scene_1",
            "entities": [],
            "units": [
                {"unit_id": "u1", "artifact_id": "scene_1", "sequence_index": 1,
                 "observables": {"participants": []}},
                {"unit_id": "u2", "artifact_id": "scene_1", "sequence_index": 2,
                 "observables": {"participants": []}}
            ]
        }"#;
        let raw2 = r#"{
            "protocol": "semantic-interaction-protocol",
            "protocol_version": "0.1.0",
            "profile": "narrative",
            "profile_version": "0.1.0",
            "artifact_id": "scene_2",
            "entities": [],
            "units": [
                {"unit_id": "u1", "artifact_id": "scene_2", "sequence_index": 1,
                 "observables": {"participants": []}}
            ]
        }"#;
        corpus.add_artifact(serde_json::from_str(raw1).unwrap());
        corpus.add_artifact(serde_json::from_str(raw2).unwrap());
        assert_eq!(corpus.unit_count(), 3);
    }

    #[test]
    fn corpus_roundtrip() {
        let mut corpus = NarrativeCorpus::new("threshold");
        corpus.declare_shared_entity(make_entity("nadia", "character"));
        corpus.story_architecture = Some(StoryArchitecture {
            title: Some("Threshold".into()),
            genre: Some("literary_fiction".into()),
            controlling_idea: None,
            genre_promise: None,
            structure_type: None,
            extra: None,
        });
        let serialized = serde_json::to_string(&corpus).unwrap();
        let corpus2: NarrativeCorpus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(corpus, corpus2);
    }
}
