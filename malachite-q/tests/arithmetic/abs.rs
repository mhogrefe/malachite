use malachite_base::num::arithmetic::traits::{Abs, AbsAssign};
use malachite_nz_test_util::generators::integer_gen;
use malachite_q::Rational;
use malachite_q_test_util::common::{
    bigrational_to_rational, rational_to_bigrational, rational_to_rug_rational,
    rug_rational_to_rational,
};
use malachite_q_test_util::generators::rational_gen;
use num::{BigRational, Signed};
use std::str::FromStr;

#[test]
fn test_abs() {
    let test = |s, out| {
        let x = Rational::from_str(s).unwrap();

        let abs = x.clone().abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let abs = (&x).abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        assert_eq!(BigRational::from_str(s).unwrap().abs().to_string(), out);
        assert_eq!(rug::Rational::from_str(s).unwrap().abs().to_string(), out);

        let mut x = x;
        x.abs_assign();
        assert!(abs.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("0", "0");
    test("123", "123");
    test("-123", "123");
    test("22/7", "22/7");
    test("-22/7", "22/7");
}

#[test]
fn abs_properties() {
    rational_gen().test_properties(|x| {
        let abs = x.clone().abs();
        assert!(abs.is_valid());

        assert_eq!(
            bigrational_to_rational(&rational_to_bigrational(&x).abs()),
            abs
        );

        assert_eq!(
            rug_rational_to_rational(&rational_to_rug_rational(&x).abs()),
            abs
        );

        let abs_alt = (&x).abs();
        assert!(abs_alt.is_valid());
        assert_eq!(abs_alt, abs);

        let mut abs_alt = x.clone();
        abs_alt.abs_assign();
        assert!(abs_alt.is_valid());
        assert_eq!(abs_alt, abs);

        assert!(abs >= 0);
        assert_eq!(abs == x, x >= 0);
        assert_eq!((&abs).abs(), abs);
    });

    integer_gen().test_properties(|x| {
        assert_eq!(Rational::from(&x).abs(), Rational::from(x.abs()));
    });
}
