use malachite_base::num::arithmetic::traits::Reciprocal;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_q::test_util::arithmetic::div::div_naive;
use malachite_q::test_util::common::{
    bigrational_to_rational, rational_to_bigrational, rational_to_rug_rational,
    rug_rational_to_rational,
};
use malachite_q::test_util::generators::{
    rational_gen, rational_gen_var_1, rational_pair_gen_var_1, rational_triple_gen_var_1,
};
use malachite_q::Rational;
use num::BigRational;
use std::str::FromStr;

#[test]
fn test_div() {
    let test = |s, t, out| {
        let u = Rational::from_str(s).unwrap();
        let v = Rational::from_str(t).unwrap();

        let mut n = u.clone();
        n /= v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n /= &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() / v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u / v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() / &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u / &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigRational::from_str(s).unwrap() / BigRational::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Rational::from_str(s).unwrap() / rug::Rational::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "1/123", "0");
    test("0", "-1/123", "0");
    test("1", "1/123", "123");
    test("1", "-1/123", "-123");
    test("-1", "1/123", "-123");
    test("-1", "-1/123", "123");
    test("123", "1", "123");
    test("123", "-1", "-123");
    test("-123", "1", "-123");
    test("-123", "-1", "123");
    test("123", "1/456", "56088");
    test("123", "-1/456", "-56088");
    test("-123", "1/456", "-56088");
    test("-123", "-1/456", "56088");
    test("22/7", "2/3", "33/7");
    test("22/7", "-2/3", "-33/7");
    test("-22/7", "2/3", "-33/7");
    test("-22/7", "-2/3", "33/7");
    test("4/5", "4/5", "1");
    test("4/5", "-4/5", "-1");
    test("-4/5", "4/5", "-1");
    test("-4/5", "-4/5", "1");
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_fail() {
    Rational::ONE / Rational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_val_ref_fail() {
    Rational::ONE / &Rational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_ref_val_fail() {
    &Rational::ONE / Rational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_ref_ref_fail() {
    &Rational::ONE / &Rational::ZERO;
}

#[test]
#[should_panic]
fn div_assign_fail() {
    let mut x = Rational::ONE;
    x /= Rational::ZERO;
}

#[test]
#[should_panic]
fn div_assign_ref_fail() {
    let mut x = Rational::ONE;
    x /= &Rational::ZERO;
}

#[allow(clippy::eq_op)]
#[test]
fn div_properties() {
    rational_pair_gen_var_1().test_properties(|(x, y)| {
        let quotient_val_val = x.clone() / y.clone();
        let quotient_val_ref = x.clone() / &y;
        let quotient_ref_val = &x / y.clone();
        let quotient = &x / &y;
        assert!(quotient_val_val.is_valid());
        assert!(quotient_val_ref.is_valid());
        assert!(quotient_ref_val.is_valid());
        assert!(quotient.is_valid());
        assert_eq!(quotient_val_val, quotient);
        assert_eq!(quotient_val_ref, quotient);
        assert_eq!(quotient_ref_val, quotient);

        let mut mut_x = x.clone();
        mut_x /= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, quotient);
        let mut mut_x = x.clone();
        mut_x /= &y;
        assert_eq!(mut_x, quotient);
        assert!(mut_x.is_valid());

        let mut mut_x = rational_to_rug_rational(&x);
        mut_x /= rational_to_rug_rational(&y);
        assert_eq!(rug_rational_to_rational(&mut_x), quotient);

        assert_eq!(
            bigrational_to_rational(&(rational_to_bigrational(&x) / rational_to_bigrational(&y))),
            quotient
        );
        assert_eq!(
            rug_rational_to_rational(
                &(rational_to_rug_rational(&x) / rational_to_rug_rational(&y))
            ),
            quotient
        );
        assert_eq!(div_naive(x.clone(), y.clone()), quotient);
        assert_eq!(&x * (&y).reciprocal(), quotient);
        assert_eq!(&quotient * &y, x);
        if quotient != 0u32 {
            assert_eq!(&y / &x, (&quotient).reciprocal());
            assert_eq!(&x / &quotient, y);
        }
        assert_eq!(-&x / &y, -&quotient);
        assert_eq!(x / -y, -quotient);
    });

    rational_gen().test_properties(|ref x| {
        assert_eq!(x / Rational::ONE, *x);
        assert_eq!(x / Rational::NEGATIVE_ONE, -x);
    });

    rational_gen_var_1().test_properties(|ref x| {
        assert_eq!(Rational::ZERO / x, 0);
        assert_eq!(Rational::ONE / x, x.reciprocal());
        assert_eq!(Rational::NEGATIVE_ONE / x, -x.reciprocal());
        assert_eq!(x / x, 1);
    });

    rational_triple_gen_var_1().test_properties(|(ref x, ref y, ref z)| {
        assert_eq!((x + y) / z, x / z + y / z);
        assert_eq!((x - y) / z, x / z - y / z);
    });
}
