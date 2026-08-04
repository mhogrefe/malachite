# Copyright © 2026 Mikhail Hogrefe
#
# This file is part of Malachite.
#
# Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
# Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
# 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

# Generates the documentation-audit checklists (see DOC-AUDIT.md): one file per crate under
# audit/, with a checkbox per documented function. Each entry is tagged with the audit dimensions
# that apply to it:
#
#   cx       the comment or rustdoc claims a worst-case complexity
#   panics   the block has a # Panics section
#   scratch  the function takes a scratch buffer (its length requirement must be rederived)
#   slen     the function is a *_scratch_len function (its formula must be rederived)
#
# Checked boxes are preserved across regenerations, keyed by (file, item name). Run from the repo
# root:
#
#     python3 doc-audit-inventory.py

import os
import re

CRATES = ["malachite-base", "malachite-nz", "malachite-q", "malachite-float"]
OUT_DIR = "audit"

ITEM_RE = re.compile(
    r"^\s*(?:pub(?:\(crate\))?\s+)?(?:const\s+)?(?:unsafe\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)"
    r"|^\s*(?:[a-z_]*test_[a-z_]*(?:fn|const_fn))!\s*\{\s*\n?\s*(?:pub\s+)?(?:const\s+)?fn?\s*([a-zA-Z_][a-zA-Z0-9_]*)?"
)
MACRO_ITEM_RE = re.compile(r"^\s*[a-z_]+test_(?:const_)?fn!\s*\{")
MACRO_NAME_RE = re.compile(r"([a-zA-Z_][a-zA-Z0-9_]*)\s*(?:<|\()")


def scan_file(path):
    with open(path, encoding="utf-8") as f:
        lines = f.readlines()
    items = []
    n = len(lines)
    for i, line in enumerate(lines):
        name = None
        m = re.match(r"^\s*(?:pub(?:\(crate\))?\s+)?(?:const\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)", line)
        if m:
            name = m.group(1)
        elif MACRO_ITEM_RE.match(line):
            # house test-visibility macro: name is on this line or the next non-attribute line
            rest = line.split("{", 1)[1]
            mm = MACRO_NAME_RE.search(rest)
            j = i + 1
            while mm is None and j < n and j < i + 6:
                s = lines[j].strip()
                if not s.startswith("#") and not s.startswith("//"):
                    mm = MACRO_NAME_RE.search(s)
                j += 1
            if mm:
                name = mm.group(1)
        if not name:
            continue
        # preceding contiguous comment/attribute block
        j = i - 1
        doc = []
        while j >= 0:
            s = lines[j].strip()
            if s.startswith("///") or s.startswith("//") or s.startswith("#[") or s.startswith("#!["):
                doc.append(s)
                j -= 1
            else:
                break
        doc_text = "\n".join(reversed(doc))
        # signature scan for scratch params: collect lines until the one that opens the body
        sig_lines = []
        for j in range(i, min(n, i + 20)):
            sig_lines.append(lines[j])
            if lines[j].rstrip().endswith("{") and j > i or (j == i and lines[j].rstrip().endswith("{") and "fn" in lines[j] and ")" in lines[j]):
                break
        sig = "".join(sig_lines)
        flags = []
        if "Worst-case complexity" in doc_text or "Expected complexity" in doc_text:
            flags.append("cx")
        if "# Panics" in doc_text:
            flags.append("panics")
        if re.search(r"scratch[a-z_]*:\s*&mut", sig):
            flags.append("scratch")
        if name.endswith("_scratch_len"):
            flags.append("slen")
        if not doc and "scratch" not in flags and "slen" not in flags:
            continue  # undocumented helper with no scratch: out of scope
        items.append((i + 1, name, flags))
    return items


def load_checked(path):
    checked = set()
    if os.path.exists(path):
        current = None
        for line in open(path, encoding="utf-8"):
            if line.startswith("## "):
                current = line[3:].strip()
            m = re.match(r"- \[x\] `([a-zA-Z0-9_]+)`", line)
            if m and current:
                checked.add((current, m.group(1)))
    return checked


def main():
    os.makedirs(OUT_DIR, exist_ok=True)
    grand = {"items": 0, "cx": 0, "scratch": 0, "slen": 0}
    for crate in CRATES:
        out_path = os.path.join(OUT_DIR, f"checklist-{crate}.md")
        checked = load_checked(out_path)
        sections = []
        for root, dirs, files in os.walk(os.path.join(crate, "src")):
            dirs[:] = [d for d in dirs if d not in ("bin_util", "test_util")]
            for fname in sorted(files):
                if not fname.endswith(".rs"):
                    continue
                if fname == "bin.rs":
                    continue
                path = os.path.join(root, fname)
                items = scan_file(path)
                if items:
                    sections.append((path, items))
        sections.sort()
        with open(out_path, "w", encoding="utf-8") as out:
            out.write(f"# Documentation-audit checklist: {crate}\n\n")
            out.write("Generated by `doc-audit-inventory.py`; see DOC-AUDIT.md for the process.\n")
            out.write("Checked boxes survive regeneration. Tags: cx = complexity claim, panics =\n")
            out.write("panics section, scratch = takes a scratch buffer, slen = scratch-length\n")
            out.write("formula.\n")
            n_items = n_cx = n_scr = n_slen = 0
            for path, items in sections:
                out.write(f"\n## {path}\n\n")
                for lineno, name, flags in items:
                    box = "x" if (path, name) in checked else " "
                    tag = f" — {', '.join(flags)}" if flags else ""
                    out.write(f"- [{box}] `{name}` (line {lineno}){tag}\n")
                    n_items += 1
                    n_cx += "cx" in flags
                    n_scr += "scratch" in flags
                    n_slen += "slen" in flags
            out.write(
                f"\n---\n{n_items} items; {n_cx} with complexity claims; "
                f"{n_scr} scratch-takers; {n_slen} scratch-length formulas.\n"
            )
        grand["items"] += n_items
        grand["cx"] += n_cx
        grand["scratch"] += n_scr
        grand["slen"] += n_slen
        print(f"{crate}: {n_items} items, {n_cx} cx, {n_scr} scratch, {n_slen} slen")
    print(f"total: {grand['items']} items, {grand['cx']} cx, {grand['scratch']} scratch, {grand['slen']} slen")


if __name__ == "__main__":
    main()
