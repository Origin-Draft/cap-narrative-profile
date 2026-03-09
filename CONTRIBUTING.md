# Contributing to Grimoire

Thank you for your interest in improving Grimoire! This guide will help you understand our standards and workflow.

---

## 🎯 Project Mission

Grimoire exists to help authors write better books with AI assistance. Every contribution should serve that goal by:
- Making templates clearer and more actionable
- Providing better context for AI collaboration
- Improving the author's creative workflow
- Maintaining consistency and quality across the project

---

## 📝 Template Style Guidelines

All Grimoire templates follow a consistent structure. When creating or editing templates:

### Required Structure

Each guided-workbook template must include:

1. **Header Block**
   ```markdown
   # [Template Title]
   
   **Phase:** [00-08 Phase Name]  
   **Purpose:** [One-line description of what this template does]  
   **Time to Complete:** [Estimated time: 15-30 min, 1-2 hours, etc.]
   ```

2. **Why This Matters** (2–4 paragraphs)
   - Explain the craft rationale behind this template
   - Why this step is important in the book-writing process
   - What problems it solves or prevents

3. **Before You Begin** (Prerequisites)
   - List which templates should be filled out first
   - Link to dependencies: `[book-brief.md](../00-start-here/book-brief.md)`
   - Note if this template can be completed standalone

4. **The Template** (Main Content)
   - Use clear section headers (`##` or `###`)
   - Include guiding questions in each section
   - Add tips as blockquotes: `> **Tip:** Stay focused on...`
   - Provide examples in collapsible sections:
     ```markdown
     <details>
     <summary>Example: [What the example shows]</summary>
     
     [Example content here]
     
     </details>
     ```
   - Use fill-in fields with square brackets: `[Your answer here]`

5. **Using This With AI**
   - Explain how to use the completed template as AI context
   - Provide 1–2 example prompt patterns
   - Link to the relevant prompt file: `[concept-prompts.md](../prompts/concept-prompts.md)`

6. **Related Templates & Workflows**
   - Link to connected templates
   - Reference relevant workflow files
   - Create a clear navigation path through the system

### Writing Style

- **Tone:** Supportive, professional, encouraging. Avoid condescension.
- **Voice:** Second person ("you"), conversational but not chatty
- **Clarity:** Prefer simple words over jargon. When technical terms are necessary, define them.
- **Brevity:** Respect the author's time. Be thorough but not verbose.
- **Inclusivity:** Use examples from diverse genres, cultures, and storytelling traditions

### Formatting

- Use **bold** for emphasis, not *italics* (which render poorly in some editors)
- Use `inline code` for technical terms, not for general emphasis
- Use tables for comparison content or structured data
- Use bullet points for lists, numbered lists only when sequence matters
- Keep line length reasonable (no hard limit, but aim for ~100 characters for readability)
- Add blank lines between sections for breathing room

---

## 🚦 Phase Gate Guidelines

Each phase directory has a `_gate.md` file that defines automated readiness checks. These gates enforce second-thought before advancing and remove ambiguity about whether a phase is complete.

### What `_gate.md` Files Are

Every `_gate.md` is an underscore-prefixed guide file (following the existing convention) with:
1. **YAML frontmatter** — machine-readable gate question definitions
2. **Markdown body** — human-readable explanation of the gate criteria, a checklist, and tag examples

### Gate Question Schema

Each gate question in the frontmatter is a YAML object:

```yaml
---
phase: "02-characters"
phase_label: "02 · Characters"
gates:
  - id: cast_profiles_complete          # unique snake_case identifier
    question: "human-readable question"  # shown in reports
    check_type: placeholder_ratio         # see check types below
    target_file: "cast-overview.md"      # relative to the phase directory
    max_placeholder_pct: 30              # check-type-specific parameter
    severity: required                   # required | recommended
---
```

### Check Types

| `check_type` | Required Parameters | What It Checks |
|---|---|---|
| `file_exists` | `target_file` | File exists and has ≥ 20 words |
| `placeholder_ratio` | `target_file`, `max_placeholder_pct` | Unfilled `[placeholder]` ratio ≤ threshold |
| `word_count_min` | `target_file`, `min_words` | Filled word count ≥ minimum |
| `checkbox_completion` | `target_file`, `min_completion_pct` | `- [x]` / total checkboxes ≥ threshold |
| `tag_cross_ref` | `source_file`, `source_tag`, `target_file`, `target_tag` | Every entity tagged in source appears in target |
| `entity_coverage` | `source_file`, `source_tag`, `target_file`, `target_tag` | Every source entity has an entry in target file |

### Severity

- **`required`** — a red result on this gate makes the entire phase red (blocks advancement)
- **`recommended`** — a red result caps the phase at yellow (attention needed, but not a hard block)

### Tag Vocabulary

Cross-reference checks rely on HTML comment annotations embedded in markdown files:

```markdown
<!-- character:iris_nakamura -->
<!-- relationship:iris_nakamura→kai_chen type:adversarial -->
<!-- beat:midpoint characters:iris_nakamura setting:warehouse_district -->
<!-- scene:opening_hook beat:inciting_incident pov:iris_nakamura setting:city_name -->
```

The full vocabulary is documented in [references/gate-tag-vocabulary.md](references/gate-tag-vocabulary.md). When contributing content to templates, add appropriate tags to enable cross-reference checks.

### Running the Gate Checker

```bash
# Check all phases
python scripts/gate_check.py

# Check a single phase
python scripts/gate_check.py --phase 02

# No color output (CI / pipe)
python scripts/gate_check.py --no-color
```

Results are written to `readiness.yaml` (machine-readable) and `readiness-report.md` (markdown).

### Adding a New Gate

When contributing a new template, also update the relevant `_gate.md` to add gate questions that check the new file. At minimum, add a `placeholder_ratio` check. If the template introduces named entities (characters, settings, beats), add appropriate tag vocabulary examples in the `_gate.md` body.

---

## 🤖 Prompt Style Guidelines

Prompts in the `prompts/` directory should be:

### Effective
- **Specific:** Tell the AI exactly what kind of output you want
- **Contextual:** Always indicate which template(s) to paste in as context
- **Actionable:** Result in concrete output the author can use

### Platform-Agnostic
- Don't reference API-specific features (no "use Code Interpreter" or "search the web")
- Avoid platform names unless necessary for clarity
- Write in plain language that works across AI assistants

### Structured

Each prompt file should include:

```markdown
# [Phase] Prompts

**Associated Templates:** [List with links]

---

## Context Block Template

Before using these prompts, paste the following as context:

```
[Show the exact format of context to provide]
```

---

## Prompts

### [Prompt Category 1]: [Brief Description]

[The actual prompt text]

**When to use:** [Scenario description]

---

[Repeat for each prompt]
```

---

## 🔄 Workflow Guidelines

Workflow documents in the `workflows/` directory define protocols for human-AI collaboration:

- **Be prescriptive:** Specify who does what, when, and how
- **Define handoffs:** Make role transitions explicit
- **Include examples:** Show the workflow in action
- **Provide escape hatches:** Explain when to override the protocol
- **Link heavily:** Connect to relevant templates and prompts

---

## 🐛 What to Contribute

### High-Value Contributions

- **Fill gaps:** Add templates for underserved writing aspects
- **Improve clarity:** Rewrite confusing sections
- **Add examples:** Real-world samples make templates actionable
- **Fix errors:** Broken links, typos, outdated info
- **Expand workflows:** More detailed protocols for the agent-author loop
- **Genre specificity:** Add genre-specific guidance or variants

### Not Currently Accepting

- Platform-specific integrations (plugins, extensions)
- Monetization features
- Templates unrelated to book writing
- Alternative directory structures (we're committed to the numbered-phase system)

---

## 🚀 Contribution Workflow

1. **Open an issue** to discuss significant changes before starting work
2. **Fork** the repository
3. **Create a branch** with a descriptive name: `add-mystery-genre-guide`
4. **Make your changes** following the style guidelines above
5. **Test your changes:**
   - Verify all internal links work
   - Ensure formatting renders correctly
   - Check that examples are clear and accurate
6. **Submit a pull request** with:
   - Clear description of what changed and why
   - Reference to any related issues
   - Notes on testing done

### Pull Request Review

Maintainers will review for:
- ✅ Adherence to template structure
- ✅ Writing quality and clarity
- ✅ Accuracy of craft advice
- ✅ Consistency with existing content
- ✅ Functional links and formatting

---

## 📚 Resources for Contributors

### Craft References
If you're adding writing advice, ground it in established craft knowledge. Useful references:
- *Story* by Robert McKee
- *Save the Cat!* by Blake Snyder
- *The Anatomy of Story* by John Truby
- *Scene and Structure* by Jack M. Bickham

### AI Prompting Best Practices
- OpenAI's prompt engineering guide
- Anthropic's prompt design documentation
- General principles: specificity, context, examples, constraints

---

## 💬 Questions?

- **Unclear guidelines?** Open an issue asking for clarification
- **Want to discuss a big idea?** Start a GitHub discussion
- **Found a bug?** Open an issue with reproduction steps

---

## 🙏 Thank You

Every contribution makes Grimoire more useful for authors worldwide. We appreciate your time and expertise!

---

**Code of Conduct:** Be kind, respectful, and constructive. We're all here to help authors tell better stories.
