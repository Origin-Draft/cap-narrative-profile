# Character Scene State

A Character Scene State describes one character's internal and relational state at the entry and exit boundaries of a single scene — their emotions, knowledge, objectives, tactics, and arc position.

Character Scene States are embedded within Scene Cards in the `character_states` array. They are not stored as separate files.

For normative field definitions, see [SPECIFICATION.md §8](../SPECIFICATION.md#8-character-scene-state).

---

## Design Principle

Character psychology in fiction is a state machine — each scene advances or arrests the state. The Character Scene State schema operationalizes two foundational claims:

1. **Character is action:** What a character *wants* in a scene (Stanislavski) determines what they *do*, which determines meaning.
2. **Character is consciousness:** What a character *knows, feels, and perceives* (Cohn) determines how the scene is rendered if they are the focalizer.

The schema bridges dramaturgical tradition (wanting/doing) and narratological tradition (knowing/perceiving) in a single structured object.

---

## Epistemic Sections (v0.2.0)

Character Scene States use the three-section epistemic structure (`observables`, `structure`, `interpretations`). See [ADR-006](../docs/decisions/ADR-006-observable-structure-interpretation.md) for rationale.

**Alias cleanup (v0.2.0):** The canonical field name is `character` (not `character_id` or `character_ref`). The canonical emotion field is `emotion` (not `primary_emotion`).

### Observables

Facts directly grounded in the artifact. These fields MUST NOT use the `interpreted_value` wrapper.

| Field | Type | Notes |
|-------|------|-------|
| `character` | slug | REQUIRED; MUST resolve to declared character in registry |
| `pov_role` | enum | REQUIRED; `focalizer`, `participant`, or `non_present` |
| `posture` | string | Visible body position |
| `body_language` | array of string | Observable physical behaviors |
| `social_circles_active` | array of string | Social groups active in scene |
| `fid_markers` | array of string | FID textual markers present in prose |

### Structure

How the character's state is organized across the scene boundary.

| Field | Type | Notes |
|-------|------|-------|
| `objective` | ObjectiveObject | Character's scene-level want |
| `tactic` | enum | How they pursue it |
| `tactic_shift` | string | Mid-scene tactic change |
| `obstacle` | string | What blocks the objective |
| `trigger_type` | enum | `enums/emotion_psychology.json#trigger_type` |
| `want_outcome` | enum | Outcome of the scene want |
| `arc_beat` | enum | Position in character arc |
| `arc_direction` | string | `advancing`, `regressing`, or `stable` |
| `wound_triggered` | boolean | Whether psychological wound was activated |
| `knowledge_at_entry` | array of KnowledgeObject | What character knows at scene open |
| `knowledge_gaps` | array of KnowledgeObject | What character does not know |
| `knowledge_gained` | array of KnowledgeObject | What character learns during scene |
| `relationships` | array of RelationshipState | Relational edges active in scene |
| `psychic_distance_shifts` | array | Dynamic distance changes during scene |

**ObjectiveObject:**

```json
{
  "verb": "to refuse",
  "object_type": "commitment",
  "target_role": "fitzwilliam_darcy",
  "constraint": "Must not appear rude to the host"
}
```

The objective `verb` MUST be a transitive action verb directed at `target_role`. "To feel better" is not a valid objective. "To convince [target] to withdraw" is valid.

**KnowledgeObject:**

```json
{
  "domain": "relationships",
  "predicate": "suspects",
  "about_role": "fitzwilliam_darcy",
  "certainty": 0.3
}
```

Information asymmetry is the primary mechanism of dramatic irony. The three-array structure tracks what changes in the character's epistemic state across the scene boundary.

### Interpretations

Inferred meaning. All fields MAY use the `interpreted_value` wrapper (`{ "value": <T>, "confidence": 0.0–1.0, "source": "<string>" }`).

| Field | Type | Notes |
|-------|------|-------|
| `emotional_state_entry` | EmotionObject | Emotion at scene open |
| `emotional_state_exit` | EmotionObject | Emotion at scene close |
| `emotional_arc` | enum | Shape of emotional movement |
| `emotion` | enum | Primary emotion felt |
| `masked_emotion` | enum | Emotion displayed to others |
| `psychic_distance` | integer 1–5 | Gardner scale for this character's rendering |
| `consciousness_mode` | enum | Cohn mode for this character's interior |
| `social_mask` | string | The public persona performed |
| `social_role` | enum | `enums/literary_theory.json#social_role_type` |
| `want_need_alignment` | string | Relationship between want and need |
| `actantial_role` | enum | `enums/character.json#actant` |
| `wound_category` | enum | `enums/character.json#wound` |
| `stakes` | object | Personal/relational stakes |
| `arc_type` | enum | `enums/character.json#arc_type` |
| `drive_model` | enum | `enums/character.json#drive_model` |

**EmotionObject:**

```json
{
  "emotion": "humiliation",
  "intensity": 4,
  "secondary_emotion": "anger",
  "masked": true
}
```

If `emotional_state_entry` equals `emotional_state_exit`, the scene has not performed emotional work for this character.

The `masked` boolean indicates whether the character is suppressing or performing a different emotion than they feel — this determines what the prose can access vs. what it must perform externally.

### Focalizer-Specific Fields

These fields apply only when `observables.pov_role == "focalizer"`. The `psychic_distance_shifts` array (in `structure`) records rendering change points. The `psychic_distance` integer and `consciousness_mode` enum (in `interpretations`) classify the focalizer's rendering mode.

---

## Example

```json
{
  "observables": {
    "character": "elizabeth_bennet",
    "pov_role": "focalizer",
    "fid_markers": ["exclamatory_syntax", "evaluative_language"]
  },
  "structure": {
    "objective": {
      "verb": "to refuse",
      "object_type": "commitment",
      "target_role": "fitzwilliam_darcy",
      "constraint": "Must maintain civility"
    },
    "tactic": "direct_confrontation",
    "want_outcome": "granted",
    "wound_triggered": true,
    "arc_direction": "advancing"
  },
  "interpretations": {
    "psychic_distance": 4,
    "consciousness_mode": "narrated_monologue",
    "emotional_state_entry": {
      "emotion": "shock",
      "intensity": 3,
      "secondary_emotion": "anger",
      "masked": false
    },
    "emotional_state_exit": {
      "emotion": "contempt",
      "intensity": 4,
      "masked": false
    }
  }
}
```
