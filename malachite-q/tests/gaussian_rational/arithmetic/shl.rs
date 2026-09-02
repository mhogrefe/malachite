// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Conjugate, PowerOf2, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_nz::test_util::generators::gaussian_integer_unsigned_pair_gen_var_1;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_signed_pair_gen_var_1,
    gaussian_rational_unsigned_pair_gen_var_1, rational_signed_pair_gen_var_1,
    rational_unsigned_pair_gen_var_1,
};
use std::ops::{Shl, ShlAssign};
use std::str::FromStr;

fn test_shl_unsigned_helper<T: PrimitiveUnsigned>()
where
    GaussianRational: Shl<T, Output = GaussianRational> + ShlAssign<T>,
    for<'a> &'a GaussianRational: Shl<T, Output = GaussianRational>,
{
    let test = |s, v: u8, out| {
        let u = GaussianRational::from_str(s).unwrap();
        let v = T::from(v);

        let mut n = u.clone();
        n <<= v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let n = u.clone() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let n = &u << v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());
    };
    test("0", 0, "0");
    test("0", 10, "0");
    test("123", 0, "123");
    test("123", 1, "246");
    test("i", 1, "2i");
    test("1/2+i/3", 1, "1+2i/3");
    test("7/22-i", 2, "14/11-4i");
    test("-123+i/8", 3, "-984+i");
    test("i", 100, "1267650600228229401496703205376i");
    test("1/1024+i/1024", 10, "1+i");
}

#[test]
fn test_shl_unsigned() {
    apply_fn_to_unsigneds!(test_shl_unsigned_helper);
}

fn test_shl_signed_helper<T: PrimitiveSigned>()
where
    GaussianRational: Shl<T, Output = GaussianRational> + ShlAssign<T>,
    for<'a> &'a GaussianRational: Shl<T, Output = GaussianRational>,
{
    let test = |s, v: i8, out| {
        let u = GaussianRational::from_str(s).unwrap();
        let v = T::from(v);

        let mut n = u.clone();
        n <<= v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let n = u.clone() << v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let n = &u << v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());
    };
    test("0", 0, "0");
    test("0", 10, "0");
    test("0", -10, "0");
    test("123", 1, "246");
    test("123", -1, "123/2");
    test("7/22-i", 2, "14/11-4i");
    test("7/22-i", -2, "7/88-i/4");
    test("123-i/2", -2, "123/4-i/8");
    test("22/7+22i/7", -2, "11/14+11i/14");
    test("1+i", -1, "1/2+i/2");
    test("i", -100, "i/1267650600228229401496703205376");
}

#[test]
fn test_shl_signed() {
    apply_fn_to_signeds!(test_shl_signed_helper);
}

fn shl_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    GaussianRational: Shl<T, Output = GaussianRational> + ShlAssign<T>,
    for<'a> &'a GaussianRational: Shl<T, Output = GaussianRational>,
    for<'a> &'a Rational: Shl<T, Output = Rational>,
    Rational: Shl<T, Output = Rational>,
    u64: TryFrom<T>,
{
    gaussian_rational_unsigned_pair_gen_var_1::<T>().test_properties(|(n, u)| {
        let mut mut_n = n.clone();
        mut_n <<= u;
        assert!(mut_n.real.is_valid());
        assert!(mut_n.imaginary.is_valid());
        let shifted = mut_n;

        let shifted_alt = &n << u;
        assert!(shifted_alt.real.is_valid());
        assert!(shifted_alt.imaginary.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone() << u;
        assert!(shifted_alt.real.is_valid());
        assert!(shifted_alt.imaginary.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert_eq!(shifted.real, &n.real << u);
        assert_eq!(shifted.imaginary, &n.imaginary << u);
        assert!(shifted.ge_abs(&n));
        assert_eq!(-&n << u, -(&n << u));
        assert_eq!((&n).conjugate() << u, (&n << u).conjugate());
        assert_eq!(
            &n << u,
            &n * GaussianRational::power_of_2(u64::exact_from(u))
        );
    });

    gaussian_rational_gen().test_properties(|n| {
        assert_eq!(&n << T::ZERO, n);
    });

    unsigned_gen::<T>().test_properties(|u| {
        assert_eq!(GaussianRational::ZERO << u, GaussianRational::ZERO);
    });

    rational_unsigned_pair_gen_var_1::<T>().test_properties(|(n, u)| {
        assert_eq!(
            GaussianRational::from(n.clone()) << u,
            GaussianRational::from(n << u)
        );
    });
}

fn shl_properties_helper_signed<T: PrimitiveSigned>()
where
    GaussianRational: Shl<T, Output = GaussianRational> + ShlAssign<T>,
    for<'a> &'a GaussianRational: Shl<T, Output = GaussianRational>
        + Shl<<T as UnsignedAbs>::Output, Output = GaussianRational>,
    for<'a> &'a Rational: Shl<T, Output = Rational>,
    Rational: Shl<T, Output = Rational>,
    i64: TryFrom<T>,
{
    gaussian_rational_signed_pair_gen_var_1::<T>().test_properties(|(n, i)| {
        let mut mut_n = n.clone();
        mut_n <<= i;
        assert!(mut_n.real.is_valid());
        assert!(mut_n.imaginary.is_valid());
        let shifted = mut_n;

        let shifted_alt = &n << i;
        assert!(shifted_alt.real.is_valid());
        assert!(shifted_alt.imaginary.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone() << i;
        assert!(shifted_alt.real.is_valid());
        assert!(shifted_alt.imaginary.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert_eq!(shifted.real, &n.real << i);
        assert_eq!(shifted.imaginary, &n.imaginary << i);
        if i >= T::ZERO {
            assert_eq!(&n << i.unsigned_abs(), shifted);
            assert!(shifted.ge_abs(&n));
        } else {
            assert!(shifted.le_abs(&n));
        }
        assert_eq!(-&n << i, -(&n << i));
        assert_eq!((&n).conjugate() << i, (&n << i).conjugate());
        assert_eq!(
            &n << i,
            &n * GaussianRational::power_of_2(i64::exact_from(i))
        );
        if let Some(neg_i) = i.checked_neg() {
            assert_eq!(&(&n << i) << neg_i, n);
        }
    });

    gaussian_rational_gen().test_properties(|n| {
        assert_eq!(&n << T::ZERO, n);
    });

    signed_gen::<T>().test_properties(|i| {
        assert_eq!(GaussianRational::ZERO << i, GaussianRational::ZERO);
    });

    rational_signed_pair_gen_var_1::<T>().test_properties(|(n, i)| {
        assert_eq!(
            GaussianRational::from(n.clone()) << i,
            GaussianRational::from(n << i)
        );
    });
}

#[test]
fn shl_properties() {
    apply_fn_to_unsigneds!(shl_properties_helper_unsigned);
    apply_fn_to_signeds!(shl_properties_helper_signed);

    gaussian_integer_unsigned_pair_gen_var_1::<u64>().test_properties(|(n, u)| {
        assert_eq!(
            GaussianRational::from(&n) << u,
            GaussianRational::from(n << u)
        );
    });
}
