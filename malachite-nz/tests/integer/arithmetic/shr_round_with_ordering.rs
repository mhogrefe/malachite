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
use malachite_nz::test_util::generators::{
    integer_unsigned_rounding_mode_ordering_quadruple_gen_var_1,
    integer_unsigned_unsigned_triple_gen_var_3,
};
use malachite_nz::test_util::integer::arithmetic::shr_round_with_ordering::*;
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;
use std::str::FromStr;

const INEXACT_MODES: [RoundingMode; 5] = [Down, Up, Floor, Ceiling, Nearest];

#[test]
fn test_shr_round_with_ordering() {
    let test = |s, bits: u64, rm: RoundingMode, o: Ordering, out, o_out: Ordering| {
        let u = Integer::from_str(s).unwrap();

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

        let (q, o_q) = integer_shr_round_with_ordering_naive(&u, bits, rm, o);
        assert_eq!(q.to_string(), out);
        assert_eq!(o_q, o_out);
    };
    // - o == Equal: the same as shr_round
    test("-384", 8, Nearest, Equal, "-2", Less);
    test("-512", 8, Exact, Equal, "-2", Equal);
    // - non-negative exact value
    test("512", 8, Down, Greater, "1", Less);
    test("512", 8, Up, Less, "3", Greater);
    test("384", 8, Nearest, Greater, "1", Less);
    test("384", 8, Nearest, Less, "2", Greater);
    // - negative, divisible
    test("-512", 8, Floor, Greater, "-3", Less);
    test("-512", 8, Ceiling, Greater, "-2", Greater);
    test("-512", 8, Down, Greater, "-2", Greater);
    test("-512", 8, Up, Greater, "-3", Less);
    test("-512", 8, Nearest, Greater, "-2", Greater);
    test("-512", 8, Floor, Less, "-2", Less);
    test("-512", 8, Ceiling, Less, "-1", Greater);
    test("-512", 8, Down, Less, "-1", Greater);
    test("-512", 8, Up, Less, "-2", Less);
    test("-512", 8, Nearest, Less, "-2", Less);
    // - negative, exact half
    test("-384", 8, Nearest, Less, "-1", Greater);
    test("-384", 8, Nearest, Greater, "-2", Less);
    // - negative, otherwise
    test("-385", 8, Nearest, Less, "-2", Less);
    test("-383", 8, Nearest, Greater, "-1", Greater);
    // - zero, with the exact value just below it
    test("0", 8, Floor, Greater, "-1", Less);
    test("0", 8, Down, Greater, "0", Greater);
    test("0", 8, Up, Greater, "-1", Less);
    test("0", 8, Ceiling, Greater, "0", Greater);
    test("0", 8, Nearest, Greater, "0", Greater);
    // - zero, with the exact value just above it
    test("0", 8, Up, Less, "1", Greater);
    test("0", 8, Down, Less, "0", Less);
    test("0", 8, Nearest, Less, "0", Less);
    // - bits == 0
    test("-5", 0, Floor, Less, "-5", Less);
    test("-5", 0, Ceiling, Less, "-4", Greater);
    test("-5", 0, Down, Less, "-4", Greater);
    test("-5", 0, Up, Greater, "-6", Less);
    test("-5", 0, Nearest, Greater, "-5", Greater);
}

#[test]
fn shr_round_with_ordering_fail() {
    assert_panic!(Integer::from(-512).shr_round_with_ordering(8, Exact, Less));
    assert_panic!(Integer::from(-512).shr_round_with_ordering(8, Exact, Greater));
    assert_panic!(Integer::from(-385).shr_round_with_ordering(8, Exact, Equal));
    assert_panic!((&Integer::from(0)).shr_round_with_ordering(8, Exact, Greater));
    assert_panic!({
        let mut x = Integer::from(0);
        x.shr_round_with_ordering_assign(8, Exact, Less)
    });
}

#[test]
fn shr_round_with_ordering_properties() {
    integer_unsigned_rounding_mode_ordering_quadruple_gen_var_1().test_properties(
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
                integer_shr_round_with_ordering_naive(&n, bits, rm, o),
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
            // Negating the input, the rounding mode, and the input's `Ordering` negates the result
            // and its `Ordering`.
            let (q_neg, o_neg) = (-&n).shr_round_with_ordering(bits, -rm, o.reverse());
            assert_eq!(-q_neg, q);
            assert_eq!(o_neg.reverse(), o_q);
        },
    );

    // Rounding twice, the second time with the first rounding's `Ordering`, is the same as rounding
    // once. With a shift of 0 bits, this holds only in the directed modes.
    integer_unsigned_unsigned_triple_gen_var_3::<u64>().test_properties(|(n, bits, j)| {
        for rm_1 in INEXACT_MODES {
            let (x, o) = (&n).shr_round(j, rm_1);
            for rm in INEXACT_MODES {
                if bits == 0 && rm == Nearest {
                    continue;
                }
                assert_eq!(
                    x.clone().shr_round_with_ordering(bits, rm, o),
                    (&n).shr_round(j + bits, rm)
                );
            }
        }
    });
}
