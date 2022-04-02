use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_from_integer() {
    let test = |s| {
        let u = Integer::from_str(s).unwrap();

        let x = Rational::from(u.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);

        let x = Rational::from(&u);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);
    };
    test("0");
    test("123");
    test("1000000000000");
    test("-123");
    test("-1000000000000");
}

#[test]
fn from_integer_properties() {
    integer_gen().test_properties(|x| {
        let rational_x = Rational::from(x.clone());
        assert!(rational_x.is_valid());
        assert_eq!(rational_x.to_string(), x.to_string());

        let rational_x_alt = Rational::from(&x);
        assert!(rational_x_alt.is_valid());
        assert_eq!(rational_x_alt, rational_x);

        assert_eq!(Integer::checked_from(&rational_x).as_ref(), Some(&x));
        assert_eq!(Integer::checked_from(rational_x), Some(x));
    });
}
