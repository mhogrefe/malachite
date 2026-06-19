// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_float::Float;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// Extracts `(mantissa, exponent)` from a positive, finite `Float` whose value equals `mantissa * 2
// ^ (exponent - 128)`, with the mantissa left-justified into a `u128` (so bit 127 is set). The
// `Float` must have at most 128 significant bits, so the mantissa holds the significand exactly.
fn extract(f: &Float) -> (u128, i16) {
    let exponent = f.get_exponent().unwrap();
    let shift = u64::exact_from(128i64 - i64::from(exponent));
    let mantissa_rational = Rational::exact_from(f) << shift;
    let mantissa = u128::exact_from(&Natural::exact_from(&mantissa_rational));
    (mantissa, i16::exact_from(exponent))
}

pub(crate) fn generate_l2b_data() {
    println!("// This section is created by l2b_data.rs.");
    println!();
    println!("// This is equivalent to `__gmpfr_l2b` in `get_str.c`, MPFR 4.x; it follows the");
    println!("// `compute_l2b` algorithm in MPFR's `tests/tl2b.c`.");
    println!("//");
    println!(
        "// For each base b in 2..=62, column 0 is a 23-bit upper bound on log2(b), and column 1 \
        is a"
    );
    println!(
        "// 77-bit upper bound on log_b(2) = 1 / log2(b). Each entry `(m, e)` represents the value \
        `m *"
    );
    println!(
        "// 2 ^ (e - 128)`; the mantissa `m` is left-justified (bit 127 is set), so it holds the"
    );
    println!("// significand exactly (both 23 and 77 fit in 128 bits).");
    println!("pub const MPFR_L2B: [[(u128, i16); 2]; 61] = [");
    for b in 2u64..=62 {
        let beta = Float::from(u32::exact_from(b));
        // Column 0: a 23-bit upper bound on log2(b) = log(b)/log(2), via log2 rounded up.
        let col_0 = beta.log_base_2_prec_round_ref(23, Ceiling).0;
        // Column 1: a 77-bit upper bound on log_b(2) = log(2)/log(b). MPFR (`compute_l2b` in
        // `tests/tl2b.c`) computes `1 / (log2(b) rounded DOWN to 77 bits)`, then rounds the
        // reciprocal up -- a deliberate two-step that is 1-2 ULP looser than a direct `log_b(2)`,
        // but matching it bit-for-bit keeps `mpfr_get_str`'s `n = 0` digit count identical to
        // MPFR's.
        let log2_b_down = beta.log_base_2_prec_round(77, Floor).0;
        let col_1 = Float::reciprocal_prec_round(log2_b_down, 77, Ceiling).0;
        let (m0, e0) = extract(&col_0);
        let (m1, e1) = extract(&col_1);
        println!("    [({m0:#034x}, {e0}), ({m1:#034x}, {e1})], // b = {b}");
    }
    println!("];");
}
