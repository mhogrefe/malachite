// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::limbs_sum_diff;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::large_type_gen_var_33;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_sum_diff() {
    let test = |x: &[Limb], y: &[Limb], carry: Limb, s_out: &[Limb], d_out: &[Limb]| {
        let n = x.len();
        let mut s = vec![0; n];
        let mut d = vec![0; n];
        assert_eq!(limbs_sum_diff(&mut s, &mut d, x, y, n), carry);
        assert_eq!(s, s_out);
        assert_eq!(d, d_out);
    };
    // - n != 0
    test(&[1], &[2], 1, &[3], &[18446744073709551615]);
    test(
        &[18446744073709551615],
        &[1],
        2,
        &[0],
        &[18446744073709551614],
    );
    test(
        &[3, 18446744073709551615],
        &[5, 1],
        2,
        &[8, 0],
        &[18446744073709551614, 18446744073709551613],
    );
    test(&[0, 0, 0], &[0, 0, 0], 0, &[0, 0, 0], &[0, 0, 0]);
    // - n == 0
    test(&[], &[], 0, &[], &[]);
}

#[test]
fn limbs_sum_diff_properties() {
    large_type_gen_var_33().test_properties(|(x, y)| {
        let n = x.len();
        let mut s = vec![0; n];
        let mut d = vec![0; n];
        let carry = limbs_sum_diff(&mut s, &mut d, &x, &y, n);
        assert!(carry < 4);
        let bits = u64::exact_from(n) << Limb::LOG_WIDTH;
        let xn = Natural::from_limbs_asc(&x);
        let yn = Natural::from_limbs_asc(&y);
        assert_eq!(
            Natural::from_limbs_asc(&s) + Natural::power_of_2(bits) * Natural::from(carry >> 1),
            &xn + &yn
        );
        assert_eq!(
            Integer::from(Natural::from_limbs_asc(&d))
                - Integer::power_of_2(bits) * Integer::from(carry & 1),
            Integer::from(xn) - Integer::from(yn)
        );
    });
}
