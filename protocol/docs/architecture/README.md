# Architecture Documents

This directory contains field-level theoretical rationale for every schema attribute in the GBR Protocol. Each document explains *why* a field exists, which scholarly tradition it derives from, and what intellectual work it performs.

## Documents

| Document | Schema | Description |
|----------|--------|-------------|
| [SCENE_CARD.md](SCENE_CARD.md) | `scene-card.schema.json` | Scene identity, structure, voice, temporal logic, subtext, canonical summary |
| [CHARACTER_STATE.md](CHARACTER_STATE.md) | `character-state.schema.json` | Consciousness, emotion, knowledge, action, arc, psychology, social, relationships |
| [REGISTRY.md](REGISTRY.md) | `registry.schema.json` | Character, voice signature, setting, relationship, want vocabulary |
| [STORY_ARCHITECTURE.md](STORY_ARCHITECTURE.md) | `story-architecture.schema.json` | Genre, collision, inciting incident, antagonist, protagonist arc, actantial map, transtextuality, themes |

## How to Read These Documents

Each doc follows this format:
1. **Design Principle** — the foundational theoretical commitment the schema makes
2. **Per-field sections** — the field name, type, and full scholarly rationale
3. **References table** — all cited works, authors, years, and which fields they inform

## Relation to THEORY.md

[`THEORY.md`](../THEORY.md) maps high-level theoretical *concepts* to theorists (e.g., "Genette → focalization → narrative time"). The architecture documents go deeper: they explain why each *specific field* is structured the way it is, what the field's vocabulary choices represent, and what is lost if the field is omitted.

## Theoretical Density: Quick Navigator

| Theorist | Primary fields |
|----------|---------------|
| Genette | `focalization`, `narrative_time.*`, `diegetic_level`, `has_frame_narrative`, `transtextuality.*` |
| Cohn | `consciousness_mode`, `fid_markers`, `psychic_distance` |
| Gardner | `psychic_distance`, `psychic_distance_shifts` |
| Greimas | `actantial_role`, `actant`, `actantial_map` |
| McKee | `beat`, `turn`, `scene_function`, `want_outcome`, `causal_role`, `controlling_idea` |
| Truby | `obstacle`, `drive_model`, `want_need_alignment`, `ghost`, `antagonist.*` |
| Weiland | `arc_type`, `lie_believed`, `truth_needed`, `wound`, `arc_direction` |
| Grice | `subtext.maxim_violated`, `subtext.violation_type` |
| Hemingway | `subtext.iceberg_category` |
| Lotman | `spatial_structure`, `setting.type` |
| Aristotle | `act`, `flaw`, `inciting_incident.type` |
| Bakhtin | `consciousness_mode` (FID/heteroglossia), `vocabulary_register` |
| Bourdieu | `collision_architecture`, `power_asymmetry` |
| Sternberg | `narrative_time.order`, `knowledge_gaps` |
| Propp | `event_type`, structural derivation of `beat` |
| Booth | `narrator_reliability`, `pov` |
| Stanislavski | `objective`, `tactic`, `trigger_type` |
| Plutchik | `emotional_state.*` |
| van der Kolk | `wound_category`, `wound_triggered` |
| Scarry | `posture`, `body_language`, `sensory_signature` |
| Goffman | `social_mask`, `social_role`, `territory_type` |
