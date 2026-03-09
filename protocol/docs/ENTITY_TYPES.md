# Entity Types Reference

This document describes the core entity types in the GBR Protocol.

**Source of Truth:**
- JSON Schema: [`schema/registry.schema.json`](../schema/registry.schema.json)
- Rust: [`grimoire-types/src/entities.rs`](../../grimoire-types/src/entities.rs)

---

## Overview

The protocol defines four core entity types:

| Type | Purpose | Storage |
|------|---------|---------|
| `Character` | Named character with traits | `registry.characters[slug]` |
| `Setting` | Named location/place | `registry.settings[slug]` |
| `Relationship` | Directed edge between characters | `registry.relationships[]` |
| `WantVocabulary` | Canonical want labels | `registry.want_vocabulary[slug]` |

All entities are identified by **slugs** — lowercase snake_case strings matching `^[a-z0-9_]+$`.

---

## Character

A declared character in the story.

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Snake-case slug (`elizabeth_bennet`) |
| `name` | string | Display name (`Elizabeth Bennet`) |

### Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `slot` | string | Cast template position (protagonist, deuteragonist, etc.) |
| `archetype` | enum | Campbell/Vogler archetype (21 variants) |
| `wound` | enum | Core psychological wound (23 variants) |
| `alignment` | enum | 9-cell moral alignment |
| `role` | enum | Story function (17 variants) |
| `drive_model` | enum | Motivation framework (5 variants) |
| `arc_type` | enum | Character arc trajectory (7 variants) |
| `actant` | enum | Greimas actantial role (6 variants) |
| `ghost` | string | Internal wound origin/backstory |
| `want` | string | External goal (what they pursue) |
| `need` | string | Thematic truth (what they actually need) |
| `flaw` | string | Fatal flaw or limitation |
| `voice_signature` | object | Voice fingerprint (see below) |

### Voice Signature

Optional character voice fingerprint for prose generation:

```json
{
  "sentence_length_tendency": "varied",
  "vocabulary_register": "formal",
  "syntax_complexity": "complex",
  "fid_markers": ["exclamatory_syntax", "evaluative_language"],
  "forbidden_words": ["awesome", "like"],
  "signature_phrases": ["I dare say", "Upon my word"]
}
```

| Field | Values |
|-------|--------|
| `sentence_length_tendency` | `short`, `medium`, `long`, `varied` |
| `vocabulary_register` | `colloquial`, `standard`, `formal`, `archaic`, `mixed` |
| `syntax_complexity` | `simple`, `moderate`, `complex` |
| `fid_markers` | List of FID marker types |
| `forbidden_words` | Words character would never use |
| `signature_phrases` | Characteristic expressions |

### Example

```json
{
  "id": "elizabeth_bennet",
  "name": "Elizabeth Bennet",
  "slot": "protagonist",
  "archetype": "hero",
  "wound": "identity_rejection",
  "alignment": "chaotic_good",
  "role": "protagonist",
  "drive_model": "perception",
  "arc_type": "positive_change",
  "actant": "subject",
  "ghost": "Her mother's favoritism toward Jane; society's dismissal of her intellect",
  "want": "To marry for love, not convenience",
  "need": "To see past her own prejudices",
  "flaw": "Quick to judge others based on first impressions",
  "voice_signature": {
    "sentence_length_tendency": "varied",
    "vocabulary_register": "formal",
    "syntax_complexity": "complex",
    "fid_markers": ["exclamatory_syntax", "evaluative_language", "unattributed_questions"],
    "signature_phrases": ["I wonder", "How strange"]
  }
}
```

---

## Setting

A declared location in the story.

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Snake-case slug |
| `name` | string | Display name |

### Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `type` | enum | Setting category (`estate_interior`, `garden`, etc.) |
| `general_vibe` | string | Atmosphere/emotional tone |
| `sensory_signature` | array | Three defining sensory details |

### Example

```json
{
  "id": "hunsford_parsonage",
  "name": "Hunsford Parsonage",
  "type": "domestic_modest",
  "general_vibe": "Modest clerical comfort shadowed by proximity to Rosings",
  "sensory_signature": [
    "The smell of beeswax and new furniture",
    "Charlotte's practical arrangements visible everywhere",
    "Windows oriented toward Rosings Park"
  ]
}
```

---

## Relationship

A directed edge connecting two characters.

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `source` | string | Source character slug |
| `target` | string | Target character slug |
| `rel_type` | enum | Relationship type (`sibling`, `lover`, `rival`, etc.) |

### Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `description` | string | Free-text description |
| `dynamic_at_start` | enum | Relationship state at story start |
| `dynamic_at_end` | enum | Relationship state at story end |
| `power_balance` | enum | Power asymmetry |

### Example

```json
{
  "source": "elizabeth_bennet",
  "target": "fitzwilliam_darcy",
  "rel_type": "suitor",
  "description": "Initial prejudice transforming through conflict into love",
  "dynamic_at_start": "hostile",
  "dynamic_at_end": "harmonious",
  "power_balance": "contested"
}
```

---

## Want Vocabulary

Maps want slugs to display labels for canonical summaries.

### Purpose

The canonical summary template renders character wants as human-readable strings:

```
Elizabeth Bennet refuses Fitzwilliam Darcy...; wants honest respect [DENIED]
                                                    ^^^^^^^^^^^^^^
                                                    from want_vocabulary
```

### Example

```json
{
  "honest_respect": "honest respect",
  "social_acceptance": "acceptance in society",
  "financial_security": "financial security",
  "romantic_love": "romantic love",
  "family_approval": "family approval",
  "independence": "independence",
  "truth": "the truth"
}
```

---

## Entity References

Scene cards and character states reference entities by slug:

```json
{
  "scene_id": "pride_ch34_s01",
  "pov": "third_limited",
  "focalizer": "elizabeth_bennet",        // Character slug
  "setting": "hunsford_parsonage",        // Setting slug
  "participants": [
    "elizabeth_bennet",                   // Character slugs
    "fitzwilliam_darcy"
  ],
  "canonical_summary": {
    "pov_character": "elizabeth_bennet",  // Character slug
    "character_want": "honest_respect",   // Want vocabulary slug
    "setting": "hunsford_parsonage"       // Setting slug
  }
}
```

### Validation

All slug references must resolve to entries in the registry:

```python
def validate_slugs(scene_card, registry):
    assert scene_card.focalizer in registry.characters
    assert scene_card.setting in registry.settings
    for p in scene_card.participants:
        assert p in registry.characters
```

---

## Registry Structure

Complete registry example:

```json
{
  "$schema": "https://grimoire.dev/protocol/v1/registry.schema.json",
  "book_id": "pride_and_prejudice",
  "title": "Pride and Prejudice",
  "author": "Jane Austen",
  
  "characters": {
    "elizabeth_bennet": { /* Character object */ },
    "fitzwilliam_darcy": { /* Character object */ },
    "jane_bennet": { /* Character object */ }
  },
  
  "settings": {
    "longbourn": { /* Setting object */ },
    "hunsford_parsonage": { /* Setting object */ },
    "pemberley": { /* Setting object */ }
  },
  
  "relationships": [
    { "source": "elizabeth_bennet", "target": "jane_bennet", "rel_type": "sibling" },
    { "source": "elizabeth_bennet", "target": "fitzwilliam_darcy", "rel_type": "suitor" }
  ],
  
  "want_vocabulary": {
    "honest_respect": "honest respect",
    "romantic_love": "romantic love",
    "social_acceptance": "acceptance in society"
  }
}
```

---

## Rust Type Mapping

| JSON Schema | Rust Type | Location |
|-------------|-----------|----------|
| `Character` | `grimoire_types::entities::Character` | `entities.rs` |
| `Setting` | `grimoire_types::entities::Setting` | `entities.rs` |
| `Relationship` | `grimoire_types::entities::Relationship` | `entities.rs` |
| `EntityRegistry` | `grimoire_types::entities::EntityRegistry` | `entities.rs` |
| `VoiceSignature` | `grimoire_types::voice::VoiceSignature` | `voice.rs` |
| All enums | `grimoire_types::enums::*` | `enums.rs` |

All Rust types derive `schemars::JsonSchema` for automatic schema generation.
