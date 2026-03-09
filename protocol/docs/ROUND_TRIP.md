# The Round-Trip Contract

The GBR Protocol's central guarantee is **lossless bidirectional conversion** between structured scene specifications and prose.

---

## The Problem

Human-authored prose summaries are **ambiguous by construction**:

```
# Human prose summary (NOT valid training data)
"Elizabeth refuses Darcy's proposal. The atmosphere is tense and the
confrontation exposes their mutual misunderstanding about pride and
prejudice. Darcy leaves humiliated."
```

This cannot be reliably parsed back to structure:
- Event type: Is this `confrontation`? `refusal`? `revelation`?
- Want outcome: `DENIED`? `PYRRHIC`?  
- Causal role: Does this `ESTABLISH` or `PIVOT`?

**A summary that cannot be parsed back to its structured source is not training data — it is noise.**

---

## The Solution: Canonical Summaries

A valid summary is the **deterministic serialization** of semantic structure into a fixed-grammar sentence.

### Template

```
{POV_CHAR} {EVENT_VERB} {PARTICIPANTS} at {LOCATION}; 
wants {WANT_OBJECT} [{OUTCOME}]; 
stakes={STAKES}, atmosphere={ATMOSPHERE}, role={CAUSAL_ROLE}.
```

### Example

**Semantic Structure (JSON):**
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

---

## The Contract

Two functions define the canonical summary:

```python
def render_summary(semantic_dict: dict, registry: Registry) -> str:
    """Convert semantic structure to canonical summary string."""
    ...

def parse_summary(summary: str, registry: Registry) -> dict:
    """Parse canonical summary string back to semantic structure."""
    ...
```

### Invariant

```python
parse_summary(render_summary(d, r), r) == d  # for all valid d
```

Any string that cannot be produced by `render_summary` is **not a valid summary** and is rejected from training data.

---

## Slot Specification

### Template Slots

| Slot | Schema Field | Type | Render Rule |
|------|--------------|------|-------------|
| `{POV_CHAR}` | `pov_character` | slug | `registry.characters[slug].name` |
| `{EVENT_VERB}` | `event_type` | enum | `EVENT_VERBS[event_type]` |
| `{PARTICIPANTS}` | `participants[]` | slugs | Comma-joined names |
| `{LOCATION}` | `setting` | slug | `registry.settings[slug].name` |
| `{WANT_OBJECT}` | `character_want` | slug | `registry.want_vocabulary[slug]` |
| `{OUTCOME}` | `want_outcome` | enum | `GRANTED`, `DENIED`, `DEFERRED`, `PYRRHIC` |
| `{STAKES}` | `stakes_domain` | enum | `physical`, `emotional`, `social`, etc. |
| `{ATMOSPHERE}` | `atmosphere` | enum | `tense`, `peaceful`, `ominous`, etc. |
| `{CAUSAL_ROLE}` | `causal_role` | enum | `ESTABLISHES`, `ESCALATES`, `PIVOTS`, etc. |

### Event Type → Verb Mapping

```python
EVENT_VERBS = {
    "arrival": "arrives with",
    "departure": "departs from",
    "confrontation": "confronts",
    "confession": "confesses to",
    "discovery": "discovers",
    "decision": "decides",
    "proposal": "proposes to",
    "refusal": "refuses",
    "acceptance": "accepts",
    "betrayal": "betrays",
    "reconciliation": "reconciles with",
    "revelation": "reveals to",
    "deception": "deceives",
    "rescue": "rescues",
    "escape": "escapes from",
    "transformation": "transforms",
    "death": "witnesses the death of",
    "celebration": "celebrates with"
}
```

---

## Implementation

### Render Function

```python
def render_summary(card: dict, registry: Registry) -> str:
    pov = registry.characters[card["pov_character"]].name
    verb = EVENT_VERBS[card["event_type"]]
    
    participants = ", ".join(
        registry.characters[p].name 
        for p in card.get("participants", [])
    )
    if not participants:
        participants = ""
        verb_phrase = verb
    else:
        verb_phrase = f"{verb} {participants}"
    
    location = registry.settings[card["setting"]].name
    want = registry.want_vocabulary.get(card.get("character_want", ""), "")
    outcome = card.get("want_outcome", "").upper()
    stakes = card.get("stakes_domain", "")
    atmosphere = card.get("atmosphere", "")
    role = card.get("causal_role", "").upper()
    
    parts = [f"{pov} {verb_phrase} at {location}"]
    
    if want:
        parts.append(f"wants {want} [{outcome}]")
    
    meta = []
    if stakes:
        meta.append(f"stakes={stakes}")
    if atmosphere:
        meta.append(f"atmosphere={atmosphere}")
    if role:
        meta.append(f"role={role}")
    
    if meta:
        parts.append(", ".join(meta))
    
    return "; ".join(parts) + "."
```

### Parse Function

```python
import re

def parse_summary(summary: str, registry: Registry) -> dict:
    # Build reverse lookups
    name_to_char = {c.name: slug for slug, c in registry.characters.items()}
    name_to_setting = {s.name: slug for slug, s in registry.settings.items()}
    want_to_slug = {v: k for k, v in registry.want_vocabulary.items()}
    verb_to_event = {v: k for k, v in EVENT_VERBS.items()}
    
    # Parse with regex
    pattern = r"^(.+?) (\w+s?) (.+?) at (.+?); wants (.+?) \[(\w+)\]; (.+)\.$"
    match = re.match(pattern, summary)
    
    if not match:
        raise ValueError(f"Cannot parse: {summary}")
    
    pov_name, verb, participants_str, location, want, outcome, meta = match.groups()
    
    return {
        "pov_character": name_to_char[pov_name],
        "event_type": verb_to_event[verb],
        "participants": [name_to_char[p.strip()] for p in participants_str.split(",") if p.strip()],
        "setting": name_to_setting[location],
        "character_want": want_to_slug[want],
        "want_outcome": outcome.lower(),
        "stakes_domain": extract_meta(meta, "stakes"),
        "atmosphere": extract_meta(meta, "atmosphere"),
        "causal_role": extract_meta(meta, "role").lower()
    }
```

---

## Validation Gate

Training data must pass the round-trip test:

```python
def validate_scene_card(card: dict, registry: Registry) -> bool:
    """Validate that canonical_summary round-trips correctly."""
    summary_data = card.get("canonical_summary", {})
    
    # Render to string
    rendered = render_summary(summary_data, registry)
    
    # Parse back
    parsed = parse_summary(rendered, registry)
    
    # Verify all fields match
    for key in summary_data:
        if summary_data[key] != parsed.get(key):
            return False
    
    return True
```

### Rejection Criteria

A scene card is rejected if:
1. Any slug doesn't resolve in registry
2. Any enum value is invalid
3. Round-trip produces different result
4. Summary cannot be parsed

---

## Why This Matters

### For Training

| Direction | Input | Output | Ambiguity |
|-----------|-------|--------|-----------|
| **Decompose** | Prose passage | Semantic dict | None — dict is typed |
| **Reconstruct** | Canonical summary + craft | Prose passage | None — summary is exact |
| **Round-trip** | Prose → summary → prose | Verifiable at semantic level | Zero by design |

The model learns a **bijection**, not a fuzzy approximation.

### For Tools

Tools can rely on:
- Scene cards always being reconstructible from prose
- Canonical summaries being deterministic
- All entity references being valid

---

## Fabula/Syuzhet Separation

Following Tomashevsky and the Russian Formalists:

| Layer | Term | Definition | Protocol Representation |
|-------|------|------------|------------------------|
| **What happens** | Fabula | Chronological events | Canonical Summary |
| **How it's told** | Syuzhet | Artistic rendering | Prose + Craft Layer |

The canonical summary captures **fabula** — structural facts. The prose expresses **syuzhet** — craft choices that render facts into literature.

Both must be **losslessly recoverable** from each other.

---

## References

- Tomashevsky, Boris. "Thematics." *Russian Formalist Criticism*, 1965.
- Chatman, Seymour. *Story and Discourse*, 1978.
- Genette, Gérard. *Narrative Discourse*, 1980.
