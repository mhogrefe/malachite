use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOfTwo, EqModPowerOfTwo, ModPowerOfTwo,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::arithmetic::eq_mod_power_of_two::{
    limbs_eq_limb_mod_power_of_two, limbs_eq_mod_power_of_two,
};
use malachite_nz::natural::Natural;
use malachite_nz_test_util::common::natural_to_rug_integer;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2,
    triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_small_unsigned, pairs_of_naturals,
    quadruples_of_natural_natural_natural_and_small_unsigned,
    triples_of_natural_natural_and_small_unsigned,
    triples_of_natural_natural_and_small_unsigned_var_1,
    triples_of_natural_natural_and_small_unsigned_var_2,
};

#[test]
fn limbs_eq_limb_mod_power_of_two_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2,
        |&(ref limbs, limb, pow)| {
            assert_eq!(
                limbs_eq_limb_mod_power_of_two(limbs, limb, pow),
                Natural::from_limbs_asc(limbs).eq_mod_power_of_two(&Natural::from(limb), pow)
            );
        },
    );
}

#[test]
fn limbs_eq_mod_power_of_two_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1,
        |&(ref xs, ref ys, pow)| {
            assert_eq!(
                limbs_eq_mod_power_of_two(xs, ys, pow),
                Natural::from_limbs_asc(xs).eq_mod_power_of_two(&Natural::from_limbs_asc(ys), pow)
            );
        },
    );
}

#[test]
fn eq_mod_power_of_two_properties() {
    test_properties(
        triples_of_natural_natural_and_small_unsigned,
        |&(ref x, ref y, pow)| {
            let eq_mod_power_of_two = x.eq_mod_power_of_two(y, pow);
            assert_eq!(
                natural_to_rug_integer(x)
                    .is_congruent_2pow(&natural_to_rug_integer(y), u32::exact_from(pow)),
                eq_mod_power_of_two
            );
            assert_eq!(y.eq_mod_power_of_two(x, pow), eq_mod_power_of_two);
            assert_eq!(
                x.mod_power_of_two(pow) == y.mod_power_of_two(pow),
                eq_mod_power_of_two
            );
        },
    );

    test_properties(
        triples_of_natural_natural_and_small_unsigned_var_1::<u64>,
        |&(ref x, ref y, pow)| {
            assert!(x.eq_mod_power_of_two(y, pow));
            assert!(natural_to_rug_integer(x)
                .is_congruent_2pow(&natural_to_rug_integer(y), u32::exact_from(pow)));
            assert!(y.eq_mod_power_of_two(x, pow));
            assert_eq!(x.mod_power_of_two(pow), y.mod_power_of_two(pow));
        },
    );

    test_properties(
        triples_of_natural_natural_and_small_unsigned_var_2::<u64>,
        |&(ref x, ref y, pow)| {
            assert!(!x.eq_mod_power_of_two(y, pow));
            assert!(!natural_to_rug_integer(x)
                .is_congruent_2pow(&natural_to_rug_integer(y), u32::exact_from(pow)));
            assert!(!y.eq_mod_power_of_two(x, pow));
            assert_ne!(x.mod_power_of_two(pow), y.mod_power_of_two(pow));
        },
    );

    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, pow)| {
        assert!(n.eq_mod_power_of_two(n, pow));
        assert_eq!(
            n.eq_mod_power_of_two(&Natural::ZERO, pow),
            n.divisible_by_power_of_two(pow)
        );
        assert_eq!(
            Natural::ZERO.eq_mod_power_of_two(n, pow),
            n.divisible_by_power_of_two(pow)
        );
    });

    test_properties(
        quadruples_of_natural_natural_natural_and_small_unsigned,
        |&(ref x, ref y, ref z, pow)| {
            if x.eq_mod_power_of_two(y, pow) && y.eq_mod_power_of_two(z, pow) {
                assert!(x.eq_mod_power_of_two(z, pow));
            }
        },
    );

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert!(x.eq_mod_power_of_two(y, 0));
    });
}
