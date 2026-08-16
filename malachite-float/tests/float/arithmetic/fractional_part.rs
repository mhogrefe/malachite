// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use gmp_mpfr_sys::mpfr::{self, rnd_t};
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::{NaN, NegativeInfinity};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::{ComparableFloat, Float};
use malachite_nz::natural::Natural;

const fn mpfr_rnd(rm: RoundingMode) -> rnd_t {
    match rm {
        Floor => rnd_t::RNDD,
        Ceiling => rnd_t::RNDU,
        Down => rnd_t::RNDZ,
        Up => rnd_t::RNDA,
        Nearest => rnd_t::RNDN,
        Exact => panic!(),
    }
}

const fn ordering_of(t: i32) -> Ordering {
    if t < 0 {
        Less
    } else if t == 0 {
        Equal
    } else {
        Greater
    }
}

fn sweep_values() -> Vec<Float> {
    let mut xs = Vec::new();
    for prec_x in [1u64, 2, 5, 10, 64, 65, 100] {
        let mut sigs = vec![Natural::power_of_2(prec_x - 1), Natural::low_mask(prec_x)];
        for t in [1, 2, prec_x / 2, prec_x.saturating_sub(2)] {
            if t >= prec_x {
                continue;
            }
            sigs.push(Natural::power_of_2(prec_x - 1) + Natural::power_of_2(t));
            if t > 1 {
                sigs.push(
                    Natural::power_of_2(prec_x - 1)
                        + Natural::power_of_2(t)
                        + Natural::power_of_2(0u64),
                );
            }
        }
        sigs.sort_unstable();
        sigs.dedup();
        for sig in sigs {
            for exp in [
                -2i64,
                0,
                1,
                2,
                i64::exact_from(prec_x / 2 + 1),
                i64::exact_from(prec_x),
                i64::exact_from(prec_x) + 10,
            ] {
                let x = Float::from_natural_prec(sig.clone(), prec_x).0
                    << (exp - i64::exact_from(prec_x));
                if x != 0u32 {
                    xs.push(x.clone());
                    xs.push(-x);
                }
            }
        }
    }
    xs
}

#[test]
fn test_fractional_part_vs_mpfr() {
    for x in sweep_values() {
        let b = rug::Float::exact_from(&x);
        for prec in [1u64, 2, 3, 10, 64, 100] {
            for rm in [Floor, Ceiling, Down, Up, Nearest] {
                let (ours, o) = x.fractional_part_prec_round_ref(prec, rm);
                let mut r = rug::Float::new(u32::exact_from(prec));
                let t = unsafe { mpfr::frac(r.as_raw_mut(), b.as_raw(), mpfr_rnd(rm)) };
                assert_eq!(
                    ComparableFloat(Float::from(&r)),
                    ComparableFloat(ours),
                    "{x} {prec} {rm}"
                );
                assert_eq!(ordering_of(t), o, "ternary {x} {prec} {rm}");
            }
        }
    }
}

#[test]
fn test_integer_and_fractional_parts_vs_mpfr() {
    for x in sweep_values() {
        let b = rug::Float::exact_from(&x);
        for (iprec, fprec) in [(1u64, 1u64), (2, 3), (10, 10), (64, 10), (10, 64), (100, 100)] {
            for rm in [Floor, Ceiling, Down, Up, Nearest] {
                let ((i_ours, i_o), (f_ours, f_o)) =
                    x.integer_and_fractional_parts_prec_round_ref(iprec, fprec, rm);
                let mut ir = rug::Float::new(u32::exact_from(iprec));
                let mut fr = rug::Float::new(u32::exact_from(fprec));
                let t = unsafe {
                    mpfr::modf(ir.as_raw_mut(), fr.as_raw_mut(), b.as_raw(), mpfr_rnd(rm))
                };
                assert_eq!(
                    ComparableFloat(Float::from(&ir)),
                    ComparableFloat(i_ours),
                    "int {x} {iprec} {fprec} {rm}"
                );
                assert_eq!(
                    ComparableFloat(Float::from(&fr)),
                    ComparableFloat(f_ours),
                    "frac {x} {iprec} {fprec} {rm}"
                );
                // decode MPFR's packed pair of ternaries: INEXPOS(i) | INEXPOS(f) << 2, where
                // INEXPOS is 0 for exact, 1 for positive, 2 for negative
                let decode = |v: i32| match v {
                    0 => Equal,
                    1 => Greater,
                    2 => Less,
                    _ => unreachable!(),
                };
                assert_eq!(decode(t & 3), i_o, "int ternary {x} {iprec} {fprec} {rm}");
                assert_eq!(
                    decode(t >> 2 & 3),
                    f_o,
                    "frac ternary {x} {iprec} {fprec} {rm}"
                );
            }
        }
    }
}

#[test]
fn fractional_part_special() {
    let (r, o) = Float::NAN.fractional_part_ref();
    assert!(r.is_nan());
    assert_eq!(o, Equal);
    // the fractional part of an infinity is a zero with the same sign
    let (r, o) = Float::NEGATIVE_INFINITY.fractional_part_ref();
    assert_eq!(ComparableFloat(r), ComparableFloat(-Float::from(0u32)));
    assert_eq!(o, Equal);
    // the integral part of an infinity is itself, and its fractional part is a signed zero
    let ((i, io), (f, fo)) = Float::NEGATIVE_INFINITY.integer_and_fractional_parts_ref();
    assert_eq!(i, Float::NEGATIVE_INFINITY);
    assert_eq!(ComparableFloat(f), ComparableFloat(-Float::from(0u32)));
    assert_eq!((io, fo), (Equal, Equal));
    // variants agree
    let x = Float::from(2.5f64);
    let a = x.fractional_part_prec_round_ref(3, Nearest);
    let b = x.clone().fractional_part_prec_round(3, Nearest);
    assert_eq!(ComparableFloat(a.0.clone()), ComparableFloat(b.0));
    assert_eq!(a.1, b.1);
    let ((i1, io1), (f1, fo1)) = x.integer_and_fractional_parts_ref();
    let ((i2, io2), (f2, fo2)) = x.clone().integer_and_fractional_parts();
    assert_eq!(ComparableFloat(i1), ComparableFloat(i2));
    assert_eq!(ComparableFloat(f1), ComparableFloat(f2));
    assert_eq!((io1, fo1), (io2, fo2));
}

#[test]
#[should_panic]
fn fractional_part_fail() {
    Float::from(3u32).fractional_part_prec_round_ref(0, Nearest);
}

#[test]
#[should_panic]
fn integer_and_fractional_parts_fail() {
    Float::from(3u32).integer_and_fractional_parts_prec_round_ref(5, 0, Nearest);
}
