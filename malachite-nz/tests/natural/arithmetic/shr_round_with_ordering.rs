// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{
    ShrRound, ShrRoundWithOrdering, ShrRoundWithOrderingAssign,
};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_unsigned_pair_gen_var_4, natural_unsigned_rounding_mode_ordering_quadruple_gen_var_1,
    natural_unsigned_unsigned_triple_gen_var_3,
};
use malachite_nz::test_util::natural::arithmetic::shr_round_with_ordering::*;
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;
use std::str::FromStr;

const INEXACT_MODES: [RoundingMode; 5] = [Down, Up, Floor, Ceiling, Nearest];

#[test]
fn test_shr_round_with_ordering() {
    let test = |s, bits: u64, rm: RoundingMode, o: Ordering, out, o_out: Ordering| {
        let u = Natural::from_str(s).unwrap();

        let (q, o_q) = u.clone().shr_round_with_ordering(bits, rm, o);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(o_q, o_out);

        let (q, o_q) = (&u).shr_round_with_ordering(bits, rm, o);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(o_q, o_out);

        let mut mut_u = u.clone();
        assert_eq!(mut_u.shr_round_with_ordering_assign(bits, rm, o), o_out);
        assert!(mut_u.is_valid());
        assert_eq!(mut_u.to_string(), out);

        let (q, o_q) = natural_shr_round_with_ordering_naive(&u, bits, rm, o);
        assert_eq!(q.to_string(), out);
        assert_eq!(o_q, o_out);
    };
    // - o == Equal: the same as shr_round
    test("384", 8, Nearest, Equal, "2", Greater);
    test("512", 8, Exact, Equal, "2", Equal);
    // - divisible, o == Greater, rm == Floor or Down
    test("512", 8, Floor, Greater, "1", Less);
    test("512", 8, Down, Greater, "1", Less);
    // - divisible, o == Less, rm == Ceiling or Up
    test("512", 8, Ceiling, Less, "3", Greater);
    test("512", 8, Up, Less, "3", Greater);
    // - divisible, otherwise
    test("512", 8, Ceiling, Greater, "2", Greater);
    test("512", 8, Nearest, Greater, "2", Greater);
    test("512", 8, Floor, Less, "2", Less);
    test("512", 8, Nearest, Less, "2", Less);
    test("0", 8, Ceiling, Greater, "0", Greater);
    test("0", 8, Nearest, Greater, "0", Greater);
    test("0", 8, Floor, Less, "0", Less);
    test("0", 8, Ceiling, Less, "1", Greater);
    test("0", 8, Nearest, Less, "0", Less);
    // - bits == 0
    test("5", 0, Floor, Greater, "4", Less);
    test("5", 0, Ceiling, Greater, "5", Greater);
    test("5", 0, Nearest, Greater, "5", Greater);
    test("5", 0, Ceiling, Less, "6", Greater);
    test("5", 0, Nearest, Less, "5", Less);
    // - exact half, o == Greater
    test("384", 8, Nearest, Greater, "1", Less);
    test("3", 1, Nearest, Greater, "1", Less);
    test(
        "1901475900342344102245054808064",
        100,
        Nearest,
        Greater,
        "1",
        Less,
    );
    // - exact half, o == Less
    test("384", 8, Nearest, Less, "2", Greater);
    test("3", 1, Nearest, Less, "2", Greater);
    test(
        "1901475900342344102245054808064",
        100,
        Nearest,
        Less,
        "2",
        Greater,
    );
    // - neither divisible nor an exact half: the same as shr_round
    test("384", 8, Floor, Less, "1", Less);
    test("384", 8, Up, Greater, "2", Greater);
    test("383", 8, Nearest, Less, "1", Less);
    test("383", 8, Nearest, Greater, "1", Less);
    test("385", 8, Nearest, Less, "2", Greater);
    test("385", 8, Nearest, Greater, "2", Greater);
}

#[test]
fn shr_round_with_ordering_fail() {
    assert_panic!(Natural::from(512u32).shr_round_with_ordering(8, Exact, Less));
    assert_panic!(Natural::from(512u32).shr_round_with_ordering(8, Exact, Greater));
    assert_panic!(Natural::from(385u32).shr_round_with_ordering(8, Exact, Equal));
    assert_panic!(Natural::from(0u32).shr_round_with_ordering(8, Floor, Greater));
    assert_panic!(Natural::from(0u32).shr_round_with_ordering(8, Down, Greater));
    assert_panic!((&Natural::from(0u32)).shr_round_with_ordering(0, Floor, Greater));
    assert_panic!({
        let mut x = Natural::from(0u32);
        x.shr_round_with_ordering_assign(8, Down, Greater)
    });
}

#[test]
fn shr_round_with_ordering_properties() {
    natural_unsigned_rounding_mode_ordering_quadruple_gen_var_1().test_properties(
        |(n, bits, rm, o)| {
            let (q, o_q) = n.clone().shr_round_with_ordering(bits, rm, o);
            assert!(q.is_valid());
            let (q_alt, o_alt) = (&n).shr_round_with_ordering(bits, rm, o);
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);
            assert_eq!(o_alt, o_q);
            let mut n_alt = n.clone();
            assert_eq!(n_alt.shr_round_with_ordering_assign(bits, rm, o), o_q);
            assert!(n_alt.is_valid());
            assert_eq!(n_alt, q);
            assert_eq!(
                natural_shr_round_with_ordering_naive(&n, bits, rm, o),
                (q.clone(), o_q)
            );
            if o == Equal {
                // With an exact input, this is shr_round.
                assert_eq!((&n).shr_round(bits, rm), (q.clone(), o_q));
            } else {
                // An inexact input never gives an exact result.
                assert_ne!(o_q, Equal);
            }
            // The result compares with the shifted exact value as its shift back compares with
            // `n`, or as `n` does when they are equal.
            let c = (&q << bits).cmp(&n);
            assert_eq!(o_q, if c == Equal { o } else { c });
            // The result is within 1 of the shifted value.
            let (floor, _) = (&n).shr_round(bits, Floor);
            let (ceiling, _) = (&n).shr_round(bits, Ceiling);
            assert!(&q + Natural::from(1u32) >= floor && q <= ceiling + Natural::from(1u32));
            // The Integer function agrees, unless the exact value lies just below zero, where
            // Integer's `Up` rounds away from zero to -1 but Natural's is `Ceiling`.
            if n != 0u32 || o != Greater {
                let (q_i, o_i) = Integer::from(&n).shr_round_with_ordering(bits, rm, o);
                assert_eq!(q_i, q);
                assert_eq!(o_i, o_q);
            }
        },
    );

    // Rounding twice, the second time with the first rounding's `Ordering`, is the same as rounding
    // once.
    natural_unsigned_unsigned_triple_gen_var_3::<u64, u64>().test_properties(|(n, bits, j)| {
        for rm_1 in INEXACT_MODES {
            let (x, o) = (&n).shr_round(j, rm_1);
            for rm in INEXACT_MODES {
                assert_eq!(
                    x.clone().shr_round_with_ordering(bits, rm, o),
                    (&n).shr_round(j + bits, rm)
                );
            }
        }
    });

    natural_unsigned_pair_gen_var_4::<u64>().test_properties(|(n, j)| {
        // A shift by 0 bits rounds the exact value correctly in the directed modes.
        for rm_1 in INEXACT_MODES {
            let (x, o) = (&n).shr_round(j, rm_1);
            for rm in [Down, Up, Floor, Ceiling] {
                assert_eq!(
                    x.clone().shr_round_with_ordering(0, rm, o),
                    (&n).shr_round(j, rm)
                );
            }
        }
    });
}
