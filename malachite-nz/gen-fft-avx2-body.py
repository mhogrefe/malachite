#!/usr/bin/env python3
# Copyright © 2026 Mikhail Hogrefe
#
# This file is part of Malachite.
#
# Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
# Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
# 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

"""Regenerates src/natural/arithmetic/mul/fft/avx2_body.rs from body.rs.

The AVX2 instantiation of the FFT multiplication is body.rs with `#[target_feature(enable =
"avx2,fma")]` on every function, which has to be a separate file because the attribute cannot be
applied to a whole module. Run this after editing body.rs; the test `test_fft_avx2_body_in_sync` in
tests/natural/arithmetic/mul.rs applies the same transformation and fails when the two files differ.
"""

import pathlib
import re

ATTRIBUTE = '#[target_feature(enable = "avx2,fma")]'
# Wrapped as superfmt wraps it, so that formatting leaves the generated file unchanged.
BANNER = (
    "// GENERATED FILE: `body.rs` with `#[target_feature(enable = \"avx2,fma\")]` on every function. Do not\n"
    "// edit; edit `body.rs` and run `python3 gen-fft-avx2-body.py` in malachite-nz.\n\n"
)
# A function header, which may be inside a macro definition (`fn $f(`). Keep in sync with
# `is_fn_line` in the test.
FN = re.compile(r"^(\s*)((?:pub(?:\((?:crate|super)\))? )?fn [A-Za-z_$])", re.M)


def generate(body):
    out = FN.sub(lambda m: f"{m.group(1)}{ATTRIBUTE}\n{m.group(1)}{m.group(2)}", body)
    # The banner goes after the license header, which the first blank line ends.
    header_end = out.index("\n\n") + 2
    return out[:header_end] + BANNER + out[header_end:]


if __name__ == "__main__":
    fft = pathlib.Path(__file__).parent / "src" / "natural" / "arithmetic" / "mul" / "fft"
    (fft / "avx2_body.rs").write_text(generate((fft / "body.rs").read_text()))
