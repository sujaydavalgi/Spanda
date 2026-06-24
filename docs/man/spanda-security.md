# spanda-security(1)

## NAME

security — Validate security policies, identities, and audit configuration.

## SYNOPSIS

```
spanda security <check|audit> [--json] <file.sd>
```

## DESCRIPTION

Validate security policies, identities, and audit configuration.

## OPTIONS

`check` — static security validation
`audit` — audit log review
`--json` — machine-readable report

## EXAMPLES

```bash
spanda security check robot.sd --json
spanda security audit robot.sd
```

## EXIT STATUS

0 when policies pass; 1 on violations.

## FILES

Security metadata in program and `spanda.toml` when present.

## SEE ALSO

spanda-verify(1), spanda-readiness(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)
