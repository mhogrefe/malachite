use malachite_nz::test_util::generators::integer_pair_gen;
use malachite_q::test_util::common::{
    bigrational_to_rational, rational_to_bigrational, rational_to_rug_rational,
    rug_rational_to_rational,
};
use malachite_q::test_util::generators::{rational_gen, rational_pair_gen};
use malachite_q::Rational;
use num::BigRational;
use rug;
use std::str::FromStr;

#[test]
#[allow(clippy::redundant_clone)]
fn test_clone() {
    let test = |u| {
        let x = Rational::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = BigRational::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);

        let x = rug::Rational::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
    };
    test("123");
    test("1000000000000");
    test("-123");
    test("-1000000000000");
    test("22/7");
    test("-22/7");
    test("100/101");
    test("-100/101");
}

#[test]
fn test_clone_from() {
    let test = |u, v| {
        let mut x = Rational::from_str(u).unwrap();
        x.clone_from(&Rational::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = BigRational::from_str(u).unwrap();
        x.clone_from(&BigRational::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);

        let mut x = rug::Rational::from_str(u).unwrap();
        x.clone_from(&rug::Rational::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
    };
    test("-123", "456");
    test("-123", "1000000000000");
    test("1000000000000", "-123");
    test("1000000000000", "2000000000000");
    test("123", "22/7");
    test("123", "-22/7");
    test("-123", "22/7");
    test("-123", "-22/7");
}

#[allow(clippy::redundant_clone)]
#[test]
fn clone_and_clone_from_properties() {
    rational_gen().test_properties(|x| {
        let mut_x = x.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, x);

        assert_eq!(
            bigrational_to_rational(&rational_to_bigrational(&x).clone()),
            x
        );
        assert_eq!(
            rug_rational_to_rational(&rational_to_rug_rational(&x).clone()),
            x
        );
    });

    rational_pair_gen().test_properties(|(x, y)| {
        let mut mut_x = x.clone();
        mut_x.clone_from(&y);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, y);

        let mut num_x = rational_to_bigrational(&x);
        num_x.clone_from(&rational_to_bigrational(&y));
        assert_eq!(bigrational_to_rational(&num_x), y);

        let mut rug_x = rational_to_rug_rational(&x);
        rug_x.clone_from(&rational_to_rug_rational(&y));
        assert_eq!(rug_rational_to_rational(&rug_x), y);
    });

    integer_pair_gen().test_properties(|(i, j)| {
        let x = Rational::from(&i);
        let y = Rational::from(&j);

        let mut mut_i = i.clone();
        let mut mut_x = x.clone();
        mut_i.clone_from(&j);
        mut_x.clone_from(&y);
        assert_eq!(mut_x, mut_i);
    });
}
