use common::test_properties;
use malachite_base::num::{One, ShrRound, Zero};
use malachite_base::round::RoundingMode;
use malachite_nz::natural::Natural;
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_u32, pairs_of_natural_and_small_u32_var_1,
    pairs_of_natural_and_small_u32_var_2, triples_of_natural_small_u32_and_small_u32,
};
use std::cmp::min;
use std::str::FromStr;

#[test]
fn test_mod_power_of_two() {
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_two_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().mod_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().mod_power_of_two_ref(v);
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
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.neg_mod_power_of_two_assign(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().neg_mod_power_of_two(v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().neg_mod_power_of_two_ref(v);
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
fn mod_power_of_two_properties() {
    test_properties(pairs_of_natural_and_small_u32, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n.mod_power_of_two_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let result_alt = n.mod_power_of_two_ref(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = n.clone().mod_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!((n >> u << u) + &result, *n);
        assert!(result < (Natural::ONE << u));
        assert_eq!(result == 0, n.divisible_by_power_of_two(u));
        assert_eq!(result.mod_power_of_two_ref(u), result);
    });

    test_properties(pairs_of_natural_and_small_u32_var_1, |&(ref n, u)| {
        assert_eq!(n.mod_power_of_two_ref(u), 0);
    });

    test_properties(pairs_of_natural_and_small_u32_var_2, |&(ref n, u)| {
        assert_ne!(n.mod_power_of_two_ref(u), 0);
        assert_eq!(
            n.mod_power_of_two_ref(u) + n.neg_mod_power_of_two_ref(u),
            Natural::ONE << u
        );
    });

    test_properties(
        triples_of_natural_small_u32_and_small_u32,
        |&(ref n, u, v)| {
            assert_eq!(
                n.mod_power_of_two_ref(u).mod_power_of_two(v),
                n.mod_power_of_two_ref(min(u, v))
            );
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n.mod_power_of_two_ref(0), 0);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(Natural::ZERO.mod_power_of_two(u), 0);
    });
}

#[test]
fn neg_mod_power_of_two_properties() {
    test_properties(pairs_of_natural_and_small_u32, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n.neg_mod_power_of_two_assign(u);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let result_alt = n.neg_mod_power_of_two_ref(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = n.clone().neg_mod_power_of_two(u);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(
            ((n.shr_round(u, RoundingMode::Ceiling) << u) - &result).as_ref(),
            Some(n)
        );
        assert!(result < (Natural::ONE << u));
        assert_eq!(result == 0, n.divisible_by_power_of_two(u));
        assert_eq!(
            result.neg_mod_power_of_two_ref(u),
            n.mod_power_of_two_ref(u)
        );
        assert_eq!((-n).mod_power_of_two(u), result);
    });

    test_properties(pairs_of_natural_and_small_u32_var_1, |&(ref n, u)| {
        assert_eq!(n.neg_mod_power_of_two_ref(u), 0);
    });

    test_properties(pairs_of_natural_and_small_u32_var_2, |&(ref n, u)| {
        let m = n.neg_mod_power_of_two_ref(u);
        assert_ne!(m, 0);
        assert_eq!(((((n >> u) + 1) << u) - &m), Some(n.clone()));
        assert_eq!(n.mod_power_of_two_ref(u) + m, Natural::ONE << u);
    });

    test_properties(naturals, |n| {
        assert_eq!(n.neg_mod_power_of_two_ref(0), 0);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(Natural::ZERO.neg_mod_power_of_two(u), 0);
    });
}
