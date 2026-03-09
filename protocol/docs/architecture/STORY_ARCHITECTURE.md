# Story Architecture: Architecture & Theoretical Rationale

**Schema:** [`schema/story-architecture.schema.json`](../schema/story-architecture.schema.json)  
**Version:** 1.0.0

The `StoryArchitecture` document captures book-level structural metadata: the genre contract, the collision architecture, the inciting incident, the antagonist design, the protagonist's arc, the actantial model, transtextual relations, and the thematic framework. It is the only schema that operates at the *whole-book* level rather than the scene or character level.

---

## Design Principle

A story is not a sequence of scenes — it is a structured argument. The thematic claim (McKee's "controlling idea," the Grimoire's premise) generates the story's moral and emotional logic, which in turn determines what kind of protagonist, antagonist, arc, and collision the story requires. The `StoryArchitecture` document makes this top-level design explicit so that scene-level generation can proceed with awareness of the whole.

This document formalizes the concept Gilles Deleuze described for cinema but which applies equally to narrative fiction: every work has a *plane of immanence* — a pre-individual level where the whole is already virtual in the parts. The architecture schema makes that plane explicit.

---

## 1. Book Identity

### `book_id` / `title` / `author`
**Rationale:** Same as Registry. The `StoryArchitecture` document is per-book and namespace-aligned with the registry it supplements.

---

## 2. Genre

### `genre.primary`
**Type:** enum (`genre_type`)  
**Required**  
**Rationale:** Genre is the reader's contract — it sets expectations of form, content, affect, and resolution. The theoretical literature on genre is vast, but three foundational frameworks inform the enum:

1. **Northrop Frye's modal theory** (*Anatomy of Criticism*, 1957) — genres (romance, tragedy, irony/satire, comedy) are defined by the hero's power relative to the world. Mode determines what resolutions are possible; tragedy cannot end in genuine triumph.  
2. **Rick Altman's semantic/syntactic theory** (*Film/Genre*, 1999) — genre has both *semantic* elements (the building blocks: icons, settings, character types) and *syntactic* elements (the structural relationships between them). Both must match for a genre classification to hold.  
3. **Derrida's "Law of Genre"** (*Parages*, 1986) — genre is never pure; every text participates in genre while marking that participation as both necessary and impossible. The `secondary` field formalizes this contamination.

### `genre.secondary` / `genre.subgenre`
**Rationale:** Genre mixing is the rule, not the exception. Derrida's contamination principle (*"Law of Genre"*, 1986) observes that texts always participate in more than one genre. Frederik Jameson's concept of *generic discontinuities* (*The Political Unconscious*, 1981) argues that genre conflict within a text is ideologically significant. The secondary genre captures the hybridization; the free-string subgenre allows specificity ("southern gothic," "cozy mystery," "climate fiction") that the enum cannot.

---

## 3. Collision Architecture

### `collision_architecture.collision_type`
**Type:** enum (`collision_type`)  
**Rationale:** The Grimoire's Phase 02 collision engine models the *structural conflict* between social worlds as distinct from character conflict. A *collision* is the meeting of two incompatible *social circles* — each with its own norms, power structures, and value systems — that the protagonist must navigate. This derives from Lukács's concept of the novel as the site where individualism collides with social totality (*Theory of the Novel*, 1920), and from Bourdieu's field theory (*The Field of Cultural Production*, 1983): each social field has its own logic, and collision occurs when characters move between fields or when fields overlap.

### `collision_architecture.collision_pattern`
**Type:** enum (`collision_pattern`)  
**Rationale:** The structural shape of the collision. Derived from Aristotle's hamartia-nemesis-catharsis pattern, expanded by Truby's 22-step structure and the Grimoire collision taxonomy. Patterns include: the outsider entering a closed world (upward mobility, immigration narratives), the insider confronting the world's corruption (institutional critique), the two-world collision (characters from incompatible social worlds forced together), the worlds-at-war pattern (systematic antagonism), and the self-collision (internal contradiction in the protagonist mirrors external social conflict). Each pattern deterministically shapes what supporting cast, obstacles, and thematic opportunities are available.

### `collision_architecture.power_asymmetry`
**Type:** enum (`power_asymmetry_type`)  
**Rationale:** Power asymmetry defines which social world has structural advantage when the collision occurs. This is Bourdieu's *habitus* in action: different social worlds carry different volumes of economic, social, cultural, and symbolic capital (*Distinction*, 1984). The collision is never between equals; the power asymmetry determines which direction pressure flows. This in turn determines the protagonist's tactical options (assimilation, resistance, subversion, transcendence) and the story's ideological valence.

---

## 4. Inciting Incident

### `inciting_incident.type`
**Type:** enum (`inciting_incident_type`)  
**Rationale:** The inciting incident is the plot event that launches the story's central action, typically by disrupting the protagonist's ordinary world. Its theoretical lineage runs from Aristotle's *metabasis* (transition from one state to another), through Freytag's *erregedes Moment* (*Technique of the Drama*, 1863), to McKee's formal definition: "an event that radically upsets the balance of forces in the protagonist's life" (*Story*, 1997). The 12-type taxonomy classifies the *structural* nature of the disruption: arrival_of_stranger, discovery, loss, threat, invitation, accident, confrontation, revelation, decision, death_loss, encounter, opportunity. Each type activates different narrative logics — a "discovery" incident creates a mystery spine; a "death_loss" incident creates a grief arc.

### `inciting_incident.chapter`
**Type:** integer  
**Rationale:** Position of the inciting incident in the story. The critical debate concerns ideal placement. McKee recommends the inciting incident within the first 25% of the text (*Story*, 1997). Snyder's formula places it at page 12/110 (~11%) in screenplay terms (*Save the Cat!*, 2005). Late inciting incidents are a characteristic of literary fiction (Flaubert delays; Chekhov's three-sister pattern establishes *before* inciting). The field enables positioning constraint for generation.

---

## 5. Antagonist

### `antagonist.antagonist_type`
**Type:** enum (`antagonist_type`)  
**Rationale:** John Truby's antagonist taxonomy from *The Anatomy of Story* (2007) distinguishes six fundamental antagonist structures: the external antagonist (separate character with opposing want), the hidden antagonist (unknown or concealed identity), the system antagonist (no individual villain — the institution, society, or environment is the opponent), the inner antagonist (the protagonist's own psychology), the double antagonist (two competing antagonist forces), and the anti-hero-as-protagonist (the protagonist is their own primary antagonist). The type determines available narrative moves — external antagonists admit of confrontation; systemic antagonists require different resolution strategies.

### `antagonist.arc_type`
**Type:** enum (`antagonist_arc_type`)  
**Rationale:** Does the antagonist change? Truby (*Anatomy of Story*, 2007) and Weiland (*Creating Character Arcs*, 2016) identify the full antagonist arc as a distinct narrative choice with consequences for theme. A static antagonist (no arc) makes the strongest thematic statement about the protagonist's change; a dynamic antagonist (positive or negative arc) creates a counterpoint to the protagonist's journey and can deepen thematic complexity. Three types: `static` (hardened by story's end), `disillusionment` (was right but is broken by opposition), `corruption` (compromises principles).

### `antagonist.opposition_level`
**Type:** enum (`opposition_level`)  
**Rationale:** The structural depth of the antagonist's challenge to the protagonist. Five levels: surface (tactical/physical conflict only), relational (emotional and personal), value (opposing worldviews and ethics), thematic (the antagonist's way of life is the story's negative image of the theme), existential (the antagonist threatens the protagonist's fundamental identity or continued existence). The opposition level must be proportional to the story's ambition — an existential protagonist arc requires at least a value-level antagonist.

### `antagonist.thematic_mirror`
**Type:** boolean  
**Rationale:** Truby's most important antagonist principle: the best antagonists are *thematic mirrors* of the protagonist — they want the same thing but pursue it through morally opposite means. This creates what Truby calls the "moral argument" at the heart of the story (*Anatomy of Story*, 2007). Walter White and Gus Fring; Hamlet and Claudius; Jane Eyre and Bertha Mason. When `thematic_mirror == true`, the system treats the antagonist's design as constrained by the protagonist's, and vice versa.

---

## 6. Protagonist Arc

### `protagonist_arc.arc_direction`
**Type:** enum (`arc_direction`)  
**Rationale:** See REGISTRY.md §2 and CHARACTER_STATE.md §7 for full rationale. At the story-architecture level, `arc_direction` is the protagonist's *macro* arc across the whole narrative — the shape of their change. This is the commitments that all scene-level arc beats must conform to.

### `protagonist_arc.drive_model`
**Type:** enum (`drive_model`)  
**Rationale:** See CHARACTER_STATE.md §7. At the story-architecture level, this is the protagonist's primary motivational system — the "why" that powers the whole arc.

### `protagonist_arc.lie_believed`
**Type:** string  
**Rationale:** K.M. Weiland's concept of the *lie the character believes* from *Creating Character Arcs* (2016) — the false belief about themselves or the world that constitutes the protagonist's wound-derived misperception. The lie is what must be overcome for a positive arc. Lisa Cron (*Wired for Story*, 2012) grounds this in neuroscience: narratives engage readers by staging the tension between a character's flawed belief and the evidence the story marshals against it. Declared as a free string because the specific content of the lie is always narrative-particular.

### `protagonist_arc.truth_needed`
**Type:** string  
**Rationale:** Weiland's complement to the lie — the *truth* the protagonist must internalize to complete the arc. The moral of the story, stated as a character-specific belief rather than a general claim. "Others can be trusted" (after a wound of betrayal), "I am worthy of love" (after a wound of shame). Free string for the same reason as the lie.

### `protagonist_arc.want_need_alignment`
**Type:** enum (`want_need_alignment`)  
**Rationale:** The macro-arc state of the want/need gap at story start. See CHARACTER_STATE.md §8. At the story level, this encodes the protagonist's *opening position* — typically `unaware` (doesn't know about the need) or `opposed` (the want actively prevents the need). The arc moves this toward `aligned` (positive arc), keeps it `opposed` (flat arc where need is already held), or moves it `opposed` from `converging` (negative arc — regression).

---

## 7. Structure

### `structure.act_count`
**Type:** integer (1–5)  
**Rationale:** Act count is a formal commitment. Three-act structure derives from Aristotle's protasis/epitasis/catastrophe (*Poetics*) and Field's paradigm (1979) for screenplays. Five acts are Shakespearean and are still used in stage drama. Two-act structures appear in comedy. Four-act structures are common in serialized fiction (four-part chapter structures). The system must know the act count to interpret `SceneCard.act` values correctly.

### `structure.chapter_count` / `structure.word_count`
**Rationale:** Physical structure of the book. Word count determines pacing expectations — a scene that takes 3,000 words in a 90,000 word literary novel has different proportional weight than the same scene in a 40,000 word novella. Chapter count enables calculation of mean chapter density.

### `structure.diegetic_level`
**Type:** enum (`diegetic_level`)  
**Rationale:** The *primary* diegetic level of the main narrative. Most novels are intradiegetic. Frame narratives begin extradiegetic. This field enables the system to correctly parse `SceneCard.diegetic_level` values — a scene declared as "extradiegetic" in a story whose primary level is already extradiegetic is structurally different from the same declaration in an intradiegetic narrative.

### `structure.has_frame_narrative`
**Type:** boolean  
**Rationale:** Frame narratives (stories-within-stories) require special handling: they imply at least two diegetic levels, two narrators, and two time registers. Genette's analysis (*Narrative Discourse*, 1972) and Gerard Genette's concept of *métalepsis* (transgression across diegetic levels) both require advance declaration of whether a frame structure exists. Examples: *Frankenstein*, *Heart of Darkness*, *The Name of the Rose*, *The Princess Bride*. Without this flag, the extraction pipeline cannot correctly interpret a scene at the "wrong" diegetic level.

---

## 8. Actantial Map

### `actantial_map[]`
**Type:** array of `{character_slug, character_role, actantial_role}`  
**Rationale:** Greimas's actantial model (*Structural Semantics*, 1966) at the story level — the global cast of positions in the narrative grammar. Each character declared in the registry may occupy one or more actantial positions in the whole-story structure. The actantial map enables:
1. **Confirmation of structural completeness** — a story without a declared Sender has an ungrounded moral framework; a story without an Opponent has no conflict.
2. **Cross-scene consistency checking** — a character declared as Subject-level should be the objective-holder in most scenes.
3. **Thematic analysis** — the Object is the story's central value (love, justice, survival, identity); making it explicit enables thematic coherence checking.

This differs from `actantial_role` in `CharacterState` (which is scene-specific and can vary) — the story-level actantial map is the macro grammar.

---

## 9. Transtextuality

### `transtextuality.intertexts[]`
**Type:** array of `{source_text, relation_type, description}`  
**Rationale:** Gérard Genette's *Palimpsests: Literature in the Second Degree* (1982/1997) defines five forms of textual transcendence:
1. **Intertextuality** — the actual presence of one text in another (quotation, allusion, plagiarism)
2. **Paratextuality** — the relationship with threshold elements (title, epigraph, preface)
3. **Metatextuality** — the commentary relation (criticism, gloss)
4. **Hypertextuality** — the relation of transformation or imitation (parody, pastiche, travesty)
5. **Architextuality** — the generic/taxonomic relation (what *kind* of text this is)

The `transtextuality_type` enum captures all five. For the extraction pipeline, identifying intertextual relations is critical: a text modeled as a hypotext transformation of another requires knowledge of the hypotext to analyze correctly. Harold Bloom's *anxiety of influence* (*The Anxiety of Influence*, 1973) provides the psychological model for why intertextual relations are rarely neutral — they are acts of revision, tribute, or contestation.

### `transtextuality.architext_genre`
**Type:** string  
**Rationale:** The genre tradition the text participates in — stated as the architext relation (Genette's fifth transtextual category). This is distinct from `genre.primary` (which classifies the text) — the architext is the genre *tradition* being joined or contested. "The Gothic novel" (Shelley, Radcliffe, Horace Walpole) is an architextual tradition; "horror" is a genre classification. Free string because genre traditions are historically specific and cannot be fully enumerated.

---

## 10. Themes

### `themes[].theme`
**Type:** string  
**Rationale:** The thematic concern as a noun phrase — not the controlling idea (which combines value and cause) but the raw subject. "Power and corruption," "the limits of maternal love," "memory and identity." Distinct from the controlling idea because a single theme can generate multiple controlling ideas depending on how the story answers it.

### `themes[].controlling_idea`
**Type:** string  
**Rationale:** Robert McKee's concept from *Story* (1997) — the story's irreducible meaning expressed as **Value + Cause**: "Justice triumphs because the protagonist risks everything for others instead of themselves." The controlling idea is not a theme (which is a question) but a claim (which takes sides). It is the one sentence that could replace the entire text without losing its essential meaning. Distinguished from Lajos Egri's *dramatic premise* (*The Art of Dramatic Writing*, 1946), which has a similar three-part structure (quality → conflict → consequence) but differs in that Egri's premise is predictive ("greed leads to destruction") while McKee's controlling idea is evaluative ("here is why love fails/succeeds, under these conditions"). Free string because controlling ideas are always specific to the story's events.

### `themes[].thematic_question`
**Type:** enum (`thematic_question`)  
**Rationale:** The human question the theme addresses — the story's deep inquiry. The vocabulary (justice_and_fairness, identity_and_self, love_and_connection, power_and_corruption, freedom_and_constraint, truth_and_deception, mortality, belonging, sacrifice, redemption, violence_and_harm, memory_and_past, faith, ambition, family, legacy) derives from the cross-cultural study of narrative themes (Martha Nussbaum, *Poetic Justice*, 1995; Nussbaum, *Love's Knowledge*, 1990) and the cognitive narratology finding that readers engage stories to model social situations. The thematic question names *which* situation type the story models.

---

## References

| Work | Author | Year | Fields |
|------|--------|------|--------|
| *Poetics* | Aristotle | c.335 BCE | `act_count`, `inciting_incident.type`, dramatic premise |
| *Anatomy of Criticism* | Frye, N. | 1957 | `genre.primary` |
| *Theory of the Novel* | Lukács, G. | 1920 | `collision_architecture` |
| *Technique of the Drama* | Freytag, G. | 1863 | `inciting_incident` |
| *The Art of Dramatic Writing* | Egri, L. | 1946 | `controlling_idea` |
| *Hero with a Thousand Faces* | Campbell, J. | 1949 | `inciting_incident.type` |
| *Narrative Discourse* | Genette, G. | 1972 | `has_frame_narrative`, `diegetic_level` |
| *Palimpsests* | Genette, G. | 1982 | `transtextuality.*` |
| *The Anxiety of Influence* | Bloom, H. | 1973 | `transtextuality.intertexts` |
| *Story* | McKee, R. | 1997 | `inciting_incident`, `controlling_idea`, `act_count` |
| *Save the Cat!* | Snyder, B. | 2005 | `inciting_incident.chapter` |
| *The Anatomy of Story* | Truby, J. | 2007 | `antagonist.*`, `collision_architecture`, `actantial_map` |
| *Creating Character Arcs* | Weiland, K.M. | 2016 | `protagonist_arc.*`, `antagonist.arc_type` |
| *Wired for Story* | Cron, L. | 2012 | `protagonist_arc.lie_believed` |
| *Structural Semantics* | Greimas, A.J. | 1966 | `actantial_map` |
| *Film/Genre* | Altman, R. | 1999 | `genre.*` |
| *"Law of Genre"* | Derrida, J. | 1986 | `genre.secondary` |
| *The Political Unconscious* | Jameson, F. | 1981 | `genre.secondary` |
| *The Field of Cultural Production* | Bourdieu, P. | 1983 | `collision_architecture.*` |
| *Distinction* | Bourdieu, P. | 1984 | `power_asymmetry` |
| *Poetic Justice* | Nussbaum, M. | 1995 | `thematic_question` |
| *Love's Knowledge* | Nussbaum, M. | 1990 | `thematic_question` |
