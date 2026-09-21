"""Checks that every rustdoc demonstration table agrees with the doctest beside it.

Several `ToLatex` and `ToTypst` implementations document themselves with a `# Examples` doctest
followed by a table of value/fragment pairs. The two are written by hand and can drift apart, which
leaves the table showing a fragment no test has ever checked. Every fragment in a table must
therefore be one of the strings the doctest asserts.

Run from the repo root:

    python3 doc-table-check.py
"""
import re, sys
from pathlib import Path

# A doctest writes each expected fragment as a string literal, raw or not. The first alternative
# consumes `char` literals, which may hold a quotation mark of their own: without it, `'"'` would
# be read as the start of a string and everything after it would be swallowed.
CHAR_LIT = r"'(?:[^'\\]|\\.)'"
RAW_HASH = r'r#"(.*?)"#'
RAW = r'r"([^"]*)"'
COOKED = r'"((?:[^"\\]|\\.)*)"'
LITERAL = re.compile("|".join([CHAR_LIT, RAW_HASH, RAW, COOKED]), re.S)

def doc_blocks(text):
    """Yields (start_line, lines) for each run of consecutive `///` lines."""
    block, start = [], 0
    for i, line in enumerate(text.split("\n"), 1):
        stripped = line.strip()
        if stripped.startswith("///"):
            if not block:
                start = i
            block.append(stripped[3:].lstrip() if len(stripped) > 3 else "")
        else:
            if block:
                yield start, block
            block = []
    if block:
        yield start, block

def tables(block):
    """Yields each contiguous run of table rows; a block may hold more than one table."""
    run = []
    for l in block:
        if l.startswith("|"):
            run.append(l)
        else:
            if run:
                yield run
            run = []
    if run:
        yield run

def split_row(row):
    """Splits a table row into cells. Markdown splits on `|` before parsing anything inline, so a
    `|` inside a cell must be written `\\|`, code span or not."""
    return [c.replace("\\|", "|").strip()
            for c in re.split(r"(?<!\\)\|", row.strip().strip("|"))]

def literals(block):
    """The strings a block's doctests assert. Only the code fences count: searching the prose would
    match a fragment against its own `renders as` cell, which is the same text with its backslashes
    doubled, and the check would pass no matter what."""
    code, in_fence = [], False
    for l in block:
        if l.startswith("```"):
            in_fence = not in_fence
        elif in_fence:
            code.append(l)
    out = set()
    for m in LITERAL.finditer("\n".join(code)):
        raw_hash, raw, cooked = m.groups()
        if raw_hash is None and raw is None and cooked is None:
            continue  # a `char` literal
        if raw_hash is not None:
            out.add(raw_hash)
        elif raw is not None:
            out.add(raw)
        elif cooked is not None:
            out.add(cooked.replace('\\"', '"').replace("\\\\", "\\"))
    return out

problems, checked = [], 0
for path in sorted(Path(".").glob("malachite*/src/**/*.rs")):
    for start, block in doc_blocks(path.read_text(encoding="utf-8")):
        found = literals(block)
        for rows in tables(block):
            if len(rows) < 3:
                continue
            header = split_row(rows[0])
            if "fragment" not in header:
                continue
            col = header.index("fragment")
            for row in rows[2:]:
                cells = split_row(row)
                if len(cells) <= col:
                    continue
                fragment = cells[col].strip("`").strip()
                if not fragment:
                    continue
                checked += 1
                if fragment not in found:
                    problems.append((path, start, fragment))

for path, line, fragment in problems:
    print(f"{path}:{line}: table fragment is not asserted by the doctest beside it: {fragment}")
print(f"{checked} table fragments checked")
if problems:
    print(f"{len(problems)} out of sync")
    sys.exit(1)
print("all tables agree with their doctests")
