use std::cmp::min;

use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOf2, ModPowerOf2, ModPowerOf2Assign, ModPowerOf2IsReduced, NegModPowerOf2,
    NegModPowerOf2Assign, PowerOf2, RemPowerOf2, RemPowerOf2Assign, ShrRound,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::arithmetic::mod_power_of_2::{
    limbs_mod_power_of_2, limbs_neg_mod_power_of_2, limbs_neg_mod_power_of_2_in_place,
    limbs_slice_mod_power_of_2_in_place, limbs_vec_mod_power_of_2_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_u64_var_4, pairs_of_unsigned_and_small_unsigned,
    pairs_of_unsigned_vec_and_small_unsigned, unsigneds,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_unsigned, pairs_of_natural_and_small_unsigned_var_1,
    pairs_of_natural_and_small_unsigned_var_2, triples_of_natural_natural_and_small_unsigned,
    triples_of_natural_small_unsigned_and_small_unsigned,
};

#[test]
fn limbs_mod_power_of_2_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, pow)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2(limbs, pow)),
                Natural::from_limbs_asc(limbs).mod_power_of_2(pow),
            );
        },
    );
}

macro_rules! limbs_slice_mod_power_of_2_in_place_helper {
    ($f: ident, $xs: ident, $pow: ident) => {
        let mut xs = $xs.to_vec();
        let old_xs = xs.clone();
        $f(&mut xs, $pow);
        let n = Natural::from_limbs_asc(&old_xs).mod_power_of_2($pow);
        assert_eq!(Natural::from_owned_limbs_asc(xs), n);
    };
}

#[test]
fn limbs_slice_mod_power_of_2_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref xs, pow)| {
            limbs_slice_mod_power_of_2_in_place_helper!(
                limbs_slice_mod_power_of_2_in_place,
                xs,
                pow
            );
        },
    );
}

#[test]
fn limbs_vec_mod_power_of_2_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref xs, pow)| {
            limbs_slice_mod_power_of_2_in_place_helper!(limbs_vec_mod_power_of_2_in_place, xs, pow);
        },
    );
}

#[test]
fn limbs_neg_mod_power_of_2_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, pow)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_neg_mod_power_of_2(limbs, pow)),
                Natural::from_limbs_asc(limbs).neg_mod_power_of_2(pow),
            );
        },
    );
}

#[test]
fn limbs_neg_mod_power_of_2_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, pow)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_neg_mod_power_of_2_in_place(&mut limbs, pow);
            let n = Natural::from_limbs_asc(&old_limbs).neg_mod_power_of_2(pow);
            assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
        },
    );
}

#[test]
fn mod_power_of_2_and_rem_power_of_2_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n.mod_power_of_2_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;
        assert!(result.mod_power_of_2_is_reduced(u));

        let result_alt = n.mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = n.clone().mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let mut mut_n = n.clone();
        mut_n.rem_power_of_2_assign(u);
        assert!(mut_n.is_valid());
        let result_alt = mut_n;
        assert_eq!(result_alt, result);

        let result_alt = n.rem_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = n.clone().rem_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert!(result <= *n);
        assert_eq!((n >> u << u) + &result, *n);
        assert!(result < Natural::power_of_2(u));
        assert_eq!(result == 0, n.divisible_by_power_of_2(u));
        assert_eq!((&result).mod_power_of_2(u), result);
        assert_eq!(n & Natural::low_mask(u), result);
    });

    test_properties(
        triples_of_natural_natural_and_small_unsigned,
        |&(ref x, ref y, u)| {
            assert_eq!(
                (x + y).mod_power_of_2(u),
                (x.mod_power_of_2(u) + y.mod_power_of_2(u)).mod_power_of_2(u)
            );
            assert_eq!(
                (x * y).mod_power_of_2(u),
                (x.mod_power_of_2(u) * y.mod_power_of_2(u)).mod_power_of_2(u)
            );
        },
    );

    test_properties(pairs_of_natural_and_small_unsigned_var_1, |&(ref n, u)| {
        assert_eq!(n.mod_power_of_2(u), 0);
    });

    test_properties(pairs_of_natural_and_small_unsigned_var_2, |&(ref n, u)| {
        assert_ne!(n.mod_power_of_2(u), 0);
        assert_eq!(
            n.mod_power_of_2(u) + n.neg_mod_power_of_2(u),
            Natural::power_of_2(u)
        );
    });

    test_properties(
        triples_of_natural_small_unsigned_and_small_unsigned,
        |&(ref n, u, v)| {
            assert_eq!(
                n.mod_power_of_2(u).mod_power_of_2(v),
                n.mod_power_of_2(min(u, v))
            );
        },
    );

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<Limb, u64>,
        |&(u, pow)| {
            assert_eq!(u.mod_power_of_2(pow), Natural::from(u).mod_power_of_2(pow));
            assert_eq!(u.rem_power_of_2(pow), Natural::from(u).rem_power_of_2(pow));
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n.mod_power_of_2(0), 0);
    });

    test_properties(unsigneds, |&u| {
        assert_eq!(Natural::ZERO.mod_power_of_2(u), 0);
    });
}

#[test]
fn neg_mod_power_of_2_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n.neg_mod_power_of_2_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;
        assert!(result.mod_power_of_2_is_reduced(u));

        let result_alt = n.neg_mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = n.clone().neg_mod_power_of_2(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!((n.shr_round(u, RoundingMode::Ceiling) << u) - &result, *n);
        assert!(result < Natural::power_of_2(u));
        assert_eq!(result == 0, n.divisible_by_power_of_2(u));
        assert_eq!((&result).neg_mod_power_of_2(u), n.mod_power_of_2(u));
        assert_eq!((-n).mod_power_of_2(u), result);
    });

    test_properties(
        triples_of_natural_natural_and_small_unsigned,
        |&(ref x, ref y, u)| {
            assert_eq!(
                (x + y).neg_mod_power_of_2(u),
                (x.mod_power_of_2(u) + y.mod_power_of_2(u)).neg_mod_power_of_2(u)
            );
            assert_eq!(
                (x * y).neg_mod_power_of_2(u),
                (x.mod_power_of_2(u) * y.mod_power_of_2(u)).neg_mod_power_of_2(u)
            );
        },
    );

    test_properties(pairs_of_natural_and_small_unsigned_var_1, |&(ref n, u)| {
        assert_eq!(n.neg_mod_power_of_2(u), 0);
    });

    test_properties(pairs_of_natural_and_small_unsigned_var_2, |&(ref n, u)| {
        let m = n.neg_mod_power_of_2(u);
        assert_ne!(m, 0);
        assert_eq!((((n >> u) + Natural::ONE) << u) - &m, *n);
        assert_eq!(n.mod_power_of_2(u) + m, Natural::power_of_2(u));
    });

    test_properties(
        pairs_of_unsigned_and_small_u64_var_4::<Limb>,
        |&(u, pow)| {
            assert_eq!(
                u.neg_mod_power_of_2(pow),
                Natural::from(u).neg_mod_power_of_2(pow)
            );
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n.neg_mod_power_of_2(0), 0);
    });

    test_properties(unsigneds, |&u| {
        assert_eq!(Natural::ZERO.neg_mod_power_of_2(u), 0);
    });
}
