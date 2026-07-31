// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{DivisibleBy, Pow};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::{RemovePower, RemovePowerAssign};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::factorization::remove_power::limbs_remove;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{integer_pair_gen_var_9, natural_pair_gen_var_16};
use std::panic::catch_unwind;
use std::str::FromStr;

// The powers of 255 and of 65535 that the last three cases use; their limbs are what actually
// exercises the wide paths, and writing them as powers keeps the intent legible.
fn pow_limbs(base: u32, exp: u64) -> Vec<Limb> {
    Natural::from(base).pow(exp).to_limbs_asc()
}

#[test]
fn test_limbs_remove() {
    fn test(up: &[Limb], vp: &[Limb], cap: usize, out: &[Limb], k: usize) {
        let mut wp = Vec::new();
        assert_eq!(limbs_remove(&mut wp, up, vp, cap), k);
        assert_eq!(wp, out);
    }
    // - the divisor does not divide at all, so the first division is rejected
    // - !slice_test_zero(&tp[..pn]) && limbs_cmp_same_length(..) != Equal first time
    test(&[1], &[3], 0, &[1], 0);
    // - one division succeeds, and the cap stops the doubling immediately
    // - qp[qn] != 0 in the first loop
    // - ((2usize << npowers) - 1) > cap
    // - pwr + usize::power_of_2(..) > cap in the second loop
    test(&[3], &[3], 0, &[1], 1);
    // - the second loop reaches its division, using the divisor itself
    // - pwpsp_offsets[i] == usize::MAX
    // - the second loop's division is rejected
    test(&[3], &[3], 2, &[1], 1);
    // - the second loop's division is accepted
    // - qp[qn] != 0 in the second loop
    test(&[9], &[3], 2, &[1], 2);
    // - the cap allows a squaring, so the powers array is allocated and the divisor is squared
    // - npowers == 1
    // - current_power_is_vp when squaring
    // - !current_power_is_vp when dividing, and that division is rejected
    test(&[3], &[3], 3, &[1], 1);
    // - the second loop uses a stored power rather than the divisor
    // - pwpsp_offsets[i] != usize::MAX
    test(&[27], &[3], 5, &[1], 3);
    // - a stored power is itself squared
    // - !current_power_is_vp when squaring
    test(&[27], &[3], 7, &[1], 3);
    // - a quotient that loses its leading limb
    // - qp[qn] == 0 in the first loop
    test(
        &[18446744073709551613, 2],
        &[3],
        0,
        &[18446744073709551615],
        1,
    );
    // - a squared power whose product needs an extra limb
    // - powers_storage[np_offset + nn] != 0
    test(&pow_limbs(255, 15), &[255], 64, &[1], 15);
    // - the next power would be wider than what is left, stopping the doubling
    // - nn > qn
    // - a stored power too wide for the remainder is skipped in the second loop
    // - qn < pn
    test(&pow_limbs(65535, 15), &[65535], 64, &[1], 15);
}

#[test]
fn test_remove_power_natural() {
    let test = |s, t, q, k| {
        let x = Natural::from_str(s).unwrap();
        let y = Natural::from_str(t).unwrap();
        assert_eq!(
            x.clone().remove_power(y.clone()),
            (Natural::from_str(q).unwrap(), k)
        );
        assert_eq!(
            x.clone().remove_power(&y),
            (Natural::from_str(q).unwrap(), k)
        );
        assert_eq!(
            (&x).remove_power(y.clone()),
            (Natural::from_str(q).unwrap(), k)
        );
        assert_eq!((&x).remove_power(&y), (Natural::from_str(q).unwrap(), k));

        let mut mut_x = x.clone();
        assert_eq!(mut_x.remove_power_assign(y.clone()), k);
        assert_eq!(mut_x.to_string(), q);
        let mut mut_x = x;
        assert_eq!(mut_x.remove_power_assign(&y), k);
        assert_eq!(mut_x.to_string(), q);
    };
    // Small values take the single-limb fast path; the multi-limb cases at the end repeat these
    // branches against the kernel.
    // - *x == 0u32: zero is left alone, since every power of the factor divides it
    test("0", "3", "0", 0);
    // - x < y: nothing to remove, and the kernel wants a dividend at least as large as its divisor
    test("1", "3", "1", 0);
    // - two_pow == 0: an odd factor goes straight to the kernel, which rejects the division
    test("7", "3", "7", 0);
    // - odd == 1u32: a factor that is a power of two never reaches the kernel
    test("12", "2", "3", 2);
    test("96", "4", "6", 2);
    // - the kernel removes a deep power of an odd factor
    test("1215", "3", "5", 5);
    // - the factor need not be prime
    test("1000", "10", "1", 3);
    // - k == odd_limit: an even factor whose two halves run out together
    test("96", "6", "16", 1);
    test("1296", "6", "1", 4);
    // - k != odd_limit: the twos run out first, so the odd part is multiplied back
    test("9", "6", "9", 0);
    test("486", "6", "81", 1);
    test("1000000000000000000000000", "10", "1", 24);
    // Values that need more than one limb skip the single-limb fast path, so these repeat the
    // branches above against the kernel.
    // - x < y
    test(
        "1267650600228229401496703205376",
        "2535301200456458802993406410752",
        "1267650600228229401496703205376",
        0,
    );
    // - two_pow == 0
    test("182364981885853932015", "3", "5", 41);
    // - odd == 1u32
    test("3802951800684688204490109616128", "4", "3", 50);
    // - k == odd_limit
    test("221073919720733357899776", "6", "1", 30);
    // - k != odd_limit
    test(
        "717897987691852588770249",
        "6",
        "717897987691852588770249",
        0,
    );
}

#[test]
fn test_remove_power_integer() {
    let test = |s, t, q, k| {
        let x = Integer::from_str(s).unwrap();
        let y = Integer::from_str(t).unwrap();
        assert_eq!(
            x.clone().remove_power(y.clone()),
            (Integer::from_str(q).unwrap(), k)
        );
        assert_eq!((&x).remove_power(&y), (Integer::from_str(q).unwrap(), k));
        let mut mut_x = x;
        assert_eq!(mut_x.remove_power_assign(&y), k);
        assert_eq!(mut_x.to_string(), q);
    };
    // - x >= 0 and y >= 0
    test("0", "3", "0", 0);
    test("12", "2", "3", 2);
    // - the quotient is the exact division by the signed power, matching GMP: x < 0, and an even
    //   power leaves a negative factor's sign alone
    test("-12", "2", "-3", 2);
    test("12", "-2", "3", 2);
    test("-12", "-2", "-3", 2);
    // - k.odd() with y < 0: a negative factor raised to an odd power flips the sign
    test("-8", "2", "-1", 3);
    test("-8", "-2", "1", 3);
    test("8", "-2", "-1", 3);
    test("-1", "3", "-1", 0);
}

#[test]
fn remove_power_fail() {
    assert_panic!(Natural::from(12u32).remove_power(Natural::ZERO));
    assert_panic!(Natural::from(12u32).remove_power(Natural::ONE));
    assert_panic!(Integer::from(12).remove_power(Integer::ZERO));
    assert_panic!(Integer::from(12).remove_power(Integer::ONE));
    assert_panic!(Integer::from(12).remove_power(Integer::NEGATIVE_ONE));
    assert_panic!(Natural::from(12u32).remove_power_assign(Natural::ONE));
}

#[test]
fn remove_power_properties() {
    natural_pair_gen_var_16().test_properties(|(x, y)| {
        let (q, k) = (&x).remove_power(&y);
        assert!(q.is_valid());
        assert_eq!(x.clone().remove_power(y.clone()), (q.clone(), k));
        assert_eq!(x.clone().remove_power(&y), (q.clone(), k));
        assert_eq!((&x).remove_power(y.clone()), (q.clone(), k));
        let mut mut_x = x.clone();
        assert_eq!(mut_x.remove_power_assign(&y), k);
        assert_eq!(mut_x, q);

        // the defining identity, and the quotient has no factor left
        assert_eq!(&q * (&y).pow(k), x, "x={x} y={y} q={q} k={k}");
        if x != 0 {
            assert!(!(&q).divisible_by(&y), "x={x} y={y} q={q} k={k}");
        } else {
            assert_eq!(k, 0);
        }
        // GMP agrees
        let (rug_q, rug_k) = rug::Integer::from(&x).remove_factor(&rug::Integer::from(&y));
        assert_eq!(Natural::exact_from(&rug_q), q);
        assert_eq!(u64::from(rug_k), k);
    });

    natural_pair_gen_var_16().test_properties(|(x, y)| {
        // a value built to contain a known power of the factor gives back at least that power;
        // random pairs almost never divide, so this is what exercises repeated removal
        if x != 0 {
            let k = 5;
            let (q, found) = (&x * (&y).pow(k)).remove_power(&y);
            assert!(found >= k);
            assert_eq!(&q * (&y).pow(found), &x * (&y).pow(k));
            assert!(!(&q).divisible_by(&y));
        }
    });

    integer_pair_gen_var_9().test_properties(|(x, y)| {
        let (q, k) = (&x).remove_power(&y);
        assert!(q.is_valid());
        let mut mut_x = x.clone();
        assert_eq!(mut_x.remove_power_assign(&y), k);
        assert_eq!(mut_x, q);

        assert_eq!(&q * (&y).pow(k), x);
        // the exponent depends only on the magnitudes
        assert_eq!(x.unsigned_abs_ref().remove_power(y.unsigned_abs_ref()).1, k);
        // GMP agrees, signs included
        let (rug_q, rug_k) = rug::Integer::from(&x).remove_factor(&rug::Integer::from(&y));
        assert_eq!(Integer::from(&rug_q), q);
        assert_eq!(u64::from(rug_k), k);
    });
}
