// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{IsPowerOf2, ShlRound};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    signed_gen, unsigned_gen_var_5, unsigned_vec_unsigned_pair_gen_var_16,
    unsigned_vec_unsigned_pair_gen_var_32, unsigned_vec_unsigned_vec_unsigned_triple_gen_var_22,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_23,
};
use malachite_nz::natural::arithmetic::shl::{
    limbs_shl, limbs_shl_to_out, limbs_shl_with_complement_to_out, limbs_slice_shl_in_place,
    limbs_vec_shl_in_place,
};
use malachite_nz::natural::logic::not::limbs_not_in_place;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_gen, natural_signed_pair_gen_var_2, natural_unsigned_pair_gen_var_4,
    natural_unsigned_unsigned_triple_gen_var_5,
};
use num::BigUint;
use rug;
use std::ops::{Shl, ShlAssign, Shr};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shl_and_limbs_vec_shl_in_place() {
    let test = |limbs: &[Limb], bits: u64, out: &[Limb]| {
        assert_eq!(limbs_shl(limbs, bits), out);

        let mut limbs = limbs.to_vec();
        limbs_vec_shl_in_place(&mut limbs, bits);
        assert_eq!(limbs, out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[]);
    test(&[], 100, &[0, 0, 0]);
    test(&[6, 7], 2, &[24, 28]);
    test(&[100, 101, 102], 10, &[102400, 103424, 104448]);
    test(&[123, 456], 1, &[246, 912]);
    test(&[123, 456], 31, &[0x80000000, 61, 228]);
    test(&[123, 456], 32, &[0, 123, 456]);
    test(&[123, 456], 100, &[0, 0, 0, 1968, 7296]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shl_to_out() {
    let test =
        |out_before: &[Limb], limbs_in: &[Limb], bits: u64, carry: Limb, out_after: &[Limb]| {
            let mut out = out_before.to_vec();
            assert_eq!(limbs_shl_to_out(&mut out, limbs_in, bits), carry);
            assert_eq!(out, out_after);
        };
    test(&[10, 10, 10, 10], &[], 5, 0, &[10, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, 0, &[24, 28, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        0,
        &[102400, 103424, 104448, 10],
    );
    test(&[10, 10, 10, 10], &[123, 456], 1, 0, &[246, 912, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        31,
        228,
        &[0x80000000, 61, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shl_to_out_fail_1() {
    limbs_shl_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shl_to_out_fail_2() {
    limbs_shl_to_out(&mut [10, 10, 10], &[123, 456], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shl_to_out_fail_3() {
    limbs_shl_to_out(&mut [10, 10, 10], &[123, 456], 100);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_shl_in_place() {
    let test = |limbs: &[Limb], bits: u64, carry: Limb, out: &[Limb]| {
        let mut limbs = limbs.to_vec();
        assert_eq!(limbs_slice_shl_in_place(&mut limbs, bits), carry);
        assert_eq!(limbs, out);
    };
    test(&[], 5, 0, &[]);
    test(&[6, 7], 2, 0, &[24, 28]);
    test(&[100, 101, 102], 10, 0, &[102400, 103424, 104448]);
    test(&[123, 456], 1, 0, &[246, 912]);
    test(&[123, 456], 31, 228, &[0x80000000, 61]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_shl_in_place_fail_1() {
    limbs_slice_shl_in_place(&mut [123, 456], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_shl_in_place_fail_2() {
    limbs_slice_shl_in_place(&mut [123, 456], 100);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_shl_with_complement_to_out() {
    let test =
        |out_before: &[Limb], limbs_in: &[Limb], bits: u64, carry: Limb, out_after: &[Limb]| {
            let mut out = out_before.to_vec();
            assert_eq!(
                limbs_shl_with_complement_to_out(&mut out, limbs_in, bits),
                carry
            );
            assert_eq!(out, out_after);
        };
    test(
        &[10, 10, 10, 10],
        &[6, 7],
        2,
        0,
        &[4294967271, 4294967267, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        0,
        &[4294864895, 4294863871, 4294862847, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        1,
        0,
        &[4294967049, 4294966383, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        31,
        228,
        &[0x7fffffff, 4294967234, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shl_with_complement_to_out_fail_1() {
    limbs_shl_with_complement_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shl_with_complement_to_out_fail_2() {
    limbs_shl_with_complement_to_out(&mut [10, 10, 10], &[123, 456], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shl_with_complement_to_out_fail_3() {
    limbs_shl_with_complement_to_out(&mut [10, 10, 10], &[123, 456], 100);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_shl_with_complement_to_out_fail_4() {
    limbs_shl_with_complement_to_out(&mut [10, 10, 10], &[], 100);
}

fn test_shl_unsigned_helper<T: PrimitiveUnsigned, F: Fn(&str, T, &str)>(f: F)
where
    Natural: Shl<T, Output = Natural> + ShlAssign<T>,
    for<'a> &'a Natural: Shl<T, Output = Natural>,
{
    let test = |s, v: u8, out| {
        let u = Natural::from_str(s).unwrap();
        let v = T::from(v);

        let mut n = u.clone();
        n <<= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        f(s, v, out);
    };
    test("0", 0, "0");
    test("0", 10, "0");
    test("123", 0, "123");
    test("123", 1, "246");
    test("123", 2, "492");
    test("123", 25, "4127195136");
    test("123", 26, "8254390272");
    test("123", 100, "155921023828072216384094494261248");
    test("2147483648", 1, "4294967296");
    test("1000000000000", 0, "1000000000000");
    test("1000000000000", 3, "8000000000000");
    test("1000000000000", 24, "16777216000000000000");
    test("1000000000000", 25, "33554432000000000000");
    test("1000000000000", 31, "2147483648000000000000");
    test("1000000000000", 32, "4294967296000000000000");
    test("1000000000000", 33, "8589934592000000000000");
    test(
        "1000000000000",
        100,
        "1267650600228229401496703205376000000000000",
    );
}

#[test]
fn test_shl_unsigned() {
    test_shl_unsigned_helper::<u8, _>(|_, _, _| {});
    test_shl_unsigned_helper::<u16, _>(|_, _, _| {});
    test_shl_unsigned_helper::<u32, _>(|u, v, out| {
        let rug_u = rug::Integer::from_str(u).unwrap();
        let mut n = rug_u.clone();
        n <<= v;
        assert_eq!(n.to_string(), out);

        let n = rug_u << v;
        assert_eq!(n.to_string(), out);

        let num_u = BigUint::from_str(u).unwrap();
        let n = num_u.clone() << usize::exact_from(v);
        assert_eq!(n.to_string(), out);

        let n = &num_u << usize::exact_from(v);
        assert_eq!(n.to_string(), out);
    });
    test_shl_unsigned_helper::<u64, _>(|_, _, _| {});
    test_shl_unsigned_helper::<u128, _>(|_, _, _| {});
    test_shl_unsigned_helper::<usize, _>(|_, _, _| {});
}

fn test_shl_signed_helper<T: PrimitiveSigned, F: Fn(&str, T, &str)>(f: F)
where
    Natural: Shl<T, Output = Natural> + ShlAssign<T>,
    for<'a> &'a Natural: Shl<T, Output = Natural>,
{
    let test = |s, v: i8, out| {
        let u = Natural::from_str(s).unwrap();
        let v = T::from(v);

        let mut n = u.clone();
        n <<= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u << v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        f(s, v, out);
    };
    test("0", 0, "0");
    test("0", 10, "0");
    test("123", 0, "123");
    test("123", 1, "246");
    test("123", 2, "492");
    test("123", 25, "4127195136");
    test("123", 26, "8254390272");
    test("123", 100, "155921023828072216384094494261248");
    test("2147483648", 1, "4294967296");
    test("1000000000000", 0, "1000000000000");
    test("1000000000000", 3, "8000000000000");
    test("1000000000000", 24, "16777216000000000000");
    test("1000000000000", 25, "33554432000000000000");
    test("1000000000000", 31, "2147483648000000000000");
    test("1000000000000", 32, "4294967296000000000000");
    test("1000000000000", 33, "8589934592000000000000");
    test(
        "1000000000000",
        100,
        "1267650600228229401496703205376000000000000",
    );

    test("0", -10, "0");
    test("123", 0, "123");
    test("245", -1, "122");
    test("246", -1, "123");
    test("247", -1, "123");
    test("491", -2, "122");
    test("492", -2, "123");
    test("493", -2, "123");
    test("4127195135", -25, "122");
    test("4127195136", -25, "123");
    test("4127195137", -25, "123");
    test("8254390271", -26, "122");
    test("8254390272", -26, "123");
    test("8254390273", -26, "123");
    test("155921023828072216384094494261247", -100, "122");
    test("155921023828072216384094494261248", -100, "123");
    test("155921023828072216384094494261249", -100, "123");
    test("4294967295", -1, "2147483647");
    test("4294967296", -1, "2147483648");
    test("4294967297", -1, "2147483648");
    test("7999999999999", -3, "999999999999");
    test("8000000000000", -3, "1000000000000");
    test("8000000000001", -3, "1000000000000");
    test("16777216000000000000", -24, "1000000000000");
    test("33554432000000000000", -25, "1000000000000");
    test("2147483648000000000000", -31, "1000000000000");
    test("4294967296000000000000", -32, "1000000000000");
    test("8589934592000000000000", -33, "1000000000000");
    test(
        "1267650600228229401496703205376000000000000",
        -100,
        "1000000000000",
    );
    test("1000000000000", -10, "976562500");
    test("980657949", -72, "0");
    test("4294967295", -31, "1");
    test("4294967295", -32, "0");
    test("4294967296", -32, "1");
    test("4294967296", -33, "0");
}

#[test]
fn test_shl_signed() {
    test_shl_signed_helper::<i8, _>(|_, _, _| {});
    test_shl_signed_helper::<i16, _>(|_, _, _| {});
    test_shl_signed_helper::<i32, _>(|u, v, out| {
        let rug_u = rug::Integer::from_str(u).unwrap();
        let mut n = rug_u.clone();
        n <<= v;
        assert_eq!(n.to_string(), out);

        let n = rug_u << v;
        assert_eq!(n.to_string(), out);
    });
    test_shl_signed_helper::<i64, _>(|_, _, _| {});
    test_shl_signed_helper::<i128, _>(|_, _, _| {});
    test_shl_signed_helper::<isize, _>(|_, _, _| {});
}

#[test]
fn limbs_shl_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(&config, |(xs, bits)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_shl(&xs, bits)),
            Natural::from_owned_limbs_asc(xs) << bits
        );
    });
}

#[test]
fn limbs_shl_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_22::<Limb, Limb>()
        .test_properties_with_config(&config, |(mut out, xs, bits)| {
            let old_out = out.clone();
            let carry = limbs_shl_to_out(&mut out, &xs, bits);
            let len = xs.len();
            let n = Natural::from_owned_limbs_asc(xs) << bits;
            let mut xs = n.into_limbs_asc();
            assert_eq!(carry != 0, xs.len() == len + 1);
            let mut actual_xs = out[..len].to_vec();
            if carry != 0 {
                actual_xs.push(carry);
            }
            xs.resize(actual_xs.len(), 0);
            assert_eq!(xs, actual_xs);
            assert_eq!(&out[len..], &old_out[len..]);
        });
}

#[test]
fn limbs_slice_shl_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_32::<Limb, Limb>().test_properties_with_config(
        &config,
        |(mut xs, bits)| {
            let old_xs = xs.clone();
            let carry = limbs_slice_shl_in_place(&mut xs, bits);
            let n = Natural::from_owned_limbs_asc(old_xs) << bits;
            let mut expected_xs = n.into_limbs_asc();
            assert_eq!(carry != 0, expected_xs.len() == xs.len() + 1);
            if carry != 0 {
                xs.push(carry);
            }
            expected_xs.resize(xs.len(), 0);
            assert_eq!(xs, expected_xs);
        },
    );
}

#[test]
fn limbs_vec_shl_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16::<Limb, u64>().test_properties_with_config(
        &config,
        |(mut xs, bits)| {
            let old_xs = xs.clone();
            limbs_vec_shl_in_place(&mut xs, bits);
            let n = Natural::from_owned_limbs_asc(old_xs) << bits;
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
        },
    );
}

#[test]
fn limbs_shl_with_complement_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_23::<Limb, Limb>()
        .test_properties_with_config(&config, |(mut out, xs, bits)| {
            let old_out = out.clone();
            let len = xs.len();
            let carry = limbs_shl_with_complement_to_out(&mut out, &xs, bits);
            limbs_not_in_place(&mut out[..len]);
            let n = Natural::from_owned_limbs_asc(xs) << bits;
            let mut xs = n.into_limbs_asc();
            assert_eq!(carry != 0, xs.len() == len + 1);
            let mut actual_xs = out[..len].to_vec();
            if carry != 0 {
                actual_xs.push(carry);
            }
            xs.resize(actual_xs.len(), 0);
            assert_eq!(xs, actual_xs);
            assert_eq!(&out[len..], &old_out[len..]);
        });
}

fn unsigned_properties<U: PrimitiveUnsigned, S: PrimitiveSigned + WrappingFrom<U>>()
where
    Natural: Shl<U, Output = Natural> + ShlAssign<U> + Shr<S, Output = Natural>,
    for<'a> &'a Natural:
        Shl<U, Output = Natural> + Shl<S, Output = Natural> + Shr<U, Output = Natural>,
{
    natural_unsigned_pair_gen_var_4::<U>().test_properties(|(n, u)| {
        let mut mut_n = n.clone();
        mut_n <<= u;
        assert!(mut_n.is_valid());
        let shifted = mut_n;

        let shifted_alt = &n << u;
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        let shifted_alt = n.clone() << u;
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert!(shifted >= n);
        assert_eq!(shifted, &n * (Natural::ONE << u));
        assert_eq!(&shifted >> u, n);

        if u <= U::low_mask(U::WIDTH - 1) {
            let u = S::wrapping_from(u);
            assert_eq!(&n << u, shifted);
            assert_eq!(n >> -u, shifted);
        }
    });

    natural_unsigned_unsigned_triple_gen_var_5::<U>().test_properties(|(n, u, v)| {
        if let Some(sum) = u.checked_add(v) {
            assert_eq!(&n << u << v, n << sum);
        }
    });

    natural_gen().test_properties(|n| {
        assert_eq!(&n << U::ZERO, n);
        assert_eq!(&n << U::ONE, n * Natural::from(2u32));
    });

    unsigned_gen_var_5::<U>().test_properties(|u| {
        assert_eq!(Natural::ZERO << u, 0);
        assert!((Natural::ONE << u).is_power_of_2());
    });
}

fn signed_properties<T: PrimitiveSigned>()
where
    Natural: Shl<T, Output = Natural> + ShlAssign<T> + ShlRound<T, Output = Natural>,
    for<'a> &'a Natural: Shl<T, Output = Natural>,
{
    natural_signed_pair_gen_var_2::<T>().test_properties(|(n, i)| {
        let mut mut_n = n.clone();
        mut_n <<= i;
        assert!(mut_n.is_valid());
        let shifted = mut_n;

        let shifted_alt = &n << i;
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        let shifted_alt = n.clone() << i;
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert_eq!(n.shl_round(i, Floor).0, shifted);
    });

    natural_gen().test_properties(|n| {
        assert_eq!(&n << T::ZERO, n);
    });

    signed_gen::<T>().test_properties(|i| {
        assert_eq!(Natural::ZERO << i, 0);
    });
}

#[test]
fn shl_properties() {
    apply_fn_to_unsigned_signed_pairs!(unsigned_properties);
    apply_fn_to_signeds!(signed_properties);

    natural_unsigned_pair_gen_var_4::<u32>().test_properties(|(n, u)| {
        let shifted = &n << u;
        let mut rug_n = rug::Integer::from(&n);
        rug_n <<= u;
        assert_eq!(Natural::exact_from(&rug_n), shifted);

        assert_eq!(
            Natural::from(&(&BigUint::from(&n) << usize::exact_from(u))),
            shifted
        );
        assert_eq!(
            Natural::from(&(BigUint::from(&n) << usize::exact_from(u))),
            shifted
        );
        assert_eq!(Natural::exact_from(&(rug::Integer::from(&n) << u)), shifted);
    });

    natural_signed_pair_gen_var_2::<i32>().test_properties(|(n, i)| {
        let shifted = &n << i;
        let mut rug_n = rug::Integer::from(&n);
        rug_n <<= i;
        assert_eq!(Natural::exact_from(&rug_n), shifted);

        assert_eq!(Natural::exact_from(&(rug::Integer::from(&n) << i)), shifted);
    });
}
