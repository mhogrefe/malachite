use malachite_base::strings::string_is_subset;
use malachite_base::strings::ToDebugString;
use malachite_nz_test_util::generators::integer_gen;
use malachite_q::Rational;
use malachite_q_test_util::common::{rational_to_bigrational, rational_to_rug_rational};
use malachite_q_test_util::generators::rational_gen;
use std::str::FromStr;

#[test]
pub fn test_to_string() {
    fn test(u: &str) {
        let x = Rational::from_str(u).unwrap();
        assert_eq!(x.to_string(), u);
    }
    test("0");
    test("2");
    test("123");
    test("1000");
    test("1000000");
    test("1000000000000000");
    test("-2");
    test("-123");
    test("-1000");
    test("-1000000");
    test("-1000000000000000");
    test("99/100");
    test("101/100");
    test("22/7");
    test("-99/100");
    test("-101/100");
    test("-22/7");
}

#[test]
pub fn test_to_debug_string() {
    fn test(u: &str, out: &str) {
        let x = Rational::from_str(u).unwrap();
        assert_eq!(x.to_debug_string(), out);
    }
    test("0", "Rational { sign: true, numerator: 0, denominator: 1 }");
    test("2", "Rational { sign: true, numerator: 2, denominator: 1 }");
    test(
        "123",
        "Rational { sign: true, numerator: 123, denominator: 1 }",
    );
    test(
        "1000",
        "Rational { sign: true, numerator: 1000, denominator: 1 }",
    );
    test(
        "1000000",
        "Rational { sign: true, numerator: 1000000, denominator: 1 }",
    );
    test(
        "1000000000000000",
        "Rational { sign: true, numerator: 1000000000000000, denominator: 1 }",
    );
    test(
        "-2",
        "Rational { sign: false, numerator: 2, denominator: 1 }",
    );
    test(
        "-123",
        "Rational { sign: false, numerator: 123, denominator: 1 }",
    );
    test(
        "-1000",
        "Rational { sign: false, numerator: 1000, denominator: 1 }",
    );
    test(
        "-1000000",
        "Rational { sign: false, numerator: 1000000, denominator: 1 }",
    );
    test(
        "-1000000000000000",
        "Rational { sign: false, numerator: 1000000000000000, denominator: 1 }",
    );
    test(
        "99/100",
        "Rational { sign: true, numerator: 99, denominator: 100 }",
    );
    test(
        "101/100",
        "Rational { sign: true, numerator: 101, denominator: 100 }",
    );
    test(
        "22/7",
        "Rational { sign: true, numerator: 22, denominator: 7 }",
    );
    test(
        "-99/100",
        "Rational { sign: false, numerator: 99, denominator: 100 }",
    );
    test(
        "-101/100",
        "Rational { sign: false, numerator: 101, denominator: 100 }",
    );
    test(
        "-22/7",
        "Rational { sign: false, numerator: 22, denominator: 7 }",
    );
}

#[test]
fn to_string_properties() {
    rational_gen().test_properties(|x| {
        let s = x.to_string();
        assert_eq!(rational_to_bigrational(&x).to_string(), s);
        assert_eq!(rational_to_rug_rational(&x).to_string(), s);
        assert!(string_is_subset(&s, "-/0123456789"));
        if x != 0 {
            assert!(!s.starts_with('0'));
        }
    });

    integer_gen().test_properties(|x| {
        assert_eq!(Rational::from(&x).to_string(), x.to_string());
    });
}

#[test]
fn to_debug_string_properties() {
    rational_gen().test_properties(|x| {
        let s = x.to_debug_string();
        assert!(string_is_subset(&s, " ,0123456789:Radefgilmnorstu{}"));
    });

    integer_gen().test_properties(|x| {
        assert_eq!(Rational::from(&x).to_string(), x.to_string());
    });
}
