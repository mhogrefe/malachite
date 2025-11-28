// Copyright © 2025 William Youmans
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::CheckedSquare;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::{ExpressAsPower, Factor};
use malachite_base::num::random::random_unsigned_bit_chunks;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::generators::{
    signed_gen, signed_unsigned_pair_gen_var_15, unsigned_gen, unsigned_pair_gen_var_29,
};

const NUM_TESTS: usize = 1000;
const BITS: u64 = 64;

#[test]
fn test_perfect_squares() {
    // Test that squares pass the test
    let iter = random_unsigned_bit_chunks::<u64>(EXAMPLE_SEED, BITS / 2).take(NUM_TESTS);
    for d in iter {
        if let Some(d_squared) = d.checked_square() {
            let res = d_squared.express_as_power();
            assert!(res.is_some());

            let (base, exp) = res.unwrap();
            assert_eq!(base.pow(u32::exact_from(exp)), d_squared);
        }
    }
}

#[test]
fn test_perfect_cubes() {
    let iter = random_unsigned_bit_chunks::<u32>(EXAMPLE_SEED, BITS / 3).take(NUM_TESTS);
    for d in iter {
        if let Some(d_pow) = d.checked_pow(3) {
            let res = d_pow.express_as_power();
            assert!(res.is_some());

            let (base, exp) = res.unwrap();
            assert_eq!(base.pow(u32::exact_from(exp)), d_pow);
        }
    }
}

#[test]
fn test_perfect_fifth_powers() {
    let iter = random_unsigned_bit_chunks::<u32>(EXAMPLE_SEED, BITS / 5).take(NUM_TESTS);
    for d in iter {
        if let Some(d_pow) = d.checked_pow(5) {
            let res = d_pow.express_as_power();
            assert!(res.is_some());

            let (base, exp) = res.unwrap();
            assert_eq!(base.pow(u32::exact_from(exp)), d_pow);
        }
    }
}

#[test]
fn test_exhaustive_other_powers() {
    // Exhaustively test all other powers This tests all bases from 2 up to 2^(WORD_BITS/5) and all
    // their powers
    let max_base = 1u64 << (64 / 5); // Limit to prevent excessive test time

    for d in 2..max_base {
        let mut n = d * d; // Start with d^2

        // Keep multiplying by d until we overflow
        loop {
            let result = n.express_as_power();

            if let Some((base, exp)) = result {
                assert_eq!(base.pow(u32::exact_from(exp)), n);
            }

            // Try to multiply by d, break if overflow
            match n.checked_mul(d) {
                Some(next_n) => n = next_n,
                None => break, // Overflow occurred
            }
        }
    }
}

#[test]
fn test_non_perfect_powers() {
    let iter = random_unsigned_bit_chunks::<u64>(EXAMPLE_SEED, 64).take(NUM_TESTS);
    for d in iter {
        // naive perfect power testing by factoring
        if d.factor().into_iter().count() != 1 {
            assert!(d.express_as_power().is_none());
        }
    }
}

#[test]
fn test_edge_cases() {
    // non-perfect powers
    let non_pows: [u64; 5] = [2, 3, 6, 11, 15];
    for x in non_pows {
        assert_eq!(x.express_as_power(), None);
    }

    let pows: [u64; 12] = [0, 1, 4, 8, 9, 16, 25, 32, 64, 81, 100, 128];
    for x in pows {
        let (base, exp) = x.express_as_power().unwrap();
        assert_eq!(x, base.pow(u32::exact_from(exp)));
    }
}

#[test]
fn test_signed() {
    assert_eq!(0i8.express_as_power().unwrap(), (0, 2));
    assert_eq!(1i16.express_as_power().unwrap(), (1, 2));
    assert_eq!(4i32.express_as_power().unwrap(), (2, 2));
    assert_eq!(8i64.express_as_power().unwrap(), (2, 3));

    assert_eq!((-1i32).express_as_power(), None);
    // 64 = 2^6 = 4^3 but 3 is the largest odd exponent, so -64 = (-4)^3 where in the unsigned case
    // we expect 2^6. etc.
    assert_eq!((-64i32).express_as_power().unwrap(), (-4, 3));
    assert_eq!((-4096i64).express_as_power().unwrap(), (-16, 3));
    assert_eq!((-3486784401i64).express_as_power().unwrap(), (-81, 5));
}

fn express_as_power_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        if let Some((p, e)) = x.express_as_power() {
            assert!(e > 1);
            assert_eq!(p.pow(e), x);
            if x > T::ONE {
                assert!(p.express_as_power().is_none());
            }
        }
    });

    unsigned_pair_gen_var_29::<T>().test_properties(|(x, y)| {
        if y > 1 {
            let power = x.pow(y);
            let ope = power.express_as_power();
            assert!(ope.is_some());
            let (p, e) = ope.unwrap();
            assert_eq!(p.pow(e), power);
            if x.express_as_power().is_none() {
                assert_eq!(x, p);
                assert_eq!(y, e);
            }
        }
    });
}

fn express_as_power_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|x| {
        if let Some((p, e)) = x.express_as_power() {
            assert!(e > 1);
            assert_eq!(p.pow(e), x);
            if x > T::ONE {
                assert!(p.express_as_power().is_none());
            }
        }
    });

    signed_unsigned_pair_gen_var_15::<T>().test_properties(|(x, y)| {
        if x != T::NEGATIVE_ONE && y > 1 {
            let power = x.pow(y);
            let ope = power.express_as_power();
            assert!(ope.is_some());
            let (p, e) = ope.unwrap();
            assert_eq!(p.pow(e), power);
            if x >= T::ZERO && x.express_as_power().is_none() {
                assert!(x.eq_abs(&p));
                assert_eq!(y, e);
            }
        }
    });
}

#[test]
fn express_as_power_properties() {
    apply_fn_to_unsigneds!(express_as_power_properties_helper_unsigned);
    apply_fn_to_signeds!(express_as_power_properties_helper_signed);
}

fn is_power_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.is_power(), x.express_as_power().is_some());
    });

    unsigned_pair_gen_var_29::<T>().test_properties(|(x, y)| {
        if y > 1 {
            assert!(x.pow(y).is_power());
        }
    });
}

fn is_power_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|x| {
        assert_eq!(x.is_power(), x.express_as_power().is_some());
    });

    signed_unsigned_pair_gen_var_15::<T>().test_properties(|(x, y)| {
        if x != T::NEGATIVE_ONE && y > 1 {
            assert!(x.pow(y).is_power());
        }
    });
}

#[test]
fn is_power_properties() {
    apply_fn_to_unsigneds!(is_power_properties_helper_unsigned);
    apply_fn_to_signeds!(is_power_properties_helper_signed);
}
