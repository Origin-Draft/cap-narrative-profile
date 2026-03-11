# Semantic Interaction Protocol — Specification v0.1.0

**Status:** Draft  
**Date:** 2026-03-11  
**Supersedes:** —  
**Profile Foundation For:** GBR Narrative Profile v1.0.0

---

## Table of Contents

1. [Overview](#1-overview)
2. [Terminology](#2-terminology)
3. [Core Principles](#3-core-principles)
4. [Data Model](#4-data-model)
5. [Core Objects](#5-core-objects)
6. [Profile Extension Mechanism](#6-profile-extension-mechanism)
7. [Serialization](#7-serialization)
8. [Validation Rules](#8-validation-rules)
9. [Conformance](#9-conformance)
10. [Adapter Interfaces](#10-adapter-interfaces)
11. [Versioning](#11-versioning)
12. [Security Considerations](#12-security-considerations)

Appendix A: [Core Enum Values](#appendix-a-core-enum-values)  
Appendix B: [Profile Registration](#appendix-b-profile-registration)  
Appendix C: [Relationship to GBR Protocol](#appendix-c-relationship-to-gbr-protocol)

---

## 1. Overview

### 1.1 Purpose

The Semantic Interaction Protocol (SIP) defines a domain-agnostic, machine-readable representation for decomposing complex artifacts into canonical structures that separate observable facts from structural organization from interpretive meaning.

SIP supports:

- Structured decomposition of any artifact type (narrative texts, software systems, legal documents, architectural designs)
- Layered epistemic separation: what is observed, how it is organized, what is inferred
- Lossless round-trip verification at each epistemic layer independently
- Domain-specific richness through a profile extension mechanism
- Multiple projections (views) over a single canonical representation

### 1.2 Core Guarantee

The protocol's central guarantee is the **layered round-trip**:

```
parse(render(canonical_structure, layer), layer) == canonical_structure
```

For each epistemic layer (observables, structure, interpretations), the round-trip invariant holds independently. A transformation may preserve observables perfectly while diverging on interpretation — and that is a meaningful, measurable distinction.

### 1.3 Scope

This specification defines the SIP core data model, core object schemas, profile extension mechanism, validation rules, conformance levels, and adapter interface contracts.

This specification does **not** define:

- Domain-specific vocabularies (entity types, event types, etc.) — these are defined by profiles
- Ingestion algorithms or generation methodology
- Domain-specific tooling or workflows
- Research evaluation frameworks

### 1.4 Normative Language

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 2. Terminology

**Adapter**  
A component that transforms between a domain-specific source artifact and the canonical SIP representation. The protocol defines four adapter interfaces (§10): Ingest, Render, Diff, Validate.

**Artifact**  
The top-level container in SIP: a complete canonical decomposition of one source work. A novel, a microservice system, a legal contract — each becomes one Artifact (§5.1).

**Canonical Representation**  
The structured decomposition from which the source can be reconstructed and against which reconstruction can be verified. Distinct from the source artifact's surface form.

**Conformance Level**  
The tier of protocol compliance a system claims. SIP defines three cumulative levels: Schema (L1), Referential (L2), Round-Trip (L3). See §9.

**Entity**  
A persistent participant or object of interest that exists across multiple Units. Declared once per Artifact and referenced by slug elsewhere (§5.2).

**Epistemic Layer**  
One of three mandatory categories for organizing fields: `observable` (facts grounded in the artifact), `structural` (how observables are organized), `interpretive` (inferred meaning with provenance). See §4.2.

**Epistemic Section**  
A named top-level key within a core object corresponding to an epistemic layer: `observables`, `structure`, `interpretations`. Profiles MAY define additional sections (§6.3).

**Information State**  
A sub-object tracking what an entity knows, does not know, and learns within a Unit (§5.9.1).

**Interpreted Value**  
The optional metadata wrapper for interpretation fields: either a plain value or `{ "value": <T>, "confidence": 0.0–1.0, "source": "<string>" }`. Observable fields MUST NOT use this wrapper (§4.4).

**Interpretation**  
A structured inference attached to any object in the protocol, carrying provenance and optional confidence (§5.8).

**Participant State**  
A per-entity state snapshot within a specific Unit, tracking how an entity enters, acts within, and exits the unit (§5.9).

**Profile**  
A domain-specific extension that registers allowed types, required fields, additional epistemic sections, semantic fingerprint grammars, and validation rules on top of the core protocol (§6).

**Relationship**  
A typed, directed link between two entities, two units, or an entity and a unit (§5.4).

**Semantic Fingerprint**  
A machine-verifiable serialization of a Unit's semantic content. The concept belongs to the core; the grammar (rendering rules and parsing contract) is profile-defined (§6.5).

**Slug**  
A stable, unique identifier in snake_case format matching `^[a-z0-9_]+$`. Used for all inter-object references within an Artifact.

**State**  
A named condition attached to an entity, group, or artifact at a specific point (§5.5).

**Step**  
An ordered atomic action within a Unit — the sub-unit decomposition (§5.3.1).

**Transition**  
The change from one state to another, representing the transformation that occurs within or across Units (§5.6).

**Type Registry**  
A profile-declared vocabulary of allowed values for a typed field (entity types, unit types, relationship types, etc.). See §6.2.

**Unit**  
An atomic transformation or interaction block — the smallest meaningful chunk of the artifact that contains a complete interaction cycle. Units correspond to "scenes" in narrative, "interactions" in software, "clauses" in legal documents, etc. (§5.3).

**View**  
A named projection over the canonical artifact — a way of reading the same data for a specific purpose (§5.7).

---

## 3. Core Principles

### Principle 1: Canonical representation is separate from source rendering

The protocol represents the *structure and meaning* of an artifact, not its surface form. Prose is not the canonical form of a novel. Source code is not the canonical form of a system. The canonical representation is a structured decomposition from which the source can be reconstructed and against which reconstruction can be verified.

### Principle 2: Observable structure is separate from interpretation

Every object in the protocol distinguishes between:

- **Observables** — facts grounded directly in the artifact, verifiable by inspection
- **Structure** — how observables are organized, derivable from the artifact
- **Interpretations** — inferred meaning layered on top, carrying provenance and confidence

This three-layer epistemic model is mandatory for all core objects that carry semantic content. Domain profiles MAY define additional epistemic sections (e.g., prescriptive intent).

### Principle 3: Core protocol is domain-agnostic

No core object, field name, or enum value references a specific domain. Terms like "character," "scene," "chapter," "function," "module," or "endpoint" never appear in the core specification. Domain-specific concepts live exclusively in profiles.

### Principle 4: Domain richness lives in profiles, not the core

The core is deliberately minimal. All domain-specific entity types, unit types, relationship types, interpretation types, enum values, validation rules, and rendering grammars are defined by domain profiles. The core provides the extension mechanism; profiles provide the vocabulary.

### Principle 5: Multiple views may be derived from one canonical artifact

A single canonical representation supports multiple projections: chronological view, dependency graph, entity trajectory, deployment view. Views are first-class objects (§5.7), not ad-hoc queries. Profiles define which views are canonical for their domain.

### Principle 6: Round-trip comparison occurs by layer

Fidelity is measured separately at each epistemic layer:

- **Observable fidelity** — are the same facts present?
- **Structural fidelity** — is the organization preserved?
- **Interpretive fidelity** — do the inferences align?

This prevents "lossless" from becoming a single vague claim. A transformation may preserve observables perfectly while diverging on interpretation — and that is a meaningful, measurable distinction.

---

## 4. Data Model

### 4.1 Object Types

SIP defines eight core object types and two sub-object types:

| Object | Role | Required in Artifact |
|--------|------|---------------------|
| **Artifact** | Top-level container | —  (IS the container) |
| **Entity** | Persistent participant | yes (≥ 1) |
| **Unit** | Atomic transformation block | yes (≥ 1) |
| **Relationship** | Typed directed link | no |
| **State** | Point-in-time condition | no (inline in Transition) |
| **Transition** | Value change (before → after) | no (inline in Unit) |
| **View** | Named projection | no |
| **Interpretation** | Structured inference | no |

Sub-objects (nested, not standalone):

| Sub-Object | Parent | Role |
|------------|--------|------|
| **Step** | Unit | Ordered sub-action |
| **InformationState** | ParticipantState | Epistemic tracking |

Additionally, **ParticipantState** is a per-entity state snapshot embedded within a Unit (§5.9).

### 4.2 Epistemic Sections

Every Unit and every Entity organizes its fields into three core epistemic sections:

| Section | Key | Content | Confidence Model |
|---------|-----|---------|-----------------|
| **Observables** | `observables` | Facts grounded directly in the artifact: named entities, quoted content, explicit markers, visible actions | Always certain; MUST NOT use `interpreted_value` wrapper |
| **Structure** | `structure` | How observables are organized: sequence, containment, adjacency, state transitions, dependency and causal links, groupings | Certain when derived from observables; may be interpretive when inferred |
| **Interpretations** | `interpretations` | Inferred meaning: motivations, classifications, themes, patterns, architectural intent | MAY carry `interpreted_value` wrapper with confidence and source |

**Rules:**

1. Units MUST contain `observables` and `structure` sections. The `interpretations` section is RECOMMENDED.
2. Entities SHOULD organize their properties into `observable_descriptors`, `structural_properties`, and `interpretations`.
3. Profiles MAY define additional epistemic sections beyond the core three (§6.3). Additional sections MUST document their confidence model.

### 4.3 Reference Model

All inter-object references within an Artifact use **slugs**: stable identifiers matching `^[a-z0-9_]+$`.

**Rules:**

1. All slugs MUST be unique within their object type per artifact (entity slugs are unique among entities; unit slugs are unique among units).
2. All references MUST resolve within the same Artifact. Cross-artifact references are not defined in SIP v0.1.0.
3. An unresolved reference is a Level 2 conformance failure (§9).

### 4.4 The Interpreted Value Wrapper

Interpretation fields MAY use a structured wrapper to carry provenance:

```json
{
  "value": "<the interpretive claim — any JSON type>",
  "confidence": 0.85,
  "source": "human"
}
```

**Rules:**

1. The wrapper is valid wherever an interpretation field appears.
2. A bare value (without the wrapper) in an `interpretations` section is always valid — the wrapper is opt-in.
3. When `confidence` is provided, it MUST be a float in the range `[0.0, 1.0]`.
4. When `source` is provided, it SHOULD indicate the origin of the interpretation. Common values: `human`, `model`, `inferred`, `consensus`. Extended source strings (e.g., `model:gpt-4`, `human:editor`) are permitted.
5. If `source` is a model identifier, `confidence` SHOULD be present.
6. Observable fields MUST NOT use this wrapper. They are grounded facts.

---

## 5. Core Objects

### 5.1 Artifact

The top-level container: a complete canonical decomposition of one source work.

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `protocol` | string | **yes** | MUST be `"semantic-interaction-protocol"` |
| `protocol_version` | semver string | **yes** | SIP version this artifact conforms to |
| `profile` | string | **yes** | Domain profile identifier (e.g., `"narrative"`, `"software"`) |
| `profile_version` | semver string | **yes** | Profile version |
| `artifact_id` | slug | **yes** | Unique identifier for this artifact |
| `metadata` | object | no | Domain-agnostic metadata (title, owner, size, creation date) |
| `entities` | Entity[] | **yes** | All persistent entities declared in this artifact |
| `units` | Unit[] | **yes** | Ordered decomposition into atomic units |
| `relationships` | Relationship[] | no | Cross-entity and cross-unit links |
| `views` | View[] | no | Named projections over the canonical data |
| `interpretations` | object[] or object | no | Artifact-level inferences (themes, architectural style, etc.) |

#### Validation Rules

1. `artifact_id` MUST be a non-empty slug matching `^[a-z0-9_]+$`.
2. `protocol` MUST be exactly `"semantic-interaction-protocol"`.
3. `entities` MUST contain at least one Entity.
4. `units` MUST contain at least one Unit.
5. Every Entity, Unit, Relationship, and View within an Artifact MUST reference only entities and units declared within the same Artifact.
6. `profile` MUST resolve to a registered domain profile (§6).

#### Schema

`schemas/artifact.schema.json`

#### Examples

**Narrative:**
```json
{
  "protocol": "semantic-interaction-protocol",
  "protocol_version": "0.1.0",
  "profile": "narrative",
  "profile_version": "1.0.0",
  "artifact_id": "threshold",
  "metadata": { "title": "Threshold", "author": "Example Author", "size": 45000 }
}
```

**Software:**
```json
{
  "protocol": "semantic-interaction-protocol",
  "protocol_version": "0.1.0",
  "profile": "software",
  "profile_version": "1.0.0",
  "artifact_id": "auth_system",
  "metadata": { "title": "Authentication Subsystem", "owner": "platform-team", "size": 12400 }
}
```

---

### 5.2 Entity

A persistent participant or object of interest that exists across multiple Units.

Entities are declared once in the Artifact and referenced by slug elsewhere. Their artifact-level properties live in the entity declaration; their per-unit state lives in `participant_states` within each Unit.

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entity_id` | slug | **yes** | Unique within artifact |
| `entity_type` | string | **yes** | Profile-defined type |
| `display_name` | string | **yes** | Human-readable name |
| `observable_descriptors` | object | no | Profile-defined observable properties |
| `structural_properties` | object | no | Profile-defined structural properties |
| `interpretations` | object | no | Profile-defined interpretive properties |

#### Validation Rules

1. `entity_id` MUST be unique within the artifact.
2. `entity_type` MUST be a value registered by the active profile's entity type registry (§6.2).
3. All entity references (`entity_ref`) elsewhere in the artifact MUST resolve to a declared entity.

#### Schema

`schemas/entity.schema.json`

#### Examples

**Narrative — Character:**
```json
{
  "entity_id": "nadia",
  "entity_type": "character",
  "display_name": "Nadia Vance",
  "observable_descriptors": { "slot": "protagonist" },
  "structural_properties": { "role": "protagonist" },
  "interpretations": { "archetype": "explorer", "wound": "grief", "arc_type": "positive_change" }
}
```

**Software — Service:**
```json
{
  "entity_id": "auth_service",
  "entity_type": "service",
  "display_name": "Authentication Service",
  "observable_descriptors": { "language": "go", "entry_point": "cmd/auth/main.go" },
  "structural_properties": { "layer": "infrastructure", "protocol": "grpc" },
  "interpretations": { "responsibility": "identity_verification", "pattern": "domain_service" }
}
```

---

### 5.3 Unit

An atomic transformation or interaction block — the smallest meaningful chunk of the artifact that contains a complete interaction cycle (pre-state → action → post-state).

Units are the atomic level of the protocol. The `unit_type` is profile-defined; profile implementations map domain concepts (scenes, interactions, clauses) to Units.

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `unit_id` | slug | **yes** | Unique within artifact |
| `artifact_id` | slug | **yes** | Back-reference to parent artifact |
| `unit_type` | string | no | Profile-defined type |
| `sequence_index` | integer ≥ 1 | **yes** | Position in artifact's primary ordering |

**Observables section** (`observables`):

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `participants` | entity_ref[] | **yes** | Entities involved in this unit |
| `context` | object | no | Profile-defined contextual observables |
| `event_type` | string | no | Profile-defined event classification |
| `source_text` | string | no | Raw artifact text or content this unit was derived from |

**Structure section** (`structure`):

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `position` | float 0.0–1.0 | no | Normalized position in artifact's primary sequence |
| `causal_role` | string | no | Function in causal chain. Core values: `setup`, `trigger`, `complication`, `resolution`. Profiles MAY extend. |
| `grouping` | object | no | Profile-defined grouping metadata |
| `steps` | Step[] | no | Ordered sub-unit decomposition (§5.3.1) |
| `transition` | Transition | no | Value change within this unit (§5.6) |
| `semantic_fingerprint` | object or string | no | Machine-verifiable semantic summary. Profile defines grammar (§6.5). |

**Interpretations section** (`interpretations`):

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| *(profile-defined)* | any | no | All unit-level interpretation fields are profile-defined |

**Other sections:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `participant_states` | ParticipantState[] | no | Per-entity state snapshots (§5.9) |
| `metadata` | object | no | Size, word count, LOC, etc. |

#### 5.3.1 Step

An ordered atomic action within a Unit — the sub-unit decomposition.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sequence_number` | integer ≥ 1 | **yes** | Order within the unit |
| `agent` | entity_ref | **yes** | Entity performing the action |
| `action` | string | **yes** | Verb or action identifier |
| `target` | string | no | What the action is directed at (entity ref, resource path, or free text) |
| `event_type` | string | no | Profile-defined event classification |
| `significance` | enum | no | `essential` or `supplementary` |
| `interpretations` | object | no | Profile-defined step-level interpretations |

#### Validation Rules

1. `unit_id` MUST be unique within the artifact.
2. `artifact_id` MUST match the parent Artifact's `artifact_id`.
3. Every entity_ref in `observables.participants` MUST resolve to a declared entity.
4. `structure.steps`, when present, MUST be ordered by `sequence_number` with no duplicates.
5. If `structure.semantic_fingerprint` is present, the active profile MUST define a `render`/`parse` contract for round-trip verification (§6.5).
6. `structure.causal_role`, when present, MUST be one of the core values (`setup`, `trigger`, `complication`, `resolution`) or a value registered by the active profile.

#### Schema

`schemas/unit.schema.json`, `schemas/step.schema.json`

#### Examples

**Narrative — Scene:**
```json
{
  "unit_id": "threshold_ch01_s01",
  "artifact_id": "threshold",
  "unit_type": "scene",
  "sequence_index": 1,
  "observables": {
    "participants": ["nadia"],
    "context": { "setting": "childhood_home", "time_of_day": "morning" }
  },
  "structure": {
    "position": 0.0,
    "causal_role": "setup",
    "transition": {
      "description": "Nadia's armor — the fiction that this is a logistical task — took the first small crack."
    }
  }
}
```

**Software — Interaction:**
```json
{
  "unit_id": "auth_login_flow",
  "artifact_id": "auth_system",
  "unit_type": "interaction",
  "sequence_index": 1,
  "observables": {
    "participants": ["api_gateway", "auth_service", "user_db"],
    "context": { "protocol": "https", "endpoint": "/api/v1/login" }
  },
  "structure": {
    "causal_role": "trigger",
    "transition": {
      "description": "User transitions from unauthenticated to authenticated with a session token."
    }
  }
}
```

---

### 5.4 Relationship

A typed, directed link between two entities, two units, or an entity and a unit.

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source` | ref (slug) | **yes** | Entity or unit ID |
| `target` | ref (slug) | **yes** | Entity or unit ID |
| `relationship_type` | string | **yes** | Profile-defined type |
| `evidence` | string | no | Observable basis for the relationship |
| `interpretations` | object | no | Profile-defined interpretive properties |

#### Validation Rules

1. `source` and `target` MUST resolve to declared entities or units within the artifact.
2. `relationship_type` MUST be a value registered by the active profile's relationship type registry.

#### Schema

`schemas/relationship.schema.json`

#### Examples

**Narrative:**
```json
{
  "source": "nadia",
  "target": "father",
  "relationship_type": "family_parent_child",
  "evidence": "Confirmed by prose references to 'her father's house'",
  "interpretations": { "dynamic": "distant", "power_balance": "target_dominant" }
}
```

**Software:**
```json
{
  "source": "api_gateway",
  "target": "auth_service",
  "relationship_type": "dependency",
  "evidence": "Import in gateway/auth_client.go",
  "interpretations": { "coupling": "tight", "criticality": "high" }
}
```

---

### 5.5 State

A named condition attached to an entity, group, or the artifact as a whole, at a specific point.

States appear inline within Transitions (as `before`/`after`) and within ParticipantStates.

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subject` | ref (slug) | no | What this state describes (entity or unit ref). Required when standalone. |
| `state_type` | string | **yes** | Profile-defined state category |
| `value` | any | **yes** | The state value (type depends on `state_type`) |
| `evidence` | string | no | Observable basis |
| `provenance` | string | no | Origin: `human`, `model`, `inferred`, `consensus`, or arbitrary source string |
| `confidence` | float 0.0–1.0 | no | Certainty of this state assignment |

#### Validation Rules

1. `state_type` MUST be a value registered by the active profile's state type registry.
2. When `confidence` is provided, it MUST be in the range `[0.0, 1.0]`.

#### Schema

`schemas/state.schema.json`

#### Examples

**Narrative:**
```json
{
  "subject": "nadia",
  "state_type": "emotional",
  "value": "grief",
  "evidence": "Narrator describes 'four months of postponement'",
  "provenance": "human",
  "confidence": 0.9
}
```

**Software:**
```json
{
  "subject": "auth_service",
  "state_type": "health",
  "value": "degraded",
  "evidence": "Error rate > 5% in last 5 minutes",
  "provenance": "inferred",
  "confidence": 0.95
}
```

---

### 5.6 Transition

The change from one state to another, representing the transformation that occurs within or across Units.

Transitions appear inline within `unit.structure.transition`.

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subject` | ref (slug) | no | Entity or unit undergoing the transition |
| `before` | State | no | Pre-transition state |
| `after` | State | no | Post-transition state |
| `trigger` | string | no | What caused the change |
| `description` | string | **yes** | Human-readable statement of what changed (the "delta") |
| `confidence` | float 0.0–1.0 | no | Certainty that this transition occurred |
| `grounding` | string | no | Evidence from the source artifact |

#### Validation Rules

1. `description` MUST state a *change*, not a *condition*. "X happened" alone is insufficient; "X changed from A to B" or "X was established where previously absent" expresses the transformation.
2. When `before` and `after` are both present, their `state_type` fields SHOULD match.

#### Schema

`schemas/transition.schema.json`

#### Examples

**Narrative:**
```json
{
  "subject": "nadia",
  "before": { "state_type": "defense", "value": "intact" },
  "after": { "state_type": "defense", "value": "cracked" },
  "trigger": "The house is smaller than she remembered",
  "description": "Nadia's armor — the fiction that this is a logistical task — took the first small crack.",
  "grounding": "She stops in the kitchen doorway without knowing why."
}
```

**Software:**
```json
{
  "subject": "user_session",
  "before": { "state_type": "authentication", "value": "unauthenticated" },
  "after": { "state_type": "authentication", "value": "authenticated" },
  "trigger": "Valid credentials submitted to /api/v1/login",
  "description": "User transitioned from unauthenticated to authenticated with a 30-minute session token."
}
```

---

### 5.7 View

A named projection over the canonical artifact — a way of reading the same data for a specific purpose.

Views do not add information; they select and organize existing information for a particular lens. One Artifact may have many Views.

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `view_id` | slug | **yes** | Unique within artifact |
| `view_type` | string | **yes** | Profile-defined view type |
| `description` | string | no | What this view shows |
| `scope` | ref[] | no | Which entities/units are included (default: all) |
| `ordering` | string | no | How items are ordered (e.g., `chronological`, `dependency`, `causal`) |
| `data` | object | no | View-specific computed data |

#### Validation Rules

1. `view_id` MUST be unique within the artifact.
2. All references in `scope` MUST resolve to declared entities or units.

#### Schema

`schemas/view.schema.json`

#### Examples

**Narrative — Entity Trajectory:**
```json
{
  "view_id": "nadia_arc",
  "view_type": "entity_trajectory",
  "description": "Nadia's emotional and psychological arc across all scenes",
  "scope": ["nadia"],
  "ordering": "chronological",
  "data": {
    "trajectory_points": [
      { "unit": "threshold_ch01_s01", "state": "armor_intact", "direction": "stable" },
      { "unit": "threshold_ch01_s02", "state": "armor_cracking", "direction": "advancing" }
    ]
  }
}
```

**Software — Dependency Graph:**
```json
{
  "view_id": "service_dependencies",
  "view_type": "dependency_graph",
  "description": "Runtime service dependency graph",
  "ordering": "dependency",
  "data": {
    "edges": [
      { "source": "api_gateway", "target": "auth_service", "type": "synchronous" },
      { "source": "auth_service", "target": "user_db", "type": "synchronous" }
    ]
  }
}
```

---

### 5.8 Interpretation

A structured inference attached to any object in the protocol. Interpretations are always logically separate from observables and always carry provenance.

Interpretations may appear in two forms:

1. **Inline** — as additional properties within an object's `interpretations` section
2. **Standalone** — as top-level Interpretation objects in an Artifact's `interpretations` array

Standalone Interpretations are useful for artifact-level inferences and cross-cutting interpretive claims.

#### Fields (Standalone Form)

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target_ref` | ref (slug) | **yes** | What this interpretation is about (entity, unit, relationship, or artifact ref) |
| `interpretation_type` | string | **yes** | Profile-defined type |
| `value` | any | **yes** | The interpretive claim |
| `confidence` | float 0.0–1.0 | no | Certainty. Defaults to 1.0 if omitted. |
| `rationale` | string | no | Why this interpretation was made |
| `evidence_refs` | ref[] | no | References to observable data supporting this interpretation |
| `source` | string | no | Provenance: `human`, `model`, `inferred`, `consensus`, or extended string |

#### Validation Rules

1. `target_ref` MUST resolve to a declared object in the artifact.
2. `interpretation_type` MUST be registered by the active profile's interpretation type registry.
3. `confidence`, when present, MUST be in the range `[0.0, 1.0]`.

#### Schema

`schemas/interpretation.schema.json`

#### Examples

**Narrative:**
```json
{
  "target_ref": "nadia",
  "interpretation_type": "motivation",
  "value": "Avoidance of grief through compulsive forward motion",
  "confidence": 0.85,
  "rationale": "Nadia's cataloguing behavior and clinical tone mask emotional processing",
  "evidence_refs": ["threshold_ch01_s01"],
  "source": "human"
}
```

**Software:**
```json
{
  "target_ref": "auth_service",
  "interpretation_type": "architectural_intent",
  "value": "Single responsibility: all authentication logic centralized",
  "confidence": 0.9,
  "evidence_refs": ["auth_login_flow"],
  "source": "inferred"
}
```

---

### 5.9 ParticipantState

Per-entity state snapshot within a specific Unit. Tracks how an entity enters, acts within, and exits a unit.

ParticipantStates appear in the `participant_states` array of a Unit.

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entity_ref` | slug | **yes** | Which entity |
| `role_in_unit` | string | no | Profile-defined role (e.g., `focalizer`, `initiator`, `responder`) |
| `pre_state` | object | no | Entity state at unit entry (profile-defined) |
| `post_state` | object | no | Entity state at unit exit (profile-defined) |
| `objective` | object | no | What this entity is trying to achieve |
| `objective.action` | string | no | Goal verb or action |
| `objective.target` | ref or string | no | Goal target |
| `obstacle` | object | no | What blocks the objective |
| `information_state` | InformationState | no | Knowledge tracking (§5.9.1) |
| `observables` | object | no | Profile-defined observable properties |
| `interpretations` | object | no | Profile-defined interpretive properties |

#### 5.9.1 InformationState

Tracks what an entity knows, does not know, and learns within a Unit. This is a core concept — information asymmetry tracking generalizes across domains (narrative dramatic irony, software service awareness, legal party knowledge).

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `knows` | InformationItem[] | no | Facts known at entry |
| `gaps` | InformationItem[] | no | Facts not known at entry |
| `gained` | InformationItem[] | no | Facts learned during unit |

**InformationItem:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subject` | string | **yes** | What the information is about (profile-defined domain) |
| `predicate` | string | **yes** | Epistemic relation (profile-defined: `knows`, `believes`, `suspects`, `assumes`) |
| `about` | ref or string | **yes** | Entity or concept the information concerns |
| `certainty` | float 0.0–1.0 | no | How certain the knowledge is |

#### Validation Rules

1. `entity_ref` MUST resolve to a declared entity.
2. `information_state` items, when present, SHOULD have `certainty` values that are consistent across `knows` (high), `gaps` (low), and `gained` (increasing).

#### Schema

`schemas/participant-state.schema.json`, `schemas/information-state.schema.json`

---

## 6. Profile Extension Mechanism

A domain profile extends the core protocol by declaring vocabularies, constraints, and contracts. The core is the skeleton; the profile is the flesh.

### 6.1 Profile Declaration

Every SIP artifact declares its profile:

```json
{
  "protocol": "semantic-interaction-protocol",
  "protocol_version": "0.1.0",
  "profile": "narrative",
  "profile_version": "1.0.0"
}
```

**Rules:**

1. `profile` MUST be a non-empty string identifying a registered profile.
2. `profile_version` MUST follow Semantic Versioning 2.0.0.
3. A single artifact MUST declare exactly one profile.

### 6.2 Type Registries

Each profile MUST register allowed values for the following typed fields:

| Registry | Core Field | Example (Narrative) | Example (Software) |
|----------|-----------|---------------------|---------------------|
| Entity Types | `entity.entity_type` | `character`, `location`, `object`, `collective` | `service`, `data_store`, `module`, `interface` |
| Unit Types | `unit.unit_type` | `scene`, `chapter_summary`, `interlude` | `interaction`, `process`, `transaction` |
| Relationship Types | `relationship.relationship_type` | `family_parent_child`, `romantic`, `mentor`, `rival` | `dependency`, `api_consumer`, `data_flow` |
| Interpretation Types | `interpretation.interpretation_type` | `motivation`, `emotion`, `theme`, `subtext` | `responsibility`, `pattern`, `trust_boundary` |
| Event Types | `unit.observables.event_type`, `step.event_type` | `action`, `dialogue`, `internal_shift`, `revelation` | `request`, `response`, `query`, `mutation`, `error` |
| State Types | `state.state_type` | `emotional`, `relational`, `psychological` | `health`, `authentication`, `capacity` |
| Causal Roles | `unit.structure.causal_role` | `catalyst`, `escalation`, `crisis` (extends core) | `initialization`, `retry`, `fallback` (extends core) |

**Rules:**

1. Profiles MUST document all registered values for each registry.
2. The core defines baseline `causal_role` values (`setup`, `trigger`, `complication`, `resolution`). Profiles extend this list; they MUST NOT redefine the semantics of core values.
3. Other registries have no core-defined values — the profile owns the vocabulary entirely.

### 6.3 Additional Epistemic Sections

Profiles MAY define epistemic sections beyond the core three (observables, structure, interpretations). Each additional section MUST document:

- **Name** — the JSON key used in the section
- **Purpose** — what kind of content belongs here
- **Confidence model** — whether fields use the `interpreted_value` wrapper, are always certain, or follow a custom model
- **Validation rules** — any constraints specific to this section

**Example:** The narrative profile defines `craft_targets` (prescriptive authorial intent — target tension, pacing, tone). Craft target fields are always intentional and do not carry the `interpreted_value` wrapper.

### 6.4 Additional Required Fields

Profiles MAY declare that specific fields are required on core objects, conditionally or unconditionally:

```yaml
entity:
  when_type: character
  required:
    - structural_properties.role
unit:
  when_type: scene
  required:
    - observables.context.focalizer
    - structure.beat
```

Profile-required fields are validated at Level 1 (Schema) conformance when the profile is active.

### 6.5 Semantic Fingerprint Contract

Each profile MUST define a `render` and `parse` contract for `unit.structure.semantic_fingerprint`:

```
render(unit, entities) → string_or_object
parse(string_or_object, entities) → unit_fragment
```

**Round-trip invariant:** `parse(render(u, e), e) == u` for the fields covered by the fingerprint.

The core protocol defines the concept of a semantic fingerprint and the round-trip requirement. The grammar — what slots exist, what values are valid, how the string is structured — is entirely profile-defined.

### 6.6 Enum Governance

Profile enums are versioned independently from the core protocol:

| Change Type | Version Component |
|-------------|------------------|
| Adding a new enum value | Minor |
| Removing or renaming an enum value | Major |
| Changing descriptions without changing values | Patch |

Core protocol enums (§Appendix A) follow the core protocol's versioning. Profile enum changes do NOT trigger core version bumps.

### 6.7 Domain-Specific Validation

Profiles MAY declare additional validation rules that participate in the conformance level system:

- **Level 1 additions:** Profile-specific schema constraints (additional required fields, enum restrictions)
- **Level 2 additions:** Profile-specific cross-reference rules (e.g., narrative requires focalizer to exist in entity registry)
- **Level 3 additions:** Profile-specific semantic constraints (e.g., semantic fingerprint grammar validation)

---

## 7. Serialization

### 7.1 Primary Format

SIP artifacts MUST be serialized as [JSON](https://www.json.org/) (ECMA-404 / RFC 8259) unless an alternative format is explicitly negotiated.

**Rules:**

1. Documents MUST use UTF-8 encoding.
2. Documents SHOULD use human-readable indentation (2 or 4 spaces).
3. Field ordering within objects is not significant.
4. Field names MUST use `snake_case`.
5. `null` values for optional fields MAY be omitted entirely; omitting an optional field is equivalent to its absence.

### 7.2 Corpus Layout

A SIP corpus is a directory containing:

```
{artifact_id}/
  artifact.json          # Artifact metadata + entity declarations
  units/
    {unit_id}.json       # One file per unit
  views/
    {view_id}.json       # One file per view (optional)
```

Profiles MAY define alternative layouts provided that the canonical data is equivalent. The directory layout is RECOMMENDED but not REQUIRED — a single-file representation containing the entire Artifact is valid.

### 7.3 References

All inter-object references use slugs matching `^[a-z0-9_]+$`. References are local to the artifact — no cross-artifact references in SIP v0.1.0.

### 7.4 Schema Declaration

SIP documents SHOULD include a `$schema` field referencing the applicable JSON Schema URI:

```json
{
  "$schema": "sip/schemas/artifact.schema.json",
  "protocol": "semantic-interaction-protocol"
}
```

---

## 8. Validation Rules

### 8.1 Schema Validation

All SIP artifacts MUST pass JSON Schema validation against the core schemas in `schemas/`:

| Object | Schema File |
|--------|------------|
| Artifact (top-level) | `schemas/artifact.schema.json` |
| Entity | `schemas/entity.schema.json` |
| Unit | `schemas/unit.schema.json` |
| Relationship | `schemas/relationship.schema.json` |
| State | `schemas/state.schema.json` |
| Transition | `schemas/transition.schema.json` |
| View | `schemas/view.schema.json` |
| Interpretation | `schemas/interpretation.schema.json` |
| ParticipantState | `schemas/participant-state.schema.json` |
| Step | `schemas/step.schema.json` |
| InformationState | `schemas/information-state.schema.json` |

### 8.2 Referential Integrity

The following constraints MUST be satisfied for Level 2 conformance:

1. **Entity references:** Every entity slug referenced in any `participants`, `entity_ref`, `source`, `target`, or `scope` field MUST exist in the Artifact's `entities` array.
2. **Unit references:** Every unit slug referenced in Views, Relationships, or Interpretations MUST exist in the Artifact's `units` array.
3. **Artifact ID consistency:** Every Unit's `artifact_id` MUST match the parent Artifact's `artifact_id`.
4. **Uniqueness:** No two entities MAY share the same `entity_id`; no two units MAY share the same `unit_id`; no two views MAY share the same `view_id`.

### 8.3 Epistemic Consistency

1. Observable fields MUST NOT use the `interpreted_value` wrapper.
2. Interpretation fields carrying provenance SHOULD use the `interpreted_value` wrapper or the standalone Interpretation form.
3. Structure fields SHOULD be derivable from observables. When a structural field is interpretive in nature (e.g., inferred causal role), it SHOULD be documented as such in the profile.

### 8.4 Profile Validation

Profile-specific validation rules (§6.7) are additive:

1. Core validation MUST pass before profile validation is attempted.
2. Profile validation failures MUST be reported with the profile identifier in the error context.
3. A core-valid artifact that fails profile validation is Core Schema Conformant (Level 1) but not Profile Schema Conformant.

### 8.5 Validation Severity Levels

| Level | Meaning | Example |
|-------|---------|---------|
| ERROR | Artifact MUST be rejected | Missing required field; unresolved reference |
| WARNING | Artifact may have issues; SHOULD be reviewed | Missing semantic fingerprint; empty interpretations |
| INFO | Informational observation; no action required | Unused entity declaration |

---

## 9. Conformance

### 9.1 Conformance Levels

SIP defines three cumulative conformance levels:

#### Level 1 — Schema Conformant

An artifact is Schema Conformant if:

- It passes JSON Schema validation against the applicable core schemas (§8.1).
- All required fields are present and correctly typed.
- All `protocol` and `protocol_version` fields are valid.
- (When a profile is active) All profile-required fields are present and enum values are declared.

#### Level 2 — Referentially Conformant

An artifact is Referentially Conformant if:

- It satisfies Level 1.
- All entity and unit references resolve within the artifact (§8.2).
- Profile-specific cross-reference rules pass (§6.7).

#### Level 3 — Round-Trip Conformant

An artifact is Round-Trip Conformant if:

- It satisfies Level 2.
- All `semantic_fingerprint` fields are present on units.
- All semantic fingerprints pass the profile-defined `parse(render(x)) == x` invariant.
- Profile-specific semantic constraints pass.

### 9.2 Severity

| Level | Meaning |
|-------|---------|
| ERROR | Artifact MUST be rejected at the claimed conformance level |
| WARNING | Issue detected; SHOULD be reviewed |
| INFO | Informational; no action required |

### 9.3 Conformance Claims

Implementations claiming SIP conformance MUST specify:

1. The conformance level they support (1, 2, or 3).
2. The core protocol version they validate against.
3. The profile(s) they support (if any).

### 9.4 Conformance Test Suite

The `conformance/` directory contains:

- `conformance/valid/` — artifacts that MUST pass all three conformance levels
- `conformance/invalid/` — artifacts with known defects; validators MUST reject each with the expected error

---

## 10. Adapter Interfaces

Adapters transform between domain-specific source artifacts and the canonical SIP representation. The core defines the interface contracts; profiles implement them.

### 10.1 Ingest Adapter

```
ingest(source_artifact, profile, config) → Artifact
```

Transforms a raw source (prose text, codebase, API trace, legal document) into a canonical SIP Artifact.

**Contract:**

- The output MUST be a valid SIP Artifact at Level 1 conformance minimum.
- The adapter SHOULD populate all fields it can confidently extract.
- Fields requiring interpretation SHOULD carry provenance (`source` field indicating the adapter).

### 10.2 Render Adapter

```
render(artifact, view_type, config) → rendered_output
```

Produces a domain-specific rendering from a canonical Artifact (prose, code skeleton, documentation, diagram).

**Contract:**

- The output format is determined by the profile and view type.
- The rendering SHOULD be derivable solely from the canonical representation.
- For Level 3 artifacts, `ingest(render(artifact))` SHOULD approximate the original artifact.

### 10.3 Diff Interface

```
diff(artifact_a, artifact_b, layer) → DiffResult
```

Compares two artifacts at a specific epistemic layer:

- `layer: observables` — are the same facts present?
- `layer: structure` — is the organization preserved?
- `layer: interpretations` — do the inferences align?

**Contract:**

- The diff MUST operate on one layer at a time.
- A DiffResult MUST enumerate additions, deletions, and modifications at the object level.
- Layer-specific diff is the foundation for meaningful round-trip verification.

### 10.4 Validate Interface

```
validate(artifact, level, profile?) → ValidationResult[]
```

Validates an artifact at the specified conformance level.

**Contract:**

- The validator MUST check all rules for the requested level and all lower levels.
- Each ValidationResult MUST include: severity (ERROR/WARNING/INFO), rule identifier, message, and object path.
- When `profile` is specified, profile-specific validation rules are included.

---

## 11. Versioning

SIP follows [Semantic Versioning 2.0.0](https://semver.org/).

### 11.1 Core Protocol Versioning

| Change Type | Version Component | Example |
|-------------|------------------|---------|
| Removing a required field; changing core enum value semantics | Major | `1.0.0` → `2.0.0` |
| Adding a new optional core field; adding a core enum value | Minor | `0.1.0` → `0.2.0` |
| Clarification; typo correction; non-semantic schema fix | Patch | `0.1.0` → `0.1.1` |

The current version is **SIP 0.1.0**.

### 11.2 Profile Versioning

Profiles version independently from the core protocol. A profile version bump does NOT trigger a core version bump.

| Change Type | Version Component |
|-------------|------------------|
| Removing a required field; removing an enum value | Major |
| Adding optional fields; adding enum values | Minor |
| Clarifications; documentation changes | Patch |

### 11.3 Compatibility

- A Profile version MUST declare which core protocol version(s) it is compatible with.
- An artifact authored against core v0.1.0 with narrative profile v1.0.0 may not validate against core v0.2.0 if breaking changes occurred.
- Implementations SHOULD support validation against multiple core versions.

---

## 12. Security Considerations

### 12.1 No Executable Content

SIP artifacts are data — they contain no executable code, scripts, or evaluation instructions. Implementations MUST NOT execute any content from SIP artifacts.

### 12.2 Reference Validation

Implementations MUST validate all references against the artifact's declarations before resolving them. Unresolved references MUST produce a validation error and MUST NOT silently succeed or fall back to partial resolution.

### 12.3 Input Size

SIP artifacts MAY be arbitrarily large (a complete novel corpus or large codebase may include hundreds of units). Implementations SHOULD enforce configurable size limits on individual documents and SHOULD stream-parse large corpora rather than loading them entirely into memory.

### 12.4 Personally Identifiable Information

SIP artifacts may contain names, biographical details, and profiles that correspond to real people (in autofiction, biographical narratives, or system logs). Authors and systems handling such artifacts are responsible for compliance with applicable data protection regulations. SIP does not mandate specific PII handling; it notes the risk exists.

### 12.5 Schema Reference Security

If implementations fetch external `$schema` URIs, they MUST use HTTPS and SHOULD pin to a known version. Implementations SHOULD support offline schema validation to avoid network-dependent security boundaries.

### 12.6 Profile Trust

Implementations SHOULD validate that a claimed profile is from a trusted source before applying profile-specific validation rules. Untrusted profiles could define validation rules that mask structural defects.

---

## Appendix A: Core Enum Values

The core protocol defines a minimal set of enum values. All other enum values are profile-defined.

### A.1 Significance

Used in `step.significance`:

| Value | Meaning |
|-------|---------|
| `essential` | Removal would break the unit's causal chain |
| `supplementary` | Provides context but is not causally necessary |

### A.2 Causal Role (Core Values)

Used in `unit.structure.causal_role`. Profiles extend this list.

| Value | Meaning |
|-------|---------|
| `setup` | Establishes conditions for future events |
| `trigger` | Initiates a causal chain |
| `complication` | Adds complexity to an existing chain |
| `resolution` | Resolves an open causal chain |

### A.3 Conformance Level

Used in validation and conformance claims:

| Value | Meaning |
|-------|---------|
| `schema` | Level 1 — Schema Conformant |
| `referential` | Level 2 — Referentially Conformant |
| `round_trip` | Level 3 — Round-Trip Conformant |

### A.4 Validation Severity

| Value | Meaning |
|-------|---------|
| `error` | Artifact MUST be rejected |
| `warning` | SHOULD be reviewed |
| `info` | Informational |

---

## Appendix B: Profile Registration

To register a new SIP profile:

1. **Choose a unique profile identifier** — a lowercase string (e.g., `narrative`, `software`, `legal`).
2. **Declare all seven type registries** (§6.2) with at least one value per registry.
3. **Define the semantic fingerprint grammar** (§6.5) with `render` and `parse` contracts.
4. **Document any additional required fields** (§6.4).
5. **Document any additional epistemic sections** (§6.3).
6. **Write a PROFILE.md** specification that covers all of the above.
7. **Provide at least one conformance-valid example artifact** demonstrating the profile.

Profiles are not required to be registered in any central authority for SIP v0.1.0. Profile interoperability depends on implementations supporting the declared profile.

---

## Appendix C: Relationship to GBR Protocol

The Semantic Interaction Protocol was extracted from the [GBR Protocol](../SPECIFICATION.md) v0.2.0, a narrative annotation protocol for fiction. See [ADR-007](../decisions/ADR-007-sip-extraction.md) for the decision rationale.

The relationship is:

```
SIP Core v0.1.0  ←  extracted from  ←  GBR v0.2.0
GBR v1.0.0       =  SIP Core v0.1.0 + Narrative Profile v1.0.0
```

A systematic audit of GBR's 251 fields found that 29 (12%) are domain-agnostic and form the SIP core, 196 (78%) are narrative-specific and form the narrative profile, and 26 (10%) needed splitting between core concept and profile vocabulary.

Key mappings:

| GBR v0.2.0 | SIP Core | Narrative Profile |
|-------------|----------|-------------------|
| Book / Corpus | Artifact | — |
| Scene Card | Unit (type: `scene`) | scene orchestration |
| Character | Entity (type: `character`) | role, archetype, wound, arc_type |
| Setting | Entity (type: `location`) | atmosphere, sensory_signature |
| Entity Registry | Artifact.entities | character/setting type registries |
| Story Architecture | Artifact.metadata + Views | themes, protagonist_arc, genre |
| Character Scene State | ParticipantState | emotion, tactic, consciousness_mode |
| Canonical Summary | semantic_fingerprint | narrative grammar (POV/WANT/OUTCOME) |
| `craft_targets` | — (profile section) | target_tension, pacing, tone |
| `scene_turns` | Steps | emotional_state, masked_emotion, tactic |
| `kernel`/`satellite` | `essential`/`supplementary` | — |

For the complete field-by-field mapping, see `docs/NARRATIVE_PROFILE_MAPPING.md`.
