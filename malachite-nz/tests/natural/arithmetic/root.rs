use malachite_base::num::arithmetic::traits::{
    CeilingRoot, CeilingRootAssign, CeilingSqrt, CheckedRoot, CheckedSqrt, FloorRoot,
    FloorRootAssign, FloorSqrt, Pow, RootAssignRem, RootRem, SqrtRem,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::generators::unsigned_pair_gen_var_32;
use malachite_nz::natural::arithmetic::root::{
    _ceiling_root_binary, _checked_root_binary, _floor_root_binary, _root_rem_binary,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_nz_test_util::generators::{natural_gen, natural_unsigned_pair_gen_var_7};
use std::str::FromStr;

#[test]
fn test_floor_root() {
    let test = |s, exp, out| {
        let n = Natural::from_str(s).unwrap();
        assert_eq!(n.clone().floor_root(exp).to_string(), out);
        assert_eq!((&n).floor_root(exp).to_string(), out);
        assert_eq!(_floor_root_binary(&n, exp).to_string(), out);

        let mut n = n;
        n.floor_root_assign(exp);
        assert_eq!(n.to_string(), out);
    };
    test("0", 1, "0");
    test("1", 1, "1");
    test("2", 1, "2");
    test("100", 1, "100");

    test("0", 2, "0");
    test("1", 2, "1");
    test("2", 2, "1");
    test("3", 2, "1");
    test("4", 2, "2");
    test("5", 2, "2");
    test("0", 3, "0");
    test("1", 3, "1");
    test("2", 3, "1");
    test("7", 3, "1");
    test("8", 3, "2");
    test("9", 3, "2");
    test("10", 2, "3");
    test("100", 2, "10");
    test("100", 3, "4");
    test("1000000000", 2, "31622");
    test("1000000000", 3, "1000");
    test("1000000000", 4, "177");
    test("1000000000", 5, "63");
    test("1000000000", 6, "31");
    test("1000000000", 7, "19");
    test("1000000000", 8, "13");
    test("1000000000", 9, "10");
    test("1000000000", 10, "7");
}

#[test]
#[should_panic]
fn floor_root_fail() {
    Natural::ONE.floor_root(0);
}

#[test]
fn test_ceiling_root() {
    let test = |s, exp, out| {
        let n = Natural::from_str(s).unwrap();
        assert_eq!(n.clone().ceiling_root(exp).to_string(), out);
        assert_eq!((&n).ceiling_root(exp).to_string(), out);
        assert_eq!(_ceiling_root_binary(&n, exp).to_string(), out);

        let mut n = n;
        n.ceiling_root_assign(exp);
        assert_eq!(n.to_string(), out);
    };
    test("0", 1, "0");
    test("1", 1, "1");
    test("2", 1, "2");
    test("100", 1, "100");

    test("0", 2, "0");
    test("1", 2, "1");
    test("2", 2, "2");
    test("3", 2, "2");
    test("4", 2, "2");
    test("5", 2, "3");
    test("0", 3, "0");
    test("1", 3, "1");
    test("2", 3, "2");
    test("7", 3, "2");
    test("8", 3, "2");
    test("9", 3, "3");
    test("10", 2, "4");
    test("100", 2, "10");
    test("100", 3, "5");
    test("1000000000", 2, "31623");
    test("1000000000", 3, "1000");
    test("1000000000", 4, "178");
    test("1000000000", 5, "64");
    test("1000000000", 6, "32");
    test("1000000000", 7, "20");
    test("1000000000", 8, "14");
    test("1000000000", 9, "10");
    test("1000000000", 10, "8");
}

#[test]
#[should_panic]
fn ceiling_root_fail() {
    Natural::ONE.ceiling_root(0);
}

#[allow(clippy::redundant_closure_for_method_calls)]
#[test]
fn test_checked_root() {
    let test = |s, exp, out: Option<&str>| {
        let n = Natural::from_str(s).unwrap();
        let out = out.map(|s| s.to_string());

        assert_eq!(n.clone().checked_root(exp).map(|x| x.to_string()), out);
        assert_eq!((&n).checked_root(exp).map(|x| x.to_string()), out);
        assert_eq!(_checked_root_binary(&n, exp).map(|x| x.to_string()), out);
    };
    test("0", 1, Some("0"));
    test("1", 1, Some("1"));
    test("2", 1, Some("2"));
    test("100", 1, Some("100"));

    test("0", 2, Some("0"));
    test("1", 2, Some("1"));
    test("2", 2, None);
    test("3", 2, None);
    test("4", 2, Some("2"));
    test("5", 2, None);
    test("0", 3, Some("0"));
    test("1", 3, Some("1"));
    test("2", 3, None);
    test("7", 3, None);
    test("8", 3, Some("2"));
    test("9", 3, None);
    test("10", 2, None);
    test("100", 2, Some("10"));
    test("100", 3, None);
    test("1000000000", 2, None);
    test("1000000000", 3, Some("1000"));
    test("1000000000", 4, None);
    test("1000000000", 5, None);
    test("1000000000", 6, None);
    test("1000000000", 7, None);
    test("1000000000", 8, None);
    test("1000000000", 9, Some("10"));
    test("1000000000", 10, None);
}

#[test]
#[should_panic]
fn checked_root_fail() {
    Natural::ONE.checked_root(0);
}

#[test]
fn test_root_rem() {
    let test = |s, exp, root_out, rem_out| {
        let n = Natural::from_str(s).unwrap();

        let (root, rem) = n.clone().root_rem(exp);
        assert_eq!(root.to_string(), root_out);
        assert_eq!(rem.to_string(), rem_out);

        let (root, rem) = (&n).root_rem(exp);
        assert_eq!(root.to_string(), root_out);
        assert_eq!(rem.to_string(), rem_out);

        let (root, rem) = _root_rem_binary(&n, exp);
        assert_eq!(root.to_string(), root_out);
        assert_eq!(rem.to_string(), rem_out);

        let mut n = n;
        assert_eq!(n.root_assign_rem(exp).to_string(), rem_out);
        assert_eq!(n.to_string(), root_out);
    };
    test("0", 1, "0", "0");
    test("1", 1, "1", "0");
    test("2", 1, "2", "0");
    test("100", 1, "100", "0");

    test("0", 2, "0", "0");
    test("1", 2, "1", "0");
    test("2", 2, "1", "1");
    test("3", 2, "1", "2");
    test("4", 2, "2", "0");
    test("5", 2, "2", "1");
    test("0", 3, "0", "0");
    test("1", 3, "1", "0");
    test("2", 3, "1", "1");
    test("7", 3, "1", "6");
    test("8", 3, "2", "0");
    test("9", 3, "2", "1");
    test("10", 2, "3", "1");
    test("100", 2, "10", "0");
    test("100", 3, "4", "36");
    test("1000000000", 2, "31622", "49116");
    test("1000000000", 3, "1000", "0");
    test("1000000000", 4, "177", "18493759");
    test("1000000000", 5, "63", "7563457");
    test("1000000000", 6, "31", "112496319");
    test("1000000000", 7, "19", "106128261");
    test("1000000000", 8, "13", "184269279");
    test("1000000000", 9, "10", "0");
    test("1000000000", 10, "7", "717524751");
}

#[test]
#[should_panic]
fn root_rem_fail() {
    Natural::ONE.root_rem(0);
}

#[test]
fn floor_root_properties() {
    natural_unsigned_pair_gen_var_7().test_properties(|(n, exp)| {
        let root = n.clone().floor_root(exp);
        assert_eq!((&n).floor_root(exp), root);
        let mut n_alt = n.clone();
        n_alt.floor_root_assign(exp);
        assert_eq!(n_alt, root);
        assert_eq!(_floor_root_binary(&n, exp), root);
        assert_eq!(
            biguint_to_natural(&natural_to_biguint(&n).nth_root(u32::exact_from(exp))),
            root
        );
        assert_eq!(
            rug_integer_to_natural(&natural_to_rug_integer(&n).root(u32::exact_from(exp))),
            root
        );

        let pow = (&root).pow(exp);
        let ceiling_root = (&n).ceiling_root(exp);
        if pow == n {
            assert_eq!(ceiling_root, root);
        } else {
            assert_eq!(ceiling_root, &root + Natural::ONE);
        }
        assert!(pow <= n);
        assert!((root + Natural::ONE).pow(exp) > n);
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).floor_root(2), (&n).floor_sqrt());
        assert_eq!((&n).floor_root(1), n);
    });

    unsigned_pair_gen_var_32::<Limb, u64>().test_properties(|(u, exp)| {
        assert_eq!(u.floor_root(exp), Natural::from(u).floor_root(exp));
    });
}

#[test]
fn ceiling_root_properties() {
    natural_unsigned_pair_gen_var_7().test_properties(|(n, exp)| {
        let root = n.clone().ceiling_root(exp);
        assert_eq!((&n).ceiling_root(exp), root);
        let mut n_alt = n.clone();
        n_alt.ceiling_root_assign(exp);
        assert_eq!(n_alt, root);
        assert_eq!(_ceiling_root_binary(&n, exp), root);
        let pow = (&root).pow(exp);
        let floor_root = (&n).floor_root(exp);
        if pow == n {
            assert_eq!(floor_root, root);
        } else {
            assert_eq!(floor_root, &root - Natural::ONE);
        }
        assert!(pow >= n);
        if n != 0 {
            assert!((root - Natural::ONE).pow(exp) < n);
        }
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).ceiling_root(2), (&n).ceiling_sqrt());
        assert_eq!((&n).ceiling_root(1), n);
    });

    unsigned_pair_gen_var_32::<Limb, u64>().test_properties(|(u, exp)| {
        assert_eq!(u.ceiling_root(exp), Natural::from(u).ceiling_root(exp));
    });
}

#[test]
fn checked_root_properties() {
    natural_unsigned_pair_gen_var_7().test_properties(|(n, exp)| {
        let root = n.clone().checked_root(exp);
        assert_eq!((&n).checked_root(exp), root);
        assert_eq!(_checked_root_binary(&n, exp), root);
        if let Some(root) = root {
            assert_eq!((&root).pow(exp), n);
            assert_eq!((&n).floor_root(exp), root);
            assert_eq!(n.ceiling_root(exp), root);
        }
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).checked_root(2), (&n).checked_sqrt());
        assert_eq!((&n).checked_root(1), Some(n));
    });

    unsigned_pair_gen_var_32::<Limb, u64>().test_properties(|(u, exp)| {
        assert_eq!(
            u.checked_root(exp).map(Natural::from),
            Natural::from(u).checked_root(exp)
        );
    });
}

#[test]
fn root_rem_properties() {
    natural_unsigned_pair_gen_var_7().test_properties(|(n, exp)| {
        let (root, rem) = n.clone().root_rem(exp);
        assert_eq!((&n).root_rem(exp), (root.clone(), rem.clone()));
        let mut n_alt = n.clone();
        assert_eq!(n_alt.root_assign_rem(exp), rem);
        assert_eq!(n_alt, root);
        assert_eq!(_root_rem_binary(&n, exp), (root.clone(), rem.clone()));
        let (rug_root, rug_rem) =
            natural_to_rug_integer(&n).root_rem(rug::Integer::new(), u32::exact_from(exp));
        assert_eq!(rug_integer_to_natural(&rug_root), root);
        assert_eq!(rug_integer_to_natural(&rug_rem), rem);

        assert_eq!((&n).floor_root(exp), root);
        assert_eq!(root.pow(exp) + rem, n);
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).root_rem(2), (&n).sqrt_rem());
        assert_eq!((&n).root_rem(1), (n, Natural::ZERO));
    });

    unsigned_pair_gen_var_32::<Limb, u64>().test_properties(|(u, exp)| {
        let (root, rem) = u.root_rem(exp);
        assert_eq!(
            (Natural::from(root), Natural::from(rem)),
            Natural::from(u).root_rem(exp)
        );
    });
}
