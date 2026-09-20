# Copyright © 2026 Mikhail Hogrefe
#
# This file is part of Malachite.
#
# Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
# Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
# 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

# Checks that backslashes inside `$...$` math spans in doc comments survive Markdown.
#
# Rustdoc runs doc comments through Markdown before KaTeX sees them, and Markdown reads a backslash
# before ASCII punctuation as an escape and drops it. So `$\text{100\%}$` reaches KaTeX as
# `$\text{100%}$`, where the bare `%` starts a LaTeX comment that swallows the rest of the line,
# including the closing delimiter; KaTeX then renders nothing and the source shows through. Writing
# `$\text{100\\%}$` is what leaves KaTeX a real `\%`.
#
# Not every such escape is a mistake. `$\sqrt\[n\]{x}$` is deliberate: the backslashes hide the
# brackets from rustdoc's intra-doc link syntax, and KaTeX wants the bare `[n]` anyway. The same
# goes for `\_`. What this checks is the cases where dropping the backslash changes the meaning:
#
#     `\%` `\$` `\#` `\&`  a bare one is a comment, a delimiter, a parameter, or an aligner
#     `\{` `\}`            a bare brace groups silently instead of being typeset
#
# Run from the repo root:
#
#     python3 math-escape-check.py

import os
import re
import sys

CRATES = ["malachite-base", "malachite-nz", "malachite-q", "malachite-float"]

# Dropping the backslash before one of these breaks the span outright.
DESTRUCTIVE = {
    "%": "a bare `%` starts a LaTeX comment and swallows the rest of the line",
    "$": "a bare `$` ends the math span early",
    "#": "a bare `#` is a macro parameter",
    "&": "a bare `&` is an alignment tab",
}
# Dropping the backslash before one of these silently loses the glyph.
LOSES_GLYPH = {
    "{": "a bare `{` groups instead of being typeset",
    "}": "a bare `}` groups instead of being typeset",
}

CODE_SPAN_RE = re.compile(r"`[^`]*`")
MATH_SPAN_RE = re.compile(r"(?<!\$)\$([^$]+)\$(?!\$)")


def problems_in_line(line):
    # Markdown does not process escapes inside code spans, so a single backslash is correct there.
    stripped = CODE_SPAN_RE.sub("", line)
    for span in MATH_SPAN_RE.findall(stripped):
        i = 0
        while i < len(span):
            if span[i] == "\\" and i + 1 < len(span):
                nxt = span[i + 1]
                # `\\` is the correct spelling: Markdown turns it into the `\` KaTeX needs.
                if nxt != "\\":
                    if nxt in DESTRUCTIVE:
                        yield f"`\\{nxt}` in a math span: {DESTRUCTIVE[nxt]}; write `\\\\{nxt}`"
                    elif nxt in LOSES_GLYPH:
                        yield f"`\\{nxt}` in a math span: {LOSES_GLYPH[nxt]}; write `\\\\{nxt}`"
                i += 2
                continue
            i += 1


def main():
    span_count = 0
    problem_count = 0
    for crate in CRATES:
        for root, _dirs, files in os.walk(os.path.join(crate, "src")):
            for fname in sorted(files):
                if not fname.endswith(".rs"):
                    continue
                path = os.path.join(root, fname)
                with open(path, encoding="utf-8") as f:
                    for i, line in enumerate(f):
                        if not line.lstrip().startswith("///") and not line.lstrip().startswith(
                            "//!"
                        ):
                            continue
                        span_count += len(MATH_SPAN_RE.findall(CODE_SPAN_RE.sub("", line)))
                        for problem in problems_in_line(line):
                            print(f"{path}:{i + 1}: {problem}")
                            problem_count += 1
    print(f"{span_count} math spans checked", file=sys.stderr)
    if problem_count:
        print(f"{problem_count} problems found", file=sys.stderr)
        sys.exit(1)
    print("all escapes survive Markdown", file=sys.stderr)


if __name__ == "__main__":
    main()
