//! PyO3 Python bindings for `grimoire-types`.
//!
//! Exposes key types to Python so the existing engine scripts can adopt
//! the Rust type system incrementally.  Each binding provides:
//! - `__init__` via dataclass-style `#[new]`
//! - `to_json()` / `from_json()` for serde round-trip
//! - `to_dict()` (returns a Python dict for downstream YAML serialisation)
//!
//! Build with `maturin develop --features python` from the workspace root.

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyDict;

use std::collections::{HashMap, HashSet};

pyo3::create_exception!(grimoire_tooling, CycleError, pyo3::exceptions::PyRuntimeError);

// ── Registration ───────────────────────────────────────────────────────────────

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Enums
    m.add_class::<PyArchetype>()?;
    m.add_class::<PyWound>()?;
    m.add_class::<PyAlignment>()?;
    m.add_class::<PyRole>()?;
    m.add_class::<PyDriveModel>()?;
    m.add_class::<PyGateStatus>()?;
    m.add_class::<PySubPhaseStatus>()?;
    m.add_class::<PyPhaseStatus>()?;
    m.add_class::<PyRevisionFlag>()?;
    // Core types
    m.add_class::<PyEntityRef>()?;
    m.add_class::<PyAnnotation>()?;
    m.add_class::<PyCharacter>()?;
    m.add_class::<PyScene>()?;
    m.add_class::<PyVoiceContract>()?;
    m.add_class::<PyVoiceSignature>()?;
    m.add_class::<PyTrainingExample>()?;
    m.add_class::<PyTrainingDataset>()?;
    m.add_class::<PyGateSpec>()?;
    m.add_class::<PyPhaseSpec>()?;
    m.add_class::<PyGateResult>()?;
    m.add_class::<PyPhaseResult>()?;
    m.add_class::<PySubPhaseSpec>()?;
    m.add_class::<PySubPhaseResult>()?;
    m.add_class::<PyDependencyEdge>()?;
    m.add_class::<PyReadinessSummary>()?;
    m.add_class::<PySubPhaseDAG>()?;
    m.add_class::<PyStoryRecipe>()?;
    // Functions
    m.add_function(wrap_pyfunction!(parse_annotation, m)?)?;
    m.add_function(wrap_pyfunction!(dump_schemas, m)?)?;
    m.add_function(wrap_pyfunction!(build_dag, m)?)?;
    // Exceptions
    m.add("CycleError", m.py().get_type_bound::<CycleError>())?;
    Ok(())
}

// ── Helper macro: JSON-based round-trip for any serde type ────────────────────

macro_rules! json_methods {
    ($py_type:ty, $inner:ty) => {
        #[pymethods]
        impl $py_type {
            fn to_json(&self) -> PyResult<String> {
                serde_json::to_string(&self.inner)
                    .map_err(|e| PyValueError::new_err(e.to_string()))
            }

            #[staticmethod]
            fn from_json(json: &str) -> PyResult<Self> {
                serde_json::from_str(json)
                    .map(|inner| Self { inner })
                    .map_err(|e| PyValueError::new_err(e.to_string()))
            }

            fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
                let json = serde_json::to_value(&self.inner)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
                let obj = pythonize::pythonize(py, &json)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
                obj.extract(py)
            }

            fn __repr__(&self) -> String {
                serde_json::to_string(&self.inner)
                    .unwrap_or_else(|_| "<grimoire_types object>".to_owned())
            }
        }
    };
}

// ── Enum wrappers ─────────────────────────────────────────────────────────────

macro_rules! py_enum {
    ($py_name:ident, $inner:ty, $($variant:ident),+) => {
        #[pyclass]
        pub struct $py_name {
            pub inner: $inner,
        }

        #[pymethods]
        impl $py_name {
            fn __str__(&self) -> String {
                format!("{}", self.inner)
            }
            fn __repr__(&self) -> String {
                format!("{:?}", self.inner)
            }
            fn value(&self) -> String {
                serde_json::to_string(&self.inner)
                    .unwrap_or_default()
                    .trim_matches('"')
                    .to_owned()
            }
            #[staticmethod]
            fn from_str(s: &str) -> PyResult<Self> {
                s.parse::<$inner>()
                    .map(|inner| Self { inner })
                    .map_err(|_| PyValueError::new_err(format!("Invalid value: {:?}", s)))
            }
        }
    };
}

py_enum!(PyArchetype, cap_narrative_types::enums::Archetype, Hero, Mentor, Trickster, Lover, Caregiver, Sage, Innocent, Rebel, Ruler, Creator, Explorer, Magician, Jester, Outlaw);
py_enum!(PyWound, cap_narrative_types::enums::Wound, Abandonment, Betrayal, GuiltAndFailure, TraumaAndAbuse, Shame, Grief, TrustViolation, Powerlessness, IdentityRejection, Injustice, Neglect, SurvivorGuilt, Displacement);
py_enum!(PyAlignment, cap_narrative_types::enums::Alignment, LawfulGood, NeutralGood, ChaoticGood, LawfulNeutral, TrueNeutral, ChaoticNeutral, LawfulEvil, NeutralEvil, ChaoticEvil);
py_enum!(PyRole, cap_narrative_types::enums::Role, Protagonist, Deuteragonist, Antagonist, LoveInterest, Mentor, Confidant, Foil, Trickster, Guardian, Herald, Shapeshifter, Contagonist, WalkOn);
py_enum!(PyDriveModel, cap_narrative_types::enums::DriveModel, Wound, Desire, Duty, Perception, Existential);
py_enum!(PyGateStatus, cap_narrative_types::enums::GateStatus, Green, Yellow, Red, Locked, Unknown);
py_enum!(PySubPhaseStatus, cap_narrative_types::enums::SubPhaseStatus, Locked, Ready, InProgress, Complete);
py_enum!(PyPhaseStatus, cap_narrative_types::enums::PhaseStatus, Green, Yellow, Red, Unknown);
py_enum!(PyRevisionFlag, cap_narrative_types::enums::RevisionFlag, TellingNotShowing, VoiceContractFail, PivotUnclear, SubtextMissing, PacingDrag, ContinuityBreak);

// ── EntityRef ─────────────────────────────────────────────────────────────────

#[pyclass]
pub struct PyEntityRef {
    inner: cap_narrative_types::tags::EntityRef,
}

#[pymethods]
impl PyEntityRef {
    #[new]
    fn new(slug: &str) -> Self {
        Self { inner: cap_narrative_types::tags::EntityRef::new(slug) }
    }
    #[getter]
    fn slug(&self) -> &str { &self.inner.slug }
    fn __str__(&self) -> &str { &self.inner.slug }
    fn __repr__(&self) -> String { format!("EntityRef('{}')", self.inner.slug) }
}

// ── Annotation ────────────────────────────────────────────────────────────────

#[pyclass]
pub struct PyAnnotation {
    inner: cap_narrative_types::tags::Annotation,
}

#[pymethods]
impl PyAnnotation {
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
    fn __repr__(&self) -> String {
        serde_json::to_string(&self.inner).unwrap_or_default()
    }
}

// ── Character ─────────────────────────────────────────────────────────────────

#[pyclass]
pub struct PyCharacter { inner: cap_narrative_types::entities::Character }
json_methods!(PyCharacter, cap_narrative_types::entities::Character);

#[pymethods]
impl PyCharacter {
    #[new]
    fn new(id: &str, name: &str) -> Self {
        use cap_narrative_types::entities::*;
        Self {
            inner: Character {
                observables: CharacterObservables {
                    id: id.to_owned(),
                    name: name.to_owned(),
                    slot: None,
                },
                structure: CharacterStructure {
                    role: None,
                    voice_signature: None,
                },
                interpretations: CharacterInterpretations {
                    archetype: None,
                    wound: None,
                    alignment: None,
                    drive_model: None,
                    arc_type: None,
                    actant: None,
                    ghost: None,
                    want: None,
                    need: None,
                    flaw: None,
                },
            },
        }
    }
    #[getter] fn id(&self) -> &str { &self.inner.observables.id }
    #[getter] fn name(&self) -> &str { &self.inner.observables.name }
}

// ── Scene ─────────────────────────────────────────────────────────────────────

#[pyclass]
pub struct PyScene { inner: cap_narrative_types::entities::Scene }
json_methods!(PyScene, cap_narrative_types::entities::Scene);

#[pymethods]
impl PyScene {
    #[new]
    fn new(id: &str) -> Self {
        Self {
            inner: cap_narrative_types::entities::Scene {
                id: id.to_owned(),
                working_title: None,
                story_position: None,
                pov_character: None,
                attending_characters: Vec::new(),
                setting: None,
                time_of_day: None,
                weather: None,
                goal: None,
                why_goal_matters: None,
                plan: None,
                opponent_or_obstacle: None,
                conflict_type: Vec::new(),
                escalation_beats: Vec::new(),
                dialogue_strategy: None,
                action_strategy: None,
                emotional_escalation: None,
                outcome_type: None,
                what_changed: None,
                new_information: None,
                plant_or_setup: None,
                sequel: None,
                dominant_sense: None,
                key_sensory_details: Vec::new(),
                emotional_weather: None,
                scene_unique_image: None,
                pacing_notes: None,
                target_word_count: None,
                complexity: None,
                priority: None,
                narrative_threads: Vec::new(),
                sequence_id: None,
                scene_type: None,
                tension_level: None,
                prose_directives: None,
            },
        }
    }
    #[getter] fn id(&self) -> &str { &self.inner.id }
}

// ── VoiceSignature ────────────────────────────────────────────────────────────

#[pyclass]
pub struct PyVoiceSignature { inner: cap_narrative_types::voice::VoiceSignature }
json_methods!(PyVoiceSignature, cap_narrative_types::voice::VoiceSignature);

#[pymethods]
impl PyVoiceSignature {
    #[new]
    fn new(character_id: &str) -> Self {
        Self {
            inner: cap_narrative_types::voice::VoiceSignature {
                character_id: character_id.to_owned(),
                ..Default::default()
            },
        }
    }
}

// ── VoiceContract ─────────────────────────────────────────────────────────────

#[pyclass]
pub struct PyVoiceContract { inner: cap_narrative_types::voice::VoiceContract }
json_methods!(PyVoiceContract, cap_narrative_types::voice::VoiceContract);

#[pymethods]
impl PyVoiceContract {
    #[new]
    fn new() -> Self {
        Self { inner: cap_narrative_types::voice::VoiceContract::default() }
    }
}

// ── TrainingExample ───────────────────────────────────────────────────────────

#[pyclass]
pub struct PyTrainingExample { inner: crate::training::TrainingExample }
json_methods!(PyTrainingExample, crate::training::TrainingExample);

#[pymethods]
impl PyTrainingExample {
    fn to_jsonl(&self) -> PyResult<String> {
        self.inner.to_jsonl().map_err(|e| PyValueError::new_err(e.to_string()))
    }
    #[getter] fn id(&self) -> &str { &self.inner.id }
}

// ── TrainingDataset ───────────────────────────────────────────────────────────

#[pyclass]
pub struct PyTrainingDataset { inner: crate::training::TrainingDataset }

#[pymethods]
impl PyTrainingDataset {
    #[getter] fn name(&self) -> &str { &self.inner.name }
    #[getter] fn example_count(&self) -> usize { self.inner.example_count }
    #[getter] fn total_word_count(&self) -> u32 { self.inner.total_word_count }

    fn to_jsonl_string(&self) -> PyResult<String> {
        let mut out = String::new();
        for line in self.inner.to_jsonl_lines() {
            out.push_str(&line.map_err(|e| PyValueError::new_err(e.to_string()))?);
            out.push('\n');
        }
        Ok(out)
    }
}

// ── Gate types ────────────────────────────────────────────────────────────────

#[pyclass]
pub struct PySubPhaseSpec { inner: crate::gates::SubPhaseSpec }
json_methods!(PySubPhaseSpec, crate::gates::SubPhaseSpec);

#[pymethods]
impl PySubPhaseSpec {
    #[new]
    fn new(id: String, label: String, order: u32, depends_on: Vec<String>) -> Self {
        Self { inner: crate::gates::SubPhaseSpec { id, label, order, depends_on } }
    }
    #[getter] fn id(&self) -> &str { &self.inner.id }
    #[getter] fn label(&self) -> &str { &self.inner.label }
    #[getter] fn order(&self) -> u32 { self.inner.order }
    #[getter] fn depends_on(&self) -> Vec<String> { self.inner.depends_on.clone() }

    fn fqid(&self, phase_id: &str) -> String { self.inner.fqid(phase_id) }
}

#[pyclass]
pub struct PyGateSpec { inner: crate::gates::GateSpec }
json_methods!(PyGateSpec, crate::gates::GateSpec);

#[pymethods]
impl PyGateSpec {
    #[new]
    #[pyo3(signature = (id, question, check_type, severity, sub_phase=None, target_file=None, source_file=None, source_tag=None, target_tag=None, max_placeholder_pct=None, min_words=None, min_completion_pct=None))]
    fn new(
        id: String,
        question: String,
        check_type: &str,
        severity: &str,
        sub_phase: Option<String>,
        target_file: Option<String>,
        source_file: Option<String>,
        source_tag: Option<String>,
        target_tag: Option<String>,
        max_placeholder_pct: Option<f32>,
        min_words: Option<u32>,
        min_completion_pct: Option<f32>,
    ) -> PyResult<Self> {
        let ct: cap_narrative_types::enums::CheckType = check_type.parse()
            .map_err(|_| PyValueError::new_err(format!("Invalid check_type: {check_type}")))?;
        let sev: cap_narrative_types::enums::Severity = severity.parse()
            .map_err(|_| PyValueError::new_err(format!("Invalid severity: {severity}")))?;
        Ok(Self {
            inner: crate::gates::GateSpec {
                id, question, check_type: ct, severity: sev, sub_phase,
                target_file, source_file, source_tag, target_tag,
                max_placeholder_pct, min_words, min_completion_pct,
            }
        })
    }
    #[getter] fn id(&self) -> &str { &self.inner.id }
    #[getter] fn question(&self) -> &str { &self.inner.question }
    #[getter] fn sub_phase(&self) -> Option<&str> { self.inner.sub_phase.as_deref() }
    #[getter] fn target_file(&self) -> Option<&str> { self.inner.target_file.as_deref() }
    #[getter] fn severity(&self) -> String { serde_json::to_string(&self.inner.severity).unwrap_or_default().trim_matches('"').to_owned() }
}

#[pyclass]
pub struct PyGateResult { inner: crate::gates::GateResult }
json_methods!(PyGateResult, crate::gates::GateResult);

#[pymethods]
impl PyGateResult {
    #[new]
    #[pyo3(signature = (gate_id, question, severity, status, detail=None, sub_phase=None))]
    fn new(
        gate_id: String,
        question: String,
        severity: &str,
        status: &str,
        detail: Option<String>,
        sub_phase: Option<String>,
    ) -> PyResult<Self> {
        let sev: cap_narrative_types::enums::Severity = severity.parse()
            .map_err(|_| PyValueError::new_err(format!("Invalid severity: {severity}")))?;
        let st: cap_narrative_types::enums::GateStatus = status.parse()
            .map_err(|_| PyValueError::new_err(format!("Invalid status: {status}")))?;
        Ok(Self {
            inner: crate::gates::GateResult {
                gate_id, question, severity: sev, status: st, detail, sub_phase,
            }
        })
    }
    #[getter] fn gate_id(&self) -> &str { &self.inner.gate_id }
    #[getter] fn question(&self) -> &str { &self.inner.question }
    #[getter] fn status(&self) -> String { serde_json::to_string(&self.inner.status).unwrap_or_default().trim_matches('"').to_owned() }
    #[getter] fn severity(&self) -> String { serde_json::to_string(&self.inner.severity).unwrap_or_default().trim_matches('"').to_owned() }
    #[getter] fn detail(&self) -> Option<&str> { self.inner.detail.as_deref() }
    #[getter] fn sub_phase(&self) -> Option<&str> { self.inner.sub_phase.as_deref() }
}

#[pyclass]
pub struct PyPhaseSpec { inner: crate::gates::PhaseSpec }
json_methods!(PyPhaseSpec, crate::gates::PhaseSpec);

#[pymethods]
impl PyPhaseSpec {
    #[new]
    fn new(phase_id: String, phase_label: String) -> Self {
        Self {
            inner: crate::gates::PhaseSpec {
                phase_id, phase_label, sub_phases: Vec::new(), gates: Vec::new(),
            }
        }
    }
    #[getter] fn phase_id(&self) -> &str { &self.inner.phase_id }
    #[getter] fn phase_label(&self) -> &str { &self.inner.phase_label }

    fn fqid(&self, sub_phase_id: &str) -> String { self.inner.fqid(sub_phase_id) }

    fn gate(&self, gate_id: &str) -> Option<PyGateSpec> {
        self.inner.gate(gate_id).map(|g| PyGateSpec { inner: g.clone() })
    }
}

#[pyclass]
pub struct PySubPhaseResult { inner: crate::gates::SubPhaseResult }
json_methods!(PySubPhaseResult, crate::gates::SubPhaseResult);

#[pymethods]
impl PySubPhaseResult {
    #[getter] fn status(&self) -> String { serde_json::to_string(&self.inner.status).unwrap_or_default().trim_matches('"').to_owned() }
    #[getter] fn score(&self) -> u32 { self.inner.score }
    #[getter] fn spec(&self) -> PySubPhaseSpec { PySubPhaseSpec { inner: self.inner.spec.clone() } }
}

#[pyclass]
pub struct PyPhaseResult { inner: crate::gates::PhaseResult }
json_methods!(PyPhaseResult, crate::gates::PhaseResult);

#[pymethods]
impl PyPhaseResult {
    #[getter] fn phase_id(&self) -> &str { &self.inner.phase_id }
    #[getter] fn phase_label(&self) -> &str { &self.inner.phase_label }
    #[getter] fn status(&self) -> String { serde_json::to_string(&self.inner.status).unwrap_or_default().trim_matches('"').to_owned() }
    #[getter] fn score(&self) -> u32 { self.inner.score }
    #[getter] fn error(&self) -> Option<&str> { self.inner.error.as_deref() }
}

#[pyclass]
pub struct PyDependencyEdge { inner: crate::gates::DependencyEdge }
json_methods!(PyDependencyEdge, crate::gates::DependencyEdge);

#[pymethods]
impl PyDependencyEdge {
    #[new]
    fn new(source: String, target: String) -> Self {
        Self { inner: crate::gates::DependencyEdge { source, target } }
    }
    #[getter] fn source(&self) -> &str { &self.inner.source }
    #[getter] fn target(&self) -> &str { &self.inner.target }
}

#[pyclass]
pub struct PyReadinessSummary { inner: crate::gates::ReadinessSummary }
json_methods!(PyReadinessSummary, crate::gates::ReadinessSummary);

#[pymethods]
impl PyReadinessSummary {
    fn completed_fqids(&self) -> Vec<String> {
        self.inner.completed_fqids().into_iter().collect()
    }
}

// ── SubPhaseDAG ───────────────────────────────────────────────────────────────

#[pyclass]
pub struct PySubPhaseDAG {
    inner: crate::dag::SubPhaseDAG,
}

#[pymethods]
impl PySubPhaseDAG {
    fn is_unlocked(&self, fqid: &str, completed: HashSet<String>) -> bool {
        self.inner.is_unlocked(fqid, &completed)
    }

    fn next_actionable(&self, completed: HashSet<String>) -> Vec<String> {
        self.inner.next_actionable(&completed)
    }

    fn blocking_path(&self, fqid: &str, completed: HashSet<String>) -> Vec<String> {
        self.inner.blocking_path(fqid, &completed)
    }

    fn topological_order(&self) -> Vec<String> {
        self.inner.topological_order().to_vec()
    }

    fn compute_status(
        &self,
        fqid: &str,
        gate_results_json: &str,
        completed: HashSet<String>,
    ) -> PyResult<String> {
        let gate_results: HashMap<String, crate::gates::GateResult> =
            serde_json::from_str(gate_results_json)
                .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let status = self.inner.compute_status(fqid, &gate_results, &completed);
        Ok(serde_json::to_string(&status).unwrap_or_default().trim_matches('"').to_owned())
    }
}

/// Build a SubPhaseDAG from a JSON string of phase specs.
///
/// Raises `CycleError` if the dependency graph contains a cycle.
#[pyfunction]
fn build_dag(phase_specs_json: &str) -> PyResult<PySubPhaseDAG> {
    let phases: HashMap<String, crate::gates::PhaseSpec> =
        serde_json::from_str(phase_specs_json)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
    crate::dag::SubPhaseDAG::build(&phases)
        .map(|dag| PySubPhaseDAG { inner: dag })
        .map_err(|e| CycleError::new_err(e.to_string()))
}

// ── StoryRecipe ───────────────────────────────────────────────────────────────

#[pyclass]
pub struct PyStoryRecipe { inner: crate::recipe::StoryRecipe }
json_methods!(PyStoryRecipe, crate::recipe::StoryRecipe);

#[pymethods]
impl PyStoryRecipe {
    #[getter] fn seed(&self) -> u64 { self.inner.seed }
}

// ── Module-level functions ─────────────────────────────────────────────────────

/// Parse a `<!-- key:value -->` annotation comment string into a list of
/// `PyAnnotation` objects.  Returns `(annotations, warnings)`.
#[pyfunction]
fn parse_annotation(
    _py: Python<'_>,
    raw: &str,
) -> PyResult<(Vec<PyAnnotation>, Vec<String>)> {
    let (anns, warns) = cap_narrative_types::tags::parse_annotation_comment(raw);
    let py_anns = anns.into_iter().map(|a| PyAnnotation { inner: a }).collect();
    Ok((py_anns, warns))
}

/// Dump all JSON Schemas as a JSON string.
/// Use from Python: `json.loads(grimoire_types.dump_schemas())`
#[pyfunction]
fn dump_schemas() -> PyResult<String> {
    serde_json::to_string_pretty(&crate::generate_all_schemas())
        .map_err(|e| PyValueError::new_err(e.to_string()))
}
