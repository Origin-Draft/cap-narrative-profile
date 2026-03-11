//! Semantic Interaction Protocol (SIP) core types.
//!
//! This module defines the domain-agnostic core type system extracted from
//! GBR v0.2.0.  These types implement SIP v0.1.0 as specified in
//! `docs/sip/SPECIFICATION.md`.
//!
//! # Module layout
//!
//! | Sub-module | Contents |
//! |------------|---------|
//! | [`enums`]  | SIP core enumerations (Significance, CausalRole, …) |
//! | [`artifact`] | Top-level [`SipArtifact`] container |
//! | [`entity`]   | [`SipEntity`] — persistent participants |
//! | [`unit`]     | [`SipUnit`], [`SipStep`] — atomic interaction blocks |
//! | [`relationship`] | [`SipRelationship`] — typed directed links |
//! | [`state`]    | [`SipState`] — point-in-time conditions |
//! | [`transition`] | [`SipTransition`] — value changes |
//! | [`view`]     | [`SipView`] — derived projections |
//! | [`interpretation`] | [`SipInterpretation`] — structured inferences |
//! | [`participant_state`] | [`SipParticipantState`], [`SipInformationState`] |
//!
//! # SIP vs. GBR
//!
//! The existing `gbr_types::{entities, enums, …}` modules represent the
//! **narrative profile** on top of SIP core.  These SIP modules are the
//! domain-agnostic substrate; they carry no narrative-specific semantics.

pub mod artifact;
pub mod entity;
pub mod enums;
pub mod interpretation;
pub mod participant_state;
pub mod relationship;
pub mod state;
pub mod transition;
pub mod unit;
pub mod view;

// Convenience re-exports for the most-used types
pub use artifact::SipArtifact;
pub use entity::SipEntity;
pub use enums::{CausalRole, ConformanceLevel, Significance, ValidationSeverity};
pub use interpretation::SipInterpretation;
pub use participant_state::{SipInformationState, SipParticipantState};
pub use relationship::SipRelationship;
pub use state::SipState;
pub use transition::SipTransition;
pub use unit::{SipStep, SipUnit};
pub use view::SipView;

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Round-trip each conformance fixture: parse JSON → SipArtifact → serialize
    /// back to JSON → parse again and assert equality.
    macro_rules! roundtrip_fixture {
        ($name:ident, $path:literal) => {
            #[test]
            fn $name() {
                let raw = include_str!($path);
                let artifact: SipArtifact = serde_json::from_str(raw)
                    .unwrap_or_else(|e| panic!("parse failed for {}: {e}", $path));

                // Serialize back to JSON
                let re_serialized = serde_json::to_string(&artifact)
                    .expect("serialize failed");

                // Parse the re-serialized form — must equal the first parse
                let artifact2: SipArtifact = serde_json::from_str(&re_serialized)
                    .expect("re-parse failed");

                assert_eq!(artifact, artifact2, "round-trip inequality for {}", $path);
            }
        };
    }

    roundtrip_fixture!(
        roundtrip_minimal,
        "../../../../docs/sip/conformance/valid/minimal-artifact.json"
    );
    roundtrip_fixture!(
        roundtrip_multi_unit,
        "../../../../docs/sip/conformance/valid/multi-unit-artifact.json"
    );
    roundtrip_fixture!(
        roundtrip_full_narrative,
        "../../../../docs/sip/conformance/valid/full-narrative-artifact.json"
    );
    roundtrip_fixture!(
        roundtrip_full_software,
        "../../../../docs/sip/conformance/valid/full-software-artifact.json"
    );
    roundtrip_fixture!(
        roundtrip_threshold_ch01_s01,
        "../../../../examples/small-story/threshold/ch01_s01.sip.json"
    );

    // ── Structural property tests ─────────────────────────────────────────────

    #[test]
    fn protocol_check_correct() {
        let artifact: SipArtifact = serde_json::from_str(include_str!(
            "../../../../docs/sip/conformance/valid/minimal-artifact.json"
        ))
        .unwrap();
        assert!(artifact.is_valid_protocol());
        assert_eq!(artifact.protocol, "semantic-interaction-protocol");
    }

    #[test]
    fn protocol_check_wrong_value() {
        let artifact: SipArtifact = serde_json::from_str(include_str!(
            "../../../../docs/sip/conformance/invalid/wrong-protocol-value.json"
        ))
        .unwrap();
        assert!(!artifact.is_valid_protocol());
    }

    #[test]
    fn entity_ids_iterator() {
        let artifact: SipArtifact = serde_json::from_str(include_str!(
            "../../../../docs/sip/conformance/valid/full-narrative-artifact.json"
        ))
        .unwrap();
        let ids: Vec<&str> = artifact.entity_ids().collect();
        assert!(ids.contains(&"nadia"), "expected 'nadia' in entity_ids");
        assert!(ids.contains(&"childhood_home"), "expected 'childhood_home' in entity_ids");
    }

    #[test]
    fn unit_ids_iterator() {
        let artifact: SipArtifact = serde_json::from_str(include_str!(
            "../../../../docs/sip/conformance/valid/multi-unit-artifact.json"
        ))
        .unwrap();
        let ids: Vec<&str> = artifact.unit_ids().collect();
        assert!(ids.len() >= 2, "multi-unit fixture should have >= 2 units");
    }

    #[test]
    fn dangling_ref_fixture_still_parses() {
        // The dangling-entity-ref fixture is structurally valid JSON (+SipArtifact)
        // even though its entity refs don't resolve. Deserialization must succeed.
        let result: Result<SipArtifact, _> = serde_json::from_str(include_str!(
            "../../../../docs/sip/conformance/invalid/dangling-entity-ref.json"
        ));
        assert!(result.is_ok(), "dangling-entity-ref.json should deserialize cleanly");
    }

    #[test]
    fn missing_observables_fixture_fails_deserialization() {
        // observables is a required field on SipUnit — deserialization MUST fail.
        let result: Result<SipArtifact, _> = serde_json::from_str(include_str!(
            "../../../../docs/sip/conformance/invalid/missing-observables.json"
        ));
        assert!(result.is_err(), "missing-observables.json should fail deserialization");
    }
}
