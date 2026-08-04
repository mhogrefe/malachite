# Copyright © 2026 Mikhail Hogrefe
#
# This file is part of Malachite.
#
# Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
# Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
# 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

# Checks that every "Worst-case complexity" block is well-formed (see DOC-AUDIT.md): it is either
# the constant form ("Constant time and additional memory.") or contains a $T$ formula, an $M$
# formula, and a "where" line; every variable used in the formulas is defined in the "where" line,
# and every variable the "where" line defines is used. This catches missing definitions, orphaned
# definitions from copy-paste (the wrong-variable class), and placeholder blocks. Run from the
# repo root:
#
#     python3 complexity-doc-check.py

import os
import re
import sys

CRATES = ["malachite-base", "malachite-nz", "malachite-q", "malachite-float"]

HEADERS = {
    "# Worst-case complexity",
    "# Worst-case complexity (amortized)",
    "# Worst-case complexity per iteration",
    "# Expected complexity",
    "# Expected complexity per iteration",
}

# Blocks that are known to contain a placeholder TODO instead of real bounds, awaiting analysis in
# the documentation audit (see DOC-AUDIT.md). Maps each file to how many such blocks it has. A
# file with more placeholders than listed fails the run; so does one with fewer, so this list
# cannot go stale — remove entries as the audit fills the bounds in.
KNOWN_PLACEHOLDERS = {
    "malachite-nz/src/natural/arithmetic/binomial_coefficient.rs": 2,
    "malachite-nz/src/natural/arithmetic/sqrt.rs": 1,
    "malachite-nz/src/integer/arithmetic/binomial_coefficient.rs": 2,
}

CONSTANT_RE = re.compile(r"Constant time and additional memory\.")
# a delegation-form block ("Same as the ... complexity of `foo`") has no formulas of its own
DELEGATION_RE = re.compile(r"[Ss]ame as")
FORMULA_RE = re.compile(r"\$([TM])\(([^)]*)\)\s*=\s*(.*?)\$")
# a "where" line defines variables as `$x$ is ...`; T and M are defined as time and memory
DEF_RE = re.compile(r"\$([a-zA-Z])\$ is")
# callee-relative bounds use subscripted symbols, defined like `$T_S$ and $M_S$ are the
# complexities of ...`; their arguments (e.g. the $n$ in $T_S(n)$) are bound by reference
SUBSCRIPTED_RE = re.compile(r"([A-Za-z]_[A-Za-z])")
SUBSCRIPTED_CALL_RE = re.compile(r"[A-Za-z]_[A-Za-z]\(([^)]*)\)")
SUBSCRIPTED_DEF_RE = re.compile(r"\$([A-Za-z]_[A-Za-z])\$")
LATEX_COMMAND_RE = re.compile(r"\\operatorname\{[a-zA-Z]+\}|\\[a-zA-Z]+")
MATH_SEGMENT_RE = re.compile(r"\$([^$]+)\$")


def comment_text(line):
    # Returns the comment text of a `///`, `//!`, or `//` line, or None for a non-comment line.
    s = line.strip()
    for prefix in ("///", "//!", "//"):
        if s.startswith(prefix):
            return s[len(prefix) :].strip()
    return None


def segment_variables(text):
    # Single-letter variables in a piece of math: every letter that is not part of a LaTeX
    # command, a subscripted complexity symbol, or the O of big-O notation.
    text = SUBSCRIPTED_RE.sub(" ", LATEX_COMMAND_RE.sub(" ", text))
    return set(c for c in re.findall(r"[a-zA-Z]", text) if c != "O")


def formula_variables(args, body):
    # Returns (argument variables, body variables), excluding arguments of callee-relative
    # complexity symbols (the $n$ in $T_S(n)$ refers to the callee's own variable).
    arg_variables = set(a.strip() for a in args.split(",") if a.strip())
    body = SUBSCRIPTED_CALL_RE.sub(" ", body)
    return arg_variables, segment_variables(body)


def check_block(path, start, lines):
    # Parses the comment block following the header at index `start` and returns a list of
    # problem strings.
    block = []
    i = start + 1
    while i < len(lines):
        text = comment_text(lines[i])
        if text is None or text.startswith("# "):
            break
        block.append(text)
        i += 1
    joined = " ".join(block)
    if "TODO" in joined:
        return ["placeholder TODO"]
    if CONSTANT_RE.search(joined):
        return []
    if DELEGATION_RE.search(joined):
        return []
    problems = []
    arg_used = set()
    body_used = set()
    have = set()
    for text in block:
        m = FORMULA_RE.search(text)
        if m:
            have.add(m.group(1))
            arg_variables, body_variables = formula_variables(m.group(2), m.group(3))
            arg_used |= arg_variables
            body_used |= body_variables
    # the `where` clause may wrap over several comment lines; take everything from the first line
    # starting with `where` to the end of the block
    where_start = next((j for j, text in enumerate(block) if text.startswith("where")), None)
    if "T" not in have:
        problems.append("no $T$ formula (and not the constant form)")
    if "M" not in have:
        problems.append("no $M$ formula (and not the constant form)")
    if where_start is None:
        problems.append("no `where` line defining the variables")
        return problems
    where_text = " ".join(block[where_start:])
    defined = set(DEF_RE.findall(where_text))
    # a variable may also be used inside another variable's definition (e.g. `$m$ is
    # $|\log_b x|$, where $b$ is ...`); such uses keep it from being reported as unused
    where_used = set()
    for segment in MATH_SEGMENT_RE.findall(where_text):
        if len(segment.strip()) > 1:
            where_used |= segment_variables(segment)
    # subscripted complexity symbols must themselves be explained in the `where` text
    used_subscripted = set()
    for text in block[:where_start]:
        used_subscripted |= set(SUBSCRIPTED_RE.findall(LATEX_COMMAND_RE.sub(" ", text)))
    undefined_subscripted = sorted(used_subscripted - set(SUBSCRIPTED_DEF_RE.findall(where_text)))
    if undefined_subscripted:
        problems.append(
            f"complexity symbols used but not defined: {', '.join(undefined_subscripted)}"
        )
    if "T" in have and "$T$ is time" not in where_text:
        problems.append("`where` line does not define $T$ as time")
    if "M" in have and "$M$ is additional memory" not in where_text:
        problems.append("`where` line does not define $M$ as additional memory")
    # in a callee-relative block, a declaration argument is a generic stand-in for the callee's
    # own variable, so only body variables need definitions there
    used = body_used if used_subscripted else arg_used | body_used
    undefined = sorted(used - defined - {"T", "M"})
    unused = sorted(defined - arg_used - body_used - where_used - {"T", "M"})
    if undefined:
        problems.append(f"variables used but not defined: {', '.join(undefined)}")
    if unused:
        problems.append(f"variables defined but not used: {', '.join(unused)}")
    return problems


def main():
    problem_count = 0
    block_count = 0
    placeholders = {}
    for crate in CRATES:
        for root, dirs, files in os.walk(os.path.join(crate, "src")):
            dirs[:] = [d for d in dirs if d not in ("bin_util", "test_util")]
            for fname in sorted(files):
                if not fname.endswith(".rs"):
                    continue
                path = os.path.join(root, fname)
                with open(path, encoding="utf-8") as f:
                    lines = f.readlines()
                for i, line in enumerate(lines):
                    text = comment_text(line)
                    if text in HEADERS:
                        block_count += 1
                        for problem in check_block(path, i, lines):
                            if problem == "placeholder TODO":
                                placeholders[path] = placeholders.get(path, 0) + 1
                            else:
                                print(f"{path}:{i + 1}: {problem}")
                                problem_count += 1
    for path, count in sorted(placeholders.items()):
        expected = KNOWN_PLACEHOLDERS.get(path, 0)
        if count != expected:
            print(f"{path}: {count} placeholder TODO blocks, but {expected} are known")
            problem_count += 1
    for path in sorted(set(KNOWN_PLACEHOLDERS) - set(placeholders)):
        print(f"{path}: listed in KNOWN_PLACEHOLDERS but has no placeholder TODO blocks")
        problem_count += 1
    print(f"{block_count} complexity blocks checked", file=sys.stderr)
    if placeholders:
        remaining = sum(placeholders.values())
        print(f"{remaining} known placeholder TODO blocks remaining (burn-down)", file=sys.stderr)
    if problem_count:
        print(f"{problem_count} problems found", file=sys.stderr)
        sys.exit(1)
    print("all well-formed", file=sys.stderr)


if __name__ == "__main__":
    main()
