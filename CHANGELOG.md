# Changelog

All notable changes to the GBR Protocol are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).  
This project uses [Semantic Versioning](VERSIONING.md).

---

## [Unreleased]

*Changes staged for the next release.*

---

## [0.2.0] — 2026-03-10

**BREAKING** — Epistemic section restructuring (ADR-006).

### Changed

**Specification**
- `SPECIFICATION.md` v0.2.0 — all document types restructured with epistemic sections (observables / structure / interpretations); Scene Cards add a fourth `craft_targets` section; new §3.6 Principle of Epistemic Separation; updated validation rules (§9–§10)

**Protocol Documents** (`protocol/`)
- `architecture.md` — added "Epistemic Sections (v0.2.0)" section
- `scene-card.md` — field tables reorganized into four epistemic sections
- `character-state.md` — fields reorganized into three epistemic sections with nested examples
- `entity-registry.md` — character, setting, and relationship sub-objects reorganized into epistemic sub-sections
- `story-architecture.md` — flat field tables replaced with observables/structure/interpretations tables

**Schemas** (`schemas/`)
- `enums.schema.json` — added `interpreted_value` `$def` (oneOf wrapper for interpretation enums)
- `scene-card.schema.json` — top-level properties replaced by `observables`, `structure`, `interpretations`, `craft_targets`; scene_turn `$def` uses internal observables/interpretations split; `setting_instance` `$def` added
- `character-state.schema.json` — top-level required reduced to `["observables"]`; three nested sections; oneOf wrappers on all interpretation enum fields; `character_id`/`character_ref` aliases removed (canonical: `character`)
- `story-architecture.schema.json` — restructured into `observables` (`inciting_incident_chapter`), `structure` (`genre` required, structural fields), `interpretations` (thematic/analytical fields with oneOf wrappers)
- `registry.schema.json` — character/setting/relationship `$defs` restructured into nested observables/structure/interpretations

**Examples** (`examples/`)
- All 17 example JSON files converted from flat v0.1.0 format to nested v0.2.0 format
- `minimal/` — 4 files: story-architecture, registry, scene-card, character-state
- `small-story/threshold/` — 5 files: story-architecture, registry, ch01_s01, ch01_s02, ch02_s01
- `small-story/metamorphosis/` — 2 files: registry, scene card (ch01_s01)
- `edge-cases/` — 6 files: omniscient_multiple_focalizer, non_present_character, iterative_scene, flat_arc_focalizer, dual_pov_split, analepsis_within_analepsis

**Conformance Tests** (`conformance/`)
- All 8 conformance test files updated to v0.2.0 nested structure
- Expected error field paths updated (e.g. `pov` → `interpretations.pov`, `character_ref` → `observables.character`)

**Reference Implementation** (`reference/rust/`)
- `entities.rs` — `Character`, `Setting`, `Relationship` structs split into `*Observables`, `*Structure`, `*Interpretations` sub-structs; added `InterpretedValue<T>` enum wrapper

### Added

**Design Documentation** (`docs/`)
- `docs/decisions/ADR-006-observable-structure-interpretation.md` — architectural decision record for the epistemic section split
- `docs/terminology.md` — 6 new definitions: observable, structural field, interpretation, interpreted value, epistemic section, craft target

### Removed
- **Field aliases**: `character_id`, `character_ref` (use `character`); `focalization_type` (use `focalization`); `primary_emotion` (use `emotion`)

---

## [0.1.0] — 2026-03-09

Initial published version of the GBR (Grimoire Book Representation) Protocol.

### Added

**Specification**
- `SPECIFICATION.md` — 14-section normative specification with RFC 2119 language
- `VERSIONING.md` — SemVer policy document
- `CONTRIBUTING.md` — protocol-focused contribution guide

**Protocol Documents** (`protocol/`)
- `architecture.md` — four-layer data model and corpus layout
- `entity-registry.md` — Entity Registry document type reference
- `scene-card.md` — Scene Card field groups and the Scene Turn Requirement
- `story-architecture.md` — Story Architecture document type reference
- `character-state.md` — Character Scene State reference
- `canonical-summary.md` — Canonical Summary template and round-trip contract

**Schemas** (`schemas/`)
- `registry.schema.json` — Entity Registry JSON Schema (Draft-07)
- `scene-card.schema.json` — Scene Card JSON Schema (Draft-07)
- `story-architecture.schema.json` — Story Architecture JSON Schema (Draft-07)
- `character-state.schema.json` — Character Scene State JSON Schema (Draft-07)
- `enums.schema.json` — Enumeration definitions JSON Schema (Draft-07)

**Enumeration Vocabularies** (`enums/`)
- `character.json` — archetype (21), wound (23), alignment (9), role (17), drive_model (5), arc_type (7), actant (6)
- `narrative_voice.json` — pov_type (5), focalization_type (5), consciousness_mode (4), diegetic_level (3), narrator_reliability_type (6), narratee_type (4), pov_role_type (3), tone (16)
- `narrative_time.json` — narrative_order (5), duration_mode (5), frequency (4)
- `scene_structure.json` — beat_type (15), scene_function (8), scene_polarity (24), event_type (11), want_outcome (6), causal_role (6), stakes_domain (8)
- `emotion_psychology.json` — emotion (30), tactic (20), trigger_type (12), emotional_arc_type (9)
- `setting.json` — setting_type (11), time_of_day (9), spatial_structure (9), atmosphere (14), territory_type (7)
- `relationship.json` — relationship_type (18), relationship_dynamic (12), power_balance (5)
- `literary_theory.json` — genre_type (21), collision_type (9), antagonist_type (8), opposition_level (5), transtextuality_type (5), irony_type (7), gaze_type (6), speech_act_category (5), freudian_mechanism (8)

**Conformance Tests** (`conformance/`)
- `valid/minimal_registry.json` — smallest valid Entity Registry
- `valid/minimal_scene.json` — smallest valid Scene Card
- `valid/minimal_story.json` — smallest valid Story Architecture
- `valid/full_scene.json` — Scene Card with all major optional fields
- `invalid/missing_scene_id.json` + sidecar — Level 1 failure: missing required field
- `invalid/unknown_enum_value.json` + sidecar — Level 1 failure: invalid enum value
- `invalid/unresolved_entity_ref.json` + sidecar — Level 2 failure: unresolved entity reference
- `invalid/invalid_canonical_summary.json` + sidecar — Level 2 failure: empty scene_turns

**Examples** (`examples/`)
- `minimal/` — complete minimal corpus: registry, story-architecture, scene-card, character-state
- `small-story/metamorphosis/` — Kafka's *The Metamorphosis* scene and registry examples
- `edge-cases/` — iterative scene, dual POV split examples

**Design Documentation** (`docs/`)
- `terminology.md` — formal glossary of all GBR terms
- `design-principles.md` — rationale for core protocol decisions
- `decisions/ADR-001-scene-as-atomic-unit.md`
- `decisions/ADR-002-canonical-summary.md`
- `decisions/ADR-003-enum-based-semantics.md`
- `decisions/ADR-004-lossless-round-trip.md`
- `decisions/ADR-005-fabula-syuzhet-separation.md`

**Grimoire Template Schemas** (`template-schemas/`)
- All Grimoire phase-based extraction schemas (moved from `schemas/` to distinguish from protocol schemas)

---

## Pre-History

Earlier work on this repository was conducted under the `grimoire-types` Rust crate and `protocol/docs/` directory. That work informed the 0.1.0 release but was not semantically versioned. The original `protocol/docs/` content (THEORY.md, ENUMS.md, ROUND_TRIP.md, ENTITY_TYPES.md, architecture/) is retained for reference pending migration.
