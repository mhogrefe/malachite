// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Conjugate, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_nz::test_util::generators::gaussian_integer_unsigned_pair_gen_var_1;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_signed_pair_gen_var_1,
    gaussian_rational_unsigned_pair_gen_var_1, rational_signed_pair_gen_var_1,
    rational_unsigned_pair_gen_var_1,
};
use std::ops::{Shl, Shr, ShrAssign};
use std::str::FromStr;

fn test_shr_unsigned_helper<T: PrimitiveUnsigned>()
where
    GaussianRational: Shr<T, Output = GaussianRational> + ShrAssign<T>,
    for<'a> &'a GaussianRational: Shr<T, Output = GaussianRational>,
{
    let test = |s, v: u8, out| {
        let u = GaussianRational::from_str(s).unwrap();
        let v = T::from(v);

        let mut n = u.clone();
        n >>= v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let n = u.clone() >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let n = &u >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());
    };
    test("0", 0, "0");
    test("0", 10, "0");
    test("123", 0, "123");
    test("246", 1, "123");
    test("123", 1, "123/2");
    test("i", 1, "i/2");
    test("1+2i/3", 1, "1/2+i/3");
    test("14/11-4i", 2, "7/22-i");
    test("-984+i", 3, "-123+i/8");
    test("1267650600228229401496703205376i", 100, "i");
    test("1+i", 10, "1/1024+i/1024");
}

#[test]
fn test_shr_unsigned() {
    apply_fn_to_unsigneds!(test_shr_unsigned_helper);
}

fn test_shr_signed_helper<T: PrimitiveSigned>()
where
    GaussianRational: Shr<T, Output = GaussianRational> + ShrAssign<T>,
    for<'a> &'a GaussianRational: Shr<T, Output = GaussianRational>,
{
    let test = |s, v: i8, out| {
        let u = GaussianRational::from_str(s).unwrap();
        let v = T::from(v);

        let mut n = u.clone();
        n >>= v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let n = u.clone() >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let n = &u >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());
    };
    test("0", 0, "0");
    test("0", 10, "0");
    test("0", -10, "0");
    test("246", 1, "123");
    test("123", -1, "246");
    test("14/11-4i", 2, "7/22-i");
    test("7/22-i", -2, "14/11-4i");
    test("123-i/2", 2, "123/4-i/8");
    test("11/14+11i/14", -2, "22/7+22i/7");
    test("1/2+i/2", -1, "1+i");
    test("i", 100, "i/1267650600228229401496703205376");
}

#[test]
fn test_shr_signed() {
    apply_fn_to_signeds!(test_shr_signed_helper);
}

fn shr_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    GaussianRational:
        Shr<T, Output = GaussianRational> + ShrAssign<T> + Shl<T, Output = GaussianRational>,
    for<'a> &'a GaussianRational:
        Shr<T, Output = GaussianRational> + Shl<T, Output = GaussianRational>,
    for<'a> &'a Rational: Shr<T, Output = Rational>,
    Rational: Shr<T, Output = Rational>,
{
    gaussian_rational_unsigned_pair_gen_var_1::<T>().test_properties(|(n, u)| {
        let mut mut_n = n.clone();
        mut_n >>= u;
        assert!(mut_n.real.is_valid());
        assert!(mut_n.imaginary.is_valid());
        let shifted = mut_n;

        let shifted_alt = &n >> u;
        assert!(shifted_alt.real.is_valid());
        assert!(shifted_alt.imaginary.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone() >> u;
        assert!(shifted_alt.real.is_valid());
        assert!(shifted_alt.imaginary.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert_eq!(shifted.real, &n.real >> u);
        assert_eq!(shifted.imaginary, &n.imaginary >> u);
        assert!(shifted.le_abs(&n));
        assert_eq!(-&n >> u, -(&n >> u));
        assert_eq!((&n).conjugate() >> u, (&n >> u).conjugate());
        assert_eq!(&n >> u, &n << u >> u >> u);
        assert_eq!(&n >> u << u, n);
    });

    gaussian_rational_gen().test_properties(|n| {
        assert_eq!(&n >> T::ZERO, n);
    });

    unsigned_gen::<T>().test_properties(|u| {
        assert_eq!(GaussianRational::ZERO >> u, GaussianRational::ZERO);
    });

    rational_unsigned_pair_gen_var_1::<T>().test_properties(|(n, u)| {
        assert_eq!(
            GaussianRational::from(n.clone()) >> u,
            GaussianRational::from(n >> u)
        );
    });
}

fn shr_properties_helper_signed<T: PrimitiveSigned>()
where
    GaussianRational:
        Shr<T, Output = GaussianRational> + ShrAssign<T> + Shl<T, Output = GaussianRational>,
    for<'a> &'a GaussianRational: Shr<T, Output = GaussianRational>
        + Shr<<T as UnsignedAbs>::Output, Output = GaussianRational>
        + Shl<T, Output = GaussianRational>,
    for<'a> &'a Rational: Shr<T, Output = Rational>,
    Rational: Shr<T, Output = Rational>,
{
    gaussian_rational_signed_pair_gen_var_1::<T>().test_properties(|(n, i)| {
        let mut mut_n = n.clone();
        mut_n >>= i;
        assert!(mut_n.real.is_valid());
        assert!(mut_n.imaginary.is_valid());
        let shifted = mut_n;

        let shifted_alt = &n >> i;
        assert!(shifted_alt.real.is_valid());
        assert!(shifted_alt.imaginary.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone() >> i;
        assert!(shifted_alt.real.is_valid());
        assert!(shifted_alt.imaginary.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert_eq!(shifted.real, &n.real >> i);
        assert_eq!(shifted.imaginary, &n.imaginary >> i);
        if i >= T::ZERO {
            assert_eq!(&n >> i.unsigned_abs(), shifted);
            assert!(shifted.le_abs(&n));
        } else {
            assert!(shifted.ge_abs(&n));
        }
        assert_eq!(-&n >> i, -(&n >> i));
        assert_eq!((&n).conjugate() >> i, (&n >> i).conjugate());
        assert_eq!(&n >> i << i, n);
        if let Some(neg_i) = i.checked_neg() {
            assert_eq!(&n >> neg_i, &n << i);
            assert_eq!(&(&n >> i) >> neg_i, n);
        }
    });

    gaussian_rational_gen().test_properties(|n| {
        assert_eq!(&n >> T::ZERO, n);
    });

    signed_gen::<T>().test_properties(|i| {
        assert_eq!(GaussianRational::ZERO >> i, GaussianRational::ZERO);
    });

    rational_signed_pair_gen_var_1::<T>().test_properties(|(n, i)| {
        assert_eq!(
            GaussianRational::from(n.clone()) >> i,
            GaussianRational::from(n >> i)
        );
    });
}

#[test]
fn shr_properties() {
    apply_fn_to_unsigneds!(shr_properties_helper_unsigned);
    apply_fn_to_signeds!(shr_properties_helper_signed);

    gaussian_integer_unsigned_pair_gen_var_1::<u64>().test_properties(|(n, u)| {
        assert_eq!(
            (GaussianRational::from(&n) >> u) << u,
            GaussianRational::from(n)
        );
    });
}
