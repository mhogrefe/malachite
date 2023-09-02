use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::NaN;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::primitive_float_gen_var_11;
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::{float_gen_var_2, float_gen_var_3};
use malachite_float::Float;
use malachite_q::Rational;
use std::cmp::Ordering;

#[test]
fn test_sign() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.sign(), out);
    };
    test("Infinity", "Infinity", Ordering::Greater);
    test("-Infinity", "-Infinity", Ordering::Less);
    test("0.0", "0x0.0", Ordering::Greater);
    test("-0.0", "-0x0.0", Ordering::Less);

    test("1.0", "0x1.0#1", Ordering::Greater);
    test("2.0", "0x2.0#1", Ordering::Greater);
    test("0.5", "0x0.8#1", Ordering::Greater);
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Ordering::Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ordering::Greater,
    );

    test("-1.0", "-0x1.0#1", Ordering::Less);
    test("-2.0", "-0x2.0#1", Ordering::Less);
    test("-0.5", "-0x0.8#1", Ordering::Less);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Ordering::Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ordering::Less,
    );
}

#[test]
#[should_panic]
fn sign_fail() {
    Float::NAN.sign();
}

#[test]
fn sign_properties() {
    float_gen_var_2().test_properties(|x| {
        let sign = x.sign();
        assert_ne!(sign, Ordering::Equal);
        assert_eq!(
            if x.is_sign_positive() {
                Ordering::Greater
            } else {
                Ordering::Less
            },
            sign
        );
        assert_eq!((-x).sign(), sign.reverse());
    });

    float_gen_var_3().test_properties(|x| {
        assert_eq!(x.sign(), Rational::exact_from(x).sign());
    });

    primitive_float_gen_var_11::<f64>().test_properties(|x| {
        assert_eq!(x.sign(), Float::from(x).sign());
    });
}
