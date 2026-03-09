# Entity Registry: Architecture & Theoretical Rationale

**Schema:** [`schema/registry.schema.json`](../schema/registry.schema.json)  
**Version:** 1.0.0

The `EntityRegistry` is the controlled vocabulary for a specific book. It is the protocol's source of truth: every character, setting, and relationship referenced by a scene card or character state must be declared here first. The registry enforces referential integrity — no anonymous entities appear in the generation pipeline.

---

## Design Principle

The registry operationalizes a claim from structuralist narratology: narrative is not the sum of free-floating events, but the combinatorial output of a finite set of **entities** acting within a **relational network**. Propp's 31 functions (*Morphology of the Folktale*, 1928) unfold from 7 roles; Greimas's narrative grammar (*Structural Semantics*, 1966) reduces all narrative events to 6 actantial positions. The registry makes this underlying entity set explicit and machine-readable.

---

## 1. Book-level Identity

### `book_id`
**Type:** string (slug)  
**Required**  
**Rationale:** The registry is scoped to a single book. The `book_id` is the namespace for all slugs within it — a character slug that exists in one registry does not automatically exist in another, enabling multi-book series with distinct casts and shared characters both.

### `title` / `author`
**Type:** string  
**Rationale:** Metadata for human identification. Not structural — but required for the extraction workflow to label its output unambiguously.

---

## 2. Characters

### `characters`
**Type:** map `slug → Character`  
**Required**  
**Rationale:** The map structure (not array) enables O(1) lookup by slug. Every scene card references characters by slug; slug resolution against this map is the protocol's primary integrity check.

### Character: `id`
**Type:** string (slug, snake_case)  
**Required**  
**Rationale:** The canonical identifier. Snake_case is enforced because character names in prose may be capitalized, suffixed with titles, or aliased — "Dr. Eleanor Marsh," "Ellie," "Dr. Marsh" are all the same person. The slug is stable; the prose representation varies. This follows URI design principles and the stable-identifier convention in linked data.

### Character: `name`
**Type:** string  
**Rationale:** Display name — the primary name used in prose. The system can generate variant forms (shortened, formal, alias) from this by convention, but the canonical prose form is declared here.

### Character: `slot`
**Type:** string (nullable)  
**Rationale:** The Grimoire cast-template slot system derived from Campbell/Vogler's archetypal cast model (*The Writer's Journey*, Vogler 1992): protagonist, deuteragonist, antagonist, mentor, trickster, shapeshifter, guardian, herald, ally. The slot is not required because not every character maps cleanly to a role, and some characters are functional-structural (their purpose is contextual rather than archetypal).

### Character: `archetype`
**Type:** enum (`archetype`)  
**Rationale:** Carl Jung's collective unconscious theory posited that recurring character *types* across cultures derive from shared deep psychological structures — the Shadow, the Anima/Animus, the Self, the Wise Old Man, the Trickster, etc. (*The Archetypes and the Collective Unconscious*, 1959). Joseph Campbell mapped these to narrative roles (*Hero with a Thousand Faces*, 1949); Christopher Vogler translated them to screenwriting practice (*The Writer's Journey*, 1992). The archetype is a reader-expectation system — the Hero activates specific genre contracts, the Trickster licensed specific behaviors, the Mentor certain revelatory functions. The vocabulary enables genre-competent generation.

### Character: `wound`
**Type:** enum (`wound`)  
**Rationale:** The backstory wound is the foundational explanatory mechanism in contemporary character arc theory. K.M. Weiland (*Creating Character Arcs*, 2016) argues that every character's external flaw traces to an internal wound sustained in the past. Lisa Cron (*Wired for Story*, 2012) grounds this neurologically: readers engage stories because they simulate social learning — and social learning is organized around understanding causes (wounds) of behavior. The 12 wound types (abandonment, betrayal, rejection, humiliation, failure, powerlessness, loss, trauma, shame, identity_crisis, abuse, grief) represent the statistically most common traumatic structures in clinical and narrative literature.

### Character: `alignment`
**Type:** enum (`alignment`)  
**Rationale:** The 9-value alignment system — originally from Dungeons & Dragons (Gygax, 1978) but now standard in narrative practice — encodes two orthogonal axes: *law/chaos* (does the character respect social order and rules?) and *good/evil* (does the character act for the benefit or harm of others?). The resulting 3×3 grid (Lawful Good through Chaotic Evil, plus True Neutral) is a useful shorthand for predicting behavior under pressure. While simplistic as psychological theory, alignment is effective as a generation constraint because it encodes *behavioral consistency* — the promise that the character will not act arbitrarily against their declared axis.

### Character: `role`
**Type:** enum (`role`)  
**Rationale:** Functional narrative role as distinct from archetype (psychological template) and slot (structural position). Role classifies what the character *does* to the plot: protagonist (the story is about their want), antagonist (opposes the protagonist's want), deuteragonist (secondary significant figure), foil (exists to contrast the protagonist), mentor/guide, comic relief, confidant, etc. The role system derives from the French classical dramaturgy tradition (protagoniste, deutéragoniste, antagoniste) and its English adaptation.

### Character: `drive_model`
**Type:** enum (`drive_model`)  
**Rationale:** See CHARACTER_STATE.md §7 for full rationale. At the registry level, `drive_model` declares the character's *macro* motivational architecture — the spring that drives them across the whole story. This is consistent across scenes (unlike `drive_model` in `CharacterState`, which applies scene-by-scene). The five-model taxonomy (wound/ghost, desire/fear, duty/code, perception, existential) derived from Weiland, Cron, and Truby.

### Character: `arc_type`
**Type:** enum (`arc_type`)  
**Rationale:** Weiland's three macro arc types (*Creating Character Arcs*, 2016): `positive_change_arc` (character overcomes the lie and embraces the truth), `negative_change_arc` (character embraces the lie and rejects truth — tragedy or corruption arc), `flat_arc` (character already has the truth; their function is to force change in others). A fourth type — `disillusionment_arc` (character begins believing the truth, loses it, ends in tragedy) — is also included. These types determine where the story's emotional and moral weight lands.

### Character: `actant`
**Type:** enum (`actant`)  
**Rationale:** Greimas's six actantial positions (*Structural Semantics*, 1966) declared at the registry level. This is the character's *default* actantial function in the story (they may occupy different actantial positions in individual scenes). Subject = hero/protagonist pursuing the Object; Sender = motivating authority; Receiver = beneficiary; Helper = aids Subject; Opponent = opposes Subject; Object = the sought value or goal-entity. This cross-cutting classification reveals story grammar beneath genre surface.

### Character: `ghost`
**Type:** string (nullable)  
**Rationale:** The ghost is Truby's term (*Anatomy of Story*, 2007) for the traumatic backstory event that haunts the character. Distinct from the `wound` field (which names the *type* of psychological damage) — the ghost names the *specific event* or situation. "Her father's abandonment when she was nine" is the ghost; "abandonment" is the wound category. The ghost is a free string because it is always narratively specific.

### Character: `want` / `need`
**Type:** string (nullable)  
**Rationale:** The foundational dramatic opposition in Truby's theory (*Anatomy of Story*, 2007). Want = what the character consciously pursues (their external goal). Need = what they must psychologically or morally attain to become whole (typically the opposite of their flaw). The tension between want and need drives the arc. They are separate fields because the gap between them is the story. "She wants to control everything" (want) / "She needs to trust others" (need). Without explicit declaration of both, want/need alignment analysis is impossible.

### Character: `flaw`
**Type:** string (nullable)  
**Rationale:** Aristotle's *hamartia* (*Poetics*, c.335 BCE) — the tragic mistake or character deficiency that drives the downward arc. In Truby's framework, the flaw is the behavioral manifestation of the wound's misbelief: the character *acts badly* in characteristic ways because of what the wound has taught them to believe. The flaw is the handle through which the story changes (or fails to change) the character. A free string because flaws are always character-specific and narratively embedded.

### Character: `voice_signature`
**Type:** object  
**Rationale:** A character's distinctive speech and prose patterns — their verbal fingerprint. Derived from the field of stylometry (Burrows' "Delta" method, 2002) applied to the challenge of maintaining voice consistency across a long-form generation task. The six sub-fields:

#### `sentence_length_tendency`
`short | medium | long | varied`  
**Rationale:** Sentence length is the most statistically reliable stylometric marker (Mosteller & Wallace, *Applied Bayesian and Classical Inference*, 1984). Short sentences signal urgency, economy, trauma; long sentences signal class, education, cognition styles. When a character is the focalizer, their consciousness mode should be rendered in their characteristic rhythm.

#### `vocabulary_register`
`colloquial | standard | formal | archaic | mixed`  
**Rationale:** Mikhail Bakhtin's concept of *raznorëchie* (heteroglossia) — the co-presence of multiple social speech registers in a text (*The Dialogic Imagination*, 1981). A character's vocabulary register identifies which social stratum's speech they inhabit. This determines lexical choice in dialogue and FID.

#### `syntax_complexity`
`simple | moderate | complex`  
**Rationale:** Syntactic complexity correlates with level of abstraction and cognitive style. A character who uses embedded clauses, passive constructions, and nominalization thinks and presents themselves differently than one who uses simple SVO structures. Derived from systemic-functional linguistics (Halliday, *An Introduction to Systemic Functional Linguistics*, 1985).

#### `fid_markers`
**Type:** array of string  
**Rationale:** Character-specific free indirect discourse markers — the habitual phrases and sentence structures that signal FID in this character's prose. "After all..." / "What was the point, really?" / "She might as well..." — these are idiolectal, and assembling them at the registry level enables consistent rendering across scenes. The theoretical basis is the same as in CHARACTER_STATE.md (Cohn, Banfield, Pascal) but applied to static character definition rather than scene-specific state.

#### `forbidden_words` / `signature_phrases`
**Type:** array of string  
**Rationale:** Negative constraints (words a character *never* uses) and positive constraints (phrase patterns they *always* use). This is a practical operationalization of idiolect — the individual's unique language use (Crystal, *The Cambridge Encyclopedia of Language*, 1987). Forbidden words in dialogue prevent a character from using vocabulary that is inconsistent with their class, education, period, or personality. Signature phrases are Proustian involuntary memories made structural — recurring verbal tics that belong to this person specifically.

### Character: `voice_embedding`
**Type:** object  
**Rationale:** A learned dense vector representation of the character's voice, created from author-provided prose samples. While `voice_signature` captures *discrete* stylometric features (sentence length, register, complexity), `voice_embedding` captures the *continuous* latent structure of the character's prose — the patterns that resist enumeration but are recognizable to readers and statistical models alike.

**Workflow:** The author provides sample passages (dialogue, FID passages, or mixed) that exemplify the character's voice. These samples are embedded using a text embedding model (e.g., OpenAI's `text-embedding-3-small` or a local sentence-transformer). The resulting vector becomes the character's voice anchor.

**Use cases:**
1. **Continuity tracking:** During drafting, new prose can be embedded and compared (cosine similarity) to the character's voice anchor. Drift below threshold triggers revision flags.
2. **Voice collision detection:** If two characters' voice embeddings are too similar (distance < threshold), the system warns that readers may confuse them.
3. **FID consistency:** When the narrator renders a character's consciousness via free indirect discourse, the prose should move *toward* that character's voice embedding. Embeddings make this measurable.

**Theoretical basis:** The embedding approach operationalizes Bakhtin's insight that each character's speech carries a distinct *social accent* — their word belongs to a specific ideological horizon (*The Dialogic Imagination*, 1981). Where `voice_signature` captures this discretely (register = "colloquial"), `voice_embedding` captures the full distribution of that social position in language-model space.

**Sub-fields:**
- `model`: Embedding model identifier (required for reproducibility)
- `source_type`: What prose was embedded — `dialogue_only`, `fid_passages`, `all_speech`, or `mixed`
- `sample_word_count`: Total words in the samples (minimum 100 for stability)
- `vector`: The embedding itself (dimension depends on model — typically 384-1536)

**Important:** Voice embeddings are *authoring tools*, not training targets. They are excluded from training datasets because the goal is to teach the model to annotate prose, not to memorize specific voice vectors.

---

## 3. Narrator

### `narrator`
**Type:** object  
**Rationale:** In heterodiegetic narration (third-person), the narrator is a voice distinct from any character. Genette (*Narrative Discourse*, 1972) distinguishes the narrator (who speaks) from the focalizer (who perceives). When no character is focalized — establishing shots, scene transitions, direct narratorial commentary — the narrator's voice is what the reader hears. This voice has its own stylistic fingerprint and should be tracked like a character's voice.

### Narrator: `type`
**Type:** enum (`first_person`, `third_limited`, `third_omniscient`, `second_person`, `frame_narrator`)  
**Rationale:** The fundamental narratological categories per Genette and Stanzel's typologies. Determines the baseline parameters of narratorial privilege (what the narrator can know) and presence (how visible the narrating act is).

### Narrator: `voice_signature`
**Type:** object (same as character `voice_signature`)  
**Rationale:** The narrator has stylistic tendencies distinct from character voices. A Jamesian narrator uses complex syntax and formal register; a Hemingway narrator uses short sentences and colloquial register. These tendencies are the stable baseline — when the prose enters FID, it moves away from the narrator's signature toward the character's.

### Narrator: `voice_embedding`
**Type:** object (same as character `voice_embedding`)  
**Rationale:** Like character voice embeddings, the narrator's voice embedding enables continuity tracking. During revision, checking that establishing passages cluster near the narrator embedding (and FID passages cluster near the appropriate character embedding) verifies that voice modulation is working as intended.

**Workflow:** Author provides samples of pure narrator voice — passages with no FID, no dialogue, no focalizer bleeding through. These are embedded to create the narrator anchor.

### Narrator: `reliability`
**Type:** enum (`reliable`, `unreliable`, `ambiguous`)  
**Rationale:** Wayne Booth's foundational distinction (*The Rhetoric of Fiction*, 1961). A reliable narrator's statements can be taken at face value; an unreliable narrator's statements require reader correction. The `ambiguous` value captures cases where the text deliberately keeps reliability indeterminate (as in some modernist and postmodernist narration).

### Narrator: `distance`
**Type:** enum (`intimate`, `close`, `neutral`, `distant`, `ironic`)  
**Rationale:** John Gardner's psychic distance scale (*The Art of Fiction*, 1984). The narrator's default distance setting determines how close the prose brings the reader to character consciousness. An intimate narrator produces prose that feels like direct access to thought; an ironic narrator maintains evaluative distance.

---

## 4. Relationships

### `relationships[]`
**Type:** directed edge array  
**Rationale:** Relationships are *directed* (source → target) because they are asymmetric — what A feels toward B is not the same as what B feels toward A. The direction encodes perspective. This is a graph structure, not a list of pairs — narrative relationships form a weighted directed graph where edge weights (power balance, dynamic) change across scenes.

### `source` / `target`
**Type:** string (slug)  
**Rationale:** Named ends of the relationship edge. Both must resolve to declared characters.

### `rel_type`
**Type:** enum (`relationship_type`)  
**Rationale:** The structural category of the relationship. 18-value vocabulary: family (parent, sibling, child, spouse), professional (colleague, subordinate, superior, rival), social (friend, enemy, mentor, protégé), romantic (lover, ex-lover, unrequited), and structural (foil, parallel, shadow). Type determines baseline interaction norms and the default power context.

### `dynamic_at_start` / `dynamic_at_end`
**Type:** enum (`relationship_dynamic`)  
**Rationale:** The qualitative state of the relationship at story open and close. The difference between them captures the relationship arc — an independent narrative line that runs parallel to the character arc. The 12-value vocabulary mirrors the `relationship_turn` vocabulary in the `CharacterState` scene-level struct. The macro arc (registry) and scene-level turns (character state) together constitute a multi-scale account of the relationship's evolution. Theoretically grounded in Truby's claim that every story is also a story about a relationship (*Anatomy of Story*, 2007).

### `power_balance`
**Type:** enum (`power_balance`)  
**Rationale:** The structural power disposition of the relationship at story start. See CHARACTER_STATE.md §10 for theoretical rationale. At the registry level, power balance is the *baseline* — scenes may shift it locally, and the `dynamic_at_end` captures whether the story permanently shifts it.

---

## 5. Settings

### `settings`
**Type:** map `slug → Setting`  
**Required**  
**Rationale:** Settings are entities, not descriptions. Lotman's *semiosphere* theory (*The Structure of the Artistic Text*, 1977) argues that fictional space is always semantically coded — a manor house is not merely a building but a structure of class, enclosure, and inherited power. By making settings first-class registry entities, the protocol ensures spatial meaning is explicit and consistent.

### Setting: `id` / `name`
**Rationale:** Same slug/display-name pattern as characters. A setting may have variant prose descriptions across scenes (nighttime vs. daytime, empty vs. crowded) while remaining the same registry entity.

### Setting: `type`
**Type:** enum (`setting_type`)  
**Rationale:** The setting's structural classification: domestic, institutional, urban, rural, wilderness, liminal, industrial, sacred, transit, and others. The type determines the social norms and power structures operative in it (Goffman, 1959). A domestic setting activates privacy norms and intimacy scripts; an institutional setting activates bureaucratic and hierarchical scripts.

### Setting: `general_vibe`
**Type:** string (nullable)  
**Rationale:** The dominant affective quality of the location — its persistent atmospheric signature. Derived from the concept of *Genius Loci* (spirit of place — originating in Virgil's *Aeneid* and theorized by Norberg-Schulz in *Genius Loci: Towards a Phenomenology of Architecture*, 1979). A setting's vibe is not merely mood — it is the location's stable contribution to any scene that takes place there. "Cold institutional light and bureaucratic smell of paper" is this setting's permanent gift to every scene, regardless of what happens.

### Setting: `sensory_signature`
**Type:** array of string (max 3)  
**Rationale:** Three defining sensory details that appear across scenes in this location. Elaine Scarry (*Dreaming by the Book*, 1999) demonstrates that specific sensory detail (tactile, olfactory, and auditory especially) creates the most vivid reader simulation of fictional places. Three details is a constraint derived from working memory capacity (Miller, "The Magical Number Seven," 1956) — three distinct sensory anchors is the maximum that can be held simultaneously without overwhelming the scene. These repeat across scenes to create continuity and build the setting's associative memory for the reader.

---

## 6. Want Vocabulary

### `want_vocabulary`
**Type:** map `slug → label`  
**Rationale:** A controlled vocabulary of wants shared across characters in the book. Wants are recurring narrative objects — multiple characters may desire the same thing (inheritance, approval of a specific person, a particular office or position). Declaring them in a shared vocabulary enables relational want-conflict mapping: when two characters hold the same want slug, the system can detect competition; when one character's want is another character's `forbidden_words` equivalent, it can detect structural opposition. Derived from Girard's mimetic desire theory (*Deceit, Desire, and the Novel*, 1961) — that desire is always triangulated through a model, which means wants are often shared.

---

## References

| Work | Author | Year | Fields |
|------|--------|------|--------|
| *Morphology of the Folktale* | Propp, V. | 1928 | Characters structural design |
| *Structural Semantics* | Greimas, A.J. | 1966 | `actant`, `actantial_role` |
| *The Archetypes and the Collective Unconscious* | Jung, C.G. | 1959 | `archetype` |
| *Hero with a Thousand Faces* | Campbell, J. | 1949 | `archetype`, `slot` |
| *The Writer's Journey* | Vogler, C. | 1992 | `archetype`, `slot` |
| *Poetics* | Aristotle | c.335 BCE | `flaw` (hamartia) |
| *The Dialogic Imagination* | Bakhtin, M. | 1981 | `vocabulary_register`, `voice_embedding` |
| *The Anatomy of Story* | Truby, J. | 2007 | `ghost`, `want`, `need`, `actant`, `power_balance`, `dynamic_at_start/end` |
| *Creating Character Arcs* | Weiland, K.M. | 2016 | `wound`, `arc_type`, `drive_model` |
| *Wired for Story* | Cron, L. | 2012 | `wound`, `drive_model` |
| *Transparent Minds* | Cohn, D. | 1978 | `fid_markers` |
| *Unspeakable Sentences* | Banfield, A. | 1982 | `fid_markers` |
| *The Dual Voice* | Pascal, R. | 1977 | `fid_markers` |
| *The Structure of the Artistic Text* | Lotman, Y. | 1977 | `settings`, `setting.type` |
| *Genius Loci* | Norberg-Schulz, C. | 1979 | `general_vibe` |
| *Dreaming by the Book* | Scarry, E. | 1999 | `sensory_signature` |
| *"The Magical Number Seven"* | Miller, G.A. | 1956 | `sensory_signature` (max 3 constraint) |
| *Deceit, Desire, and the Novel* | Girard, R. | 1961 | `want_vocabulary` |
| *The Presentation of Self in Everyday Life* | Goffman, E. | 1959 | `setting.type` |
| *Applied Bayesian and Classical Inference* | Mosteller & Wallace | 1984 | `sentence_length_tendency` |
| *Introduction to Systemic Functional Linguistics* | Halliday, M.A.K. | 1985 | `syntax_complexity` |
| *Dungeons & Dragons Player's Handbook* | Gygax, G. | 1978 | `alignment` |
| *Narrative Discourse* | Genette, G. | 1972 | `narrator.type`, narrator/focalizer distinction |
| *The Rhetoric of Fiction* | Booth, W.C. | 1961 | `narrator.reliability` |
| *The Art of Fiction* | Gardner, J. | 1984 | `narrator.distance` |
