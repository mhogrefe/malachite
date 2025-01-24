// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::logic::traits::{BitConvertible, BitIterable};
use malachite_base::test_util::generators::unsigned_gen;
use malachite_base::test_util::num::logic::bit_convertible::{to_bits_asc_alt, to_bits_desc_alt};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_gen;
use malachite_nz::test_util::natural::logic::to_bits::{to_bits_asc_naive, to_bits_desc_naive};
use std::str::FromStr;

#[test]
fn test_to_bits_asc() {
    let test = |n, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(n.bits().collect_vec(), out);
        assert_eq!(n.to_bits_asc(), out);
        assert_eq!(to_bits_asc_naive(&n), out);
        assert_eq!(to_bits_asc_alt(&n), out);
    };
    test("0", vec![]);
    test("1", vec![true]);
    test("6", vec![false, true, true]);
    test("105", vec![true, false, false, true, false, true, true]);
    test(
        "1000000000000",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, false, true, false, true, false, false, true, false, true, false,
            false, true, false, true, false, true, true, false, false, false, true, false, true,
            true, true,
        ],
    );
    test(
        "4294967295",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true,
        ],
    );
    test(
        "4294967296",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, true,
        ],
    );
    test(
        "18446744073709551615",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true,
        ],
    );
    test(
        "18446744073709551616",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, true,
        ],
    );
}

#[test]
fn test_to_bits_desc() {
    let test = |n, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(n.bits().rev().collect_vec(), out);
        assert_eq!(n.to_bits_desc(), out);
        assert_eq!(to_bits_desc_naive(&n), out);
        assert_eq!(to_bits_desc_alt(&n), out);
    };
    test("0", vec![]);
    test("1", vec![true]);
    test("6", vec![true, true, false]);
    test("105", vec![true, true, false, true, false, false, true]);
    test(
        "1000000000000",
        vec![
            true, true, true, false, true, false, false, false, true, true, false, true, false,
            true, false, false, true, false, true, false, false, true, false, true, false, false,
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false,
        ],
    );
    test(
        "4294967295",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true,
        ],
    );
    test(
        "4294967296",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false,
        ],
    );
    test(
        "18446744073709551615",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true,
        ],
    );
    test(
        "18446744073709551616",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false,
        ],
    );
}

#[test]
fn to_bits_asc_properties() {
    natural_gen().test_properties(|x| {
        let bits = x.to_bits_asc();
        assert_eq!(to_bits_asc_naive(&x), bits);
        assert_eq!(to_bits_asc_alt(&x), bits);
        assert_eq!(x.bits().collect_vec(), bits);
        assert_eq!(Natural::from_bits_asc(bits.iter().copied()), x);
        if x != 0 {
            assert_eq!(*bits.last().unwrap(), true);
        }
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(u.to_bits_asc(), Natural::from(u).to_bits_asc());
    });
}

#[test]
fn to_bits_desc_properties() {
    natural_gen().test_properties(|x| {
        let bits = x.to_bits_desc();
        assert_eq!(to_bits_desc_naive(&x), bits);
        assert_eq!(to_bits_desc_alt(&x), bits);
        assert_eq!(x.bits().rev().collect_vec(), bits);
        assert_eq!(Natural::from_bits_desc(bits.iter().copied()), x);
        if x != 0 {
            assert_eq!(bits[0], true);
        }
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(u.to_bits_desc(), Natural::from(u).to_bits_desc());
    });
}
