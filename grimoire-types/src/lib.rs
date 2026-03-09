#![recursion_limit = "512"]
//! `grimoire-types` — Strongly-typed data model for the Grimoire writing system.
//!
//! # Module overview
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`enums`] | All enumerations (alignment, archetype, wound, POV, …) |
//! | [`tags`] | Typed annotation system (`<!-- key:value -->` → [`tags::Annotation`]) |
//! | [`entities`] | Core declared entities: Character, Setting, Beat, Scene, … |
//! | [`catalogs`] | YAML catalog entry shapes with no `extra: dict` |
//! | [`voice`] | VoiceContract, VoiceSignature, FocalizationConfig, TtsVoiceProfile |
//! | [`training`] | SceneContext, ProsePassage, TrainingExample for LLM fine-tuning |
//! | [`gates`] | Gate system structs (PhaseSpec, GateSpec, GateResult, …) |
//! | [`dag`] | Sub-phase dependency DAG (Kahn topological sort) |
//! | [`recipe`] | StoryRecipe — top-level pipeline output |
//! | [`constraints`] | Formal tag constraint graph (implies/excludes/requires/correlates) |
//!
//! # Feature flags
//!
//! - `python` — enables PyO3 bindings; build with `maturin build --features python`

pub mod catalogs;
pub mod constraints;
pub mod dag;
pub mod entities;
pub mod enums;
pub mod gates;
pub mod ontology;
pub mod recipe;
pub mod tags;
pub mod training;
pub mod voice;

// ── Python bindings (opt-in) ──────────────────────────────────────────────────

#[cfg(feature = "python")]
mod python;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn grimoire_types(m: &Bound<'_, PyModule>) -> PyResult<()> {
    python::register(m)
}

// ── Schema generation ─────────────────────────────────────────────────────────

/// Generate the complete JSON Schema for all public types.
///
/// This replaces the hand-maintained `schemas/_base.schema.json` and all
/// template schemas. Call `grimoire-validate-catalogs --dump-schema` to
/// write the generated schemas to disk.
pub fn generate_all_schemas() -> serde_json::Value {
    use schemars::schema_for;
    serde_json::json!({
        "entities": {
            "Character": schema_for!(entities::Character),
            "Setting": schema_for!(entities::Setting),
            "Beat": schema_for!(entities::Beat),
            "Scene": schema_for!(entities::Scene),
            "SceneSequence": schema_for!(entities::SceneSequence),
            "Chapter": schema_for!(entities::Chapter),
            "Motif": schema_for!(entities::Motif),
            "Symbol": schema_for!(entities::Symbol),
            "Leitmotif": schema_for!(entities::Leitmotif),
            "Thread": schema_for!(entities::Thread),
            "Promise": schema_for!(entities::Promise),
        },
        "voice": {
            "VoiceContract": schema_for!(voice::VoiceContract),
            "VoiceSignature": schema_for!(voice::VoiceSignature),
            "ProseStyleGuide": schema_for!(voice::ProseStyleGuide),
            "NarrativeVoice": schema_for!(voice::NarrativeVoice),
            "FocalizationConfig": schema_for!(voice::FocalizationConfig),
            "TtsVoiceProfile": schema_for!(voice::TtsVoiceProfile),
        },
        "training": {
            "SceneContext": schema_for!(training::SceneContext),
            "TrainingExample": schema_for!(training::TrainingExample),
            "ProsePassage": schema_for!(training::ProsePassage),
            "TrainingDataset": schema_for!(training::TrainingDataset),
            "TierConfig": schema_for!(training::TierConfig),
            "ProseIntent": schema_for!(training::ProseIntent),
            "NarrativeFunction": schema_for!(training::NarrativeFunction),
            "Paragraph": schema_for!(training::Paragraph),
            "Sentence": schema_for!(training::Sentence),
        },
        "catalogs": {
            // Character
            "ArchetypeCatalogEntry":            schema_for!(catalogs::ArchetypeCatalogEntry),
            "WoundCatalogEntry":                schema_for!(catalogs::WoundCatalogEntry),
            "RoleCatalogEntry":                 schema_for!(catalogs::RoleCatalogEntry),
            "DriveCatalogEntry":                schema_for!(catalogs::DriveCatalogEntry),
            "RelationshipRoleCatalogEntry":     schema_for!(catalogs::RelationshipRoleCatalogEntry),
            "AlignmentSystemCatalog":           schema_for!(catalogs::AlignmentSystemCatalog),
            // Concept / plot
            "PlotTypeCatalogEntry":             schema_for!(catalogs::PlotTypeCatalogEntry),
            "CollisionPatternCatalogEntry":     schema_for!(catalogs::CollisionPatternCatalogEntry),
            "SocialCircleCatalogEntry":         schema_for!(catalogs::SocialCircleCatalogEntry),
            "TropeCatalogEntry":                schema_for!(catalogs::TropeCatalogEntry),
            "GenreTropeCatalogEntry":           schema_for!(catalogs::GenreTropeCatalogEntry),
            "IntertextualRelationEntry":        schema_for!(catalogs::IntertextualRelationEntry),
            "IncitingIncidentEntry":            schema_for!(catalogs::IncitingIncidentEntry),
            // Plot & structure
            "NarrativeTimeModeEntry":           schema_for!(catalogs::NarrativeTimeModeEntry),
            "EmplotmentTypeEntry":              schema_for!(catalogs::EmplotmentTypeEntry),
            "ProppFunctionEntry":               schema_for!(catalogs::ProppFunctionEntry),
            "SerialityTypeEntry":               schema_for!(catalogs::SerialityTypeEntry),
            "RomanceBeatsCatalog":              schema_for!(catalogs::RomanceBeatsCatalog),
            // World-building
            "SpatialModeEntry":                 schema_for!(catalogs::SpatialModeEntry),
            // Drafting — focalization / voice
            "FocalizationModeEntry":            schema_for!(catalogs::FocalizationModeEntry),
            "SpeechActEntry":                   schema_for!(catalogs::SpeechActEntry),
            "SubtextModeEntry":                 schema_for!(catalogs::SubtextModeEntry),
            // Drafting — figurative language / style
            "MetaphorTypeCatalogEntry":         schema_for!(catalogs::MetaphorTypeCatalogEntry),
            "IronyTypeCatalogEntry":            schema_for!(catalogs::IronyTypeCatalogEntry),
            "ComicModeCatalogEntry":            schema_for!(catalogs::ComicModeCatalogEntry),
            "ImageSchemaEntry":                 schema_for!(catalogs::ImageSchemaEntry),
            "VerseProsodyEntry":                schema_for!(catalogs::VerseProsodyEntry),
            "ExperimentalNarrationEntry":       schema_for!(catalogs::ExperimentalNarrationEntry),
            // Drafting — theory
            "TraumaModeEntry":                  schema_for!(catalogs::TraumaModeEntry),
            "PentadElementEntry":               schema_for!(catalogs::PentadElementEntry),
            "PhilosophyFictionEntry":           schema_for!(catalogs::PhilosophyFictionEntry),
            "PsychoanalyticMechanismEntry":     schema_for!(catalogs::PsychoanalyticMechanismEntry),
            "AutofictionModeEntry":             schema_for!(catalogs::AutofictionModeEntry),
            "AdaptationModeEntry":              schema_for!(catalogs::AdaptationModeEntry),
            "TranslationModeEntry":             schema_for!(catalogs::TranslationModeEntry),
            "GraphicNarrativeModeEntry":        schema_for!(catalogs::GraphicNarrativeModeEntry),
            "YaNarrativeModeEntry":             schema_for!(catalogs::YaNarrativeModeEntry),
            "SemioLinguisticFunctionEntry":     schema_for!(catalogs::SemioLinguisticFunctionEntry),
            "GenreReadingModeEntry":            schema_for!(catalogs::GenreReadingModeEntry),
            // Revision — critical theory
            "RevisionPassEntry":                schema_for!(catalogs::RevisionPassEntry),
            "PostcolonialModeEntry":            schema_for!(catalogs::PostcolonialModeEntry),
            "FeministNarrativeEntry":           schema_for!(catalogs::FeministNarrativeEntry),
            "QueerNarrativeModeEntry":          schema_for!(catalogs::QueerNarrativeModeEntry),
            "DisabilityRepModeEntry":           schema_for!(catalogs::DisabilityRepModeEntry),
            "EcocriticalModeEntry":             schema_for!(catalogs::EcocriticalModeEntry),
            "MarxistNarrativeModeEntry":        schema_for!(catalogs::MarxistNarrativeModeEntry),
            "IndigenousNarrativeModeEntry":     schema_for!(catalogs::IndigenousNarrativeModeEntry),
            "PosthumanModeEntry":               schema_for!(catalogs::PosthumanModeEntry),
            "NarrativeEthicsModeEntry":         schema_for!(catalogs::NarrativeEthicsModeEntry),
            "AffectModeEntry":                  schema_for!(catalogs::AffectModeEntry),
            // Cognitive / cross-cutting
            "CognitiveNarrativeModeEntry":      schema_for!(catalogs::CognitiveNarrativeModeEntry),
            "SignifyingModeEntry":              schema_for!(catalogs::SignifyingModeEntry),
            "ParatextZoneEntry":                schema_for!(catalogs::ParatextZoneEntry),
            // Phase VI–VII — new theory catalogs
            "DramaticTheoryModeEntry":          schema_for!(catalogs::DramaticTheoryModeEntry),
            "ShortFictionModeEntry":            schema_for!(catalogs::ShortFictionModeEntry),
            "NonfictionNarrativeModeEntry":     schema_for!(catalogs::NonfictionNarrativeModeEntry),
            "IntersectionalModeEntry":          schema_for!(catalogs::IntersectionalModeEntry),
            "ScreenwritingModeEntry":           schema_for!(catalogs::ScreenwritingModeEntry),
        },
        "gates": {
            "SubPhaseSpec": schema_for!(gates::SubPhaseSpec),
            "GateSpec": schema_for!(gates::GateSpec),
            "GateResult": schema_for!(gates::GateResult),
            "SubPhaseResult": schema_for!(gates::SubPhaseResult),
            "PhaseSpec": schema_for!(gates::PhaseSpec),
            "PhaseResult": schema_for!(gates::PhaseResult),
            "DependencyEdge": schema_for!(gates::DependencyEdge),
            "ReadinessSummary": schema_for!(gates::ReadinessSummary),
        },
        "ontology": {
            "OntologyLayer": schema_for!(ontology::OntologyLayer),
            "PrimitiveKind": schema_for!(ontology::PrimitiveKind),
            "ProjectionType": schema_for!(ontology::ProjectionType),
            "OntologyPrimitive": schema_for!(ontology::OntologyPrimitive),
            "PrimitiveProjection": schema_for!(ontology::PrimitiveProjection),
        },
        "recipe": {
            "StoryRecipe": schema_for!(recipe::StoryRecipe),
        },
        "tags": {
            "Annotation": schema_for!(tags::Annotation),
            "ParagraphAnnotations": schema_for!(tags::ParagraphAnnotations),
            "SentenceAnnotations": schema_for!(tags::SentenceAnnotations),
        },
        "constraints": {
            "ConstraintGraph": schema_for!(constraints::ConstraintGraph),
            "TagConstraint": schema_for!(constraints::TagConstraint),
        },
    })
}
