# Character State: Architecture & Theoretical Rationale

**Schema:** [`schema/character-state.schema.json`](../schema/character-state.schema.json)  
**Version:** 1.0.0

A `CharacterState` describes the internal and relational state of one character at the boundary moments of a scene — entering and exiting. It is designed to capture all variables that govern how a character behaves, speaks, and is perceived during the scene, while also encoding what changes for them.

---

## Design Principle

Character psychology in fiction is not static — it is a *state machine*. Each scene advances or arrests the state. The `CharacterState` schema operationalizes two foundational claims:

1. **Character is action:** What a character *wants* in a scene (Stanislavski, *An Actor's Work*, 1938) determines what they *do*, which determines meaning.
2. **Character is consciousness:** What a character *knows, feels, and perceives* (Cohn, *Transparent Minds*, 1978) determines how the scene is rendered if they are the focalizer.

The schema bridges these two traditions — dramaturgy (wanting/doing) and narratology (knowing/perceiving) — in a single structured object.

---

## 1. Identity & Role

### `character`
**Type:** string (slug)  
**Required**  
**Rationale:** References a declared `Character` entity in the scene's `EntityRegistry`. Using a slug (rather than a name) guarantees referential integrity — "Lena" is unambiguous because it resolves to a specific character record with a full attribute bundle. The protocol is name-agnostic; the slug is the canonical identifier.

### `pov_role`
**Type:** enum (`pov_role_type`)  
**Rationale:** Distinguishes whether this character is the **focalizer** (whose perception filters the scene), a **participant** (present and acting), or a **non-present** character (referenced but absent). The distinction is crucial: the focalizer's state determines what the *prose* can access; a participant's state is enacted through dialogue and action alone. Derived from Gerard Genette's separation of **voice** (who narrates) and **focalization** (who perceives) in *Narrative Discourse* (1972). A model generating prose must know whether it can access a character's inner state or must infer it externally.

---

## 2. Narrative Voice (Focalizer-Specific)

These fields apply only when `pov_role == focalizer`.

### `psychic_distance`
**Type:** integer (1–5)  
**Rationale:** John Gardner's five-level axis from *The Art of Fiction* (1983). See SCENE_CARD.md §3 for primary rationale. The character-state version captures a character's *baseline* distance for this scene, distinct from the scene-card's declared level (which is the author's target). The two must align; any declared mismatch is a generation constraint, not an error.

### `psychic_distance_shifts`
**Type:** array of `{trigger, from_level, to_level}`  
**Rationale:** Psychic distance is not static within a scene. Key moments — revelation, emotional overwhelm, dissociation — shift it. Tracking shifts at the character level allows the model to modulate distance dynamically. The three-field object encodes: what causes the shift (`trigger`), the level being left, and the level being entered. This derives from Genette's observation that *variable internal focalization* (switching focalizer or distance mid-scene) is one of fiction's most powerful tools.

### `consciousness_mode`
**Type:** enum (`consciousness_mode`)  
**Rationale:** Dorrit Cohn's three-mode taxonomy from *Transparent Minds* (1978). See SCENE_CARD.md §3 for primary rationale. At the character-state level, this specifies which mode governs *this character's* interior rendering. Multiple characters in one scene can operate in different modes — the antagonist rendered via psychonarration while the protagonist is narrated monologue.

### `fid_markers`
**Type:** array of string  
**Rationale:** Free Indirect Discourse (FID) is identifiable by specific linguistic markers — temporal-deictic substitution ("tomorrow" for "the next day"), exclamatory questions ("What was she thinking?"), modal verbs of necessity ("She had to go"), and second-person intrusion ("After all, what had she to lose?"). These markers were catalogued by Pascal (*The Dual Voice*, 1977), Banfield (*Unspeakable Sentences*, 1982), and McHale ("Free Indirect Discourse: A Survey of Recent Accounts," 1978). The `fid_markers` field provides example strings that should be *present* in the generated prose when FID is active. This makes a theoretically abstract concept into a verifiable generation constraint.

---

## 3. Emotional State

### `emotional_state_entry` / `emotional_state_exit`
**Type:** object `{emotion, intensity, secondary_emotion, masked}`  
**Rationale:** The dual-state structure (entry and exit) operationalizes the scene's emotional arc at the character level. A scene changes emotional state; if the two states are identical, the scene has not done emotional work. The four-field object draws on:

- **`emotion`** — from Paul Ekman's 6 basic emotion framework (*"Emotions Revealed"*, 2003) and the expanded Plutchik wheel (*"A General Psychoevolutionary Theory of Emotion"*, 1980), which posits 8 primary and 24 secondary emotions. The vocabulary used includes both primaries and narrative-relevant blends.
- **`intensity`** (1–5) — Plutchik's intensity dimension. The same emotion at intensity 1 (annoyance) vs. 5 (rage) requires entirely different prose texture, body language, and speech patterns.
- **`secondary_emotion`** — Plutchik's dyadic blends (e.g., love = joy × trust; guilt = sadness × fear). Secondary emotions are often the more interesting ones narratively — the emotion the character is *also* experiencing beneath the primary.
- **`masked`** (boolean) — Whether the character is suppressing or performing something other than what they feel. Ties to the social self literature (Goffman, *The Presentation of Self*, 1959) and psychological self-deception (Sartre's *bad faith*, *Being and Nothingness*, 1943). A masked emotion is performed for other characters but may be accessible to the focalizing prose.

### `emotional_arc`
**Type:** enum (`emotional_arc_type`)  
**Rationale:** The shape of the emotional movement across the scene: `ascending` (intensification), `descending` (release/resolution), `plateau` (sustained state), `oscillating` (back and forth), `reversal` (dramatic flip). This is not the same as `turn` (which is structural value-sign change) — it's the *felt* trajectory of the emotional experience. Derived from music theory's dynamic markings applied to narrative emotion by Silvia and colleagues in affective narratology literature.

---

## 4. Epistemic State

### `knowledge_at_entry` / `knowledge_gaps` / `knowledge_gained`
**Type:** array of `{domain, predicate, about_role, secondary_role, certainty}`  
**Rationale:** Information asymmetry is the primary driver of dramatic irony — Sternberg's foundational analysis in *Expositional Modes and Temporal Ordering in Fiction* (1978). The three-array structure tracks what a character *knows*, what they *do not know but should or want to*, and what they *learn* during the scene.

The five-field knowledge object uses a lightweight predicate logic:
- **`domain`** (secrets, plans, relationships, identity, past, future, feelings, allegiances) — the semantic category of the knowledge
- **`predicate`** (knows, believes, suspects, denies, fears, desires) — the epistemic attitude
- **`about_role`** — the character slug the knowledge concerns
- **`secondary_role`** — when knowledge involves a third party (A knows/suspects that B did something to C)
- **`certainty`** (0.0–1.0) — epistemic confidence. Derives from possible-worlds logic (Doležel, *Heterocosmica*, 1998) where characters inhabit their own doxastic accessible worlds. A certainty of 0.3 encodes suspicion; 0.9 encodes near-certainty. The continuum enables modeling of doubt, suspicion, and dramatic irony in a formally tractable way.

`knowledge_gaps` provides the information voids that generate *curiosity* in the reader who identifies with the character (Sternberg's curiosity = reader knows less than the character knows they don't know) and *dramatic irony* when the reader knows more.

---

## 5. Action Grammar

### `objective`
**Type:** object `{verb, object_type, target_role, constraint}`  
**Rationale:** The foundational unit of Stanislavski's action method: every character in every scene has a *objetivo* — a specific, active, achievable want directed at someone or something (*An Actor's Work*, Stanislavski, 1938). The four-field objective object uses:
- **`verb`** (transitive action verb) — forces the want to be actionable, not adjectival. "To win approval" is wrong; "To convince [target_role]" is correct.
- **`object_type`** (information, commitment, submission, approval, resource, alliance, escape, forgiveness, recognition) — the *type* of thing being sought, abstracting across surface content
- **`target_role`** — the character slug who holds or can grant what is wanted
- **`constraint`** — what the character must NOT do or cannot do while pursuing the objective (rules of the scene, social constraints, moral limits). Derives from Brecht's concept of the *Gestus* — the social constraint on behavior that makes every action socially significant.

### `tactic`
**Type:** string  
**Rationale:** Stanislavski distinguishes *objective* (what you want) from *tactic* (how you try to get it). The tactic changes; the objective is stable through the scene. Uta Hagen (*Respect for Acting*, 1973) systematized tactics as active verbs: to seduce, to placate, to intimidate, to confide, to mock. The field is a free string because tactics are too numerous to enumerate but must be agent-specific — a character's tactical vocabulary is part of their voice.

### `tactic_shift`
**Type:** string  
**Rationale:** When the initial tactic fails, characters shift. This is McKee's scene-level reversal mechanism: resistance to the tactic causes the character to try something different. The `tactic_shift` captures the new approach. A scene with no tactic shift is either a monologue or a success — both rare and structurally marked.

### `obstacle`
**Type:** object `{type, source_role}`  
**Rationale:** Truby's principle: conflict requires an obstacle that is proportional to the objective. The five obstacle types — internal (character's own psychology), relational (another character), social (norms/institutions), physical (environment/circumstance), informational (not knowing what is needed) — derive from Truby's *Anatomy of Story* (2007). `source_role` identifies who or what presents the obstacle, enabling the extraction pipeline to reconstruct the full conflict structure of a scene.

---

## 6. Stakes

### `stakes.personal`
**Type:** string  
**Rationale:** Stakes are what the character stands to lose if the objective is not achieved. Personal stakes (what it means to *this* character specifically) are distinguished from relational stakes because they ground the reader's identification and emotional investment. McKee's formulation: stakes define the *value* at risk. Low stakes = low reader engagement regardless of plot complexity.

### `stakes.relational`
**Type:** string  
**Rationale:** What the *relationship* stands to lose. Relational stakes are often higher than personal ones because they affect multiple characters and the network of meaning between them. The field is distinct from personal because a character can achieve their objective at great relational cost (pyrrhic outcome).

### `stakes.level`
**Type:** enum (`stakes_level`)  
**Rationale:** Nominal, moderate, high, or existential — derived from Maslow's hierarchy applied to narrative by Thomas Foster, McKee, and Truby. Existential stakes (death, identity dissolution, moral corruption) require a different prose register and are the prerequisite for tragic or high-action genre claims. The level is declared explicitly to prevent generation under-calibration.

---

## 7. Character Arc

### `personal_arc_beat`
**Type:** enum (`arc_beat_type`)  
**Rationale:** Blake Snyder's Save the Cat! (2005) identifies 15 beats on the main character's personal arc (as distinct from the plot's five-act structure). The personal arc beat specifies where this scene falls on the character's internal journey — `ordinary_world`, `call_to_adventure`, `refusal`, `crossing_threshold`, `tests_allies_enemies`, `approach`, `ordeal`, `reward`, `road_back`, `resurrection`, `return_with_elixir`. Aligned with Campbell's monomyth (*Hero with a Thousand Faces*, 1949) and Vogler's adaptation (*The Writer's Journey*, 1992). For non-arc characters (flat arc or inverted arc), the field maps to their function relative to the protagonist's journey.

### `arc_direction`
**Type:** enum (`arc_direction`)  
**Rationale:** K.M. Weiland (*Creating Character Arcs*, 2016) defines three macro arc trajectories: `positive_change` (character is transformed toward the theme's truth), `negative_change` (moral regression), `flat_arc` (character already knows the truth; their arc is impact on others). Michael Hague (*Writing Screenplays That Sell*, 1988) adds the distinction between "identity" (outer self — the mask) and "essence" (inner self). The `arc_direction` names not the full arc but the *local direction* at this scene — a character can have a positive macro arc but a locally negative beat (descent into the dark night).

### `drive_model`
**Type:** enum (`drive_model`)  
**Rationale:** The psychological model of character motivation. The five-model vocabulary from the Grimoire `character-drives.yaml`:
- **`wound/ghost_drive`** — Weiland/Cron: the character acts from an unresolved past wound (*Creating Character Arcs*, Weiland 2016; *Wired for Story*, Cron 2012)
- **`desire/fear_drive`** — Truby: the surface want conceals a deeper fear (*Anatomy of Story*, Truby 2007)
- **`duty/code_drive`** — character operates from obligation or code (military, honor, loyalty)
- **`perception_drive`** — character acts on the basis of a misperception of themselves or the world (classic tragic flaw structure since Aristotle)
- **`existential_drive`** — character acts from questions about meaning/existence (Sartrean or absurdist frameworks)

The `drive_model` field enables the extraction pipeline to classify characters' motivational architecture, which is essential for long-form continuity.

---

## 8. Psychology

### `wound_triggered`
**Type:** boolean  
**Rationale:** The psychological wound (backstory trauma) is the source of character's misbelief and their arc's obstacle. K.M. Weiland (*Creating Character Arcs*, 2016) and Lisa Cron (*Wired for Story*, 2012) both ground character motivation in wound/ghost structures. A wound *triggered* in a scene means the character's behavior becomes partly reactive rather than intentional — they respond from the wound's logic rather than their conscious objective. This is a binary flag because a wound either fires or does not in a given scene.

### `wound_category`
**Type:** enum (`wound_category`)  
**Rationale:** 12-value taxonomy for the semantic type of wound: abandonment, betrayal, rejection, humiliation, failure, powerlessness, loss, trauma, shame, identity_crisis, abuse, grief. Derived from the clinical literature (Judith Herman, *Trauma and Recovery*, 1992; Bessel van der Kolk, *The Body Keeps the Score*, 2014) and adapted to narrative theory by Weiland and Cron. The category determines the *content* of the reactivity — a character wounded by abandonment performs differently when that wound fires than one wounded by shame.

### `want_need_alignment`
**Type:** enum (`alignment_type`)  
**Rationale:** Truby's core dramatic tension: the character's *want* (conscious goal) diverges from their *need* (what will actually restore them). The gap between them is the story's moral engine. Five alignment states: `aligned` (want and need are the same — rare, usually at the end of a positive arc), `opposed` (want actively prevents need — the classic midpoint dark night setup), `converging` (beginning to merge — typically act 3), `diverging` (moving further apart — typically act 2B), `unaware` (character doesn't perceive the need at all — early arc). Derived from Truby's *Anatomy of Story* (2007) and Weiland's extension in *Creating Character Arcs* (2016).

### `actantial_role`
**Type:** enum (`actantial_role`)  
**Rationale:** Algirdas Julien Greimas's actantial model from *Structural Semantics* (1966) — one of the most influential structuralist contributions to narrative theory. The six roles — Subject, Object, Sender, Receiver, Helper, Opponent — are positions in the scene's action grammar, not fixed character identities. A character who is the Subject (pursuer of the Object) in one scene may be the Opponent in another. The schema captures the *role* for *this scene*, enabling reconstruction of how characters shift through the structural grammar across the narrative.

---

## 9. Somatic & Social

### `posture` / `body_language`
**Type:** string  
**Rationale:** Body is both character signal and prose generator. Elaine Scarry (*Dreaming by the Book*, 1999) demonstrates how specific physical detail — weight, resistance, texture — produces the reader's vivid mental simulation of fictional persons. The phenomenological tradition (Merleau-Ponty, *Phenomenology of Perception*, 1945; Sheets-Johnstone, *The Primacy of Movement*, 1999) grounds this: body states are primary, not secondary to mental states. The prose must embody the emotional state; `posture` and `body_language` provide explicit generation targets.

### `social_circles_active`
**Type:** array of string (slugs)  
**Rationale:** From the Grimoire social circles model (Phase 02). Characters carry multiple social identities (family, profession, class, community, intimates) into each scene, and the active circles determine which social norms, obligations, and power dynamics apply. A scene between two colleagues operates under professional norms; the same characters as lovers operate under different ones. The `social_circles_active` field specifies which of the character's circles are operative in this context. Derives from Cooley's "looking-glass self" theory (1902) and Goffman's social performance framework (*The Presentation of Self*, 1959).

### `social_mask`
**Type:** string  
**Rationale:** Goffman's *dramaturgy* — all social behavior is performance. The persona a character presents in this scene (the mask) may diverge from their `emotional_state_entry`. The gap between mask and feeling is the source of subtext. The `social_mask` field names the *performance* — "confident professional," "caring friend," "unperturbed bystander" — that the character enacts even while the internal state tells a different story.

### `social_role`
**Type:** string  
**Rationale:** The structural position the character occupies in the scene's social power hierarchy — leader, follower, challenger, mediator, outsider, host. Distinct from `actantial_role` (which is narrative grammar) and `social_mask` (which is performance). Social role is structural context: it determines who defers to whom, who has home ground advantage, who sets the agenda. Derives from sociology (Simmel, 1908) and applied to narrative character studies by Auerbach (*Mimesis*, 1946).

---

## 10. Relationships

### `relationships[]`
**Type:** array of relationship objects  
**Rationale:** In scene, a character's behavior is always relational — they are always responding to, performing for, or struggling against someone else. The relationship array captures the full dyadic context for each pairing active in the scene. This is not the static registry relationship (what they are to each other) but the *scene-specific state* of each dyad.

### `target_role`
**Type:** string (slug)  
**Rationale:** The other character in the dyad. Referenced by slug for registry resolution.

### `power_balance`
**Type:** enum (`power_balance`) with float delta  
**Rationale:** Power asymmetry is the central structural variable in relationships. Derived from sociology (Weber's definition of power, 1922) and applied to fiction systematically by Truby (*Anatomy of Story*, 2007) who argues that every relationship has a power structure and every arc involves a power shift. The five-level scale — `subordinate`, `slight_disadvantage`, `equal`, `slight_advantage`, `dominant` — provides a tractable vocabulary. The float delta tracks the *change* across the scene, formalizing the claim that scenes must shift power.

### `power_source`
**Type:** enum (`power_source`)  
**Rationale:** Following French and Raven's classic taxonomy (*"The Bases of Social Power"*, 1959) adapted to narrative: information, physical, social, moral, emotional, coercive, legitimate (positional), expert, resource. Identifying the power *source* is essential — informational power operates through what is withheld or revealed, while emotional power operates through affect regulation. These determine what the scene's tactical moves look like.

### `underlying_conflict`
**Type:** string  
**Rationale:** Truby's argument: no scene exists without conflict, and every visible conflict (the "surface tension") conceals an underlying one. The underlying conflict is the relational wound, the competing value system, or the structural opposition (Greimas's S1/S2 semiotic square) that the scene expresses. This field names the deep source, not the surface manifestation.

### `wants_from_other` / `perceives_other_as`
**Rationale:** The scene's interpersonal logic. What the character seeks from this specific person (always more specific than the abstract `objective`) and how they *construct* the other in their perception. The constructed image of the other (perceive_other_as) derives from Sartre's *Being and Nothingness* (1943) — the look (*le regard*) that objectifies, and the recognition that humanizes. These two fields together generate the misrecognition dynamics that are central to tragedy and irony.

### `trigger_type`
**Type:** enum (`trigger_type`)  
**Rationale:** What in the scene activates this relationship's particular charge — proximity, specific speech act, topic, physical touch, power shift, memory reference. This is the *stimulus* that shifts the relationship from neutral to activated. Enables generation of the precise moment when latent conflict becomes manifest. Grounded in Stanislavski's "if" — "if X happened, how would my character respond?" (*An Actor's Work*, 1938).

### `relationship_turn`
**Type:** object `{from, to}` using `relationship_state`  
**Rationale:** The relationship-level analogue to the scene-card's `turn`. Every scene must change the relationship state (McKee's scene design principle applies at the dyadic level as well as the narrative level overall). The 12-value `relationship_state` vocabulary (trust, affection, alliance, hostility, suspicion, indifference, dependence, rivalry, protection, manipulation, grief, estrangement) encodes the *quality* of the relationship before and after. A relationship that hasn't turned by scene end is a continuity event, not a dramatic one.

---

## References

| Work | Author | Year | Fields |
|------|--------|------|--------|
| *Structural Semantics* | Greimas, A.J. | 1966 | `actantial_role` |
| *Being and Nothingness* | Sartre, J.-P. | 1943 | `emotional_state.masked`, `perceives_other_as` |
| *Phenomenology of Perception* | Merleau-Ponty, M. | 1945 | `posture`, `body_language` |
| *Narrative Discourse* | Genette, G. | 1972 | `pov_role`, `psychic_distance`, `consciousness_mode` |
| *An Actor's Work* | Stanislavski, K. | 1938 | `objective`, `tactic`, `trigger_type` |
| *Transparent Minds* | Cohn, D. | 1978 | `consciousness_mode`, `fid_markers` |
| *Expositional Modes* | Sternberg, M. | 1978 | `knowledge_at_entry`, `knowledge_gaps` |
| *The Art of Fiction* | Gardner, J. | 1983 | `psychic_distance`, `psychic_distance_shifts` |
| *Unspeakable Sentences* | Banfield, A. | 1982 | `fid_markers` |
| *Hero with a Thousand Faces* | Campbell, J. | 1949 | `personal_arc_beat` |
| *The Writer's Journey* | Vogler, C. | 1992 | `personal_arc_beat` |
| *Narratology* | Bal, M. | 1985 | `pov_role` |
| *The Presentation of Self* | Goffman, E. | 1959 | `social_mask`, `social_role`, `emotional_state.masked` |
| *The Anatomy of Story* | Truby, J. | 2007 | `obstacle`, `drive_model`, `want_need_alignment`, `power_balance`, `underlying_conflict` |
| *Save the Cat!* | Snyder, B. | 2005 | `personal_arc_beat` |
| *Creating Character Arcs* | Weiland, K.M. | 2016 | `arc_direction`, `drive_model`, `wound_triggered`, `want_need_alignment` |
| *Wired for Story* | Cron, L. | 2012 | `drive_model`, `wound_triggered` |
| *Writing Screenplays That Sell* | Hauge, M. | 1988 | `arc_direction` |
| *Heterocosmica* | Doležel, L. | 1998 | `knowledge.certainty` |
| *Trauma and Recovery* | Herman, J. | 1992 | `wound_category` |
| *The Body Keeps the Score* | van der Kolk, B. | 2014 | `wound_category`, `wound_triggered` |
| *Dreaming by the Book* | Scarry, E. | 1999 | `posture`, `body_language` |
| *The Primacy of Movement* | Sheets-Johnstone, M. | 1999 | `posture`, `body_language` |
| *"General Psychoevolutionary Theory of Emotion"* | Plutchik, R. | 1980 | `emotional_state.*` |
| *Emotions Revealed* | Ekman, P. | 2003 | `emotional_state.emotion` |
| *"The Bases of Social Power"* | French & Raven | 1959 | `power_source` |
| *Story* | McKee, R. | 1997 | `relationship_turn`, `stakes.level`, `tactic_shift` |
| *Respect for Acting* | Hagen, U. | 1973 | `tactic` |
