use conversion::traits::ContinuedFraction;
use itertools::Itertools;
use malachite_base::num::arithmetic::traits::{Reciprocal, RoundToMultiple};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::exhaustive::exhaustive_positive_naturals;
use malachite_nz::natural::Natural;
use std::cmp::{min, Ordering};
use Rational;

// Slow! Only run for rationals with small denominators
pub fn simplest_rational_in_open_interval_naive(x: &Rational, y: &Rational) -> Rational {
    assert!(x < y);
    if *x < 0u32 && *y > 0u32 {
        return Rational::ZERO;
    }
    let neg_x;
    let neg_y;
    let (neg, x, y) = if *x < 0u32 {
        neg_x = -x;
        neg_y = -y;
        (true, &neg_y, &neg_x)
    } else {
        (false, x, y)
    };
    for d in exhaustive_positive_naturals() {
        let dr = Rational::from(d).reciprocal();
        let mut q = x.round_to_multiple(&dr, RoundingMode::Ceiling);
        if q == *x {
            q += dr;
        }
        if q < *y {
            return if neg { -q } else { q };
        }
    }
    unreachable!()
}

fn simplest_rational_helper(
    floor_x: &Integer,
    floor_y: &Integer,
    cf_x: &[Natural],
    cf_y: &[Natural],
) -> Rational {
    if floor_x != floor_y {
        return Rational::from(min(floor_x, floor_y) + Integer::ONE);
    }
    let floor = floor_x;
    for (i, (x, y)) in cf_x.iter().zip(cf_y.iter()).enumerate() {
        if x != y {
            let mut cf = cf_x[..i].to_vec();
            cf.push(min(x, y) + Natural::ONE);
            return Rational::from_continued_fraction_ref(floor, cf.iter());
        }
    }
    let x_len = cf_x.len();
    let y_len = cf_y.len();
    Rational::from_continued_fraction(
        floor.clone(),
        match x_len.cmp(&y_len) {
            Ordering::Equal => panic!(),
            Ordering::Greater => {
                let mut cf = cf_y.to_vec();
                cf.push(cf_x[y_len].clone() + Natural::ONE);
                cf.into_iter()
            }
            Ordering::Less => {
                let mut cf = cf_x.to_vec();
                cf.push(cf_y[x_len].clone() + Natural::ONE);
                cf.into_iter()
            }
        },
    )
}

fn cf_variants(x: &Rational) -> (Integer, Integer, Vec<Natural>, Vec<Natural>) {
    let (floor_1, cf_1) = x.continued_fraction();
    let cf_1 = cf_1.collect_vec();
    let mut cf_2 = cf_1.clone();
    let mut floor_2 = floor_1.clone();
    if let Some(last) = cf_2.last_mut() {
        *last -= Natural::ONE;
    } else {
        floor_2 -= Integer::ONE;
    }
    cf_2.push(Natural::ONE);
    (floor_1, floor_2, cf_1, cf_2)
}

pub fn simplest_rational_in_open_interval_explicit(x: &Rational, y: &Rational) -> Rational {
    assert!(x < y);
    if *x < 0u32 && *y > 0u32 {
        return Rational::ZERO;
    }
    let neg_x;
    let neg_y;
    let (neg, x, y) = if *x < 0u32 {
        neg_x = -x;
        neg_y = -y;
        (true, &neg_y, &neg_x)
    } else {
        (false, x, y)
    };
    let (floor_x_1, floor_x_2, cf_x_1, cf_x_2) = cf_variants(x);
    let (floor_y_1, floor_y_2, cf_y_1, cf_y_2) = cf_variants(y);
    let mut best: Option<Rational> = None;
    for (floor_x, cf_x) in [(&floor_x_1, &cf_x_1), (&floor_x_2, &cf_x_2)] {
        for (floor_y, cf_y) in [(&floor_y_1, &cf_y_1), (&floor_y_2, &cf_y_2)] {
            let candidate = simplest_rational_helper(floor_x, floor_y, cf_x, cf_y);
            if candidate > *x
                && candidate < *y
                && (best.is_none()
                    || candidate.denominator_ref() < best.as_ref().unwrap().denominator_ref())
            {
                best = Some(candidate);
            }
        }
    }
    let best = best.unwrap();
    if neg {
        -best
    } else {
        best
    }
}

// Slow! Only run for rationals with small denominators
pub fn simplest_rational_in_closed_interval_naive(x: &Rational, y: &Rational) -> Rational {
    assert!(x <= y);
    if *x <= 0u32 && *y >= 0u32 {
        return Rational::ZERO;
    }
    let neg_x;
    let neg_y;
    let (neg, x, y) = if *x < 0u32 {
        neg_x = -x;
        neg_y = -y;
        (true, &neg_y, &neg_x)
    } else {
        (false, x, y)
    };
    for d in exhaustive_positive_naturals() {
        let dr = Rational::from(d).reciprocal();
        let q = x.round_to_multiple(&dr, RoundingMode::Ceiling);
        if q <= *y {
            return if neg { -q } else { q };
        }
    }
    unreachable!()
}
