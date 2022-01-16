use malachite_base::num::arithmetic::traits::CoprimeWith;
use malachite_base::num::basic::traits::One;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_nz_test_util::generators::{
    integer_gen, integer_pair_gen_var_1, natural_gen, natural_natural_bool_triple_gen_var_1,
    natural_pair_gen_var_5,
};
use malachite_q::Rational;
use malachite_q_test_util::common::{bigrational_to_rational, rug_rational_to_rational};
use num::{BigInt, BigRational};
use std::str::FromStr;

#[test]
fn test_from_naturals() {
    let test = |n, d, out| {
        let n = Natural::from_str(n).unwrap();
        let d = Natural::from_str(d).unwrap();
        let x = Rational::from_naturals(n.clone(), d.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);

        let x = Rational::from_naturals_ref(&n, &d);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("0", "1", "0");
    test("0", "5", "0");
    test("3", "6", "1/2");
    test("100", "101", "100/101");
}

#[test]
fn from_naturals_properties() {
    natural_pair_gen_var_5().test_properties(|(n, d)| {
        let x = Rational::from_naturals(n.clone(), d.clone());
        assert!(x.is_valid());

        let x_alt = Rational::from_naturals_ref(&n, &d);
        assert!(x.is_valid());
        assert_eq!(x, x_alt);

        if (&n).coprime_with(&d) {
            assert_eq!(x.into_numerator_and_denominator(), (n, d));
        }
    });

    natural_gen()
        .test_properties(|n| assert_eq!(Rational::from_naturals_ref(&n, &Natural::ONE), n));
}

#[test]
fn test_from_integers() {
    let test = |s, t, out| {
        let n = Integer::from_str(s).unwrap();
        let d = Integer::from_str(t).unwrap();
        let x = Rational::from_integers(n.clone(), d.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);

        let x = Rational::from_integers_ref(&n, &d);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);

        assert_eq!(
            BigRational::new(BigInt::from_str(s).unwrap(), BigInt::from_str(t).unwrap())
                .to_string(),
            out
        );
        assert_eq!(
            rug::Rational::from((
                rug::Integer::from_str(s).unwrap(),
                rug::Integer::from_str(t).unwrap()
            ))
            .to_string(),
            out
        );
    };
    test("0", "1", "0");
    test("0", "-1", "0");
    test("0", "5", "0");
    test("0", "-5", "0");
    test("3", "6", "1/2");
    test("3", "-6", "-1/2");
    test("-3", "6", "-1/2");
    test("-3", "-6", "1/2");
    test("100", "101", "100/101");
    test("100", "-101", "-100/101");
    test("-100", "101", "-100/101");
    test("-100", "-101", "100/101");
}

#[test]
fn from_integers_properties() {
    integer_pair_gen_var_1().test_properties(|(n, d)| {
        let x = Rational::from_integers(n.clone(), d.clone());
        assert!(x.is_valid());

        let x_alt = Rational::from_integers_ref(&n, &d);
        assert!(x.is_valid());
        assert_eq!(x, x_alt);

        assert_eq!(
            bigrational_to_rational(&BigRational::new(
                integer_to_bigint(&n),
                integer_to_bigint(&d)
            )),
            x
        );
        assert_eq!(
            rug_rational_to_rational(&rug::Rational::from((
                integer_to_rug_integer(&n),
                integer_to_rug_integer(&d)
            ))),
            x
        );

        if n != 0 {
            assert_eq!(x >= 0, (n >= 0) == (d >= 0));
        }
        if n >= 0 && d > 0 && (n.unsigned_abs_ref()).coprime_with(d.unsigned_abs_ref()) {
            let (n_2, d_2) = x.into_numerator_and_denominator();
            assert_eq!((Integer::from(n_2), Integer::from(d_2)), (n, d));
        }
    });

    integer_gen()
        .test_properties(|n| assert_eq!(Rational::from_integers_ref(&n, &Integer::ONE), n));
}

#[test]
fn test_from_sign_and_naturals() {
    let test = |sign, n, d, out| {
        let n = Natural::from_str(n).unwrap();
        let d = Natural::from_str(d).unwrap();
        let x = Rational::from_sign_and_naturals(sign, n.clone(), d.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);

        let x = Rational::from_sign_and_naturals_ref(sign, &n, &d);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(false, "0", "1", "0");
    test(true, "0", "1", "0");
    test(false, "0", "5", "0");
    test(true, "0", "5", "0");
    test(false, "3", "6", "-1/2");
    test(true, "3", "6", "1/2");
    test(false, "100", "101", "-100/101");
    test(true, "100", "101", "100/101");
}

#[test]
fn from_sign_and_naturals_properties() {
    natural_natural_bool_triple_gen_var_1().test_properties(|(n, d, sign)| {
        let x = Rational::from_sign_and_naturals(sign, n.clone(), d.clone());
        assert!(x.is_valid());

        let x_alt = Rational::from_sign_and_naturals_ref(sign, &n, &d);
        assert!(x.is_valid());
        assert_eq!(x, x_alt);

        if n != 0 {
            assert_eq!(x >= 0, sign);
        }
        if (&n).coprime_with(&d) {
            assert_eq!(x.into_numerator_and_denominator(), (n, d));
        }
    });

    natural_pair_gen_var_5().test_properties(|(n, d)| {
        assert_eq!(
            Rational::from_naturals(n.clone(), d.clone()),
            Rational::from_sign_and_naturals(true, n, d)
        );
    });

    natural_gen().test_properties(|n| {
        assert_eq!(
            Rational::from_sign_and_naturals_ref(true, &n, &Natural::ONE),
            n
        )
    });
}
