# Enumeration Vocabulary Reference

This document provides a categorized reference of all typed enumerations in the GBR Protocol.

**Source of Truth:** 
- JSON Schema: [`schema/enums.schema.json`](../schema/enums.schema.json)
- Rust: [`grimoire-types/src/enums.rs`](../../grimoire-types/src/enums.rs)

---

## Character Enums

### `archetype` (21 variants)
Campbell/Vogler character archetypes:

| Value | Description |
|-------|-------------|
| `hero` | The protagonist who faces the ordeal |
| `mentor` | Wise guide who advises the hero |
| `threshold_guardian` | Tests the hero at boundaries |
| `herald` | Announces change/adventure |
| `shapeshifter` | Ambiguous, changing loyalty |
| `shadow` | Dark reflection/antagonist |
| `ally` | Companion who supports hero |
| `trickster` | Comic relief, chaos agent |
| `everyman` | Relatable ordinary person |
| `lover` | Driven by passion/connection |
| `caregiver` | Protector, nurturer |
| `sage` | Truth-seeker, philosopher |
| `innocent` | Optimist seeking safety |
| `rebel` | Overthrows what isn't working |
| `ruler` | Control, responsibility |
| `outcast` | Rejected by society |
| `creator` | Innovator, artist |
| `explorer` | Seeker of new experiences |
| `magician` | Transforms reality |
| `jester` | Lives in the moment |
| `outlaw` | Rules-breaker |

### `wound` (23 variants)
Core psychological wounds (13 umbrella + 10 fine-grained):

**Umbrella Categories:**
`abandonment`, `betrayal`, `guilt_and_failure`, `trauma_and_abuse`, `shame`, `grief`, `trust_violation`, `powerlessness`, `identity_rejection`, `injustice`, `neglect`, `survivor_guilt`, `displacement`

**Fine-Grained:**
`parental_abandonment`, `romantic_abandonment`, `social_exile`, `institutional_betrayal`, `public_humiliation`, `identity_shame`, `loss_of_purpose`, `catastrophic_failure`, `moral_compromise`, `smothering`

### `alignment` (9 variants)
D&D-style moral alignment grid:

| | Lawful | Neutral | Chaotic |
|---|--------|---------|---------|
| **Good** | `lawful_good` | `neutral_good` | `chaotic_good` |
| **Neutral** | `lawful_neutral` | `true_neutral` | `chaotic_neutral` |
| **Evil** | `lawful_evil` | `neutral_evil` | `chaotic_evil` |

### `role` (17 variants)
Story function roles:

`protagonist`, `deuteragonist`, `antagonist`, `love_interest`, `mentor`, `confidant`, `ally`, `foil`, `catalyst`, `bridge_character`, `ghost_character`, `trickster`, `guardian`, `herald`, `shapeshifter`, `contagonist`, `walk_on`

### `drive_model` (5 variants)
Character motivation frameworks:

| Value | Description |
|-------|-------------|
| `wound` | Weiland/Cron: driven by psychological wound |
| `desire` | Wants-based motivation |
| `duty` | Obligation-based motivation |
| `perception` | Epistemology-based (seeking truth) |
| `existential` | Meaning-based motivation |

### `arc_type` (7 variants)
Character arc trajectories:

`positive_change`, `negative_fall`, `flat`, `minor`, `disillusionment`, `corruption`, `none`

### `actant` (6 variants)
Greimas actantial model:

| Value | Description |
|-------|-------------|
| `subject` | Who desires |
| `object` | What is desired |
| `sender` | Who initiates the quest |
| `receiver` | Who benefits |
| `helper` | Who aids |
| `opponent` | Who impedes |

---

## Narrative Voice Enums

### `pov_type` (5 variants)
Point of view narrator types:

`first_person`, `second_person`, `third_limited`, `third_omniscient`, `third_objective`

### `focalization_type` (5 variants)
Genette focalization categories:

| Value | Description |
|-------|-------------|
| `zero` | Omniscient (non-focalized) |
| `internal_fixed` | Single character's perspective |
| `internal_variable` | Shifts between characters |
| `internal_multiple` | Same event, multiple perspectives |
| `external` | Camera-eye, no interiority |

### `consciousness_mode` (4 variants)
Dorrit Cohn's representation modes:

| Value | Description |
|-------|-------------|
| `psychonarration` | Narrator reports character's thoughts |
| `quoted_monologue` | Direct thought (tagged) |
| `narrated_monologue` | Free indirect discourse |
| `mixed` | Combination |

### `psychic_distance` (5 levels)
John Gardner's scale (1-5):

| Level | Mode | Example |
|-------|------|---------|
| 1 | `distant_narrator` | "It was winter of the year 1853..." |
| 2 | `close_narrator` | "Henry Dowell was growing old..." |
| 3 | `neutral_filter` | "He thought the town looked cold..." |
| 4 | `deep_filter` | "Cold, yes, the town was cold..." |
| 5 | `stream_of_consciousness` | "Cold. Bitter cold. God how he hated..." |

### `diegetic_level` (3 variants)
Genette narrative levels:

| Value | Description |
|-------|-------------|
| `extradiegetic` | Outer frame narrator |
| `intradiegetic` | Main story level |
| `metadiegetic` | Story-within-story |

### `narrator_reliability_type` (6 variants)
Booth/Nünning reliability classification:

`reliable`, `factually_unreliable`, `interpretively_unreliable`, `evaluatively_unreliable`, `self_deceptive`, `culturally_contingent`

---

## Narrative Time Enums (Genette)

### `narrative_order` (5 variants)
Temporal order:

| Value | Description |
|-------|-------------|
| `chronological` | Events in order |
| `analepsis` | Flashback |
| `prolepsis` | Flash-forward |
| `braided` | Interwoven timelines |
| `in_medias_res` | Starting mid-action |

### `duration_mode` (5 variants)
Story time vs. discourse time:

| Value | Description |
|-------|-------------|
| `scene` | Real-time (ST ≈ DT) |
| `summary` | Compressed (ST > DT) |
| `ellipsis` | Omitted (DT = 0) |
| `pause` | Stopped (ST = 0) |
| `stretch` | Slow-motion (ST < DT) |

### `frequency` (4 variants)
How often events are narrated:

| Value | Description |
|-------|-------------|
| `singulative` | Once happened, once told |
| `iterative` | Many happened, told as one |
| `repetitive` | Once happened, told many times |
| `multiple_singulative` | Many happened, each told separately |

---

## Scene Function Enums

### `beat_type` (15 variants)
Story beat types:

`hook`, `setup`, `catalyst`, `debate`, `threshold`, `tests`, `approach`, `ordeal`, `reward`, `road_back`, `resurrection`, `return`, `crisis`, `climax`, `resolution`

### `scene_function` (8 variants)
What structural work the scene does:

`establish`, `complicate`, `reveal`, `confront`, `decide`, `transform`, `resolve`, `transition`

### `scene_polarity` (24 variants)
Dramatic value poles for scene turns:

`hope`, `fear`, `trust`, `suspicion`, `ignorance`, `knowledge`, `connection`, `isolation`, `control`, `chaos`, `safety`, `danger`, `certainty`, `doubt`, `love`, `hate`, `calm`, `agitation`, `power`, `powerlessness`, `belonging`, `alienation`, `clarity`, `confusion`

### `causal_role` (5 variants)
Scene's causal position in plot:

| Value | Description |
|-------|-------------|
| `establishes` | Sets up subsequent events |
| `escalates` | Raises stakes/tension |
| `pivots` | Changes direction |
| `resolves` | Concludes a thread |
| `echoes` | Recalls earlier events |

---

## Emotion & Psychology Enums

### `emotion` (39 variants)
Core emotional states grouped by valence:

**Positive:** `joy`, `contentment`, `hope`, `pride`, `love`, `gratitude`, `relief`

**Anger cluster:** `anger`, `frustration`, `resentment`, `contempt`, `disgust`

**Fear cluster:** `fear`, `anxiety`, `dread`, `panic`

**Sadness cluster:** `sadness`, `grief`, `despair`, `melancholy`, `loneliness`

**Shame cluster:** `shame`, `guilt`, `embarrassment`, `humiliation`

**Surprise cluster:** `surprise`, `shock`, `confusion`, `disbelief`

**Cognitive:** `curiosity`, `interest`, `anticipation`

**Low arousal:** `boredom`, `apathy`, `resignation`

**Social:** `jealousy`, `envy`

**Meta:** `neutral`, `conflicted`, `mixed`

### `tactic` (30 variants)
How characters pursue objectives:

`direct_request`, `demand`, `persuasion`, `manipulation`, `charm`, `flattery`, `seduction`, `bribery`, `threat`, `intimidation`, `aggression`, `deflection`, `avoidance`, `withdrawal`, `silence`, `honesty`, `confession`, `vulnerability`, `deception`, `misdirection`, `omission`, `appeal_to_emotion`, `appeal_to_logic`, `appeal_to_authority`, `bargaining`, `compromise`, `submission`, `defiance`, `mockery`, `sarcasm`, `passive_observation`, `information_gathering`

---

## Setting Enums

### `setting_type` (28 variants)
Location categories:

`estate_interior`, `estate_exterior`, `manor_house`, `cottage`, `public_building`, `religious_building`, `government_building`, `commercial`, `inn_tavern`, `market`, `domestic_modest`, `domestic_grand`, `domestic_poor`, `transport`, `carriage`, `ship`, `on_foot`, `natural_landscape`, `garden`, `forest`, `field`, `water_body`, `urban_street`, `urban_square`, `urban_alley`, `military`, `prison`, `hospital`, `school`

### `time_of_day` (9 variants)
`dawn`, `morning`, `midday`, `afternoon`, `dusk`, `evening`, `night`, `midnight`, `pre_dawn`

### `spatial_structure` (6 variants)
Lotman semantic spaces:

`enclosed`, `threshold`, `open`, `liminal`, `vertical`, `horizontal`

---

## Relationship Enums

### `relationship_type` (26 variants)
Types of relationships:

**Romantic:** `spouse`, `betrothed`, `lover`, `ex_lover`, `suitor`

**Family:** `parent`, `child`, `sibling`, `cousin`, `guardian`, `ward`

**Social:** `friend`, `confidant`, `rival`, `enemy`, `nemesis`

**Professional:** `mentor`, `student`, `employer`, `employee`, `servant`, `master`

**Other:** `ally`, `associate`, `acquaintance`, `stranger`

### `relationship_dynamic` (17 variants)
Relationship states:

`harmonious`, `cordial`, `neutral`, `strained`, `hostile`, `adoring`, `admiring`, `respectful`, `trusting`, `distrustful`, `dependent`, `controlling`, `manipulative`, `protective`, `alienated`, `estranged`, `reconciling`

### `power_balance` (5 variants)
Power asymmetry:

`source_dominant`, `target_dominant`, `equal`, `contested`, `shifting`

---

## Literary Theory Enums

### `irony_type` (8 variants)
Booth/Hutcheon irony taxonomy:

`verbal`, `situational`, `dramatic`, `structural`, `cosmic`, `socratic`, `romantic`, `none`

### `transtextuality_type` (6 variants)
Genette transtextual relations:

| Value | Description |
|-------|-------------|
| `intertextuality` | Quotation, allusion |
| `paratextuality` | Titles, prefaces, notes |
| `metatextuality` | Commentary, criticism |
| `hypertextuality` | Transformation (parody, pastiche) |
| `architextuality` | Genre relation |
| `none` | No significant relation |

### `freudian_mechanism` (9 variants)
Psychoanalytic mechanisms:

`condensation`, `displacement`, `uncanny`, `repetition_compulsion`, `projection`, `rationalization`, `sublimation`, `repression`, `none`

### `lacan_register` (4 variants)
Lacanian psychic registers:

| Value | Description |
|-------|-------------|
| `real` | Pre-linguistic, traumatic, unsymbolizable |
| `symbolic` | Language, law, social order |
| `imaginary` | Image, ego, mirror-stage |
| `none` | Not applicable |

### `gaze_type` (7 variants)
Mulvey gaze theory:

`active`, `passive_spectacle`, `scopophilic`, `power_gaze`, `counter_gaze`, `neutral`, `none`

---

## Canonical Summary Enums

### `event_type` (18 variants)
Event verb categories for canonical summaries:

`arrival`, `departure`, `confrontation`, `confession`, `discovery`, `decision`, `proposal`, `refusal`, `acceptance`, `betrayal`, `reconciliation`, `revelation`, `deception`, `rescue`, `escape`, `transformation`, `death`, `celebration`

### `want_outcome` (4 variants)
Character want resolution:

| Value | Description |
|-------|-------------|
| `granted` | Want achieved |
| `denied` | Want blocked |
| `deferred` | Want postponed |
| `pyrrhic` | Want achieved at great cost |

### `stakes_domain` (5 variants)
What category of stakes:

`physical`, `emotional`, `social`, `professional`, `existential`

### `atmosphere` (12 variants)
Scene atmosphere:

`tense`, `peaceful`, `ominous`, `joyful`, `melancholic`, `romantic`, `hostile`, `mysterious`, `chaotic`, `formal`, `intimate`, `mundane`

---

## Full Enum Count Summary

| Category | Enums | Total Variants |
|----------|-------|----------------|
| Character | 7 | ~150 |
| Narrative Voice | 8 | ~50 |
| Narrative Time | 6 | ~25 |
| Scene Function | 4 | ~60 |
| Emotion/Psychology | 6 | ~120 |
| Setting | 8 | ~80 |
| Relationship | 4 | ~50 |
| Literary Theory | 12 | ~100 |
| Canonical Summary | 4 | ~40 |
| **Total** | **~60** | **~700** |
