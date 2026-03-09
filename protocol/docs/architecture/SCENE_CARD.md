# Scene Card: Architecture & Theoretical Rationale

**Schema:** [`schema/scene-card.schema.json`](../schema/scene-card.schema.json)  
**Version:** 1.0.0

A SceneCard is the atomic unit of the GBR Protocol. It specifies everything a model needs to generate a prose passage, and everything that must be extractable from one. Every field represents a theoretical commitment about what constitutes a complete, agentic description of a scene.

---

## Design Principle

The scene card operationalizes a fundamental distinction from Russian Formalism:

> *Fabula* (what happens) must be fully recoverable from *syuzhet* (how it is told), and vice versa.  
> — Tomashevsky, "Thematics" (1925); Chatman, *Story and Discourse* (1978)

Structured fields capture the fabula. The `prose` field holds the syuzhet. The `canonical_summary` is the deterministic bridge between them, enabling the round-trip contract.

---

## 1. Identity

### `scene_id`
**Type:** string (slug)  
**Rationale:** Scenes require stable, unique identifiers because story structure is a directed graph, not a linear list. Callbacks, chapter position, and act structure all reference scenes by ID. Slug format enforces human-readability and URL-safety.

### `book_id`
**Type:** string (slug)  
**Rationale:** Scenes are always scoped to a book's registry. `book_id` is the namespace; it links the card to its `EntityRegistry` for slug resolution.

### `chapter` / `scene_index`
**Type:** integer  
**Rationale:** Chapter position is a structural fact in most narrative forms. Scene index within chapter enables tracking of scene density — a proxy for pacing. Both fields are required by the canonical summary template's position context.

---

## 2. Story Structure

### `act`
**Type:** integer (1–5)  
**Rationale:** Act structure is the oldest formal division in Western dramaturgy. The three-act model derives from Aristotle's *Poetics* (c. 335 BCE) — protasis, epitasis, catastrophe. Field Syd Field's paradigm (1979) operationalized it for screenwriting. The schema supports up to five acts to accommodate Shakespeare's five-act structure and longer serial forms. See also Gulino, *Screenwriting: The Sequence Approach* (2004).

### `sequence`
**Type:** enum (`sequence_type`)  
**Rationale:** Derived from Blake Snyder's Save the Cat! (2005) beat sheet and Joseph Gulino's eight-sequence architecture. Sequences are mid-level units between scene and act — typically 8–15 minutes in film, roughly 20–30 pages. The `sequence` field names the story-structural phase the scene belongs to (e.g., `fun_and_games`, `bad_guys_close_in`). This is coarser than `beat` and addresses the *dramatic question* of the sequence, not the individual scene.

### `beat`
**Type:** enum (`beat_type`)  
**Required**  
**Rationale:** Beats are the atomic plot events that change value sign (from positive to negative or vice versa). The term descends from Stanislavski's unit-and-objective system (1936) and was formalized in story structure by McKee (*Story*, 1997) and Snyder (*Save the Cat!*, 2005). The GBR beat taxonomy combines: Campbell/Vogler's monomyth stages (*Hero with a Thousand Faces*, 1949; *The Writer's Journey*, 1992), Snyder's 15-beat sheet, and Truby's 22 building blocks (*The Anatomy of Story*, 2007). A scene without a beat assignment lacks structural meaning — hence required.

### `arc_position`
**Type:** float (0.0–1.0)  
**Rationale:** Precise story position enables quantitative analysis of where a scene falls in the arc. Chatman's kernel/satellite distinction (1978) implies kernels cluster at structurally significant positions. Arc position allows this to be tested empirically. Derived from Freytag's pyramid (1863) and modernized by Field's paradigm percentages (act breaks at 25% and 75%).

### `scene_function`
**Type:** enum (`scene_function`)  
**Rationale:** Every scene performs structural work within the narrative. The eight-value vocabulary — `establish`, `complicate`, `reveal`, `confront`, `decide`, `transform`, `resolve`, `transition` — derives from McKee's functional taxonomy in *Story* (1997). This is distinct from `beat` (which is plot arc position) and `turn` (which is value sign change). A scene's function persists even if the beat changes on revision.

### `turn`
**Type:** object `{from, to}` using `scene_polarity`  
**Rationale:** McKee's scene design axiom: every scene must turn. A scene that ends with the same value sign it began with is not dramatically functional — it is passage. The `scene_polarity` vocabulary captures 24 opposing value pairs (hope/fear, trust/suspicion, knowledge/ignorance, etc.) derived from McKee's value taxonomy and extended by Truby's moral argument framework. The object structure forces explicit declaration of both poles, preventing implicit or vague turn descriptions.

---

## 3. Narrative Voice

### `pov`
**Type:** enum (`pov_type`)  
**Required**  
**Rationale:** Point of view determines the epistemic horizon of the narrative — what can be known, shown, or withheld. The five-value taxonomy (first person, second person, third limited, third omniscient, third objective) derives from the Wayne Booth lineage (*The Rhetoric of Fiction*, 1961), codified by Genette (*Narrative Discourse*, 1972/1980) and synthesized by Rimmon-Kenan (*Narrative Fiction*, 1983). Required because pov is not derivable from prose without ambiguity — it must be declared.

### `focalization`
**Type:** enum (`focalization_type`)  
**Rationale:** Genette's crucial distinction between *who narrates* (voice) and *who sees* (focalization). Focalization operates independently of person — a third-person narrator can be internally focalized through a character. The five modes (zero, internal fixed, internal variable, internal multiple, external) derive directly from Genette (*Narrative Discourse*, 1972). Mieke Bal extends this with her focalizer/focalized object distinction (*Narratology*, 1985). Without this field, the schema cannot discriminate between an omniscient narrator and one locked to a single perspective.

### `focalizer`
**Type:** string (character slug)  
**Required**  
**Rationale:** When focalization is internal, the focalizer must be explicitly identified. The field references a character slug rather than a role or name to ensure it resolves unambiguously to a registry entity. Required because the focalizer controls what details can appear in the prose — a scene cannot be generated without knowing whose consciousness filters the world.

### `psychic_distance`
**Type:** integer (1–5)  
**Rationale:** John Gardner's five-level scale from *The Art of Fiction* (1983). Operationalizes the felt distance between reader and character consciousness: 1 = maximally distant authorial narrator, 5 = stream of consciousness. This is distinct from focalization (which is structural) — two scenes can have the same focalization but very different psychic distance. Gardner's levels are illustrated with examples: level 1 ("It was winter of the year 1853..."), level 5 (direct thought without attribution). Required for prose generation because it determines diction, sentence structure, introspective depth, and FID deployment. See also Dorrit Cohn (*Transparent Minds*, 1978) for the theoretical underpinning.

### `consciousness_mode`
**Type:** enum (`consciousness_mode`)  
**Rationale:** Dorrit Cohn's taxonomy from *Transparent Minds* (1978) — the definitive study of how prose fiction represents mental life. The three modes — psychonarration (narrator tells us what character thinks), quoted monologue (character thinks in their own voice, tagged), narrated monologue (free indirect discourse, untagged) — produce fundamentally different prose textures. FID (narrated monologue) was theorized by Bakhtin as *heteroglossia* and *double-voiced discourse* (*The Dialogic Imagination*, 1981): the text carries both narrator and character voice simultaneously. This field must be specified because it determines whether the assistant turn should produce psychonarration ("She felt humiliated") vs. narrated monologue ("How could he have said such a thing?"). Ann Banfield's generativist account (*Unspeakable Sentences*, 1982) further grounds the formal properties of each mode.

### `diegetic_level`
**Type:** enum (`diegetic_level`)  
**Rationale:** Genette's three narrative levels from *Narrative Discourse* (1972): extradiegetic (the narrator's level, outside the story), intradiegetic (the story level), metadiegetic (a story within the story). Most scenes are intradiegetic, but frame narratives, embedded tales, and metafictional works require tracking which level is active. If diegetic level is undeclared, it is impossible to reason about a narrator who is also a character in the story they are narrating.

### `narratee_type`
**Type:** enum (`narratee_type`)  
**Rationale:** Gerald Prince's contribution to narratology — the *narratee*, the fictional entity to whom the narrator addresses the discourse (*"Introduction to the Study of the Narratee"*, 1971; *Narratology*, 1982). The four types — explicit (directly named/addressed), implicit (constructed by narration), dramatized (narratee is a character with visible reactions), zero-degree (unmarked, universal reader position) — affect prose register, what must be explained vs. assumed, and whether the narrator's rhetoric is addressed to a specific person. Robinson Crusoe addresses an explicit narratee differently than Emma's unmarked third-person narrator.

### `narrator_reliability`
**Type:** enum (`narrator_reliability_type`)  
**Rationale:** Wayne Booth coined "unreliable narrator" in *The Rhetoric of Fiction* (1961) as a narrator whose account diverges from the implied author's norms. Ansgar Nünning (*"Reconceptualizing Unreliable Narration"*, 2008) extended this to six failure modes now captured in the schema: factually unreliable (misreports events), interpretively unreliable (misreads their meaning), evaluatively unreliable (misjudges moral weight), self-deceptive (believes their own distortions), and culturally contingent (reliability depends on reader's normative frame). This field is essential for extraction tasks — a model must know whether to trust the narrator's account when parsing.

---

## 4. Narrative Time

The `narrative_time` object operationalizes Gérard Genette's full temporal analysis from *Narrative Discourse* (1972). Genette identifies three dimensions of narrative temporality — Order, Duration, Frequency — each now a schema field.

### `narrative_time.order`
**Rationale:** Genette's *ordre* — the relationship between the chronological sequence of story events (histoire) and the narrative sequence of the text. Values: `chronological` (histoire = récit), `analepsis` (retrospective; flashback), `prolepsis` (anticipatory; flashforward), `braided` (multiple simultaneous timelines), `in_medias_res` (beginning mid-event). Every deviation from chronological narration is a rhetorical choice that creates curiosity, suspense, or dramatic irony — Menakhem Sternberg's three "reader position" effects from *Expositional Modes and Temporal Ordering in Fiction* (1978).

### `narrative_time.analepsis_type` / `prolepsis_type`
**Rationale:** Genette's sub-classification of anachrony. Analepsis can be external (reaches back before the story's start), internal (within the story's duration), complete (fully narrated), or partial (leaves gaps). These distinctions matter for continuity tracking — an external analepsis cannot reference events still in the future relative to story time, but an internal analepsis must be reconciled with what has already been narrated.

### `narrative_time.duration_mode`
**Rationale:** Genette's *durée* — the relationship between story time (time elapsing in the story world) and discourse time (time it takes to read). The five modes: `scene` (story time ≈ discourse time; dramatic scene), `summary` (story time > discourse time; compression), `ellipsis` (story time passes, discourse time = 0; gap), `pause` (discourse time > 0, story time = 0; description or reflection), `stretch` (discourse time > story time; slow-motion). This field is critical for pacing — a reader cannot experience tension if every scene is summary, nor forward motion if every scene is stretch.

### `narrative_time.frequency`
**Rationale:** Genette's *fréquence* — how many times an event occurs in the story versus how many times it is narrated. `singulative` (once/once — default), `iterative` (many times/once — "Every morning she would..."), `repetitive` (once/many times — same event narrated from multiple perspectives), `multiple_singulative` (many occurrences, each narrated separately). Iterative narration is particularly important for characterizing habitual states and background social conditions. Its formal properties were studied by Käte Hamburger (*The Logic of Literature*, 1957/1973).

### `narrative_time.duration`
**Rationale:** Absolute story time covered by the scene. Required for continuity tracking — scenes must be sequenced temporally to detect discontinuities, ellipses, and overlaps. This enables verification of the `temporal_gap_type` field in continuity schemas.

---

## 5. Craft Settings

### `target_tension`
**Type:** integer (1–5)  
**Rationale:** Tension is the reader's experience of suspended stakes. The 1–5 scale operationalizes Alfred Hitchcock's distinction between surprise (no tension) and suspense (foreknowledge creates tension — the bomb under the table). Validated by Noël Carroll's theory of "erotetic" narrative (*The Philosophy of Horror*, 1990): narratives generate emotion by posing questions and deferring answers. The ordinal scale provides a generation target that can be verified post-hoc by analyzing syntax (shorter sentences, more fragments at high tension — confirmed by corpus stylometrics).

### `target_pacing`
**Type:** enum (`duration_mode`)  
**Rationale:** Pacing is the author's control of perceived time. Reuses `duration_mode` enum because pacing is the craft-level counterpart to the structural temporal concept. High tension scenes typically use `scene` (real-time) or `stretch`; transitional passages use `summary` or `ellipsis`. James Wood in *How Fiction Works* (2008) argues rhythm — the sentence-level counterpart of pacing — is the prose's most immediate expressive tool.

### `tone`
**Type:** enum (`tone`)  
**Rationale:** Tone is the narrator's or implied author's attitude toward subject matter and audience. M.H. Abrams (*A Glossary of Literary Terms*, 1957) distinguishes tone from mood (which belongs to atmosphere) and voice (which belongs to the narrator). The 45-value vocabulary captures both affect-register pairs (tender/intimate, ominous/menacing) and stylo-register pairs (clinical/detached, formal/ceremonial). Tone constrains lexical choice and figurative language during generation.

---

## 6. Setting & Space

### `setting`
**Type:** string (slug)  
**Rationale:** Connects the scene to a declared `Setting` entity in the registry. Physical location is not optional decoration but a structural element — Yuri Lotman's *semiosphere* concept (*The Structure of the Artistic Text*, 1977) argues space is always semantically charged, encoding oppositions such as safe/dangerous, known/unknown, sacred/profane. The slug requirement forces setting to be a shared, named entity rather than an implicit or described location.

### `setting_instance.time_of_day` / `weather`
**Rationale:** Atmospheric conditions are classical vehicles of *objective correlative* — T.S. Eliot's term for external phenomena that embody interior states (*"Hamlet and His Problems"*, 1919). In practice, weather and light have been systematically correlated with tone since at least Shakespeare (storms in *King Lear*, mist in Dickens). These fields ensure the generation target is explicit rather than implicit.

### `setting_instance.lighting_source` / `lighting_quality`
**Rationale:** Light is one of the most studied symbolic registers in literature. Gaston Bachelard (*The Flame of a Candle*, 1961) distinguishes candlelight (intimate, flickering, personal) from electric light (public, flat, modern). The fields encode both the source and quality because the same location at firelight vs. gas light produces entirely different prose textures.

### `setting_instance.spatial_structure`
**Type:** enum (`spatial_structure`)  
**Rationale:** Lotman's theory of semantic space (*The Structure of the Artistic Text*, 1977) identifies six spatial configurations: enclosed (containment, safety or entrapment), threshold (crossing boundary, transformation site), open (freedom or vulnerability), liminal (ambiguous, between), vertical (hierarchy encoded spatially — above/below), horizontal (movement, journey). These are not decorative but structurally determinative — the scene's spatial configuration constrains what actions are possible and what they mean.

### `setting_instance.territory_type`
**Type:** enum (`territory_type`)  
**Rationale:** From sociology and Goffman's *The Presentation of Self in Everyday Life* (1959): whose territory a scene takes place in fundamentally alters power dynamics. Home ground gives behavioral latitude; foreign territory constrains performance. The five values (home_ground, shared_space, rival_territory, neutral_ground, contested) encode this structural power variable.

### `setting_instance.props`
**Rationale:** Chekhov's gun principle — objects introduced must serve narrative function. The `prop_function` field enforces this by requiring classification: `set_dressing` (atmosphere only), `action_object` (used by a character), `symbolic` (carries thematic meaning), `chekhov_gun` (must fire later), `continuity` (tracks across scenes). Roland Barthes identified the "reality effect" (*L'effet de réel*, 1968): excessive specific detail creates an illusion of reality. The field forces the annotator to declare whether a prop is functional or decorative.

### `setting_instance.motifs_present`
**Rationale:** Track the thematic vocabulary across the text. A motif is a recurrent image, phrase, or structure that accumulates meaning through repetition — theorized by Leitmotiv in music (Wagner) and formalized in literary criticism by Northrop Frye (*Anatomy of Criticism*, 1957). The `motif_deployment` field (introduced/reinforced/varied/inverted/culminating) traces the motif's semantic evolution across the story arc.

---

## 7. Subtext

### `subtext.technique`
**Type:** enum (`subtext_technique`)  
**Rationale:** Harold Pinter's theatre established pause and silence as primary dramatic materials. Ernest Hemingway's iceberg theory ("The dignity of movement of an iceberg is due to only one-eighth of it being above water" — *Death in the Afternoon*, 1932) identifies omission as the core prose technique. The subtext techniques — gricean_violation, deflection, loaded_silence, iceberg, pinter_pause, apparent_irrelevance — are distinct mechanisms for creating a gap between utterance and meaning.

### `subtext.maxim_violated`
**Type:** enum (`gricean_maxim`)  
**Rationale:** H.P. Grice's Cooperative Principle (*"Logic and Conversation"*, 1975) posits that speakers normally observe four maxims: Quantity (say enough, not too much), Quality (be truthful), Relation (be relevant), Manner (be clear). Flouting any maxim *conversationally implicates* additional meaning. Dan Sperber and Deirdre Wilson's Relevance Theory (*Relevance*, 1986) extends this. In fiction, dialogue subtext almost always involves maxim violation: characters say too little (Quantity), say something irrelevant (Relation), or speak unclearly (Manner) to signal what they cannot or will not say directly. The field forces explicit identification of *which* maxim is being violated.

### `subtext.violation_type`
**Type:** enum (`violation_signature`)  
**Rationale:** The 10-value vocabulary names concrete prose signatures of maxim violation — `single_syllable_shutdown`, `topic_change`, `answering_different_question`, `excessive_detail`, `vagueness_from_precise_speaker`, etc. These are derived from corpus studies of dialogue in high-subtext prose (Pinter, Austen, Highsmith). This field enables a model to learn the stylistic *signatures* of subtext, not just its conceptual category.

### `subtext.iceberg_category`
**Type:** enum (`iceberg_category`)  
**Rationale:** Hemingway's iceberg principle requires specification of *what* is beneath the surface, not merely that something is submerged. The 10 categories (past_trauma, secret_love, hidden_resentment, guilt, fear, desire, family_secret, betrayal, loss, ambition) provide a controlled vocabulary for the suppressed content. This enables the model to understand that a scene about discussing the weather is actually about a character's fear of abandonment — and to render the prose accordingly.

---

## 8. Canonical Summary

The `canonical_summary` object is the protocol's core innovation. See [`ROUND_TRIP.md`](ROUND_TRIP.md) for the full specification. Brief field rationale:

### `event_type`
**Rationale:** The scene's principal action classified into 18 cross-culturally stable event categories. Derived from Propp's 31 functions (*Morphology of the Folktale*, 1928) collapsed to their semantic essence, and from McKee's scene design taxonomy. These 18 types (arrival, departure, confrontation, confession, discovery, decision, proposal, refusal, acceptance, betrayal, reconciliation, revelation, deception, rescue, escape, transformation, death, celebration) appear across genres and periods — they are the deep grammar of narrative events.

### `want_outcome`
**Rationale:** McKee's scene design principle: every scene must end with a character either achieving, failing to achieve, or partly achieving their want. The four outcomes — `granted`, `denied`, `deferred`, `pyrrhic` — derive from McKee's *Story* (1997) and Truby's *The Anatomy of Story* (2007). Pyrrhic is essential: a want that is granted at catastrophic cost produces a qualitatively different emotional register than simple achievement.

### `stakes_domain`
**Rationale:** Stakes are what is genuinely at risk. The five domains — physical, emotional, social, professional, existential — derive from Maslow's hierarchy of needs (1943) mapped to narrative theory by numerous practitioners. The `stakes_domain` field answers: *what category of wellbeing is threatened?* Without this, a scene's want outcome is uninterpretable — "DENIED" means differently when the at-risk domain is physical survival vs. social reputation.

### `atmosphere`
**Rationale:** The affective register of the setting as experienced by the reader (distinct from `tone`, which is the narrator's attitude). The 12 values derive from Aristotle's *katharsis* and the Romantic conception of *Stimmung* (mood-atmosphere) developed in German aesthetics (Vischer, 1872; Bollnow, *Das Wesen der Stimmungen*, 1941). Atmosphere is a collective emotional field — the "mood" of a scene — as theorized by Hans Ulrich Gumbrecht (*Atmosphere, Mood, Stimmung*, 2012).

### `causal_role`
**Rationale:** Chatman's kernel/satellite distinction (*Story and Discourse*, 1978) restated as a five-value causal taxonomy: `establishes` (creates conditions), `escalates` (intensifies existing forces), `pivots` (reverses direction), `resolves` (closes an open question), `echoes` (recalls and recontextualizes). This field answers the essential story causation question: "And therefore...? But then...?" E.M. Forster's distinction between story ("the king died and then the queen died") and plot ("the king died and then the queen died of grief") — *Aspects of the Novel* (1927) — is operationalized here.

---

## 9. Prose

### `prose`
**Type:** string  
**Rationale:** The actual narrative passage aligned to this scene card. This is the syuzhet — Tomashevsky's layer of artistic arrangement. The `prose` field is the generation output and extraction input. Its length is bounded by practical training constraints (target: 500–2000 words) derived from the observation that canonical literature rarely sustains single scenes beyond this range without a chapter break.

### `word_count`
**Rationale:** Provides an objective, computable pacing signal. Ratio of `word_count` to `narrative_time.duration` gives discourse density — the compression ratio of story time into discourse time, formalizing Genette's *durée* at the passage level.

### `chapter_position`
**Rationale:** Where a scene falls within its chapter: `opening`, `early`, `middle`, `late`, `closing`, `entire_chapter`. Affects reader experience: opening scenes create chapter expectation, closing scenes trigger chapter-level satisfaction or frustration (Sternberg's *curiosity/suspense/surprise* model, *Expositional Modes*, 1978). Final scenes bear the weight of chapter-level resolution; opening scenes bear the weight of commitment.

---

## References

| Work | Author | Year | Fields |
|------|--------|------|--------|
| *Poetics* | Aristotle | c.335 BCE | `act`, `scene_function` |
| *Aspects of the Novel* | Forster, E.M. | 1927 | `causal_role` |
| *Morphology of the Folktale* | Propp, V. | 1928 | `event_type`, `beat` |
| *Death in the Afternoon* | Hemingway, E. | 1932 | `subtext.iceberg_category` |
| *Anatomy of Criticism* | Frye, N. | 1957 | `motifs_present` |
| *The Rhetoric of Fiction* | Booth, W. C. | 1961 | `pov`, `narrator_reliability` |
| *The Flame of a Candle* | Bachelard, G. | 1961 | `lighting_source` |
| *Narrative Discourse* | Genette, G. | 1972 | `focalization`, `narrative_time.*`, `diegetic_level` |
| *Story and Discourse* | Chatman, S. | 1978 | `arc_position`, `causal_role`, `event_significance` |
| *Expositional Modes* | Sternberg, M. | 1978 | `narrative_time.order`, `chapter_position` |
| *Transparent Minds* | Cohn, D. | 1978 | `consciousness_mode`, `psychic_distance` |
| *Narratology* | Bal, M. | 1985 | `focalization`, `focalizer` |
| *The Art of Fiction* | Gardner, J. | 1983 | `psychic_distance` |
| *Narrative Fiction* | Rimmon-Kenan, S. | 1983 | `pov`, `focalization` |
| *"Logic and Conversation"* | Grice, H.P. | 1975 | `subtext.maxim_violated` |
| *Story* | McKee, R. | 1997 | `beat`, `turn`, `want_outcome`, `causal_role` |
| *The Anatomy of Story* | Truby, J. | 2007 | `beat`, `scene_function`, `turn` |
| *Save the Cat!* | Snyder, B. | 2005 | `sequence`, `beat` |
| *How Fiction Works* | Wood, J. | 2008 | `target_pacing` |
| *The Structure of the Artistic Text* | Lotman, Y. | 1977 | `setting`, `spatial_structure` |
| *"Hamlet and His Problems"* | Eliot, T.S. | 1919 | `weather`, `lighting_quality` |
| *"The Reality Effect"* | Barthes, R. | 1968 | `props` |
| *The Presentation of Self* | Goffman, E. | 1959 | `territory_type` |
| *The Philosophy of Horror* | Carroll, N. | 1990 | `target_tension` |
