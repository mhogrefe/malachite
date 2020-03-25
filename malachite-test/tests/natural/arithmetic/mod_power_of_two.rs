use std::cmp::min;
use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOfTwo, ModPowerOfTwo, ModPowerOfTwoAssign, ModPowerOfTwoIsReduced,
    NegModPowerOfTwo, NegModPowerOfTwoAssign, PowerOfTwo, RemPowerOfTwo, RemPowerOfTwoAssign,
    ShrRound,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::LowMask;
use malachite_base::round::RoundingMode;
use malachite_nz::natural::arithmetic::mod_power_of_two::{
    limbs_mod_power_of_two, limbs_mod_power_of_two_in_place, limbs_neg_mod_power_of_two,
    limbs_neg_mod_power_of_two_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned, unsigneds,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_unsigned, pairs_of_natural_and_small_unsigned_var_1,
    pairs_of_natural_and_small_unsigned_var_2, triples_of_natural_natural_and_small_unsigned,
    triples_of_natural_small_unsigned_and_small_unsigned,
};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_two_and_limbs_mod_power_of_two_in_place() {
    let test = |limbs: &[Limb], pow: u64, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_two(limbs, pow), out);

        let mut limbs = limbs.to_vec();
        limbs_mod_power_of_two_in_place(&mut limbs, pow);
        assert_eq!(limbs, out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[]);
    test(&[], 100, &[]);
    test(&[6, 7], 2, &[2]);
    test(&[100, 101, 102], 10, &[100]);
    test(&[123, 456], 0, &[]);
    test(&[123, 456], 1, &[1]);
    test(&[123, 456], 10, &[123]);
    test(&[123, 456], 33, &[123, 0]);
    test(&[123, 456], 40, &[123, 200]);
    test(&[123, 456], 100, &[123, 456]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_mod_power_of_two_and_limbs_neg_mod_power_of_two_in_place() {
    let test = |limbs: &[Limb], pow: u64, out: &[Limb]| {
        assert_eq!(limbs_neg_mod_power_of_two(limbs, pow), out);

        let mut limbs = limbs.to_vec();
        limbs_neg_mod_power_of_two_in_place(&mut limbs, pow);
        assert_eq!(limbs, out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[0]);
    test(&[], 100, &[0, 0, 0, 0]);
    test(&[6, 7], 2, &[2]);
    test(&[100, 101, 102], 10, &[924]);
    test(&[123, 456], 0, &[]);
    test(&[123, 456], 1, &[1]);
    test(&[123, 456], 10, &[901]);
    test(&[123, 456], 33, &[4_294_967_173, 1]);
    test(&[123, 456], 40, &[4_294_967_173, 55]);
    test(
        &[123, 456],
        100,
        &[4_294_967_173, 4_294_966_839, 4_294_967_295, 15],
    );
}

#[test]
fn test_mod_power_of_two_and_rem_power_of_two() {
    let test = |u, v: u64, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_two_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_power_of_two_is_reduced(v));

        let n = Natural::from_str(u).unwrap().mod_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n.rem_power_of_two_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().rem_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).rem_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 0, "0");
    test("260", 8, "4");
    test("1611", 4, "11");
    test("123", 100, "123");
    test("1000000000000", 0, "0");
    test("1000000000000", 12, "0");
    test("1000000000001", 12, "1");
    test("999999999999", 12, "4095");
    test("1000000000000", 15, "4096");
    test("1000000000000", 100, "1000000000000");
    test("1000000000000000000000000", 40, "1020608380928");
    test("1000000000000000000000000", 64, "2003764205206896640");
    test("4294967295", 31, "2147483647");
    test("4294967295", 32, "4294967295");
    test("4294967295", 33, "4294967295");
    test("4294967296", 31, "0");
    test("4294967296", 32, "0");
    test("4294967296", 33, "4294967296");
    test("4294967297", 31, "1");
    test("4294967297", 32, "1");
    test("4294967297", 33, "4294967297");
}

#[test]
fn test_neg_mod_power_of_two() {
    let test = |u, v: u64, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.neg_mod_power_of_two_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_power_of_two_is_reduced(v));

        let n = Natural::from_str(u).unwrap().neg_mod_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).neg_mod_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };

    test("0", 0, "0");
    test("260", 8, "252");
    test("1611", 4, "5");
    test("123", 100, "1267650600228229401496703205253");
    test("1000000000000", 0, "0");
    test("1000000000000", 12, "0");
    test("1000000000001", 12, "4095");
    test("999999999999", 12, "1");
    test("1000000000000", 15, "28672");
    test("1000000000000", 100, "1267650600228229400496703205376");
    test("1000000000000000000000000", 40, "78903246848");
    test("1000000000000000000000000", 64, "16442979868502654976");
    test("4294967295", 31, "1");
    test("4294967295", 32, "1");
    test("4294967295", 33, "4294967297");
    test("4294967296", 31, "0");
    test("4294967296", 32, "0");
    test("4294967296", 33, "4294967296");
    test("4294967297", 31, "2147483647");
    test("4294967297", 32, "4294967295");
    test("4294967297", 33, "4294967295");
}

#[test]
fn limbs_mod_power_of_two_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, pow)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_two(limbs, pow)),
                Natural::from_limbs_asc(limbs).mod_power_of_two(pow),
            );
        },
    );
}

#[test]
fn limbs_mod_power_of_two_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, pow)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_mod_power_of_two_in_place(&mut limbs, pow);
            let n = Natural::from_limbs_asc(&old_limbs).mod_power_of_two(pow);
            assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
        },
    );
}

#[test]
fn limbs_neg_mod_power_of_two_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, pow)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_neg_mod_power_of_two(limbs, pow)),
                Natural::from_limbs_asc(limbs).neg_mod_power_of_two(pow),
            );
        },
    );
}

#[test]
fn limbs_neg_mod_power_of_two_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, pow)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_neg_mod_power_of_two_in_place(&mut limbs, pow);
            let n = Natural::from_limbs_asc(&old_limbs).neg_mod_power_of_two(pow);
            assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
        },
    );
}

#[test]
fn mod_power_of_two_and_rem_power_of_two_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n.mod_power_of_two_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;
        assert!(result.mod_power_of_two_is_reduced(u));

        let result_alt = n.mod_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = n.clone().mod_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let mut mut_n = n.clone();
        mut_n.rem_power_of_two_assign(u);
        assert!(mut_n.is_valid());
        let result_alt = mut_n;
        assert_eq!(result_alt, result);

        let result_alt = n.rem_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = n.clone().rem_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!((n >> u << u) + &result, *n);
        assert!(result < Natural::power_of_two(u));
        assert_eq!(result == 0, n.divisible_by_power_of_two(u));
        assert_eq!((&result).mod_power_of_two(u), result);
        assert_eq!(n & Natural::low_mask(u), result);
    });

    test_properties(
        triples_of_natural_natural_and_small_unsigned,
        |&(ref x, ref y, u)| {
            assert_eq!(
                (x + y).mod_power_of_two(u),
                (x.mod_power_of_two(u) + y.mod_power_of_two(u)).mod_power_of_two(u)
            );
            assert_eq!(
                (x * y).mod_power_of_two(u),
                (x.mod_power_of_two(u) * y.mod_power_of_two(u)).mod_power_of_two(u)
            );
        },
    );

    test_properties(pairs_of_natural_and_small_unsigned_var_1, |&(ref n, u)| {
        assert_eq!(n.mod_power_of_two(u), 0);
    });

    test_properties(pairs_of_natural_and_small_unsigned_var_2, |&(ref n, u)| {
        assert_ne!(n.mod_power_of_two(u), 0);
        assert_eq!(
            n.mod_power_of_two(u) + n.neg_mod_power_of_two(u),
            Natural::power_of_two(u)
        );
    });

    test_properties(
        triples_of_natural_small_unsigned_and_small_unsigned,
        |&(ref n, u, v)| {
            assert_eq!(
                n.mod_power_of_two(u).mod_power_of_two(v),
                n.mod_power_of_two(min(u, v))
            );
        },
    );

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<Limb, u64>,
        |&(u, pow)| {
            assert_eq!(
                u.mod_power_of_two(pow),
                Natural::from(u).mod_power_of_two(pow)
            );
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n.mod_power_of_two(0), 0);
    });

    test_properties(unsigneds, |&u| {
        assert_eq!(Natural::ZERO.mod_power_of_two(u), 0);
    });
}

#[test]
fn neg_mod_power_of_two_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n.neg_mod_power_of_two_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;
        assert!(result.mod_power_of_two_is_reduced(u));

        let result_alt = n.neg_mod_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = n.clone().neg_mod_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!((n.shr_round(u, RoundingMode::Ceiling) << u) - &result, *n);
        assert!(result < Natural::power_of_two(u));
        assert_eq!(result == 0, n.divisible_by_power_of_two(u));
        assert_eq!((&result).neg_mod_power_of_two(u), n.mod_power_of_two(u));
        assert_eq!((-n).mod_power_of_two(u), result);
    });

    test_properties(
        triples_of_natural_natural_and_small_unsigned,
        |&(ref x, ref y, u)| {
            assert_eq!(
                (x + y).neg_mod_power_of_two(u),
                (x.mod_power_of_two(u) + y.mod_power_of_two(u)).neg_mod_power_of_two(u)
            );
            assert_eq!(
                (x * y).neg_mod_power_of_two(u),
                (x.mod_power_of_two(u) * y.mod_power_of_two(u)).neg_mod_power_of_two(u)
            );
        },
    );

    test_properties(pairs_of_natural_and_small_unsigned_var_1, |&(ref n, u)| {
        assert_eq!(n.neg_mod_power_of_two(u), 0);
    });

    test_properties(pairs_of_natural_and_small_unsigned_var_2, |&(ref n, u)| {
        let m = n.neg_mod_power_of_two(u);
        assert_ne!(m, 0);
        assert_eq!((((n >> u) + Natural::ONE) << u) - &m, *n);
        assert_eq!(n.mod_power_of_two(u) + m, Natural::power_of_two(u));
    });

    test_properties(naturals, |n| {
        assert_eq!(n.neg_mod_power_of_two(0), 0);
    });

    test_properties(unsigneds, |&u| {
        assert_eq!(Natural::ZERO.neg_mod_power_of_two(u), 0);
    });
}
