#!/usr/bin/env python3
# Copyright © 2026 Mikhail Hogrefe
#
# This file is part of Malachite.
#
# Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
# Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
# 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

# Finds .rs files that are not included in any crate: files that no chain of `mod` declarations
# reaches from a compilation root, so the compiler never sees them (and no rustc-based lint can
# flag them). Run by additional-lints.sh; exits nonzero if any unincluded file is found.
#
# A crate's roots are its Cargo.toml targets (any `path = "....rs"`) plus the conventional
# automatic targets: src/lib.rs, src/main.rs, build.rs, src/bin/*.rs, src/bin/*/main.rs, and the
# direct children of tests/, benches/, and examples/. From the roots, `mod name;` declarations are
# resolved to `name.rs` or `name/mod.rs` — relative to the file's directory for roots and mod.rs
# files and to a directory named after the file otherwise, with the names of any enclosing inline
# modules (`mod outer { mod name; }`, as in bin.rs and tests/lib.rs) appended to the base — and
# followed transitively. `#[path]` attributes are not supported (nothing in Malachite uses them).

import re
import sys
from pathlib import Path

CRATES = [
    "malachite-base",
    "malachite-nz",
    "malachite-q",
    "malachite-float",
    "malachite",
    "malachite-bigint",
    "malachite-criterion-bench",
    "malachite-lints",
]

TARGET_PATH = re.compile(r'^path = "(.*\.rs)"', re.M)
LINE_COMMENT = re.compile(r"//[^\n]*")
BLOCK_COMMENT = re.compile(r"/\*.*?\*/", re.S)
STRING_LITERAL = re.compile(r'"(?:[^"\\]|\\.)*"')
CHAR_LITERAL = re.compile(r"'(?:[^'\\]|\\.)'")
# a `mod` declaration (with or without a body) or a lone brace, in source order
EVENT = re.compile(r"(?:\bmod\s+([A-Za-z_][A-Za-z0-9_]*)\s*([;{]))|([{}])")


def crate_roots(crate: Path) -> set[Path]:
    roots = set()
    cargo_toml = crate / "Cargo.toml"
    if cargo_toml.is_file():
        for target_path in TARGET_PATH.findall(cargo_toml.read_text()):
            path = crate / target_path
            if path.is_file():
                roots.add(path.resolve())
    for conventional in ["src/lib.rs", "src/main.rs", "build.rs"]:
        path = crate / conventional
        if path.is_file():
            roots.add(path.resolve())
    for auto_dir, pattern in [
        ("src/bin", "*.rs"),
        ("src/bin", "*/main.rs"),
        ("tests", "*.rs"),
        ("benches", "*.rs"),
        ("examples", "*.rs"),
    ]:
        directory = crate / auto_dir
        if directory.is_dir():
            for path in directory.glob(pattern):
                roots.add(path.resolve())
    return roots


# The file-level `mod name;` declarations in the given source text, each with the names of its
# enclosing inline modules. Comments, string literals, and char literals are stripped first so
# that brace counting is reliable.
def mod_declarations(source: str) -> list[tuple[list[str], str]]:
    source = BLOCK_COMMENT.sub(" ", source)
    source = LINE_COMMENT.sub(" ", source)
    source = STRING_LITERAL.sub('""', source)
    source = CHAR_LITERAL.sub("' '", source)
    declarations = []
    depth = 0
    inline_stack: list[tuple[str, int]] = []
    for match in EVENT.finditer(source):
        name, mod_kind, brace = match.groups()
        if brace == "{" or mod_kind == "{":
            if mod_kind == "{":
                inline_stack.append((name, depth))
            depth += 1
        elif brace == "}":
            depth -= 1
            if inline_stack and inline_stack[-1][1] == depth:
                inline_stack.pop()
        else:
            declarations.append(([inline_name for inline_name, _ in inline_stack], name))
    return declarations


def included_files(roots: set[Path]) -> set[Path]:
    included = set(roots)
    stack = list(roots)
    while stack:
        file = stack.pop()
        # `mod name;` in lib.rs, main.rs, mod.rs, or a compilation root resolves relative to the
        # file's own directory; in any other file it resolves relative to a directory named after
        # the file. Enclosing inline modules add their names to the path.
        if file.name in ("lib.rs", "main.rs", "mod.rs", "build.rs") or file in roots:
            base = file.parent
        else:
            base = file.parent / file.stem
        for inline_names, name in mod_declarations(file.read_text()):
            file_base = base.joinpath(*inline_names)
            for candidate in [file_base / f"{name}.rs", file_base / name / "mod.rs"]:
                candidate = candidate.resolve()
                if candidate.is_file() and candidate not in included:
                    included.add(candidate)
                    stack.append(candidate)
    return included


def main() -> int:
    repo = Path(__file__).parent
    unincluded = []
    for crate_name in CRATES:
        crate = repo / crate_name
        included = included_files(crate_roots(crate))
        for source_dir in ["src", "tests", "benches", "examples"]:
            directory = crate / source_dir
            if not directory.is_dir():
                continue
            for path in sorted(directory.rglob("*.rs")):
                if path.resolve() not in included:
                    unincluded.append(path.relative_to(repo))
    if unincluded:
        print("The following files are not included in any crate (no chain of `mod` declarations")
        print("reaches them from a compilation root), so the compiler never sees them:")
        for path in unincluded:
            print(f"    {path}")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
