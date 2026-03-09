# Grimoire Book Representation Protocol (GBR)

**Version:** 1.0.0 | **Status:** Draft

A formal standard for representing narrative fiction in a structured, machine-readable format. GBR enables bidirectional conversion between prose and structured scene specifications, designed for fine-tuning LLMs on narrative generation/analysis and building writing-assistance tools.

---

## Core Principle: Lossless Round-Trip

```
parse(render(semantic_structure)) == semantic_structure
```

Any prose passage generated from a scene specification can be decomposed back into that exact specification. This is achieved through:

1. **Typed enumerations** — no free-text where structure is possible
2. **Canonical summaries** — deterministic serialization of scene semantics
3. **Entity registries** — controlled vocabularies per book

---

## Repository Layout

```
gbr-protocol/
  protocol/             # GBR Protocol specification
    schema/             # JSON Schema for all protocol types
    docs/               # ENUMS, THEORY, ROUND_TRIP, architecture docs
    examples/           # Annotated real-book examples
  grimoire-types/       # Rust crate — canonical source of truth for types + CLIs
    src/
      enums.rs          # All typed enumerations
      entities.rs       # Character, Setting, Relationship, EntityRegistry
      voice.rs          # VoiceSignature
      tags.rs           # Annotation tag vocabulary
      bin/              # CLI binaries
  schemas/              # JSON Schema extraction schemas for Grimoire templates
  scripts/
    validate_against_schema.py   # grimoire-validate CLI
```

---

## Data Model

### Layer 1: Entity Registry
Per-book controlled vocabulary for all named entities (characters, settings, relationships). Schema: [`protocol/schema/registry.schema.json`](protocol/schema/registry.schema.json)

### Layer 2: Story Architecture
Book-level structural metadata — genre, collision pattern, protagonist arc, actantial model. Schema: [`protocol/schema/story-architecture.schema.json`](protocol/schema/story-architecture.schema.json)

### Layer 3: Scene Card
Per-scene specification:

| Category | Fields |
|----------|--------|
| **Identity** | `scene_id`, `book_id`, `chapter`, `scene_index` |
| **Structure** | `act`, `beat`, `arc_position`, `scene_function` |
| **Voice** | `pov`, `focalization`, `focalizer`, `diegetic_level` |
| **Narrative Time** | `order`, `duration`, `frequency` (Genette) |
| **Craft** | `target_tension`, `target_pacing`, `tone` |
| **State** | `character_states[]`, `setting_instance` |
| **Semantics** | `canonical_summary` (deterministic render) |

Schema: [`protocol/schema/scene-card.schema.json`](protocol/schema/scene-card.schema.json)

### Layer 4: Character Scene State
Per-character state at scene boundaries — emotional/knowledge state, objectives, tactics, obstacles, arc position. Schema: [`protocol/schema/character-state.schema.json`](protocol/schema/character-state.schema.json)

---

## Enumeration Vocabulary

All categorical fields use typed enumerations defined in:

- **Rust:** [`grimoire-types/src/enums.rs`](grimoire-types/src/enums.rs)
- **JSON Schema:** [`protocol/schema/enums.schema.json`](protocol/schema/enums.schema.json)
- **Docs:** [`protocol/docs/ENUMS.md`](protocol/docs/ENUMS.md)

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

## Installation

### Rust CLI binaries

```bash
cd grimoire-types && cargo build --release
# Installs: grimoire-gate-check, grimoire-generate,
#           grimoire-validate-catalogs, grimoire-export-training
```

### Python validation CLI

```bash
pip install -e .
grimoire-validate --help
```

---

## Validation

```bash
# Validate a registry against schema
jsonschema -i book/registry.json protocol/schema/registry.schema.json

# Validate a scene card
jsonschema -i book/scenes/ch01_s01.json protocol/schema/scene-card.schema.json

# Validate a Grimoire template against its schema
grimoire-validate path/to/template.md
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

See [`protocol/docs/THEORY.md`](protocol/docs/THEORY.md) for concept-level mappings.

---

## Corpus File Structure

A GBR-compliant book corpus:

```
{book_id}/
  registry.json              # Entity registry
  story_architecture.json    # Book-level structure
  scenes/
    ch01_s01.json            # Scene cards
    ch01_s02.json
  prose/
    ch01_s01.txt             # Aligned prose passages
    ch01_s02.txt
```

---

## Contributing

Schema changes require:
1. Update Rust types in `grimoire-types/`
2. Regenerate JSON schemas via `cargo run --bin grimoire-validate-catalogs`
3. Update protocol documentation
4. Bump protocol version in `protocol/README.md` changelog

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

---

## License

MIT — see [LICENSE](LICENSE)
