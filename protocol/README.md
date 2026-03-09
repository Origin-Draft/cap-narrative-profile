# Grimoire Book Representation Protocol (GBR)

**Version:** 1.0.0  
**Status:** Draft  
**Canonical Source:** [grimoire-types](../grimoire-types/) (Rust)

---

## Overview

The Grimoire Book Representation (GBR) Protocol is a formal standard for representing narrative fiction in a structured, machine-readable format. It enables bidirectional conversion between:

1. **Structured scene specifications** → **Prose** (generation)
2. **Prose** → **Structured scene specifications** (extraction)

The protocol is designed for:
- Fine-tuning LLMs on prose generation/analysis
- Building writing assistance tools
- Audiobook and screenplay adaptation pipelines
- Literary analysis and annotation workflows

---

## Core Principle: Lossless Round-Trip

The protocol's central guarantee:

```
parse(render(semantic_structure)) == semantic_structure
```

Any prose passage that can be generated from a scene specification can be decomposed back into that exact specification. This is achieved through:

1. **Typed enumerations** — No free-text where structure is possible
2. **Canonical summaries** — Deterministic serialization of scene semantics
3. **Entity registries** — Controlled vocabularies per book

---

## Data Model

### Layer 1: Entity Registry

Per-book controlled vocabulary defining all named entities:

```
registry/
  ├── characters     → slug → Character struct
  ├── settings       → slug → Setting struct  
  ├── relationships  → [Relationship structs]
  └── want_vocabulary → slug → display label
```

**Schema:** [`schema/registry.schema.json`](schema/registry.schema.json)

### Layer 2: Story Architecture

Book-level structural metadata:

- Genre classification
- Collision patterns (social circle dynamics)
- Antagonist design
- Protagonist arc (lie/truth, want/need)
- Actantial model mapping
- Transtextual references

**Schema:** [`schema/story-architecture.schema.json`](schema/story-architecture.schema.json)

### Layer 3: Scene Card

Per-scene specification containing:

| Category | Fields |
|----------|--------|
| **Identity** | `scene_id`, `book_id`, `chapter`, `scene_index` |
| **Structure** | `act`, `beat`, `arc_position`, `scene_function` |
| **Voice** | `pov`, `focalization`, `focalizer`, `diegetic_level` |
| **Narrative Time** | `order`, `duration`, `frequency` (Genette) |
| **Craft** | `target_tension`, `target_pacing`, `tone` |
| **State** | `character_states[]`, `setting_instance` |
| **Semantics** | `canonical_summary` (deterministic render) |

**Schema:** [`schema/scene-card.schema.json`](schema/scene-card.schema.json)

### Layer 4: Character Scene State

Per-character state at scene boundaries:

- Emotional state (entry/exit)
- Knowledge state (what they know, what they learn)
- Objectives, tactics, obstacles
- Relationship dynamics changes
- Wound activation, arc position

**Schema:** [`schema/character-state.schema.json`](schema/character-state.schema.json)

---

## Enumeration Vocabulary

All categorical fields use typed enumerations. The complete vocabulary is defined in:

- **JSON Schema:** [`schema/enums.schema.json`](schema/enums.schema.json)
- **Rust Source:** [`grimoire-types/src/enums.rs`](../grimoire-types/src/enums.rs)
- **Documentation:** [`docs/ENUMS.md`](docs/ENUMS.md)

### Category Summary

| Domain | Enum Count | Total Variants |
|--------|------------|----------------|
| Character | 7 | ~150 |
| Scene | 12 | ~200 |
| Narrative Voice | 8 | ~80 |
| Relationship | 4 | ~60 |
| Setting | 6 | ~100 |
| Temporal | 6 | ~50 |
| Thematic | 5 | ~80 |

---

## Canonical Summary Template

The backbone of the round-trip guarantee is the **canonical summary** — a deterministic serialization of scene semantics:

```
{POV_CHAR} {EVENT_VERB} {PARTICIPANTS} at {LOCATION}; 
wants {WANT_OBJECT} [{OUTCOME}]; 
stakes={STAKES}, atmosphere={ATMOSPHERE}, role={CAUSAL_ROLE}.
```

### Example

**Scene Card (semantic):**
```json
{
  "pov_character": "elizabeth_bennet",
  "event_type": "refusal",
  "participants": ["fitzwilliam_darcy"],
  "setting": "hunsford_parsonage",
  "character_want": "honest_respect",
  "want_outcome": "denied",
  "stakes_domain": "social",
  "atmosphere": "tense",
  "causal_role": "pivots"
}
```

**Canonical Summary (rendered):**
```
Elizabeth Bennet refuses Fitzwilliam Darcy at Hunsford Parsonage; 
wants honest respect [DENIED]; stakes=social, atmosphere=tense, role=PIVOTS.
```

**Contract:**
```python
assert parse_summary(render_summary(card, registry), registry) == card
```

See [`docs/ROUND_TRIP.md`](docs/ROUND_TRIP.md) for full specification.

---

## Implementation Bindings

### Rust (Canonical)

The Rust crate `grimoire-types` is the source of truth:

```rust
use grimoire_types::{
    entities::{Character, Setting, Relationship, EntityRegistry},
    enums::{Archetype, Wound, Alignment, Role, DriveModel, ArcType, Actant},
    voice::VoiceSignature,
};
```

All structs derive `schemars::JsonSchema` for automatic schema generation.

**Location:** [`grimoire-types/`](../grimoire-types/)

### JSON Schema

Protocol-compliant JSON schemas generated from Rust types:

```
protocol/schema/
  ├── registry.schema.json
  ├── scene-card.schema.json
  ├── story-architecture.schema.json
  ├── character-state.schema.json
  └── enums.schema.json
```

### Python (Planned)

Pydantic models generated from JSON Schema are planned for `v1.1.0`.

---

## File Organization

A GBR-compliant book corpus follows this structure:

```
{book_id}/
  ├── registry.json              # Entity registry
  ├── story_architecture.json    # Book-level structure
  ├── scenes/
  │   ├── ch01_s01.json          # Scene cards
  │   ├── ch01_s02.json
  │   └── ...
  └── prose/
      ├── ch01_s01.txt           # Aligned prose passages
      ├── ch01_s02.txt
      └── ...
```

---

## Validation

### Schema Validation

```bash
# Validate a registry against schema
jsonschema -i book/registry.json protocol/schema/registry.schema.json

# Validate a scene card
jsonschema -i book/scenes/ch01_s01.json protocol/schema/scene-card.schema.json
```

### Round-Trip Validation

```bash
# Verify canonical summary round-trips
python -m grimoire.validate_roundtrip book/scenes/*.json
```

---

## Theoretical Foundations

The protocol encodes concepts from:

| Domain | Theorists | Concepts |
|--------|-----------|----------|
| Narratology | Genette, Bal, Rimmon-Kenan | Focalization, diegetic levels, narrative time |
| Structuralism | Greimas, Propp | Actantial model, narrative functions |
| Character | Truby, Weiland | Want/need, lie/truth, arc types |
| Rhetoric | Burke | Pentad (act/scene/agent/agency/purpose) |
| Psychoanalysis | Freud, Lacan, Kristeva | Mechanisms, registers, abjection |
| Speech Acts | Austin, Searle, Grice | Illocutionary force, maxim violation |

See [`docs/THEORY.md`](docs/THEORY.md) for concept-level mappings.

For **field-level rationale** (why each specific schema attribute exists and which scholarly tradition it derives from), see [`docs/architecture/`](docs/architecture/):

| Document | Schema |
|----------|--------|
| [`docs/architecture/SCENE_CARD.md`](docs/architecture/SCENE_CARD.md) | `scene-card.schema.json` |
| [`docs/architecture/CHARACTER_STATE.md`](docs/architecture/CHARACTER_STATE.md) | `character-state.schema.json` |
| [`docs/architecture/REGISTRY.md`](docs/architecture/REGISTRY.md) | `registry.schema.json` |
| [`docs/architecture/STORY_ARCHITECTURE.md`](docs/architecture/STORY_ARCHITECTURE.md) | `story-architecture.schema.json` |

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2026-03-06 | Initial protocol specification |

---

## Contributing

The protocol is developed as part of the [Grimoire](https://github.com/your-org/grimoire) project.

Schema changes require:
1. Update Rust types in `grimoire-types/`
2. Regenerate JSON schemas
3. Update protocol documentation
4. Bump protocol version

---

## License

MIT License — see [LICENSE](../LICENSE)
