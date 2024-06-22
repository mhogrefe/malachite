// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{IsPowerOf2, PowerOf2, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{
    signed_gen, signed_gen_var_5, unsigned_gen, unsigned_gen_var_5,
};
use malachite_float::test_util::common::{parse_hex_string, to_hex_string};
use malachite_float::test_util::generators::{
    float_gen, float_signed_pair_gen_var_2, float_unsigned_pair_gen_var_2,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::integer::Integer;
use std::ops::{Shl, Shr, ShrAssign};

fn test_shr_unsigned_helper<T: PrimitiveUnsigned, F: Fn(Float, T, Float)>(f: F)
where
    Float: ShrAssign<T> + Shr<T, Output = Float>,
    for<'a> &'a Float: Shr<T, Output = Float>,
{
    let test = |s, s_hex, v: u8, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let v = T::from(v);

        let mut n = x.clone();
        n >>= v;
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert!(n.is_valid());

        let n = x.clone() >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &x >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        f(x, v, n);
    };
    test("NaN", "NaN", 0, "NaN", "NaN");
    test("NaN", "NaN", 10, "NaN", "NaN");
    test("Infinity", "Infinity", 0, "Infinity", "Infinity");
    test("Infinity", "Infinity", 10, "Infinity", "Infinity");
    test("-Infinity", "-Infinity", 0, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 10, "-Infinity", "-Infinity");
    test("0.0", "0x0.0", 10, "0.0", "0x0.0");
    test("-0.0", "-0x0.0", 10, "-0.0", "-0x0.0");

    test("123.0", "0x7b.0#7", 0, "123.0", "0x7b.0#7");
    test("123.0", "0x7b.0#7", 1, "61.5", "0x3d.8#7");
    test("123.0", "0x7b.0#7", 10, "0.12", "0x0.1ec#7");
    test("123.0", "0x7b.0#7", 100, "9.7e-29", "0x7.bE-24#7");

    test("-123.0", "-0x7b.0#7", 0, "-123.0", "-0x7b.0#7");
    test("-123.0", "-0x7b.0#7", 1, "-61.5", "-0x3d.8#7");
    test("-123.0", "-0x7b.0#7", 10, "-0.12", "-0x0.1ec#7");
    test("-123.0", "-0x7b.0#7", 100, "-9.7e-29", "-0x7.bE-24#7");

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        0,
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "0.0030679615757712823",
        "0x0.00c90fdaa22168c0#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        "2.4782796245465248e-30",
        "0x3.243f6a8885a30E-25#53",
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        0,
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "-1.5707963267948966",
        "-0x1.921fb54442d18#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-0.0030679615757712823",
        "-0x0.00c90fdaa22168c0#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        "-2.4782796245465248e-30",
        "-0x3.243f6a8885a30E-25#53",
    );
}

#[test]
fn test_shr_unsigned() {
    test_shr_unsigned_helper::<u8, _>(|_, _, _| {});
    test_shr_unsigned_helper::<u16, _>(|_, _, _| {});
    test_shr_unsigned_helper::<u32, _>(|x, v, shifted| {
        let mut n = rug::Float::exact_from(&x);
        n >>= v;
        assert_eq!(
            ComparableFloatRef(&Float::from(&n)),
            ComparableFloatRef(&shifted)
        );

        let n = rug::Float::exact_from(&x) >> v;
        assert_eq!(
            ComparableFloatRef(&Float::from(&n)),
            ComparableFloatRef(&shifted)
        );
    });
    test_shr_unsigned_helper::<u64, _>(|_, _, _| {});
    test_shr_unsigned_helper::<u128, _>(|_, _, _| {});
    test_shr_unsigned_helper::<usize, _>(|_, _, _| {});
}

fn test_shr_signed_helper<T: PrimitiveSigned, F: Fn(Float, T, Float)>(f: F)
where
    Float: ShrAssign<T> + Shr<T, Output = Float>,
    for<'a> &'a Float: Shr<T, Output = Float>,
{
    let test = |s, s_hex, v: i8, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let v = T::from(v);

        let mut n = x.clone();
        n >>= v;
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert!(n.is_valid());

        let n = x.clone() >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &x >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        f(x, v, n);
    };
    test("NaN", "NaN", 0, "NaN", "NaN");
    test("NaN", "NaN", 10, "NaN", "NaN");
    test("NaN", "NaN", -10, "NaN", "NaN");
    test("Infinity", "Infinity", 0, "Infinity", "Infinity");
    test("Infinity", "Infinity", 10, "Infinity", "Infinity");
    test("Infinity", "Infinity", -10, "Infinity", "Infinity");
    test("-Infinity", "-Infinity", 0, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 10, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", -10, "-Infinity", "-Infinity");
    test("0.0", "0x0.0", 10, "0.0", "0x0.0");
    test("0.0", "0x0.0", -10, "0.0", "0x0.0");
    test("-0.0", "-0x0.0", 10, "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", -10, "-0.0", "-0x0.0");

    test("123.0", "0x7b.0#7", 0, "123.0", "0x7b.0#7");
    test("123.0", "0x7b.0#7", 1, "61.5", "0x3d.8#7");
    test("123.0", "0x7b.0#7", 10, "0.12", "0x0.1ec#7");
    test("123.0", "0x7b.0#7", 100, "9.7e-29", "0x7.bE-24#7");
    test("123.0", "0x7b.0#7", -1, "246.0", "0xf6.0#7");
    test("123.0", "0x7b.0#7", -10, "1.26e5", "0x1.ecE+4#7");
    test("123.0", "0x7b.0#7", -100, "1.56e32", "0x7.bE+26#7");

    test("-123.0", "-0x7b.0#7", 0, "-123.0", "-0x7b.0#7");
    test("-123.0", "-0x7b.0#7", 1, "-61.5", "-0x3d.8#7");
    test("-123.0", "-0x7b.0#7", 10, "-0.12", "-0x0.1ec#7");
    test("-123.0", "-0x7b.0#7", 100, "-9.7e-29", "-0x7.bE-24#7");
    test("-123.0", "-0x7b.0#7", -1, "-246.0", "-0xf6.0#7");
    test("-123.0", "-0x7b.0#7", -10, "-1.26e5", "-0x1.ecE+4#7");
    test("-123.0", "-0x7b.0#7", -100, "-1.56e32", "-0x7.bE+26#7");

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        0,
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "0.0030679615757712823",
        "0x0.00c90fdaa22168c0#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        "2.4782796245465248e-30",
        "0x3.243f6a8885a30E-25#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        -1,
        "6.283185307179586",
        "0x6.487ed5110b460#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        -10,
        "3216.9908772759482",
        "0xc90.fdaa22168c0#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        -100,
        "3.9824418129956972e30",
        "0x3.243f6a8885a30E+25#53",
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        0,
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "-1.5707963267948966",
        "-0x1.921fb54442d18#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-0.0030679615757712823",
        "-0x0.00c90fdaa22168c0#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        "-2.4782796245465248e-30",
        "-0x3.243f6a8885a30E-25#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -1,
        "-6.283185307179586",
        "-0x6.487ed5110b460#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -10,
        "-3216.9908772759482",
        "-0xc90.fdaa22168c0#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -100,
        "-3.9824418129956972e30",
        "-0x3.243f6a8885a30E+25#53",
    );
}

#[test]
fn test_shr_signed() {
    test_shr_signed_helper::<i8, _>(|_, _, _| {});
    test_shr_signed_helper::<i16, _>(|_, _, _| {});
    test_shr_signed_helper::<i32, _>(|x, v, shifted| {
        let mut n = rug::Float::exact_from(&x);
        n >>= v;
        assert_eq!(
            ComparableFloatRef(&Float::from(&n)),
            ComparableFloatRef(&shifted)
        );

        let n = rug::Float::exact_from(&x) >> v;
        assert_eq!(
            ComparableFloatRef(&Float::from(&n)),
            ComparableFloatRef(&shifted)
        );
    });
    test_shr_signed_helper::<i64, _>(|_, _, _| {});
    test_shr_signed_helper::<i128, _>(|_, _, _| {});
    test_shr_signed_helper::<isize, _>(|_, _, _| {});
}

fn shr_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    for<'a> &'a Integer: Shr<T, Output = Integer>,
    Float: Shr<T, Output = Float> + ShrAssign<T>,
    for<'a> &'a Float: Shr<T, Output = Float>,
    i64: TryFrom<T>,
{
    float_unsigned_pair_gen_var_2::<T>().test_properties(|(n, u)| {
        let mut mut_n = n.clone();
        mut_n >>= u;
        assert!(mut_n.is_valid());
        let shifted = mut_n;

        let shifted_alt = &n >> u;
        assert!(shifted_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        let shifted_alt = n.clone() >> u;
        assert!(shifted_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(n.get_prec(), shifted.get_prec());

        if !n.is_nan() {
            assert!((&n >> u).le_abs(&n));
        }
        assert_eq!(ComparableFloat(-&n >> u), ComparableFloat(-(&n >> u)));

        assert_eq!(
            ComparableFloat(&n >> u),
            ComparableFloat(n * Float::power_of_2(i64::exact_from(u).checked_neg().unwrap()))
        );
    });

    float_gen().test_properties(|n| {
        assert_eq!(ComparableFloat(&n >> T::ZERO), ComparableFloat(n));
    });

    unsigned_gen::<T>().test_properties(|u| {
        assert!((Float::NAN >> u).is_nan());
        assert_eq!(Float::INFINITY >> u, Float::INFINITY);
        assert_eq!(Float::NEGATIVE_INFINITY >> u, Float::NEGATIVE_INFINITY);
        assert_eq!(
            ComparableFloat(Float::ZERO >> u),
            ComparableFloat(Float::ZERO)
        );
        assert_eq!(
            ComparableFloat(Float::NEGATIVE_ZERO >> u),
            ComparableFloat(Float::NEGATIVE_ZERO)
        );
    });

    unsigned_gen_var_5::<T>().test_properties(|u| {
        assert!((Float::ONE >> u).is_power_of_2());
    });
}

fn shr_properties_helper_signed<T: PrimitiveSigned>()
where
    for<'a> &'a Integer: Shr<T, Output = Integer>,
    Float: Shl<T, Output = Float> + ShrAssign<T> + Shr<T, Output = Float>,
    for<'a> &'a Float: Shl<T, Output = Float>
        + Shr<T, Output = Float>
        + Shr<<T as UnsignedAbs>::Output, Output = Float>,
    i64: TryFrom<T>,
{
    float_signed_pair_gen_var_2::<T>().test_properties(|(n, i)| {
        let mut mut_n = n.clone();
        mut_n >>= i;
        assert!(mut_n.is_valid());
        let shifted = mut_n;

        let shifted_alt = &n >> i;
        assert!(shifted_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        let shifted_alt = n.clone() >> i;
        assert!(shifted_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(n.get_prec(), shifted.get_prec());

        if i >= T::ZERO {
            assert_eq!(
                ComparableFloat(&n >> i.unsigned_abs()),
                ComparableFloat(shifted)
            );
        }
        assert_eq!(ComparableFloat(-&n >> i), ComparableFloat(-(&n >> i)));
        if let Some(neg_i) = i.checked_neg() {
            assert_eq!(ComparableFloat(&n >> neg_i), ComparableFloat(&n << i));
        }

        assert_eq!(
            ComparableFloat(&n >> i),
            ComparableFloat(n * Float::power_of_2(i64::exact_from(i).checked_neg().unwrap()))
        );
    });

    float_gen().test_properties(|n| {
        assert_eq!(ComparableFloat(&n >> T::ZERO), ComparableFloat(n));
    });

    signed_gen::<T>().test_properties(|i| {
        assert!((Float::NAN >> i).is_nan());
        assert_eq!(Float::INFINITY >> i, Float::INFINITY);
        assert_eq!(Float::NEGATIVE_INFINITY >> i, Float::NEGATIVE_INFINITY);
        assert_eq!(
            ComparableFloat(Float::ZERO >> i),
            ComparableFloat(Float::ZERO)
        );
        assert_eq!(
            ComparableFloat(Float::NEGATIVE_ZERO >> i),
            ComparableFloat(Float::NEGATIVE_ZERO)
        );
    });

    signed_gen_var_5::<T>().test_properties(|i| {
        assert!((Float::ONE >> i).is_power_of_2());
    });
}

#[test]
fn shr_properties() {
    apply_fn_to_unsigneds!(shr_properties_helper_unsigned);
    apply_fn_to_signeds!(shr_properties_helper_signed);

    float_unsigned_pair_gen_var_2::<u32>().test_properties(|(n, u)| {
        let shifted = &n >> u;
        let mut rug_n = rug::Float::exact_from(&n);
        rug_n >>= u;
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_n)),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(
            ComparableFloat(Float::from(&(rug::Float::exact_from(&n) >> u))),
            ComparableFloat(shifted)
        );
    });

    float_signed_pair_gen_var_2::<i32>().test_properties(|(n, i)| {
        let shifted = &n >> i;
        let mut rug_n = rug::Float::exact_from(&n);
        rug_n >>= i;
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_n)),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(
            ComparableFloat(Float::from(&(rug::Float::exact_from(&n) >> i))),
            ComparableFloat(shifted)
        );
    });
}
