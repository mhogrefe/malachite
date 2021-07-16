use malachite_base::num::arithmetic::traits::{
    CeilingSqrt, CeilingSqrtAssign, CheckedSqrt, FloorSqrt, FloorSqrtAssign, SqrtRem,
    SqrtRemAssign, Square,
};
use malachite_base::num::basic::traits::One;
use malachite_base_test_util::generators::unsigned_gen;
use malachite_nz::natural::arithmetic::sqrt::{
    _ceiling_sqrt_binary, _checked_sqrt_binary, _floor_sqrt_binary, _sqrt_rem_binary,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_nz_test_util::generators::natural_gen;
use std::str::FromStr;

#[test]
fn test_floor_sqrt() {
    let test = |s, out| {
        let n = Natural::from_str(s).unwrap();
        assert_eq!(n.clone().floor_sqrt().to_string(), out);
        assert_eq!((&n).floor_sqrt().to_string(), out);
        assert_eq!(_floor_sqrt_binary(&n).to_string(), out);

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
fn test_ceiling_sqrt() {
    let test = |s, out| {
        let n = Natural::from_str(s).unwrap();
        assert_eq!(n.clone().ceiling_sqrt().to_string(), out);
        assert_eq!((&n).ceiling_sqrt().to_string(), out);
        assert_eq!(_ceiling_sqrt_binary(&n).to_string(), out);

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

#[allow(clippy::redundant_closure_for_method_calls)]
#[test]
fn test_checked_sqrt() {
    let test = |s, out: Option<&str>| {
        let n = Natural::from_str(s).unwrap();
        let out = out.map(|s| s.to_string());

        assert_eq!(n.clone().checked_sqrt().map(|x| x.to_string()), out);
        assert_eq!((&n).checked_sqrt().map(|x| x.to_string()), out);
        assert_eq!(_checked_sqrt_binary(&n).map(|x| x.to_string()), out);
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
fn test_sqrt_rem() {
    let test = |s, sqrt_out, rem_out| {
        let n = Natural::from_str(s).unwrap();

        let (sqrt, rem) = n.clone().sqrt_rem();
        assert_eq!(sqrt.to_string(), sqrt_out);
        assert_eq!(rem.to_string(), rem_out);

        let (sqrt, rem) = (&n).sqrt_rem();
        assert_eq!(sqrt.to_string(), sqrt_out);
        assert_eq!(rem.to_string(), rem_out);

        let (sqrt, rem) = _sqrt_rem_binary(&n);
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
fn floor_sqrt_properties() {
    natural_gen().test_properties(|n| {
        let sqrt = n.clone().floor_sqrt();
        assert_eq!((&n).floor_sqrt(), sqrt);
        let mut n_alt = n.clone();
        n_alt.floor_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        assert_eq!(_floor_sqrt_binary(&n), sqrt);
        assert_eq!(biguint_to_natural(&natural_to_biguint(&n).sqrt()), sqrt);
        assert_eq!(
            rug_integer_to_natural(&natural_to_rug_integer(&n).sqrt()),
            sqrt
        );

        let square = (&sqrt).square();
        let ceiling_sqrt = (&n).ceiling_sqrt();
        if square == n {
            assert_eq!(ceiling_sqrt, sqrt);
        } else {
            assert_eq!(ceiling_sqrt, &sqrt + Natural::ONE);
        }
        assert!(square <= n);
        assert!((sqrt + Natural::ONE).square() > n);
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(u.floor_sqrt(), Natural::from(u).floor_sqrt());
    });
}

#[test]
fn ceiling_sqrt_properties() {
    natural_gen().test_properties(|n| {
        let sqrt = n.clone().ceiling_sqrt();
        assert_eq!((&n).ceiling_sqrt(), sqrt);
        let mut n_alt = n.clone();
        n_alt.ceiling_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        assert_eq!(_ceiling_sqrt_binary(&n), sqrt);
        let square = (&sqrt).square();
        let floor_sqrt = (&n).floor_sqrt();
        if square == n {
            assert_eq!(floor_sqrt, sqrt);
        } else {
            assert_eq!(floor_sqrt, &sqrt - Natural::ONE);
        }
        assert!(square >= n);
        if n != 0 {
            assert!((sqrt - Natural::ONE).square() < n);
        }
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(u.ceiling_sqrt(), Natural::from(u).ceiling_sqrt());
    });
}

#[test]
fn checked_sqrt_properties() {
    natural_gen().test_properties(|n| {
        let sqrt = n.clone().checked_sqrt();
        assert_eq!((&n).checked_sqrt(), sqrt);
        assert_eq!(_checked_sqrt_binary(&n), sqrt);
        if let Some(sqrt) = sqrt {
            assert_eq!((&sqrt).square(), n);
            assert_eq!((&n).floor_sqrt(), sqrt);
            assert_eq!(n.ceiling_sqrt(), sqrt);
        }
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(
            u.checked_sqrt().map(Natural::from),
            Natural::from(u).checked_sqrt()
        );
    });
}

#[test]
fn sqrt_rem_properties() {
    natural_gen().test_properties(|n| {
        let (sqrt, rem) = n.clone().sqrt_rem();
        assert_eq!((&n).sqrt_rem(), (sqrt.clone(), rem.clone()));
        let mut n_alt = n.clone();
        assert_eq!(n_alt.sqrt_rem_assign(), rem);
        assert_eq!(n_alt, sqrt);
        assert_eq!(_sqrt_rem_binary(&n), (sqrt.clone(), rem.clone()));
        let (rug_sqrt, rug_rem) = natural_to_rug_integer(&n).sqrt_rem(rug::Integer::new());
        assert_eq!(rug_integer_to_natural(&rug_sqrt), sqrt);
        assert_eq!(rug_integer_to_natural(&rug_rem), rem);

        assert_eq!((&n).floor_sqrt(), sqrt);
        assert!(rem <= &sqrt << 1);
        assert_eq!(sqrt.square() + rem, n);
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        let (sqrt, rem) = u.sqrt_rem();
        assert_eq!(
            (Natural::from(sqrt), Natural::from(rem)),
            Natural::from(u).sqrt_rem()
        );
    });
}
