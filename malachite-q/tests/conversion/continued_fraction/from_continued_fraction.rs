use itertools::Itertools;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::vecs::vec_from_str;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::generators::{integer_gen, natural_vec_integer_pair_gen_var_1};
use malachite_q::conversion::traits::ContinuedFraction;
use malachite_q::Rational;
use malachite_q_test_util::conversion::continued_fraction::from_continued_fraction::*;
use std::str::FromStr;

#[test]
fn test_from_continued_fraction() {
    let test = |floor: &str, xs: &str, out: &str| {
        let floor = Integer::from_str(floor).unwrap();
        let xs: Vec<Natural> = vec_from_str(xs).unwrap();
        let x = Rational::from_continued_fraction(floor.clone(), xs.clone());
        assert!(x.is_valid());
        assert_eq!(Rational::from_continued_fraction_ref(&floor, &xs), x);
        assert_eq!(from_continued_fraction_alt(floor, xs), x);
        assert_eq!(x.to_string(), out);
    };
    test("0", "[]", "0");
    test("3", "[7]", "22/7");
    test("3", "[6, 1]", "22/7");
    test("-4", "[1, 6]", "-22/7");
    test(
        "3",
        "[7, 15, 1, 292, 1, 1, 1, 2, 1, 3, 1, 14, 2, 1, 1, 2, 2, 2, 2]",
        "14885392687/4738167652",
    );
}

#[test]
#[should_panic]
fn from_continued_fraction_fail_1() {
    Rational::from_continued_fraction(Integer::ONE, vec![Natural::ZERO]);
}

#[test]
#[should_panic]
fn from_continued_fraction_fail_2() {
    Rational::from_continued_fraction(
        Integer::ONE,
        vec![Natural::from(3u32), Natural::ZERO, Natural::ONE],
    );
}

#[test]
#[should_panic]
fn from_continued_fraction_ref_fail_1() {
    Rational::from_continued_fraction_ref(&Integer::ONE, &[Natural::ZERO]);
}

#[test]
#[should_panic]
fn from_continued_fraction_ref_fail_2() {
    Rational::from_continued_fraction_ref(
        &Integer::ONE,
        &[Natural::from(3u32), Natural::ZERO, Natural::ONE],
    );
}

#[test]
fn from_continued_fraction_properties() {
    natural_vec_integer_pair_gen_var_1().test_properties(|(xs, floor)| {
        let x = Rational::from_continued_fraction(floor.clone(), xs.clone());
        assert!(x.is_valid());
        assert_eq!(Rational::from_continued_fraction_ref(&floor, &xs), x);
        assert_eq!(from_continued_fraction_alt(floor.clone(), xs.clone()), x);
        if xs.last() != Some(&Natural::ONE) {
            let (floor_alt, cf) = (&x).continued_fraction();
            let xs_alt = cf.collect_vec();
            assert_eq!(floor_alt, floor);
            assert_eq!(xs_alt, xs);
        }
        if !xs.is_empty() {
            let mut alt_xs = xs;
            let last = alt_xs.last_mut().unwrap();
            if *last > 1u32 {
                *last -= Natural::ONE;
                alt_xs.push(Natural::ONE);
                assert_eq!(Rational::from_continued_fraction(floor, alt_xs), x);
            }
        }
    });

    integer_gen().test_properties(|x| {
        assert_eq!(Rational::from_continued_fraction_ref(&x, &[]), x);
    });
}
