use malachite_base::num::arithmetic::traits::{
    CeilingRoot, CeilingRootAssign, CheckedRoot, FloorRoot, FloorRootAssign, Parity, Pow,
};
use malachite_base::num::basic::traits::{NegativeOne, One};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::generators::signed_unsigned_pair_gen_var_18;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_nz_test_util::generators::{
    integer_unsigned_pair_gen_var_3, natural_unsigned_pair_gen_var_7,
};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_floor_root() {
    let test = |s, exp, out| {
        let n = Integer::from_str(s).unwrap();
        assert_eq!(n.clone().floor_root(exp).to_string(), out);
        assert_eq!((&n).floor_root(exp).to_string(), out);

        let mut n = n;
        n.floor_root_assign(exp);
        assert_eq!(n.to_string(), out);
    };
    test("0", 1, "0");
    test("1", 1, "1");
    test("2", 1, "2");
    test("100", 1, "100");

    test("0", 2, "0");
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

    test("-1", 1, "-1");
    test("-2", 1, "-2");
    test("-100", 1, "-100");

    test("-1", 3, "-1");
    test("-2", 3, "-2");
    test("-7", 3, "-2");
    test("-8", 3, "-2");
    test("-9", 3, "-3");
    test("-100", 3, "-5");
    test("-1000000000", 3, "-1000");
    test("-1000000000", 5, "-64");
    test("-1000000000", 7, "-20");
    test("-1000000000", 9, "-10");
}

#[test]
fn floor_root_fail() {
    assert_panic!(Integer::ONE.floor_root(0));
    assert_panic!(Integer::NEGATIVE_ONE.floor_root(0));
    assert_panic!(Integer::NEGATIVE_ONE.floor_root(2));
    assert_panic!(Integer::NEGATIVE_ONE.floor_root(4));
    assert_panic!(Integer::NEGATIVE_ONE.floor_root(100));
}

#[test]
fn test_ceiling_root() {
    let test = |s, exp, out| {
        let n = Integer::from_str(s).unwrap();
        assert_eq!(n.clone().ceiling_root(exp).to_string(), out);
        assert_eq!((&n).ceiling_root(exp).to_string(), out);

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

    test("-1", 1, "-1");
    test("-2", 1, "-2");
    test("-100", 1, "-100");

    test("-1", 3, "-1");
    test("-2", 3, "-1");
    test("-7", 3, "-1");
    test("-8", 3, "-2");
    test("-9", 3, "-2");
    test("-100", 3, "-4");
    test("-1000000000", 3, "-1000");
    test("-1000000000", 5, "-63");
    test("-1000000000", 7, "-19");
    test("-1000000000", 9, "-10");
}

#[test]
fn ceiling_root_fail() {
    assert_panic!(Integer::ONE.ceiling_root(0));
    assert_panic!(Integer::NEGATIVE_ONE.ceiling_root(0));
    assert_panic!(Integer::NEGATIVE_ONE.ceiling_root(2));
    assert_panic!(Integer::NEGATIVE_ONE.ceiling_root(4));
    assert_panic!(Integer::NEGATIVE_ONE.ceiling_root(100));
}

#[allow(clippy::redundant_closure_for_method_calls)]
#[test]
fn test_checked_root() {
    let test = |s, exp, out: Option<&str>| {
        let n = Integer::from_str(s).unwrap();
        let out = out.map(|s| s.to_string());

        assert_eq!(n.clone().checked_root(exp).map(|x| x.to_string()), out);
        assert_eq!((&n).checked_root(exp).map(|x| x.to_string()), out);
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

    test("-1", 1, Some("-1"));
    test("-2", 1, Some("-2"));
    test("-100", 1, Some("-100"));

    test("-1", 3, Some("-1"));
    test("-2", 3, None);
    test("-7", 3, None);
    test("-8", 3, Some("-2"));
    test("-9", 3, None);
    test("-100", 3, None);
    test("-1000000000", 3, Some("-1000"));
    test("-1000000000", 5, None);
    test("-1000000000", 7, None);
    test("-1000000000", 9, Some("-10"));
}

#[test]
fn checked_root_fail() {
    assert_panic!(Integer::ONE.checked_root(0));
    assert_panic!(Integer::NEGATIVE_ONE.checked_root(0));
    assert_panic!(Integer::NEGATIVE_ONE.checked_root(2));
    assert_panic!(Integer::NEGATIVE_ONE.checked_root(4));
    assert_panic!(Integer::NEGATIVE_ONE.checked_root(100));
}

#[test]
fn floor_root_properties() {
    integer_unsigned_pair_gen_var_3().test_properties(|(n, exp)| {
        let root = n.clone().floor_root(exp);
        assert_eq!((&n).floor_root(exp), root);
        let mut n_alt = n.clone();
        n_alt.floor_root_assign(exp);
        assert_eq!(n_alt, root);
        if n >= 0 {
            assert_eq!(
                bigint_to_integer(&integer_to_bigint(&n).nth_root(u32::exact_from(exp))),
                root
            );
            assert_eq!(
                rug_integer_to_integer(&integer_to_rug_integer(&n).root(u32::exact_from(exp))),
                root
            );
        }

        let pow = (&root).pow(exp);
        let ceiling_root = (&n).ceiling_root(exp);
        if pow == n {
            assert_eq!(ceiling_root, root);
        } else {
            assert_eq!(ceiling_root, &root + Integer::ONE);
        }
        assert!(pow <= n);
        assert!((&root + Integer::ONE).pow(exp) > n);
        if exp.odd() {
            assert_eq!(-(-n).ceiling_root(exp), root);
        }
    });

    natural_unsigned_pair_gen_var_7().test_properties(|(n, exp)| {
        assert_eq!((&n).floor_root(exp), Integer::from(n).floor_root(exp));
    });

    signed_unsigned_pair_gen_var_18::<SignedLimb, u64>().test_properties(|(i, exp)| {
        assert_eq!(i.floor_root(exp), Integer::from(i).floor_root(exp));
    });
}

#[test]
fn ceiling_root_properties() {
    integer_unsigned_pair_gen_var_3().test_properties(|(n, exp)| {
        let root = n.clone().ceiling_root(exp);
        assert_eq!((&n).ceiling_root(exp), root);
        let mut n_alt = n.clone();
        n_alt.ceiling_root_assign(exp);
        assert_eq!(n_alt, root);
        if n < 0 {
            assert_eq!(
                bigint_to_integer(&integer_to_bigint(&n).nth_root(u32::exact_from(exp))),
                root
            );
            assert_eq!(
                rug_integer_to_integer(&integer_to_rug_integer(&n).root(u32::exact_from(exp))),
                root
            );
        }
        let pow = (&root).pow(exp);
        let floor_root = (&n).floor_root(exp);
        if pow == n {
            assert_eq!(floor_root, root);
        } else {
            assert_eq!(floor_root, &root - Integer::ONE);
        }
        assert!(pow >= n);
        if n != 0 {
            assert!((&root - Integer::ONE).pow(exp) < n);
        }
        if exp.odd() {
            assert_eq!(-(-n).floor_root(exp), root);
        }
    });

    natural_unsigned_pair_gen_var_7().test_properties(|(n, exp)| {
        assert_eq!((&n).ceiling_root(exp), Integer::from(n).ceiling_root(exp));
    });

    signed_unsigned_pair_gen_var_18::<SignedLimb, u64>().test_properties(|(i, exp)| {
        assert_eq!(i.ceiling_root(exp), Integer::from(i).ceiling_root(exp));
    });
}

#[test]
fn checked_root_properties() {
    integer_unsigned_pair_gen_var_3().test_properties(|(n, exp)| {
        let root = n.clone().checked_root(exp);
        assert_eq!((&n).checked_root(exp), root);
        if let Some(root) = root {
            assert_eq!((&root).pow(exp), n);
            assert_eq!((&n).floor_root(exp), root);
            assert_eq!(n.ceiling_root(exp), root);
        }
    });

    natural_unsigned_pair_gen_var_7().test_properties(|(n, exp)| {
        assert_eq!(
            (&n).checked_root(exp).map(Integer::from),
            Integer::from(n).checked_root(exp)
        );
    });

    signed_unsigned_pair_gen_var_18::<SignedLimb, u64>().test_properties(|(i, exp)| {
        assert_eq!(
            i.checked_root(exp).map(Integer::from),
            Integer::from(i).checked_root(exp)
        );
    });
}
