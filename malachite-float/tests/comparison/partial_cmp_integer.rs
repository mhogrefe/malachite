use malachite_base::num::conversion::traits::ExactFrom;
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::{
    float_float_integer_triple_gen, float_integer_integer_triple_gen, float_integer_pair_gen,
    float_integer_pair_gen_var_1,
};
use malachite_float::Float;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_pair_gen;
use malachite_q::Rational;
use rug;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_cmp_integer() {
    let test = |s, s_hex, t, out| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);
        let v = Integer::from_str(t).unwrap();

        assert_eq!(u.partial_cmp(&v), out);
        assert_eq!(v.partial_cmp(&u), out.map(Ordering::reverse));
        assert_eq!(
            rug::Float::exact_from(&u).partial_cmp(&rug::Integer::from(&v)),
            out
        );
    };
    test("NaN", "NaN", "0", None);
    test("Infinity", "Infinity", "0", Some(Ordering::Greater));
    test("-Infinity", "-Infinity", "0", Some(Ordering::Less));
    test("0.0", "0x0.0", "0", Some(Ordering::Equal));
    test("-0.0", "-0x0.0", "0", Some(Ordering::Equal));
    test("1.0", "0x1.0#1", "0", Some(Ordering::Greater));
    test("2.0", "0x2.0#1", "0", Some(Ordering::Greater));
    test("0.5", "0x0.8#1", "0", Some(Ordering::Greater));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "0",
        Some(Ordering::Greater),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "0",
        Some(Ordering::Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "0",
        Some(Ordering::Greater),
    );
    test("3.0e120", "0x1.0E+100#1", "0", Some(Ordering::Greater));
    test("4.0e-121", "0x1.0E-100#1", "0", Some(Ordering::Greater));
    test("-1.0", "-0x1.0#1", "0", Some(Ordering::Less));
    test("-2.0", "-0x2.0#1", "0", Some(Ordering::Less));
    test("-0.5", "-0x0.8#1", "0", Some(Ordering::Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "0",
        Some(Ordering::Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "0",
        Some(Ordering::Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "0",
        Some(Ordering::Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", "0", Some(Ordering::Less));
    test("-4.0e-121", "-0x1.0E-100#1", "0", Some(Ordering::Less));

    test("NaN", "NaN", "1", None);
    test("Infinity", "Infinity", "1", Some(Ordering::Greater));
    test("-Infinity", "-Infinity", "1", Some(Ordering::Less));
    test("0.0", "0x0.0", "1", Some(Ordering::Less));
    test("-0.0", "-0x0.0", "1", Some(Ordering::Less));
    test("1.0", "0x1.0#1", "1", Some(Ordering::Equal));
    test("2.0", "0x2.0#1", "1", Some(Ordering::Greater));
    test("0.5", "0x0.8#1", "1", Some(Ordering::Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "1",
        Some(Ordering::Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "1",
        Some(Ordering::Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1",
        Some(Ordering::Greater),
    );
    test("3.0e120", "0x1.0E+100#1", "1", Some(Ordering::Greater));
    test("4.0e-121", "0x1.0E-100#1", "1", Some(Ordering::Less));
    test("-1.0", "-0x1.0#1", "1", Some(Ordering::Less));
    test("-2.0", "-0x2.0#1", "1", Some(Ordering::Less));
    test("-0.5", "-0x0.8#1", "1", Some(Ordering::Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "1",
        Some(Ordering::Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "1",
        Some(Ordering::Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1",
        Some(Ordering::Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", "1", Some(Ordering::Less));
    test("-4.0e-121", "-0x1.0E-100#1", "1", Some(Ordering::Less));

    test("NaN", "NaN", "100", None);
    test("Infinity", "Infinity", "100", Some(Ordering::Greater));
    test("-Infinity", "-Infinity", "100", Some(Ordering::Less));
    test("0.0", "0x0.0", "100", Some(Ordering::Less));
    test("-0.0", "-0x0.0", "100", Some(Ordering::Less));
    test("1.0", "0x1.0#1", "100", Some(Ordering::Less));
    test("2.0", "0x2.0#1", "100", Some(Ordering::Less));
    test("0.5", "0x0.8#1", "100", Some(Ordering::Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "100",
        Some(Ordering::Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "100",
        Some(Ordering::Less),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "100",
        Some(Ordering::Less),
    );
    test("3.0e120", "0x1.0E+100#1", "100", Some(Ordering::Greater));
    test("4.0e-121", "0x1.0E-100#1", "100", Some(Ordering::Less));
    test("-1.0", "-0x1.0#1", "100", Some(Ordering::Less));
    test("-2.0", "-0x2.0#1", "100", Some(Ordering::Less));
    test("-0.5", "-0x0.8#1", "100", Some(Ordering::Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "100",
        Some(Ordering::Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "100",
        Some(Ordering::Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "100",
        Some(Ordering::Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", "100", Some(Ordering::Less));
    test("-4.0e-121", "-0x1.0E-100#1", "100", Some(Ordering::Less));

    let s = "2582249878086908589655919172003011874329705792829223512830659356540647622016841194629\
    645353280137831435903171972747493376";
    test("NaN", "NaN", s, None);
    test("Infinity", "Infinity", s, Some(Ordering::Greater));
    test("-Infinity", "-Infinity", s, Some(Ordering::Less));
    test("0.0", "0x0.0", s, Some(Ordering::Less));
    test("-0.0", "-0x0.0", s, Some(Ordering::Less));
    test("1.0", "0x1.0#1", s, Some(Ordering::Less));
    test("2.0", "0x2.0#1", s, Some(Ordering::Less));
    test("0.5", "0x0.8#1", s, Some(Ordering::Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        s,
        Some(Ordering::Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        s,
        Some(Ordering::Less),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        s,
        Some(Ordering::Less),
    );
    test("3.0e120", "0x1.0E+100#1", s, Some(Ordering::Equal));
    test("4.0e-121", "0x1.0E-100#1", s, Some(Ordering::Less));
    test("-1.0", "-0x1.0#1", s, Some(Ordering::Less));
    test("-2.0", "-0x2.0#1", s, Some(Ordering::Less));
    test("-0.5", "-0x0.8#1", s, Some(Ordering::Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        s,
        Some(Ordering::Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        s,
        Some(Ordering::Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        s,
        Some(Ordering::Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", s, Some(Ordering::Less));
    test("-4.0e-121", "-0x1.0E-100#1", s, Some(Ordering::Less));

    // off by 1
    let s = "2582249878086908589655919172003011874329705792829223512830659356540647622016841194629\
    645353280137831435903171972747493377";
    test("NaN", "NaN", s, None);
    test("Infinity", "Infinity", s, Some(Ordering::Greater));
    test("-Infinity", "-Infinity", s, Some(Ordering::Less));
    test("0.0", "0x0.0", s, Some(Ordering::Less));
    test("-0.0", "-0x0.0", s, Some(Ordering::Less));
    test("1.0", "0x1.0#1", s, Some(Ordering::Less));
    test("2.0", "0x2.0#1", s, Some(Ordering::Less));
    test("0.5", "0x0.8#1", s, Some(Ordering::Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        s,
        Some(Ordering::Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        s,
        Some(Ordering::Less),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        s,
        Some(Ordering::Less),
    );
    test("3.0e120", "0x1.0E+100#1", s, Some(Ordering::Less));
    test("4.0e-121", "0x1.0E-100#1", s, Some(Ordering::Less));
    test("-1.0", "-0x1.0#1", s, Some(Ordering::Less));
    test("-2.0", "-0x2.0#1", s, Some(Ordering::Less));
    test("-0.5", "-0x0.8#1", s, Some(Ordering::Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        s,
        Some(Ordering::Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        s,
        Some(Ordering::Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        s,
        Some(Ordering::Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", s, Some(Ordering::Less));
    test("-4.0e-121", "-0x1.0E-100#1", s, Some(Ordering::Less));

    test("NaN", "NaN", "-1", None);
    test("Infinity", "Infinity", "-1", Some(Ordering::Greater));
    test("-Infinity", "-Infinity", "-1", Some(Ordering::Less));
    test("0.0", "0x0.0", "-1", Some(Ordering::Greater));
    test("-0.0", "-0x0.0", "-1", Some(Ordering::Greater));
    test("1.0", "0x1.0#1", "-1", Some(Ordering::Greater));
    test("2.0", "0x2.0#1", "-1", Some(Ordering::Greater));
    test("0.5", "0x0.8#1", "-1", Some(Ordering::Greater));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "-1",
        Some(Ordering::Greater),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-1",
        Some(Ordering::Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1",
        Some(Ordering::Greater),
    );
    test("3.0e120", "0x1.0E+100#1", "-1", Some(Ordering::Greater));
    test("4.0e-121", "0x1.0E-100#1", "-1", Some(Ordering::Greater));
    test("-1.0", "-0x1.0#1", "-1", Some(Ordering::Equal));
    test("-2.0", "-0x2.0#1", "-1", Some(Ordering::Less));
    test("-0.5", "-0x0.8#1", "-1", Some(Ordering::Greater));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "-1",
        Some(Ordering::Greater),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-1",
        Some(Ordering::Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1",
        Some(Ordering::Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", "-1", Some(Ordering::Less));
    test("-4.0e-121", "-0x1.0E-100#1", "-1", Some(Ordering::Greater));

    test("NaN", "NaN", "-100", None);
    test("Infinity", "Infinity", "-100", Some(Ordering::Greater));
    test("-Infinity", "-Infinity", "-100", Some(Ordering::Less));
    test("0.0", "0x0.0", "-100", Some(Ordering::Greater));
    test("-0.0", "-0x0.0", "-100", Some(Ordering::Greater));
    test("1.0", "0x1.0#1", "-100", Some(Ordering::Greater));
    test("2.0", "0x2.0#1", "-100", Some(Ordering::Greater));
    test("0.5", "0x0.8#1", "-100", Some(Ordering::Greater));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "-100",
        Some(Ordering::Greater),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-100",
        Some(Ordering::Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-100",
        Some(Ordering::Greater),
    );
    test("3.0e120", "0x1.0E+100#1", "-100", Some(Ordering::Greater));
    test("4.0e-121", "0x1.0E-100#1", "-100", Some(Ordering::Greater));
    test("-1.0", "-0x1.0#1", "-100", Some(Ordering::Greater));
    test("-2.0", "-0x2.0#1", "-100", Some(Ordering::Greater));
    test("-0.5", "-0x0.8#1", "-100", Some(Ordering::Greater));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "-100",
        Some(Ordering::Greater),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-100",
        Some(Ordering::Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-100",
        Some(Ordering::Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", "-100", Some(Ordering::Less));
    test(
        "-4.0e-121",
        "-0x1.0E-100#1",
        "-100",
        Some(Ordering::Greater),
    );

    let s = "-258224987808690858965591917200301187432970579282922351283065935654064762201684119462\
    9645353280137831435903171972747493376";
    test("NaN", "NaN", s, None);
    test("Infinity", "Infinity", s, Some(Ordering::Greater));
    test("-Infinity", "-Infinity", s, Some(Ordering::Less));
    test("0.0", "0x0.0", s, Some(Ordering::Greater));
    test("-0.0", "-0x0.0", s, Some(Ordering::Greater));
    test("1.0", "0x1.0#1", s, Some(Ordering::Greater));
    test("2.0", "0x2.0#1", s, Some(Ordering::Greater));
    test("0.5", "0x0.8#1", s, Some(Ordering::Greater));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        s,
        Some(Ordering::Greater),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        s,
        Some(Ordering::Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        s,
        Some(Ordering::Greater),
    );
    test("3.0e120", "0x1.0E+100#1", s, Some(Ordering::Greater));
    test("4.0e-121", "0x1.0E-100#1", s, Some(Ordering::Greater));
    test("-1.0", "-0x1.0#1", s, Some(Ordering::Greater));
    test("-2.0", "-0x2.0#1", s, Some(Ordering::Greater));
    test("-0.5", "-0x0.8#1", s, Some(Ordering::Greater));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        s,
        Some(Ordering::Greater),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        s,
        Some(Ordering::Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        s,
        Some(Ordering::Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", s, Some(Ordering::Equal));
    test("-4.0e-121", "-0x1.0E-100#1", s, Some(Ordering::Greater));

    // off by 1
    let s = "-258224987808690858965591917200301187432970579282922351283065935654064762201684119462\
    9645353280137831435903171972747493377";
    test("NaN", "NaN", s, None);
    test("Infinity", "Infinity", s, Some(Ordering::Greater));
    test("-Infinity", "-Infinity", s, Some(Ordering::Less));
    test("0.0", "0x0.0", s, Some(Ordering::Greater));
    test("-0.0", "-0x0.0", s, Some(Ordering::Greater));
    test("1.0", "0x1.0#1", s, Some(Ordering::Greater));
    test("2.0", "0x2.0#1", s, Some(Ordering::Greater));
    test("0.5", "0x0.8#1", s, Some(Ordering::Greater));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        s,
        Some(Ordering::Greater),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        s,
        Some(Ordering::Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        s,
        Some(Ordering::Greater),
    );
    test("3.0e120", "0x1.0E+100#1", s, Some(Ordering::Greater));
    test("4.0e-121", "0x1.0E-100#1", s, Some(Ordering::Greater));
    test("-1.0", "-0x1.0#1", s, Some(Ordering::Greater));
    test("-2.0", "-0x2.0#1", s, Some(Ordering::Greater));
    test("-0.5", "-0x0.8#1", s, Some(Ordering::Greater));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        s,
        Some(Ordering::Greater),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        s,
        Some(Ordering::Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        s,
        Some(Ordering::Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", s, Some(Ordering::Greater));
    test("-4.0e-121", "-0x1.0E-100#1", s, Some(Ordering::Greater));
}

#[test]
fn partial_cmp_integer_properties() {
    float_integer_pair_gen().test_properties(|(x, y)| {
        let cmp = x.partial_cmp(&y);
        assert_eq!(x.partial_cmp(&Float::from(&y)), cmp);
        assert_eq!(
            rug::Float::exact_from(&x).partial_cmp(&rug::Integer::from(&y)),
            cmp
        );
        assert_eq!(y.partial_cmp(&x), cmp.map(Ordering::reverse));
    });

    float_float_integer_triple_gen().test_properties(|(x, z, y)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });

    float_integer_integer_triple_gen().test_properties(|(y, x, z)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });

    integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Float::from(&x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Float::from(&y)), Some(x.cmp(&y)));
    });

    float_integer_pair_gen_var_1().test_properties(|(x, y)| {
        assert_eq!(Rational::exact_from(&x).partial_cmp(&y), x.partial_cmp(&y));
    });
}
