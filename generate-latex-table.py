"""Generates malachite-base's LaTeX character table from pylatexenc's `uni2latex` map.

The table in `malachite-base/src/chars/latex_table.rs` is generated, not maintained by hand. To
regenerate it, check out pylatexenc (https://github.com/phfaist/pylatexenc) and run

    python3 generate-latex-table.py <path-to-pylatexenc-checkout>

from the repo root, then run `cargo fmt` and `bash ../superfmt.sh` from `malachite-base`.

pylatexenc is MIT-licensed; its table is in turn adapted from latexcodec, also MIT. Both notices
are reproduced in the generated file, as both licenses require.
"""
import sys, re
sys.path.insert(0, sys.argv[1] if len(sys.argv) > 1 else '../pylatexenc')
from pylatexenc.latexencode._uni2latexmap import uni2latex

PURE_MATH = re.compile(r'^\\ensuremath\{(.*)\}$')

# Entries resolved by hand into a single math-mode spelling.
#
# The first three mix a text-mode part with an \ensuremath part, and there is no way to derive them
# mechanically.
#
# The last three are the superscript digits one, two, and three. Unicode splits the superscript
# digits between Latin-1 Supplement (¹²³) and Superscripts and Subscripts (⁰ and ⁴-⁹), and
# pylatexenc mirrors the split: the first three get text-mode \textonesuperior and friends, the
# other seven get math-mode ^0, ^4, and so on. A string like "2¹⁰" would then be spelled in two
# different modes at once. Spelling all ten the same way lets a run of them coalesce into a single
# superscript, and avoids \text*superior, which not every renderer provides.
HAND: dict[int, str] = {
    0x038F: r'\acute{\Omega}',   # was \'{}\ensuremath{\Omega}
    0x2109: r'{}^\circ\mathrm{F}',  # was \ensuremath{^\circ}F
    0x25AA: r'\blacksquare',     # was {\small\ensuremath{\blacksquare}}, dropping the \small
    0x00B9: r'^1',               # was \textonesuperior
    0x00B2: r'^2',               # was \texttwosuperior
    0x00B3: r'^3',               # was \textthreesuperior
}

entries = []  # (codepoint, is_math, latex)
for cp, v in sorted(uni2latex.items()):
    if cp in HAND:
        entries.append((cp, True, HAND[cp]))
        continue
    m = PURE_MATH.match(v)
    if m:
        entries.append((cp, True, m.group(1)))
    elif '\\ensuremath' not in v:
        entries.append((cp, False, v))
    else:
        sys.exit(f'unhandled mixed entry U+{cp:04X}: {v!r}')

# sanity: sorted, unique, no empty spellings
cps = [e[0] for e in entries]
assert cps == sorted(cps) and len(set(cps)) == len(cps), 'table not sorted/unique'
# U+2061 FUNCTION APPLICATION is invisible, so an empty spelling is correct for it. Assert it is
# the only one, so that a future table revision with new empties is caught here.
empties = [e[0] for e in entries if not e[2]]
assert empties == [0x2061], f'unexpected empty spellings: {[hex(c) for c in empties]}'

def rust_char(cp):
    return f"'\\u{{{cp:X}}}'"

def rust_str(s):
    return '"' + s.replace('\\', '\\\\').replace('"', '\\"') + '"'

n_math = sum(1 for e in entries if e[1])
out = []
out.append(f'''// Copyright © 2026 Mikhail Hogrefe
//
// The character table below is adapted from pylatexenc, which in turn adapted it from latexcodec.
//
//      pylatexenc: Copyright © 2015-2023 Philippe Faist
//
//      latexcodec: Copyright © 2011-2014 Matthias C. M. Troffaes
//
//      Permission is hereby granted, free of charge, to any person obtaining a copy of this
//      software and associated documentation files (the "Software"), to deal in the Software
//      without restriction, including without limitation the rights to use, copy, modify, merge,
//      publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons
//      to whom the Software is furnished to do so, subject to the following conditions:
//
//      The above copyright notice and this permission notice shall be included in all copies or
//      substantial portions of the Software.
//
//      THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
//      INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
//      PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE
//      FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
//      OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
//      DEALINGS IN THE SOFTWARE.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// This file is generated; see the note in `chars::latex`. Each entry maps a `char` to the LaTeX
// that depicts it, together with whether that LaTeX is math-mode ({n_math} entries, written outside a
// `\\text{{}}` group) or text-mode ({len(entries) - n_math} entries, written inside one).
pub(crate) static LATEX_TABLE: [(char, bool, &str); {len(entries)}] = [''')
for cp, is_math, latex in entries:
    out.append(f'    ({rust_char(cp)}, {str(is_math).lower()}, {rust_str(latex)}),')
out.append('];')
text = '\n'.join(out) + '\n'

path = 'malachite-base/src/chars/latex_table.rs'
open(path, 'w', encoding='utf-8').write(text)
print(f'wrote {path}')
print(f'  {len(entries)} entries: {n_math} math, {len(entries) - n_math} text')
long = [l for l in text.split('\n') if len(l) > 100]
print(f'  lines over 100 cols: {len(long)}')
for l in long[:3]:
    print('   ', l[:110])
