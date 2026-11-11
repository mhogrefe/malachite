// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_float::test_util::big_exponent_float::big_exponent_float_negate;
use malachite_float::test_util::generators::{
    signed_unsigned_rounding_mode_triple_gen_var_3,
    unsigned_unsigned_rounding_mode_triple_gen_var_5,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};

// A non-generic conversion, which a generic helper's bounds cannot shadow.
fn bef(f: &Float) -> BigExponentFloat {
    BigExponentFloat::from(f)
}

#[test]
fn test_const_from() {
    const X: BigExponentFloat = BigExponentFloat::const_from_unsigned(0);
    assert_eq!(format!("{X:#x}"), "0x0.0");
    const Y: BigExponentFloat = BigExponentFloat::const_from_unsigned_times_power_of_2(3, -3);
    assert_eq!(format!("{Y:#x}"), "0x0.6");
    const Z: BigExponentFloat = BigExponentFloat::const_from_signed(-123);
    assert_eq!(format!("{Z:#x}"), "-0x7b.0");
    const W: BigExponentFloat = BigExponentFloat::const_from_signed_times_power_of_2(-1, 100);
    assert_eq!(format!("{W:#x}"), "-0x1.0E+25");
    for x in [X, Y, Z, W] {
        assert!(x.is_valid());
    }
}

fn from_unsigned_properties_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T>,
    Float: From<T>,
    BigExponentFloat: From<T>,
{
    unsigned_gen::<T>().test_properties(|n| {
        let x = BigExponentFloat::from(n);
        assert!(x.is_valid());
        assert_eq!(format!("{x:#x}"), format!("{:#x}", bef(&Float::from(n))));
    });

    unsigned_unsigned_rounding_mode_triple_gen_var_5::<T>().test_properties(|(n, prec, rm)| {
        let (x, o) = BigExponentFloat::from_unsigned_prec_round(n, prec, rm);
        assert!(x.is_valid());
        let (f, o_f) = Float::from_unsigned_prec_round(n, prec, rm);
        assert_eq!(format!("{:#x}", bef(&f)), format!("{x:#x}"));
        assert_eq!(o_f, o);
        let (x, o) = BigExponentFloat::from_unsigned_prec(n, prec);
        let (f, o_f) = Float::from_unsigned_prec(n, prec);
        assert_eq!(format!("{:#x}", bef(&f)), format!("{x:#x}"));
        assert_eq!(o_f, o);
    });
}

fn from_signed_properties_helper<T: PrimitiveSigned>()
where
    Integer: From<T>,
    Float: From<T>,
    BigExponentFloat: From<T>,
{
    signed_gen::<T>().test_properties(|n| {
        let x = BigExponentFloat::from(n);
        assert!(x.is_valid());
        assert_eq!(format!("{x:#x}"), format!("{:#x}", bef(&Float::from(n))));
    });

    signed_unsigned_rounding_mode_triple_gen_var_3::<T>().test_properties(|(n, prec, rm)| {
        let (x, o) = BigExponentFloat::from_signed_prec_round(n, prec, rm);
        assert!(x.is_valid());
        let (f, o_f) = Float::from_signed_prec_round(n, prec, rm);
        assert_eq!(format!("{:#x}", bef(&f)), format!("{x:#x}"));
        assert_eq!(o_f, o);
        let (x, o) = BigExponentFloat::from_signed_prec(n, prec);
        let (f, o_f) = Float::from_signed_prec(n, prec);
        assert_eq!(format!("{:#x}", bef(&f)), format!("{x:#x}"));
        assert_eq!(o_f, o);
    });
}

#[test]
fn from_primitive_int_properties() {
    apply_fn_to_unsigneds!(from_unsigned_properties_helper);
    apply_fn_to_signeds!(from_signed_properties_helper);

    unsigned_gen::<Limb>().test_properties(|n| {
        assert_eq!(
            format!("{:#x}", BigExponentFloat::const_from_unsigned(n)),
            format!("{:#x}", BigExponentFloat::from(n))
        );
        for pow in [-100, -1, 0, 1, 100] {
            assert_eq!(
                format!(
                    "{:#x}",
                    BigExponentFloat::const_from_unsigned_times_power_of_2(n, pow)
                ),
                format!(
                    "{:#x}",
                    BigExponentFloat::from_integer_mantissa_and_exponent(
                        Natural::from(n),
                        Integer::from(pow)
                    )
                    .unwrap()
                )
            );
        }
    });

    signed_gen::<SignedLimb>().test_properties(|n| {
        assert_eq!(
            format!("{:#x}", BigExponentFloat::const_from_signed(n)),
            format!("{:#x}", BigExponentFloat::from(n))
        );
        for pow in [-100, 0, 100] {
            let x = BigExponentFloat::const_from_signed_times_power_of_2(n, pow);
            assert!(x.is_valid());
            assert_eq!(x.is_sign_negative(), n < 0);
            assert_eq!(
                format!(
                    "{:#x}",
                    if n < 0 {
                        big_exponent_float_negate(x)
                    } else {
                        x
                    }
                ),
                format!(
                    "{:#x}",
                    BigExponentFloat::const_from_unsigned_times_power_of_2(n.unsigned_abs(), pow)
                )
            );
        }
    });
    let _ = SignedLimb::WIDTH;
}
