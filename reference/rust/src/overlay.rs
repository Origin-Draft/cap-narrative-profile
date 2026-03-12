//! Typed narrative overlay structs for GBR's SIP profile.
//!
//! These types give typed, zero-copy-style access to the `Option<Value>`
//! extension fields in every SIP core struct when reading a narrative artifact.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use gbr_types::sip::overlay::SipUnitOverlayExt;
//! use gbr_types::overlay::NarrativeContext;
//!
//! // artifact: SipArtifact loaded from JSON
//! # let artifact = unimplemented!();
//! # let unit: &gbr_types::sip::SipUnit = unimplemented!();
//! if let Some(Ok(ctx)) = unit.context_as::<NarrativeContext>() {
//!     println!("focalizer: {:?}", ctx.focalizer);
//! }
//! ```

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::sip::ProfileOverlay;

// ── Unit — observables.context ────────────────────────────────────────────────

/// Typed form of `unit.observables.context` for the GBR narrative profile.
///
/// All fields are optional; unknown fields are ignored during deserialization.
/// (PROFILE.md §5, §7.3)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct NarrativeContext {
    /// The character that perceives and filters the unit (entity ref slug).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focalizer: Option<String>,

    /// Point-of-view type (PROFILE.md §5, `narrative_voice.json → pov_type`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pov: Option<String>,

    /// Genette diegetic level (e.g. `"extradiegetic"`, `"intradiegetic"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diegetic_level: Option<String>,

    /// Location entity ref slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting: Option<String>,

    /// When the unit occurs (`setting.json → time_of_day`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_of_day: Option<String>,

    /// Environmental mood (`setting.json → atmosphere`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atmosphere: Option<String>,

    /// Spatial configuration (`setting.json → spatial_structure`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_structure: Option<String>,

    /// Genette narrative-time fields (order, duration_mode, frequency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub narrative_time: Option<Value>,
}

// ── Unit — structure.grouping ─────────────────────────────────────────────────

/// Typed form of `unit.structure.grouping` for the GBR narrative profile.
/// (PROFILE.md §7.4–§7.5)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct NarrativeGrouping {
    /// Macro-arc position (e.g. `"status_quo"`, `"revelation"`, `"climax"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beat: Option<String>,

    /// Scene function within the sequence (`scene_structure.json → scene_function`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_function: Option<String>,

    /// 1-based position of this scene within its chapter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_number_in_chapter: Option<u32>,
}

// ── Unit — interpretations ────────────────────────────────────────────────────

/// Summary of the unit's core narrative want, obstacle, and outcome.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct CanonicalSummary {
    /// What the focalizer wants to achieve in this unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub want: Option<String>,
    /// What prevents achievement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obstacle: Option<String>,
    /// How the want resolves (`"achieved"`, `"blocked"`, `"partially_achieved"`, …).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
}

/// Typed form of `unit.interpretations` for the GBR narrative profile.
/// (PROFILE.md §7.7)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct NarrativeUnitInterpretations {
    /// Point-of-view type (redundant with context.pov; kept for flat-access convenience).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pov: Option<String>,

    /// Genette focalization type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focalization: Option<String>,

    /// Dorrit Cohn consciousness mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consciousness_mode: Option<String>,

    /// Gardner psychic distance (1 = very close, 5 = very distant).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psychic_distance: Option<u8>,

    /// Narrator reliability flag or degree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub narrator_reliability: Option<String>,

    /// Domain of the scene's primary stakes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stakes_domain: Option<String>,

    /// Structured quality metrics (word count, sentence entropy, …).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_metrics: Option<Value>,

    /// Free-form motif tag annotations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motif_tags: Option<Value>,

    /// Literary / theoretical annotations (PROFILE.md §7.7).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theory_notes: Option<Value>,

    /// Want / obstacle / outcome for this unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_summary: Option<CanonicalSummary>,

    /// Arc type moment for the primary character in this unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_type: Option<String>,
}

// ── Unit — craft_targets ──────────────────────────────────────────────────────

/// Typed form of `unit.craft_targets` for the GBR narrative profile.
/// (PROFILE.md §4)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct NarrativeCraftTargets {
    /// Target tone/authorial attitude (`narrative_voice.json → tone`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tone: Option<String>,

    /// Target narrative tension (0.0 = none, 1.0 = maximum).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tension: Option<f64>,

    /// Target pacing mode (`scene_structure.json → pacing`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pacing: Option<String>,
}

// ── ParticipantState overlays ─────────────────────────────────────────────────

/// Typed form of `participant_state.observables` for the GBR narrative profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct NarrativeParticipantObservables {
    /// Micro-tactical action the character takes to advance their objective.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tactic: Option<String>,
}

/// Typed form of `participant_state.structure` for the GBR narrative profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct NarrativeParticipantStructure {
    /// Where on the character arc this unit falls (e.g. `"crack"`, `"pivot"`, `"break"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_beat: Option<String>,

    /// What triggered this participant's tactical shift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
}

/// Typed form of `participant_state.interpretations` for the GBR narrative profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct NarrativeParticipantInterpretations {
    /// Character arc trajectory for this unit (`ArcType` value).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_type: Option<String>,

    /// Active drive model (`DriveModel` value).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_model: Option<String>,

    /// Concealed emotional state beneath the surface action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_emotion: Option<String>,
}

// ── Step overlays ─────────────────────────────────────────────────────────────

/// Typed form of `step.interpretations` for the GBR narrative profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct NarrativeStepInterpretations {
    /// Emotional state of the acting character at this step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotional_state: Option<String>,

    /// Concealed emotion underneath the observable action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_emotion: Option<String>,

    /// Significance of the step (`"essential"` / `"supplementary"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub significance: Option<String>,
}

// ── Entity overlays (generic) ─────────────────────────────────────────────────
//
// Entity observable/structural/interpretation overlays diverge by entity_type
// (character vs. location vs. motif). The typed structs are in `entities.rs`.
// For the ProfileOverlay associated types, we use serde_json::Value so callers
// can choose the right concrete type via entity_observables_as::<CharacterObservables>()
// or entity_observables_as::<SettingObservables>() as appropriate.

/// Placeholder for entity overlay types — use `entity_observables_as::<T>()` with the
/// concrete entity-type struct from `entities.rs` instead.
pub type NarrativeEntityValue = Value;

// ── ProfileOverlay marker impl ────────────────────────────────────────────────

/// Marker struct: the GBR narrative profile.
///
/// Implement this on unit/entity/participant accessors to get typed overlay access:
///
/// ```rust,no_run
/// use gbr_types::overlay::GbrNarrativeProfile;
/// use gbr_types::sip::ProfileOverlay;
/// // Use P::Context = NarrativeContext in generic code
/// type Ctx = <GbrNarrativeProfile as ProfileOverlay>::Context;
/// ```
pub struct GbrNarrativeProfile;

impl ProfileOverlay for GbrNarrativeProfile {
    type Context = NarrativeContext;
    type Grouping = NarrativeGrouping;
    type UnitInterpretations = NarrativeUnitInterpretations;
    type CraftTargets = NarrativeCraftTargets;
    type EntityObservables = NarrativeEntityValue;
    type EntityStructure = NarrativeEntityValue;
    type EntityInterpretations = NarrativeEntityValue;
    type ParticipantObservables = NarrativeParticipantObservables;
    type ParticipantStructure = NarrativeParticipantStructure;
    type ParticipantInterpretations = NarrativeParticipantInterpretations;
    type StepInterpretations = NarrativeStepInterpretations;
}
