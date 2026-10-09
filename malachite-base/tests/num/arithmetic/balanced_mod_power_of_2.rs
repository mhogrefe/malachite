// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    BalancedMod, BalancedModPowerOf2, BalancedModPowerOf2Assign, UnsignedAbs,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_1, signed_unsigned_pair_gen_var_10, unsigned_pair_gen_var_2,
    unsigned_pair_gen_var_20,
};
use std::panic::catch_unwind;

#[test]
fn test_balanced_mod_power_of_2_unsigned() {
    assert_eq!(19u32.balanced_mod_power_of_2(3), 3);
    assert_eq!(23u32.balanced_mod_power_of_2(3), -1);
    // - exactly half the modulus is the top of the range, so it stays positive
    assert_eq!(20u32.balanced_mod_power_of_2(3), 4);
    assert_eq!(3u8.balanced_mod_power_of_2(1), 1);
    assert_eq!(64u8.balanced_mod_power_of_2(7), 64);
    assert_eq!(65u8.balanced_mod_power_of_2(7), -63);
    assert_eq!(128u8.balanced_mod_power_of_2(7), 0);
    // - modulo 2^0, everything is 0
    assert_eq!(255u8.balanced_mod_power_of_2(0), 0);
    // - pow equal to the width: the bits are reinterpreted around zero
    assert_eq!(127u8.balanced_mod_power_of_2(8), 127);
    assert_eq!(129u8.balanced_mod_power_of_2(8), -127);
    assert_eq!(200u8.balanced_mod_power_of_2(8), -56);
    assert_eq!(u64::MAX.balanced_mod_power_of_2(64), -1);
    // - pow above the width: small values are their own remainders
    assert_eq!(100u8.balanced_mod_power_of_2(9), 100);
    assert_eq!(0u8.balanced_mod_power_of_2(100), 0);
    assert_eq!(127u8.balanced_mod_power_of_2(100), 127);
}

#[test]
fn test_balanced_mod_power_of_2_signed() {
    assert_eq!(19i32.balanced_mod_power_of_2(3), 3);
    assert_eq!(23i32.balanced_mod_power_of_2(3), -1);
    assert_eq!(20i32.balanced_mod_power_of_2(3), 4);
    // - a negative value is reduced into the same range
    assert_eq!((-19i32).balanced_mod_power_of_2(3), -3);
    assert_eq!((-20i32).balanced_mod_power_of_2(3), 4);
    assert_eq!((-1i8).balanced_mod_power_of_2(1), 1);
    assert_eq!((-3i8).balanced_mod_power_of_2(2), 1);
    assert_eq!((-64i8).balanced_mod_power_of_2(7), 64);
    assert_eq!((-65i8).balanced_mod_power_of_2(7), 63);
    assert_eq!(i8::MIN.balanced_mod_power_of_2(7), 0);
    assert_eq!((-5i8).balanced_mod_power_of_2(0), 0);
    // - pow equal to the width: every value but the minimum is its own remainder
    assert_eq!((-1i8).balanced_mod_power_of_2(8), -1);
    assert_eq!(i8::MAX.balanced_mod_power_of_2(8), i8::MAX);
    assert_eq!((i8::MIN + 1).balanced_mod_power_of_2(8), i8::MIN + 1);
    // - pow above the width: every value is its own remainder
    assert_eq!(i8::MIN.balanced_mod_power_of_2(9), i8::MIN);
    assert_eq!((-100i8).balanced_mod_power_of_2(100), -100);
    // - the in-place form
    let mut x = 23i32;
    x.balanced_mod_power_of_2_assign(3);
    assert_eq!(x, -1);
}

fn balanced_mod_power_of_2_fail_helper<
    U: BalancedModPowerOf2<Output = S> + PrimitiveUnsigned,
    S: BalancedModPowerOf2<Output = S> + BalancedModPowerOf2Assign + PrimitiveSigned,
>() {
    // The results would be 2^(W-1), which does not fit in the signed type.
    assert_panic!(U::power_of_2(U::WIDTH - 1).balanced_mod_power_of_2(U::WIDTH));
    assert_panic!(U::MAX.balanced_mod_power_of_2(U::WIDTH + 1));
    assert_panic!(S::MIN.balanced_mod_power_of_2(S::WIDTH));
    assert_panic!({
        let mut x = S::MIN;
        x.balanced_mod_power_of_2_assign(S::WIDTH);
    });
}

#[test]
fn balanced_mod_power_of_2_fail() {
    apply_fn_to_unsigned_signed_pairs!(balanced_mod_power_of_2_fail_helper);
}

// The congruence and the range determine the balanced remainder uniquely, so checking both is a
// complete specification. For powers no greater than the width, the congruence is checked on the
// low bits, which wrapping conversions keep.
fn check_range_and_congruence<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x_low_bits: U,
    r: S,
    pow: u64,
) {
    assert!(pow <= U::WIDTH);
    assert_eq!(
        U::wrapping_from(r).mod_power_of_2(pow),
        x_low_bits.mod_power_of_2(pow)
    );
    if pow == 0 {
        assert_eq!(r, S::ZERO);
        return;
    }
    let half = U::power_of_2(pow - 1);
    let abs_r = r.unsigned_abs();
    assert!(abs_r <= half);
    // the endpoint at exactly half the modulus belongs to the positive side
    if abs_r == half {
        assert!(r > S::ZERO);
    }
}

fn check_unsigned<
    U: BalancedMod<U, Output = S>
        + BalancedModPowerOf2<Output = S>
        + PrimitiveUnsigned
        + WrappingFrom<S>,
    S: BalancedModPowerOf2<Output = S>
        + ExactFrom<U>
        + PrimitiveSigned
        + UnsignedAbs<Output = U>
        + WrappingFrom<U>,
>(
    x: U,
    pow: u64,
) {
    // Skip the inputs whose result does not fit; balanced_mod_power_of_2_fail covers them.
    let top = U::power_of_2(U::WIDTH - 1);
    if pow == U::WIDTH && x == top || pow > U::WIDTH && x >= top {
        return;
    }
    let r = x.balanced_mod_power_of_2(pow);
    if pow > U::WIDTH {
        assert_eq!(r, S::exact_from(x));
        return;
    }
    check_range_and_congruence(x, r, pow);
    if pow < U::WIDTH {
        assert_eq!(x.balanced_mod(U::power_of_2(pow)), r);
    }
    // Only the low bits matter, so the signed reinterpretation gives the same result.
    assert_eq!(S::wrapping_from(x).balanced_mod_power_of_2(pow), r);
}

fn check_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: BalancedMod<S, Output = S>
        + BalancedModPowerOf2<Output = S>
        + BalancedModPowerOf2Assign
        + PrimitiveSigned
        + UnsignedAbs<Output = U>,
>(
    x: S,
    pow: u64,
) {
    if pow == S::WIDTH && x == S::MIN {
        return;
    }
    let r = x.balanced_mod_power_of_2(pow);
    let mut mut_x = x;
    mut_x.balanced_mod_power_of_2_assign(pow);
    assert_eq!(mut_x, r);
    if pow > S::WIDTH {
        assert_eq!(r, x);
        return;
    }
    check_range_and_congruence(U::wrapping_from(x), r, pow);
    if pow + 1 < S::WIDTH {
        assert_eq!(x.balanced_mod(S::power_of_2(pow)), r);
    }
}

fn balanced_mod_power_of_2_properties_helper<
    U: BalancedMod<U, Output = S>
        + BalancedModPowerOf2<Output = S>
        + PrimitiveUnsigned
        + WrappingFrom<S>,
    S: BalancedMod<S, Output = S>
        + BalancedModPowerOf2<Output = S>
        + BalancedModPowerOf2Assign
        + ExactFrom<U>
        + PrimitiveSigned
        + UnsignedAbs<Output = U>
        + WrappingFrom<U>,
>() {
    // Small powers of any size, and powers mostly within the width.
    unsigned_pair_gen_var_2::<U, u64>().test_properties(|(x, pow)| check_unsigned::<U, S>(x, pow));
    unsigned_pair_gen_var_20::<U>().test_properties(|(x, pow)| check_unsigned::<U, S>(x, pow));
    signed_unsigned_pair_gen_var_1::<S, u64>()
        .test_properties(|(x, pow)| check_signed::<U, S>(x, pow));
    signed_unsigned_pair_gen_var_10::<S>().test_properties(|(x, pow)| check_signed::<U, S>(x, pow));
}

#[test]
fn balanced_mod_power_of_2_properties() {
    apply_fn_to_unsigned_signed_pairs!(balanced_mod_power_of_2_properties_helper);
}
