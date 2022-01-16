use malachite_nz::natural::Natural;
use malachite_nz_test_util::common::natural_to_rug_integer;
use malachite_nz_test_util::generators::natural_pair_gen;
use malachite_q::Rational;
use malachite_q_test_util::common::rational_to_rug_rational;
use malachite_q_test_util::generators::rational_natural_pair_gen;
use rug;
use std::str::FromStr;

#[test]
fn test_rational_partial_eq_natural() {
    let test = |s, t, out| {
        let u = Rational::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(v == u, out);
        assert_eq!(
            rug::Rational::from_str(s).unwrap() == rug::Rational::from_str(t).unwrap(),
            out
        );
    };
    test("0", "0", true);
    test("0", "5", false);
    test("123", "123", true);
    test("-123", "123", false);
    test("123", "5", false);
    test("1000000000000", "123", false);
    test("123", "1000000000000", false);
    test("1000000000000", "1000000000000", true);
    test("-1000000000000", "1000000000000", false);
    test("22/7", "3", false);
    test("1/2", "2", false);
    test("-1/2", "2", false);
}

#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_natural_properties() {
    rational_natural_pair_gen().test_properties(|(x, y)| {
        let eq = x == y;
        assert_eq!(y == x, eq);
        assert_eq!(x == Rational::from(&y), eq);
        assert_eq!(
            rational_to_rug_rational(&x) == natural_to_rug_integer(&y),
            eq
        );
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Rational::from(&x) == y, x == y);
        assert_eq!(x == Rational::from(&y), x == y);
    });
}
