# Assurance Cases

Assurance cases link **evidence sources** into deployable safety arguments, composing with existing verification subsystems.

## Syntax

```spanda
assurance_case RoverSafetyCase {
    evidence hardware_verification;
    evidence capability_traceability;
    evidence health_checks;
    evidence simulation_replay;
    evidence safety_tests;
}
```

## Core types

| Type | Role |
|------|------|
| `AssuranceCase` | Named case with evidence records |
| `EvidenceRecord` | Source, kind, status |
| `VerificationEvidence` | Hardware + capability compatibility |
| `SafetyEvidence` | Safety rules and kill switches |
| `TraceabilityEvidence` | Matrix rows from capability/hardware/readiness |

## CLI

```bash
spanda assure rover.sd [--json]
```

The assurance report composes:

- `spanda verify` / hardware verification
- Capability and health traceability
- Mission verification
- Safety case report (`spanda safety-report`)
- Certification proofs

## Package

Evidence anchoring scaffolds: **`spanda-assurance`** (`assurance.evidence`).

For audit ledger integration, see **`spanda-ledger`** (`provenance.ledger`).

## Example

See `examples/assurance/rover_assurance.sd` and `examples/mission/mission_assurance.sd`.
