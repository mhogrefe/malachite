// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::Factorial;
use malachite_base::num::basic::traits::Infinity;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::test_util::common::{rug_round_try_from_rounding_mode, to_hex_string};
use malachite_float::test_util::float::arithmetic::factorial::rug_factorial_prec_round;
use malachite_float::test_util::generators::unsigned_unsigned_rounding_mode_triple_gen_var_11;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::natural::Natural;
use std::panic::catch_unwind;

// Together with the property suite, these rows were chosen to cover every branch of
// factorial_prec_round:
// - n <= 1
// - an exact accumulated product (the loop's o1 == Equal fast path), both fitting the target
//   precision and needing a re-round
// - an inexact product that can_round accepts, under every rounding mode and with the Ziv
//   retry exercised by the property generator's precision spread
// - the Exact rounding mode, delegated to the exact composition
// - both overflow paths: the upfront exact bound (test_factorial_overflow) and the in-loop
//   exponent bound (factorial_overflow_window_high, release-only)
// The symmetric-rounding restart (opposite signs from the two rounding stages) was not observed;
// MPFR's own comment questions whether it is reachable, and it is kept as translated.
#[test]
fn test_factorial() {
    let test = |n: u64, prec, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let (f, o) = Float::factorial_prec_round(n, prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, o_out);

        // the exact composition computes every bit of n! and rounds once; the dedicated
        // implementation must match it exactly
        let (f_alt, o_alt) = Float::from_natural_prec_round(Natural::factorial(n), prec, rm);
        assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (f_alt, o_alt) = Float::factorial_prec(n, prec);
            assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_f, rug_o) = rug_factorial_prec_round(u32::try_from(n).unwrap(), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_f)),
                ComparableFloatRef(&f)
            );
            assert_eq!(rug_o, o);
        }
    };
    test(0, 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test(1, 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test(2, 10, Nearest, "2.0000", "0x2.00#10", Equal);
    test(3, 1, Nearest, "8.0", "0x8.0#1", Greater);
    test(4, 10, Nearest, "24.000", "0x18.00#10", Equal);
    test(5, 4, Floor, "120.0", "0x78.0#4", Equal);
    test(5, 4, Ceiling, "120.0", "0x78.0#4", Equal);
    test(5, 4, Nearest, "120.0", "0x78.0#4", Equal);
    test(5, 4, Down, "120.0", "0x78.0#4", Equal);
    test(5, 4, Up, "120.0", "0x78.0#4", Equal);
    test(5, 5, Exact, "120.0", "0x78.0#5", Equal);
    test(10, 20, Nearest, "3628800.0", "0x375f00.0#20", Equal);
    test(
        20,
        30,
        Nearest,
        "2.4329020103e18",
        "0x2.1c3677dE+15#30",
        Greater,
    );
    test(100, 10, Floor, "9.3318e157", "0x1.b30E+131#10", Less);
    test(100, 10, Ceiling, "9.3426e157", "0x1.b38E+131#10", Greater);
    test(
        100,
        100,
        Nearest,
        "9.3326215443944152681699238856278e157",
        "0x1.b30964ec395dc24069528d54cE+131#100",
        Greater,
    );
    test(1000, 2, Floor, "3.1e2567", "0x2.0E+2132#2", Less);
    test(1000, 2, Ceiling, "4.6e2567", "0x3.0E+2132#2", Greater);
}

// The upfront exact overflow bound: n = u64::MAX is rejected without iteration, with
// mpfr_overflow's per-mode results.
#[test]
fn test_factorial_overflow() {
    for (rm, inf) in [(Nearest, true), (Up, true), (Ceiling, true), (Floor, false), (Down, false)] {
        let (f, o) = Float::factorial_prec_round(u64::MAX, 10, rm);
        if inf {
            assert_eq!(ComparableFloat(f), ComparableFloat(Float::INFINITY));
            assert_eq!(o, Greater);
        } else {
            assert_eq!(
                ComparableFloat(f),
                ComparableFloat(Float::max_finite_value_with_prec(10))
            );
            assert_eq!(o, Less);
        }
    }
}

// The in-loop exponent bound: 60000000! overflows, but the upfront bound cannot tell, so the
// loop runs until the accumulated exponent passes the limit. Release-only; this iterates tens of
// millions of small multiplications.
#[test]
fn factorial_overflow_window_high() {
    let (f, o) = Float::factorial_prec_round(60_000_000, 10, Nearest);
    assert_eq!(ComparableFloat(f), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (f, o) = Float::factorial_prec_round(60_000_000, 10, Down);
    assert_eq!(
        ComparableFloat(f),
        ComparableFloat(Float::max_finite_value_with_prec(10))
    );
    assert_eq!(o, Less);
}

#[test]
fn factorial_prec_round_properties() {
    unsigned_unsigned_rounding_mode_triple_gen_var_11().test_properties(|(n, prec, rm)| {
        let (f, o) = Float::factorial_prec_round(n, prec, rm);
        assert!(f.is_valid());

        let (f_alt, o_alt) = Float::from_natural_prec_round(Natural::factorial(n), prec, rm);
        assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
        assert_eq!(o_alt, o);

        if f.is_normal() {
            assert_eq!(f.get_prec(), Some(prec));
        }
        assert!(f > 0u32);

        if let (Ok(rug_rm), Ok(n32)) = (rug_round_try_from_rounding_mode(rm), u32::try_from(n)) {
            let (rug_f, rug_o) = rug_factorial_prec_round(n32, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_f)),
                ComparableFloatRef(&f)
            );
            assert_eq!(rug_o, o);
        }

        if rm == Nearest {
            let (f_alt, o_alt) = Float::factorial_prec(n, prec);
            assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
            assert_eq!(o_alt, o);
        }

        if o == Equal {
            for rm2 in exhaustive_rounding_modes() {
                let (f_alt, o_alt) = Float::factorial_prec_round(n, prec, rm2);
                assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
                assert_eq!(o_alt, Equal);
            }
        } else {
            assert_panic!(Float::factorial_prec_round(n, prec, Exact));
        }
    });
}

#[test]
fn factorial_fail() {
    assert_panic!(Float::factorial_prec_round(5, 0, Nearest));
    assert_panic!(Float::factorial_prec(5, 0));
    assert_panic!(Float::factorial_prec_round(5, 3, Exact));
}

// The hardcoded primitive-float factorial tables in malachite-base, verified against the exact
// Natural factorials: every finite entry is correctly rounded, and the first overflowing input
// really overflows.
#[test]
fn test_primitive_float_factorial_tables() {
    use malachite_base::num::conversion::traits::RoundingFrom;
    use malachite_base::num::float::NiceFloat;
    for n in 0u64..=200 {
        assert_eq!(
            NiceFloat(f64::factorial(n)),
            NiceFloat(f64::rounding_from(&Natural::factorial(n), Nearest).0),
            "{n}"
        );
    }
    assert_eq!(f64::factorial(u64::MAX), f64::INFINITY);
    for n in 0u64..=60 {
        assert_eq!(
            NiceFloat(f32::factorial(n)),
            NiceFloat(f32::rounding_from(&Natural::factorial(n), Nearest).0),
            "{n}"
        );
    }
    assert_eq!(f32::factorial(u64::MAX), f32::INFINITY);
}
