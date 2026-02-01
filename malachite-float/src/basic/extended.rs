// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, NaN};
use crate::conversion::rational_from_float::RationalFromFloatError;
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use core::mem::swap;
use core::ops::{Add, Mul, Shr, ShrAssign};
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, Parity, Sqrt, SqrtAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingInto};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::natural::arithmetic::float_sub::exponent_shift_compare;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

pub_crate_test_struct! {
#[derive(Clone)]
ExtendedFloat {
    pub(crate) x: Float,
    exp: i64,
}}

impl ExtendedFloat {
    fn is_valid(&self) -> bool {
        if self.x == 0u32 && self.exp != 0 {
            return false;
        }
        let exp = self.x.get_exponent();
        exp.is_none() || exp == Some(0)
    }

    fn from_rational_prec_round(value: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        if value == 0 {
            return (
                Self {
                    x: Float::ZERO,
                    exp: 0,
                },
                Equal,
            );
        }
        let exp = value.floor_log_base_2_abs() + 1;
        let (x, o) = Float::from_rational_prec_round(value >> exp, prec, rm);
        let new_exp = x.get_exponent().unwrap();
        (
            Self {
                x: x >> new_exp,
                exp: i64::from(new_exp) + exp,
            },
            o,
        )
    }

    pub(crate) fn from_rational_prec_round_ref(
        value: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if *value == 0 {
            return (
                Self {
                    x: Float::ZERO,
                    exp: 0,
                },
                Equal,
            );
        }
        let exp = value.floor_log_base_2_abs() + 1;
        if exp >= i64::from(Float::MIN_EXPONENT) && exp <= i64::from(Float::MAX_EXPONENT) {
            let (x, o) = Float::from_rational_prec_round_ref(value, prec, rm);
            let exp = x.get_exponent().unwrap();
            return (
                Self {
                    x: x >> exp,
                    exp: i64::from(exp),
                },
                o,
            );
        }
        let (x, o) = Float::from_rational_prec_round(value >> exp, prec, rm);
        let new_exp = x.get_exponent().unwrap();
        (
            Self {
                x: x >> new_exp,
                exp: i64::from(new_exp) + exp,
            },
            o,
        )
    }

    fn add_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        assert!(self.is_valid());
        assert!(other.is_valid());
        assert!(self.x.is_normal());
        assert!(other.x.is_normal());
        Self::from_rational_prec_round(
            Rational::exact_from(self) + Rational::exact_from(other),
            prec,
            Nearest,
        )
    }

    fn sub_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        assert!(self.is_valid());
        assert!(other.is_valid());
        assert!(self.x.is_normal());
        assert!(other.x.is_normal());
        Self::from_rational_prec_round(
            Rational::exact_from(self) - Rational::exact_from(other),
            prec,
            Nearest,
        )
    }

    fn sub_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        assert!(self.is_valid());
        assert!(other.is_valid());
        assert!(self.x.is_normal());
        assert!(other.x.is_normal());
        Self::from_rational_prec_round(
            Rational::exact_from(self) - Rational::exact_from(other),
            prec,
            Nearest,
        )
    }

    fn mul_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        assert!(self.is_valid());
        assert!(other.is_valid());
        assert!(self.x.is_normal());
        assert!(other.x.is_normal());
        let (mut product, o) = self.x.mul_prec_ref_ref(&other.x, prec);
        let mut product_exp = self.exp + other.exp;
        let extra_exp = product.get_exponent().unwrap();
        product >>= extra_exp;
        product_exp = product_exp.checked_add(i64::from(extra_exp)).unwrap();
        (
            Self {
                x: product,
                exp: product_exp,
            },
            o,
        )
    }

    fn div_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        assert!(self.is_valid());
        assert!(other.is_valid());
        assert!(self.x.is_normal());
        assert!(other.x.is_normal());
        let (mut quotient, o) = self.x.div_prec_ref_ref(&other.x, prec);
        let mut quotient_exp = self.exp - other.exp;
        let extra_exp = quotient.get_exponent().unwrap();
        quotient >>= extra_exp;
        quotient_exp = quotient_exp.checked_add(i64::from(extra_exp)).unwrap();
        (
            Self {
                x: quotient,
                exp: quotient_exp,
            },
            o,
        )
    }

    fn div_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let mut x = Self {
            x: Float::ZERO,
            exp: 0,
        };
        swap(self, &mut x);
        let (q, o) = x.div_prec_val_ref(other, prec);
        *self = q;
        o
    }

    fn square_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let mut x = Self {
            x: Float::ZERO,
            exp: 0,
        };
        swap(self, &mut x);
        let (mut square, o) = x.x.square_round(rm);
        let mut square_exp = x.exp << 1;
        let extra_exp = square.get_exponent().unwrap();
        square >>= extra_exp;
        square_exp = square_exp.checked_add(i64::from(extra_exp)).unwrap();
        *self = Self {
            x: square,
            exp: square_exp,
        };
        o
    }

    fn from_extended_float_prec_round(x: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        if let Ok(x) = Rational::try_from(&x) {
            Self::from_rational_prec_round(x, prec, rm)
        } else {
            (x, Equal)
        }
    }

    pub_crate_test! {from_extended_float_prec_round_ref(
        x: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if let Ok(x) = Rational::try_from(x) {
            Self::from_rational_prec_round(x, prec, rm)
        } else {
            (x.clone(), Equal)
        }
    }}

    fn shr_prec_round<T: PrimitiveInt>(
        self,
        bits: T,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        // assumes no overflow or underflow
        let (out_x, o) = Self::from_extended_float_prec_round(self, prec, rm);
        let out_exp =
            i64::exact_from(i128::from(out_x.exp) - SaturatingInto::<i128>::saturating_into(bits));
        (
            Self {
                x: out_x.x,
                exp: out_exp,
            },
            o,
        )
    }

    fn shr_round_assign<T: PrimitiveInt>(&mut self, bits: T, _rm: RoundingMode) -> Ordering {
        // assumes no overflow or underflow
        if self.x.is_normal() {
            let out_exp = i64::exact_from(
                i128::from(self.exp) - SaturatingInto::<i128>::saturating_into(bits),
            );
            self.exp = out_exp;
        }
        Equal
    }

    pub(crate) fn into_float_helper(
        mut self,
        prec: u64,
        rm: RoundingMode,
        previous_o: Ordering,
    ) -> (Float, Ordering) {
        let o = self
            .x
            .shl_prec_round_assign_helper(self.exp, prec, rm, previous_o);
        (self.x, o)
    }

    pub(crate) fn increment(&mut self) {
        self.x.increment();
        if let Some(exp) = self.x.get_exponent()
            && exp == 1
        {
            self.x >>= 1u32;
            self.exp = 0;
        }
    }
}

impl From<Float> for ExtendedFloat {
    fn from(value: Float) -> Self {
        if let Some(exp) = value.get_exponent() {
            Self {
                x: value >> exp,
                exp: i64::from(exp),
            }
        } else {
            Self { x: value, exp: 0 }
        }
    }
}

impl TryFrom<ExtendedFloat> for Rational {
    type Error = RationalFromFloatError;

    fn try_from(value: ExtendedFloat) -> Result<Self, Self::Error> {
        Self::try_from(value.x).map(|x| x << value.exp)
    }
}

impl<'a> TryFrom<&'a ExtendedFloat> for Rational {
    type Error = RationalFromFloatError;

    fn try_from(value: &'a ExtendedFloat) -> Result<Self, Self::Error> {
        Self::try_from(&value.x).map(|x| x << value.exp)
    }
}

impl PartialEq for ExtendedFloat {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.exp == other.exp
    }
}

impl PartialOrd for ExtendedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        assert!(self.is_valid());
        assert!(other.is_valid());
        let self_sign = self.x > 0u32;
        let other_sign = other.x > 0u32;
        match self_sign.cmp(&other_sign) {
            Greater => Some(Greater),
            Less => Some(Less),
            Equal => match self.exp.cmp(&other.exp) {
                Greater => Some(if self_sign { Greater } else { Less }),
                Less => Some(if self_sign { Less } else { Greater }),
                Equal => self.x.partial_cmp(&other.x),
            },
        }
    }
}

impl Add<&ExtendedFloat> for &ExtendedFloat {
    type Output = ExtendedFloat;

    fn add(self, other: &ExtendedFloat) -> Self::Output {
        let prec = max(self.x.significant_bits(), other.x.significant_bits());
        self.add_prec_ref_ref(other, prec).0
    }
}

impl Mul<&ExtendedFloat> for &ExtendedFloat {
    type Output = ExtendedFloat;

    fn mul(self, other: &ExtendedFloat) -> Self::Output {
        let prec = max(self.x.significant_bits(), other.x.significant_bits());
        self.mul_prec_ref_ref(other, prec).0
    }
}

impl SqrtAssign for ExtendedFloat {
    fn sqrt_assign(&mut self) {
        if self.exp.odd() {
            self.x <<= 1;
            self.exp = self.exp.checked_sub(1).unwrap();
        }
        self.x.sqrt_assign();
        self.exp >>= 1;
        if let Some(new_exp) = self.x.get_exponent() {
            self.exp = self.exp.checked_add(i64::from(new_exp)).unwrap();
            self.x >>= new_exp;
        }
        assert!(self.is_valid());
    }
}

impl Sqrt for ExtendedFloat {
    type Output = Self;

    fn sqrt(mut self) -> Self::Output {
        self.sqrt_assign();
        self
    }
}

impl Shr<u32> for ExtendedFloat {
    type Output = Self;

    fn shr(mut self, bits: u32) -> Self::Output {
        self.shr_round_assign(bits, Nearest);
        self
    }
}

impl ShrAssign<u32> for ExtendedFloat {
    fn shr_assign(&mut self, bits: u32) {
        self.shr_round_assign(bits, Nearest);
    }
}

fn cmp2_helper_extended(b: &ExtendedFloat, c: &ExtendedFloat, cancel: &mut u64) -> Ordering {
    match (&b.x, &c.x) {
        (
            Float(Finite {
                precision: x_prec,
                significand: x,
                ..
            }),
            Float(Finite {
                precision: y_prec,
                significand: y,
                ..
            }),
        ) => {
            let (o, c) = exponent_shift_compare(
                x.as_limbs_asc(),
                b.exp,
                *x_prec,
                y.as_limbs_asc(),
                c.exp,
                *y_prec,
            );
            *cancel = c;
            o
        }
        _ => panic!(),
    }
}

pub(crate) fn agm_prec_round_normal_extended(
    mut a: ExtendedFloat,
    mut b: ExtendedFloat,
    prec: u64,
    rm: RoundingMode,
) -> (ExtendedFloat, Ordering) {
    if a.x < 0u32 || b.x < 0u32 {
        return (
            ExtendedFloat {
                x: float_nan!(),
                exp: 0,
            },
            Equal,
        );
    }
    let q = prec;
    let mut working_prec = q + q.ceiling_log_base_2() + 15;
    // b (op2) and a (op1) are the 2 operands but we want b >= a
    match a.partial_cmp(&b).unwrap() {
        Equal => return ExtendedFloat::from_extended_float_prec_round(a, prec, rm),
        Greater => swap(&mut a, &mut b),
        _ => {}
    }
    let mut increment = Limb::WIDTH;
    let mut v;
    let mut scaleit;
    loop {
        let mut err: u64 = 0;
        let mut u = a.mul_prec_ref_ref(&b, working_prec).0;
        v = a.add_prec_ref_ref(&b, working_prec).0;
        u.sqrt_assign();
        v >>= 1u32;
        scaleit = 0;
        let mut n: u64 = 1;
        let mut eq = 0;
        while cmp2_helper_extended(&u, &v, &mut eq) != Equal && eq <= working_prec - 2 {
            let mut vf;
            vf = (&u + &v) >> 1;
            // See proof in algorithms.tex
            if eq > working_prec >> 2 {
                // vf = V(k)
                let low_p = (working_prec + 1) >> 1;
                let mut w = v.sub_prec_ref_ref(&u, low_p).0; // e = V(k-1)-U(k-1)
                w.square_round_assign(Nearest); // e = e^2
                w.shr_round_assign(4, Nearest); // e*= (1/2)^2*1/4
                w.div_prec_assign_ref(&vf, low_p); // 1/4*e^2/V(k)
                let vf_exp = vf.exp;
                v = vf.sub_prec(w, working_prec).0;
                // 0 or 1
                err = u64::exact_from(vf_exp - v.exp);
                break;
            }
            let uf = &u * &v;
            u = uf.sqrt();
            swap(&mut v, &mut vf);
            n += 1;
        }
        // the error on v is bounded by (18n+51) ulps, or twice if there was an exponent loss in the
        // final subtraction
        //
        // 18n+51 should not overflow since n is about log(p)
        err += (18 * n + 51).ceiling_log_base_2();
        // we should have n+2 <= 2^(p/4) [see algorithms.tex]
        if (n + 2).ceiling_log_base_2() <= working_prec >> 2
            && float_can_round(v.x.significand_ref().unwrap(), working_prec - err, q, rm)
        {
            break;
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
    v.shr_prec_round(scaleit, prec, rm)
}

pub_crate_test! {agm_prec_round_normal_ref_ref_extended<'a>(
    mut a: &'a ExtendedFloat,
    mut b: &'a ExtendedFloat,
    prec: u64,
    rm: RoundingMode,
) -> (ExtendedFloat, Ordering) {
    if a.x < 0u32 || b.x < 0u32 {
        return (
            ExtendedFloat {
                x: float_nan!(),
                exp: 0,
            },
            Equal,
        );
    }
    let q = prec;
    let mut working_prec = q + q.ceiling_log_base_2() + 15;
    // b (op2) and a (op1) are the 2 operands but we want b >= a
    match a.partial_cmp(b).unwrap() {
        Equal => return ExtendedFloat::from_extended_float_prec_round_ref(a, prec, rm),
        Greater => swap(&mut a, &mut b),
        _ => {}
    }
    let mut increment = Limb::WIDTH;
    let mut v;
    let mut scaleit;
    loop {
        let mut err: u64 = 0;
        let mut u = a.mul_prec_ref_ref(b, working_prec).0;
        v = a.add_prec_ref_ref(b, working_prec).0;
        u.sqrt_assign();
        v >>= 1u32;
        scaleit = 0;
        let mut n: u64 = 1;
        let mut eq = 0;
        while cmp2_helper_extended(&u, &v, &mut eq) != Equal && eq <= working_prec - 2 {
            let mut vf;
            vf = (&u + &v) >> 1;
            // See proof in algorithms.tex
            if eq > working_prec >> 2 {
                // vf = V(k)
                let low_p = (working_prec + 1) >> 1;
                let mut w = v.sub_prec_ref_ref(&u, low_p).0; // e = V(k-1)-U(k-1)
                assert!(w.is_valid());
                assert!(w.x.is_normal());
                w.square_round_assign(Nearest); // e = e^2
                assert!(w.is_valid());
                assert!(w.x.is_normal());
                w.shr_round_assign(4, Nearest); // e*= (1/2)^2*1/4
                assert!(w.is_valid());
                assert!(w.x.is_normal());
                w.div_prec_assign_ref(&vf, low_p); // 1/4*e^2/V(k)
                let vf_exp = vf.exp;
                v = vf.sub_prec(w, working_prec).0;
                // 0 or 1
                err = u64::exact_from(vf_exp - v.exp);
                break;
            }
            let uf = &u * &v;
            u = uf.sqrt();
            swap(&mut v, &mut vf);
            n += 1;
        }
        // the error on v is bounded by (18n+51) ulps, or twice if there was an exponent loss in the
        // final subtraction
        //
        // 18n+51 should not overflow since n is about log(p)
        err += (18 * n + 51).ceiling_log_base_2();
        // we should have n+2 <= 2^(p/4) [see algorithms.tex]
        if (n + 2).ceiling_log_base_2() <= working_prec >> 2
            && float_can_round(v.x.significand_ref().unwrap(), working_prec - err, q, rm)
        {
            break;
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
    v.shr_prec_round(scaleit, prec, rm)
}}
