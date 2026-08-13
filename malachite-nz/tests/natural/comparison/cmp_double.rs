// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::comparison::traits::{OrdAbs, OrdAbsDouble, OrdDouble};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::comparison::cmp::limbs_cmp_double;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{integer_pair_gen, natural_pair_gen};
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

const HIGH_BIT: Limb = 1 << (Limb::WIDTH - 1);

#[test]
fn test_limbs_cmp_double() {
    let test = |xs: &[Limb], ys: &[Limb], out: Ordering| {
        assert_eq!(limbs_cmp_double(xs, ys), out);
    };
    // - the doubled value fits in the same number of limbs
    test(&[4], &[2], Equal);
    test(&[3], &[2], Less);
    test(&[5], &[2], Greater);
    // - doubling carries into a limb of its own
    test(&[0, 1], &[HIGH_BIT], Equal);
    test(&[1, 1], &[HIGH_BIT], Greater);
    test(&[Limb::MAX], &[HIGH_BIT], Less);
    // - the carried bit moves between limbs
    test(&[0, 3], &[HIGH_BIT, 1], Equal);
    test(&[1, 3], &[HIGH_BIT, 1], Greater);
    // - lengths settle it without comparing limbs
    test(&[1, 1], &[1], Greater);
    test(&[1], &[1, 1], Less);
}

#[test]
fn test_cmp_double() {
    let test = |s, t, out| {
        let x = Natural::from_str(s).unwrap();
        let y = Natural::from_str(t).unwrap();
        assert_eq!(x.cmp_double(&y), out);
        // the allocating form this exists to avoid
        assert_eq!(x.cmp(&(&y << 1u64)), out);
    };
    test("0", "0", Equal);
    test("0", "1", Less);
    test("1", "0", Greater);
    test("4", "2", Equal);
    test("3", "2", Less);
    test("5", "2", Greater);
    // - across the single-limb boundary
    test("18446744073709551616", "9223372036854775808", Equal);
    test("18446744073709551615", "9223372036854775808", Less);
    test("18446744073709551617", "9223372036854775808", Greater);
    test("18446744073709551616", "1", Greater);
    test("1", "18446744073709551616", Less);
}

#[test]
fn test_cmp_abs_double() {
    let test = |s, t, out| {
        let x = Integer::from_str(s).unwrap();
        let y = Integer::from_str(t).unwrap();
        assert_eq!(x.cmp_abs_double(&y), out);
    };
    test("4", "2", Equal);
    // - only the magnitudes matter
    test("-4", "2", Equal);
    test("4", "-2", Equal);
    test("-4", "-2", Equal);
    test("3", "-2", Less);
    test("-5", "2", Greater);
    test("0", "0", Equal);
}

#[test]
fn cmp_double_properties() {
    natural_pair_gen().test_properties(|(x, y)| {
        let c = x.cmp_double(&y);
        // the allocating comparison it replaces
        assert_eq!(c, x.cmp(&(&y << 1u64)));
        // a doubled value is at least the original, so this is bounded by the plain comparison
        if c == Greater {
            assert_eq!(x.cmp(&y), Greater);
        }
        assert_eq!(
            x == Natural::ZERO && y == Natural::ZERO,
            c == Equal && x == y
        );
    });

    integer_pair_gen().test_properties(|(x, y)| {
        let c = x.cmp_abs_double(&y);
        assert_eq!(c, x.unsigned_abs_ref().cmp_double(y.unsigned_abs_ref()));
        assert_eq!(c, x.cmp_abs(&(&y << 1u64)));
        // sign changes are invisible
        assert_eq!((-&x).cmp_abs_double(&y), c);
        assert_eq!(x.cmp_abs_double(&-&y), c);
    });
}
