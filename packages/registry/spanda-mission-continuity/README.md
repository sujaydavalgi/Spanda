# spanda-mission-continuity

Official Spanda package for mission assurance: **mission-continuity**.

Core planning lives in `spanda-assurance`; this package exposes provider hooks:

| Export | Role |
|--------|------|
| `should_resume_from_checkpoint` | Gate resume when progress &gt; 0 |
| `validate_successor` | Basic successor name validation |
| `verify` | Package smoke entry |

Use with `continuity_policy` in fleet programs and `assurance.continuity` capability dispatch.
