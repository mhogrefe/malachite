use malachite_base::num::basic::traits::Zero;
use malachite_nz_test_util::generators::integer_pair_gen;
use malachite_q::Rational;
use malachite_q_test_util::arithmetic::add::add_naive;
use malachite_q_test_util::common::{
    bigrational_to_rational, rational_to_bigrational, rational_to_rug_rational,
    rug_rational_to_rational,
};
use malachite_q_test_util::generators::{rational_gen, rational_pair_gen, rational_triple_gen};
use num::BigRational;
use std::str::FromStr;

#[test]
fn test_add() {
    let test = |s, t, out| {
        let u = Rational::from_str(s).unwrap();
        let v = Rational::from_str(t).unwrap();

        let mut n = u.clone();
        n += v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n += &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() + v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u + v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() + &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u + &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigRational::from_str(s).unwrap() + BigRational::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Rational::from_str(s).unwrap() + rug::Rational::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "579");
    test("0", "-123", "-123");
    test("123", "-123", "0");
    test("123", "-456", "-333");
    test("0", "123", "123");
    test("-123", "0", "-123");
    test("-123", "456", "333");
    test("0", "-123", "-123");
    test("-123", "123", "0");
    test("1/2", "1/3", "5/6");
    test("1/2", "-1/3", "1/6");
    test("-1/2", "1/3", "-1/6");
    test("-1/2", "-1/3", "-5/6");
    test("1/2", "1/2", "1");
    test("1/2", "-1/2", "0");
    test("-1/2", "1/2", "0");
    test("-1/2", "-1/2", "-1");
}

#[test]
fn add_properties() {
    rational_pair_gen().test_properties(|(x, y)| {
        let sum_val_val = x.clone() + y.clone();
        let sum_val_ref = x.clone() + &y;
        let sum_ref_val = &x + y.clone();
        let sum = &x + &y;
        assert!(sum_val_val.is_valid());
        assert!(sum_val_ref.is_valid());
        assert!(sum_ref_val.is_valid());
        assert!(sum.is_valid());
        assert_eq!(sum_val_val, sum);
        assert_eq!(sum_val_ref, sum);
        assert_eq!(sum_ref_val, sum);

        let mut mut_x = x.clone();
        mut_x += y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x += &y;
        assert_eq!(mut_x, sum);
        assert!(mut_x.is_valid());

        let mut mut_x = rational_to_rug_rational(&x);
        mut_x += rational_to_rug_rational(&y);
        assert_eq!(rug_rational_to_rational(&mut_x), sum);

        assert_eq!(
            bigrational_to_rational(&(rational_to_bigrational(&x) + rational_to_bigrational(&y))),
            sum
        );
        assert_eq!(
            rug_rational_to_rational(
                &(rational_to_rug_rational(&x) + rational_to_rug_rational(&y))
            ),
            sum
        );
        assert_eq!(add_naive(x.clone(), y.clone()), sum);
        assert_eq!(&y + &x, sum);
        assert_eq!(&sum - &x, y);
        assert_eq!(sum - y, x);
    });

    rational_gen().test_properties(|ref x| {
        assert_eq!(x + Rational::ZERO, *x);
        assert_eq!(Rational::ZERO + x, *x);
        assert_eq!(x + x, x << 1);
        assert_eq!(x + (-x), 0)
    });

    rational_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!((&x + &y) + &z, x + (y + z));
    });

    integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(&x + &y, Rational::from(x) + Rational::from(y));
    });
}
