#!/usr/bin/env python3
"""Validate solution blueprint governance rules.

Blueprints under examples/solutions/ must compose platform capabilities only —
no workspace crates, no Rust sources, no npm packages.

See docs/platform-architecture.md and docs/design-principles.md.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MANIFEST = Path(__file__).resolve().parent / "architecture-manifest.json"

ALLOWED_EXTENSIONS = {
    ".sd",
    ".toml",
    ".md",
    ".trace",
    ".json",
    ".txt",
    ".yaml",
    ".yml",
}

FORBIDDEN_FILENAMES = {
    "Cargo.toml",
    "package.json",
    "pyproject.toml",
    "go.mod",
}


def load_blueprint_roots() -> list[dict]:
    if not MANIFEST.exists():
        return []
    data = json.loads(MANIFEST.read_text(encoding="utf-8"))
    return data.get("solution_blueprints", [])


def validate_blueprint(root: Path) -> list[str]:
    errors: list[str] = []
    if not root.is_dir():
        errors.append(f"Blueprint path missing: {root}")
        return errors

    for path in root.rglob("*"):
        if not path.is_file():
            continue
        if path.name in FORBIDDEN_FILENAMES:
            errors.append(f"Forbidden manifest in blueprint: {path.relative_to(ROOT)}")
            continue
        if path.suffix == ".rs":
            errors.append(f"Rust source not allowed in blueprint: {path.relative_to(ROOT)}")
            continue
        if path.suffix and path.suffix not in ALLOWED_EXTENSIONS:
            errors.append(
                f"Unexpected file type in blueprint: {path.relative_to(ROOT)} "
                f"(allowed: {', '.join(sorted(ALLOWED_EXTENSIONS))})"
            )
    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate solution blueprint governance.")
    parser.add_argument(
        "--roots",
        nargs="*",
        help="Blueprint roots (default: from architecture-manifest.json)",
    )
    args = parser.parse_args()

    if args.roots:
        roots = [Path(r) for r in args.roots]
    else:
        roots = [ROOT / entry["path"] for entry in load_blueprint_roots()]

    all_errors: list[str] = []
    for root in roots:
        all_errors.extend(validate_blueprint(root))

    print(f"Blueprint roots checked: {len(roots)}")
    for err in all_errors:
        print(f"  ERROR: {err}")

    if all_errors:
        print("\nBlueprint validation FAILED.")
        return 1

    print("\nBlueprint validation passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
