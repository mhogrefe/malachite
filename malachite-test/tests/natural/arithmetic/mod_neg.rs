use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    DivisibleBy, Mod, ModIsReduced, ModNeg, ModNegAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigneds_var_5;
use malachite_test::inputs::natural::pairs_of_naturals_var_2;

#[test]
fn test_mod_neg() {
    let test = |u, v, out| {
        assert!(Natural::from_str(u)
            .unwrap()
            .mod_is_reduced(&Natural::from_str(v).unwrap()));
        assert_eq!(
            Natural::from_str(u)
                .unwrap()
                .mod_neg(Natural::from_str(v).unwrap())
                .to_string(),
            out
        );
        assert_eq!(
            Natural::from_str(u)
                .unwrap()
                .mod_neg(&Natural::from_str(v).unwrap())
                .to_string(),
            out
        );
        assert_eq!(
            (&Natural::from_str(u).unwrap())
                .mod_neg(Natural::from_str(v).unwrap())
                .to_string(),
            out
        );
        assert_eq!(
            (&Natural::from_str(u).unwrap())
                .mod_neg(&Natural::from_str(v).unwrap())
                .to_string(),
            out
        );

        let mut n = Natural::from_str(u).unwrap();
        n.mod_neg_assign(Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.mod_neg_assign(&Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
    };
    test("0", "5", "0");
    test("7", "10", "3");
    test("100", "101", "1");
    test("4294967294", "4294967295", "1");
    test("1", "4294967295", "4294967294");
    test("7", "1000000000000", "999999999993");
    test("999999999993", "1000000000000", "7");
}

#[test]
fn mod_neg_properties() {
    test_properties(pairs_of_naturals_var_2, |(n, modulus)| {
        assert!(n.mod_is_reduced(modulus));
        let neg = n.mod_neg(modulus);
        assert!(neg.is_valid());
        assert!(neg.mod_is_reduced(modulus));

        let neg_alt = n.mod_neg(modulus.clone());
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        let neg_alt = n.clone().mod_neg(modulus);
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        let neg_alt = n.clone().mod_neg(modulus.clone());
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        let mut n_alt = n.clone();
        n_alt.mod_neg_assign(modulus);
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        let mut n_alt = n.clone();
        n_alt.mod_neg_assign(modulus.clone());
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        assert_eq!(neg, (-n).mod_op(Integer::from(modulus)));
        assert_eq!((&neg).mod_neg(modulus), *n);
        //TODO use mod_add
        assert!((n + &neg).divisible_by(modulus));
        assert_eq!(*n == neg, *n == Natural::ZERO || n << 1 == *modulus);
    });

    test_properties(pairs_of_unsigneds_var_5::<Limb>, |&(n, modulus)| {
        assert_eq!(
            n.mod_neg(modulus),
            Natural::from(n).mod_neg(Natural::from(modulus))
        );
    });
}
