use malachite_base::num::arithmetic::traits::{
    CeilingSqrt, CeilingSqrtAssign, CheckedSqrt, FloorSqrt, FloorSqrtAssign, SqrtRem,
    SqrtRemAssign, Square,
};
use malachite_base::num::basic::traits::{NegativeOne, One};
use malachite_base_test_util::generators::signed_gen_var_2;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_nz_test_util::generators::{integer_gen_var_4, natural_gen};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_floor_sqrt() {
    let test = |s, out| {
        let n = Integer::from_str(s).unwrap();
        assert_eq!(n.clone().floor_sqrt().to_string(), out);
        assert_eq!((&n).floor_sqrt().to_string(), out);

        let mut n = n;
        n.floor_sqrt_assign();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("2", "1");
    test("3", "1");
    test("4", "2");
    test("5", "2");
    test("10", "3");
    test("100", "10");
    test("1000000000", "31622");
    test("152415765279683", "12345677");
    test("152415765279684", "12345678");
    test("152415765279685", "12345678");
    test(
        "10000000000000000000000000000000000000000",
        "100000000000000000000",
    );
    test(
        "100000000000000000000000000000000000000000",
        "316227766016837933199",
    );
}

#[test]
pub fn floor_sqrt_fail() {
    assert_panic!(Integer::NEGATIVE_ONE.floor_sqrt());
}

#[test]
fn test_ceiling_sqrt() {
    let test = |s, out| {
        let n = Integer::from_str(s).unwrap();
        assert_eq!(n.clone().ceiling_sqrt().to_string(), out);
        assert_eq!((&n).ceiling_sqrt().to_string(), out);

        let mut n = n;
        n.ceiling_sqrt_assign();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("2", "2");
    test("3", "2");
    test("4", "2");
    test("5", "3");
    test("10", "4");
    test("100", "10");
    test("1000000000", "31623");
    test("152415765279683", "12345678");
    test("152415765279684", "12345678");
    test("152415765279685", "12345679");
    test(
        "10000000000000000000000000000000000000000",
        "100000000000000000000",
    );
    test(
        "100000000000000000000000000000000000000000",
        "316227766016837933200",
    );
}

#[test]
pub fn ceiling_sqrt_fail() {
    assert_panic!(Integer::NEGATIVE_ONE.ceiling_sqrt());
}

#[allow(clippy::redundant_closure_for_method_calls)]
#[test]
fn test_checked_sqrt() {
    let test = |s, out: Option<&str>| {
        let n = Integer::from_str(s).unwrap();
        let out = out.map(|s| s.to_string());

        assert_eq!(n.clone().checked_sqrt().map(|x| x.to_string()), out);
        assert_eq!((&n).checked_sqrt().map(|x| x.to_string()), out);
    };
    test("0", Some("0"));
    test("1", Some("1"));
    test("2", None);
    test("3", None);
    test("4", Some("2"));
    test("5", None);
    test("10", None);
    test("100", Some("10"));
    test("1000000000", None);
    test("152415765279683", None);
    test("152415765279684", Some("12345678"));
    test("152415765279685", None);
    test(
        "10000000000000000000000000000000000000000",
        Some("100000000000000000000"),
    );
    test("100000000000000000000000000000000000000000", None);
}

#[test]
pub fn checked_sqrt_fail() {
    assert_panic!(Integer::NEGATIVE_ONE.checked_sqrt());
}

#[test]
fn test_sqrt_rem() {
    let test = |s, sqrt_out, rem_out| {
        let n = Integer::from_str(s).unwrap();

        let (sqrt, rem) = n.clone().sqrt_rem();
        assert_eq!(sqrt.to_string(), sqrt_out);
        assert_eq!(rem.to_string(), rem_out);

        let (sqrt, rem) = (&n).sqrt_rem();
        assert_eq!(sqrt.to_string(), sqrt_out);
        assert_eq!(rem.to_string(), rem_out);

        let mut n = n;
        assert_eq!(n.sqrt_rem_assign().to_string(), rem_out);
        assert_eq!(n.to_string(), sqrt_out);
    };
    test("0", "0", "0");
    test("1", "1", "0");
    test("2", "1", "1");
    test("3", "1", "2");
    test("4", "2", "0");
    test("5", "2", "1");
    test("10", "3", "1");
    test("100", "10", "0");
    test("1000000000", "31622", "49116");
    test("152415765279683", "12345677", "24691354");
    test("152415765279684", "12345678", "0");
    test("152415765279685", "12345678", "1");
    test(
        "10000000000000000000000000000000000000000",
        "100000000000000000000",
        "0",
    );
    test(
        "100000000000000000000000000000000000000000",
        "316227766016837933199",
        "562477137586013626399",
    );
}

#[test]
pub fn sqrt_rem_fail() {
    assert_panic!(Integer::NEGATIVE_ONE.sqrt_rem());
}

#[test]
fn floor_sqrt_properties() {
    integer_gen_var_4().test_properties(|n| {
        let sqrt = n.clone().floor_sqrt();
        assert_eq!((&n).floor_sqrt(), sqrt);
        let mut n_alt = n.clone();
        n_alt.floor_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        assert_eq!(bigint_to_integer(&integer_to_bigint(&n).sqrt()), sqrt);
        assert_eq!(
            rug_integer_to_integer(&integer_to_rug_integer(&n).sqrt()),
            sqrt
        );

        let square = (&sqrt).square();
        let ceiling_sqrt = (&n).ceiling_sqrt();
        if square == n {
            assert_eq!(ceiling_sqrt, sqrt);
        } else {
            assert_eq!(ceiling_sqrt, &sqrt + Integer::ONE);
        }
        assert!(square <= n);
        assert!((sqrt + Integer::ONE).square() > n);
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).floor_sqrt(), Integer::from(n).floor_sqrt());
    });

    signed_gen_var_2::<SignedLimb>().test_properties(|i| {
        assert_eq!(i.floor_sqrt(), Integer::from(i).floor_sqrt());
    });
}

#[test]
fn ceiling_sqrt_properties() {
    integer_gen_var_4().test_properties(|n| {
        let sqrt = n.clone().ceiling_sqrt();
        assert_eq!((&n).ceiling_sqrt(), sqrt);
        let mut n_alt = n.clone();
        n_alt.ceiling_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        let square = (&sqrt).square();
        let floor_sqrt = (&n).floor_sqrt();
        if square == n {
            assert_eq!(floor_sqrt, sqrt);
        } else {
            assert_eq!(floor_sqrt, &sqrt - Integer::ONE);
        }
        assert!(square >= n);
        if n != 0 {
            assert!((sqrt - Integer::ONE).square() < n);
        }
    });

    natural_gen().test_properties(|n| {
        assert_eq!((&n).ceiling_sqrt(), Integer::from(n).ceiling_sqrt());
    });

    signed_gen_var_2::<SignedLimb>().test_properties(|i| {
        assert_eq!(i.ceiling_sqrt(), Integer::from(i).ceiling_sqrt());
    });
}

#[test]
fn checked_sqrt_properties() {
    integer_gen_var_4().test_properties(|n| {
        let sqrt = n.clone().checked_sqrt();
        assert_eq!((&n).checked_sqrt(), sqrt);
        if let Some(sqrt) = sqrt {
            assert_eq!((&sqrt).square(), n);
            assert_eq!((&n).floor_sqrt(), sqrt);
            assert_eq!(n.ceiling_sqrt(), sqrt);
        }
    });

    natural_gen().test_properties(|n| {
        assert_eq!(
            (&n).checked_sqrt().map(Integer::from),
            Integer::from(n).checked_sqrt()
        );
    });

    signed_gen_var_2::<SignedLimb>().test_properties(|i| {
        assert_eq!(
            i.checked_sqrt().map(Integer::from),
            Integer::from(i).checked_sqrt()
        );
    });
}

#[test]
fn sqrt_rem_properties() {
    integer_gen_var_4().test_properties(|n| {
        let (sqrt, rem) = n.clone().sqrt_rem();
        assert_eq!((&n).sqrt_rem(), (sqrt.clone(), rem.clone()));
        let mut n_alt = n.clone();
        assert_eq!(n_alt.sqrt_rem_assign(), rem);
        assert_eq!(n_alt, sqrt);
        let (rug_sqrt, rug_rem) = integer_to_rug_integer(&n).sqrt_rem(rug::Integer::new());
        assert_eq!(rug_integer_to_integer(&rug_sqrt), sqrt);
        assert_eq!(rug_integer_to_integer(&rug_rem), rem);

        assert_eq!((&n).floor_sqrt(), sqrt);
        let rem = Integer::from(rem);
        assert!(rem <= &sqrt << 1);
        assert_eq!(sqrt.square() + rem, n);
    });

    natural_gen().test_properties(|n| {
        let (sqrt, rem) = (&n).sqrt_rem();
        assert_eq!((Integer::from(sqrt), rem), Integer::from(n).sqrt_rem());
    });

    signed_gen_var_2::<SignedLimb>().test_properties(|i| {
        let (sqrt, rem) = i.sqrt_rem();
        assert_eq!(
            (Integer::from(sqrt), Natural::from(rem)),
            Integer::from(i).sqrt_rem()
        );
    });
}
