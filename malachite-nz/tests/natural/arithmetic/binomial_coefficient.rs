use malachite_base::num::arithmetic::traits::{BinomialCoefficient, DivExact, Gcd};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_pair_gen_var_44;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::common::{natural_to_rug_integer, rug_integer_to_natural};
use malachite_nz::test_util::generators::{
    natural_gen, natural_gen_var_2, natural_pair_gen_var_15,
};
use malachite_nz::test_util::natural::arithmetic::binomial_coefficient::binomial_coefficient_naive;
use std::str::FromStr;

#[test]
fn test_binomial_coefficient() {
    fn test(n: &str, k: &str, out: &str) {
        let n = Natural::from_str(n).unwrap();
        let k = Natural::from_str(k).unwrap();
        let b = Natural::binomial_coefficient(n.clone(), k.clone());
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);

        let b_alt = Natural::binomial_coefficient(&n, &k);
        assert!(b_alt.is_valid());
        assert_eq!(b_alt, b);

        assert_eq!(binomial_coefficient_naive(n.clone(), k.clone()), b);
        assert_eq!(
            natural_to_rug_integer(&n)
                .binomial(u32::exact_from(&k))
                .to_string(),
            out,
        );
    }
    test("0", "0", "1");
    test("1", "0", "1");
    test("1", "1", "1");
    test("2", "0", "1");
    test("2", "1", "2");
    test("2", "2", "1");
    test("3", "0", "1");
    test("3", "1", "3");
    test("3", "2", "3");
    test("3", "3", "1");
    test("4", "0", "1");
    test("4", "1", "4");
    test("4", "2", "6");
    test("4", "3", "4");
    test("4", "4", "1");
    test("1", "2", "0");
    test("10", "5", "252");
    test("100", "50", "100891344545564193334812497256");
}

#[test]
fn binomial_coefficient_properties() {
    natural_pair_gen_var_15().test_properties(|(n, k)| {
        let b = Natural::binomial_coefficient(n.clone(), k.clone());
        assert!(b.is_valid());

        let b_alt = Natural::binomial_coefficient(&n, &k);
        assert!(b_alt.is_valid());
        assert_eq!(b, b_alt);

        assert_eq!(binomial_coefficient_naive(n.clone(), k.clone()), b);
        assert_eq!(
            rug_integer_to_natural(&natural_to_rug_integer(&n).binomial(u32::exact_from(&k))),
            b
        );
        assert_eq!(b == 0u32, n < k);
        if n >= k {
            assert_eq!(Natural::binomial_coefficient(&n, &(&n - &k)), b);
        }
        if n != 0u32 && k != 0u32 {
            let c = Natural::binomial_coefficient(&n - Natural::ONE, &k - Natural::ONE);
            assert_eq!(
                Natural::binomial_coefficient(&(&n - Natural::ONE), &k) + &c,
                b
            );
            let gcd = (&n).gcd(&k);
            assert_eq!(c.div_exact(k.div_exact(&gcd)) * n.div_exact(gcd), b);
        }
    });

    natural_gen().test_properties(|n| {
        assert_eq!(Natural::binomial_coefficient(&n, &Natural::ZERO), 1u32);
        assert_eq!(Natural::binomial_coefficient(&n, &Natural::ONE), n);
    });

    natural_gen_var_2().test_properties(|n| {
        assert_eq!(Natural::binomial_coefficient(&n, &n), 1u32);
        assert_eq!(Natural::binomial_coefficient(&n, &(&n - Natural::ONE)), n);
        assert_eq!(Natural::binomial_coefficient(Natural::ZERO, n), 0u32);
    });

    unsigned_pair_gen_var_44::<Limb>().test_properties(|(n, k)| {
        assert_eq!(
            Natural::binomial_coefficient(Natural::from(n), Natural::from(k)),
            Limb::binomial_coefficient(n, k)
        );
    });
}
