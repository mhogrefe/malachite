use malachite_base::num::arithmetic::traits::{
    CoprimeWith, DivExact, DivisibleBy, Gcd, GcdAssign, Lcm,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base_test_util::generators::unsigned_pair_gen_var_27;
use malachite_nz::natural::arithmetic::gcd::{_gcd_binary, _gcd_euclidean};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_nz_test_util::generators::{natural_gen, natural_pair_gen, natural_triple_gen};
use num::BigUint;
use num::Integer as rug_integer;
use std::str::FromStr;

#[test]
fn test_gcd() {
    let test = |s, t, out| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        let mut n = u.clone();
        n.gcd_assign(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.gcd_assign(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().gcd(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).gcd(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().gcd(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).gcd(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigUint::from_str(s)
            .unwrap()
            .gcd(&BigUint::from_str(t).unwrap());
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(s)
            .unwrap()
            .gcd(&rug::Integer::from_str(t).unwrap());
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "6", "6");
    test("6", "0", "6");
    test("1", "6", "1");
    test("6", "1", "1");
    test("8", "12", "4");
    test("54", "24", "6");
    test("42", "56", "14");
    test("48", "18", "6");
    test("3", "5", "1");
    test("12", "60", "12");
    test("12", "90", "6");
    test("12345678987654321", "98765432123456789", "1");
    test("12345678987654321", "98765432123456827", "37");
}

#[test]
fn gcd_properties() {
    natural_pair_gen().test_properties(|(x, y)| {
        let gcd_val_val = x.clone().gcd(y.clone());
        let gcd_val_ref = x.clone().gcd(&y);
        let gcd_ref_val = (&x).gcd(y.clone());
        let gcd = (&x).gcd(&y);
        assert!(gcd_val_val.is_valid());
        assert!(gcd_val_ref.is_valid());
        assert!(gcd_ref_val.is_valid());
        assert!(gcd.is_valid());
        assert_eq!(gcd_val_val, gcd);
        assert_eq!(gcd_val_ref, gcd);
        assert_eq!(gcd_ref_val, gcd);

        let mut mut_x = x.clone();
        mut_x.gcd_assign(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, gcd);

        let mut mut_x = x.clone();
        mut_x.gcd_assign(&y);
        assert_eq!(mut_x, gcd);
        assert!(mut_x.is_valid());

        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(&x).gcd(&natural_to_biguint(&y)))),
            gcd
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(&x).gcd(&natural_to_rug_integer(&y)))),
            gcd
        );

        assert_eq!(_gcd_euclidean(x.clone(), y.clone()), gcd);
        assert_eq!(_gcd_binary(x.clone(), y.clone()), gcd);

        assert_eq!((&y).gcd(&x), gcd);
        assert!((&x).divisible_by(&gcd));
        assert!((&y).divisible_by(&gcd));
        assert_eq!(gcd == 0, x == 0 && y == 0);
        if gcd != 0 {
            assert!(((&x).div_exact(&gcd)).coprime_with((&y).div_exact(&gcd)));
        }
        if x != 0 && y != 0 {
            assert_eq!(&x * &y / x.lcm(y), gcd);
        }
    });

    natural_gen().test_properties(|x| {
        assert_eq!((&x).gcd(&x), x);
        assert_eq!((&x).gcd(Natural::ONE), 1);
        assert_eq!((&x).gcd(Natural::ZERO), x);
    });

    natural_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!((&x).gcd(&y).gcd(&z), x.gcd(y.gcd(z)));
    });

    unsigned_pair_gen_var_27::<Limb>().test_properties(|(x, y)| {
        assert_eq!(Natural::from(x).gcd(Natural::from(y)), x.gcd(y));
    });
}
