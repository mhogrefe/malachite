// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{
    IntegerMantissaAndExponent, RawMantissaAndExponent, SciMantissaAndExponent,
};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_negate, big_exponent_float_power_of_2, big_exponent_float_to_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

#[test]
fn test_mantissa_and_exponent() {
    let x = big_exponent_float_power_of_2(Integer::from(1u64 << 40));
    let (m, e) = (&x).integer_mantissa_and_exponent();
    assert_eq!(m, 1u32);
    assert_eq!(e, Integer::from(1u64 << 40));
    let (m, e) = (&x).sci_mantissa_and_exponent();
    assert_eq!(format!("{m:#x}"), "0x1.0");
    assert_eq!(e, Integer::from(1u64 << 40));
    assert_eq!((&x).raw_exponent(), Integer::from((1u64 << 40) + 1));
    assert!(
        BigExponentFloat::from_sci_mantissa_and_exponent(
            BigExponentFloat::from(3u32),
            Integer::from(1)
        )
        .is_none()
    );
}

#[test]
#[should_panic]
fn raw_mantissa_and_exponent_fail() {
    BigExponentFloat::from(0u32).raw_mantissa_and_exponent();
}

#[test]
#[should_panic]
fn from_raw_mantissa_and_exponent_fail() {
    BigExponentFloat::from_raw_mantissa_and_exponent(Natural::from(3u32), Integer::from(1));
}

#[test]
fn mantissa_and_exponent_properties() {
    big_exponent_float_gen().test_properties(|x| {
        if !x.is_normal() {
            return;
        }
        let (m, e) = (&x).raw_mantissa_and_exponent();
        assert_eq!(
            (m.clone(), e.clone()),
            x.clone().raw_mantissa_and_exponent()
        );
        let y = BigExponentFloat::from_raw_mantissa_and_exponent(m, e);
        assert!(y.is_valid());
        assert_eq!(
            format!("{y:#x}"),
            format!(
                "{:#x}",
                x.to_finite()
                    .map(|x| if x.is_sign_negative() {
                        big_exponent_float_negate(x)
                    } else {
                        x
                    })
                    .unwrap()
            )
        );

        let (m, e) = (&x).integer_mantissa_and_exponent();
        assert_eq!(
            (m.clone(), e.clone()),
            x.clone().integer_mantissa_and_exponent()
        );
        assert!(m.odd());
        let y = BigExponentFloat::from_integer_mantissa_and_exponent(m, e).unwrap();
        assert!(y.is_valid());
        assert_eq!(y.get_prec(), x.get_prec());

        let (m, e) = (&x).sci_mantissa_and_exponent();
        assert_eq!(
            format!("{m:#x}"),
            format!("{:#x}", x.clone().sci_mantissa_and_exponent().0)
        );
        assert_eq!(e, x.get_exponent().unwrap() - Integer::ONE);
        assert_eq!(m.get_exponent().unwrap(), &Integer::ONE);
        let y = BigExponentFloat::from_sci_mantissa_and_exponent(m, e).unwrap();
        assert!(y.is_valid());
        assert_eq!(y.get_exponent(), x.get_exponent());

        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (m, e, o) = x.sci_mantissa_and_exponent_round::<f64>(rm).unwrap();
            assert!((1.0..2.0).contains(&m));
            assert!(
                e == x.get_exponent().unwrap() - Integer::ONE || e == *x.get_exponent().unwrap()
            );
            let _ = o;
        }
    });

    big_exponent_float_gen_var_1().test_properties(|x| {
        if !x.is_normal() {
            return;
        }
        let f = big_exponent_float_to_float(&x).unwrap();
        let (m, e) = (&x).raw_mantissa_and_exponent();
        let (m_f, e_f) = (&f).raw_mantissa_and_exponent();
        assert_eq!(m, m_f);
        assert_eq!(e, e_f);
        let (m, e) = (&x).integer_mantissa_and_exponent();
        let (m_f, e_f) = (&f).integer_mantissa_and_exponent();
        assert_eq!(m, m_f);
        assert_eq!(e, e_f);
        let e_f = <&Float as SciMantissaAndExponent<Float, i32, Float>>::sci_exponent(&f);
        assert_eq!((&x).sci_mantissa_and_exponent().1, e_f);
        for rm in [Floor, Ceiling, Down, Up, Nearest, Exact] {
            assert_eq!(
                x.sci_mantissa_and_exponent_round::<f32>(rm)
                    .map(|(m, e, o)| (NiceFloat(m), e, o)),
                f.sci_mantissa_and_exponent_round::<f32>(rm)
                    .map(|(m, e, o)| (NiceFloat(m), Integer::from(e), o))
            );
        }
    });
}
