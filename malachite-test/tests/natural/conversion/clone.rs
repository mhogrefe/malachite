use std::str::FromStr;

use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::BigUint;
use rug;

use common::test_properties;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{pairs_of_unsigneds, unsigneds};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals};

#[test]
fn test_clone() {
    let test = |u| {
        let x = Natural::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = BigUint::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);

        let x = rug::Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
    };
    test("123");
    test("1000000000000");
}

#[test]
fn test_clone_and_clone_from() {
    let test = |u, v| {
        // clone_from
        let mut x = Natural::from_str(u).unwrap();
        x.clone_from(&Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = BigUint::from_str(u).unwrap();
        x.clone_from(&BigUint::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);

        let mut x = rug::Integer::from_str(u).unwrap();
        x.clone_from(&rug::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
    };
    test("123", "456");
    test("123", "1000000000000");
    test("1000000000000", "123");
    test("1000000000000", "2000000000000");
}

#[test]
fn clone_and_clone_from_properties() {
    test_properties(naturals, |x| {
        let mut_x = x.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, *x);

        assert_eq!(biguint_to_natural(&natural_to_biguint(x).clone()), *x);
        assert_eq!(
            rug_integer_to_natural(&natural_to_rug_integer(x).clone()),
            *x
        );
    });

    test_properties(unsigneds::<Limb>, |&u| {
        let n = Natural::from(u);
        let cloned_u = u;
        let cloned_n = n.clone();
        assert_eq!(cloned_u, cloned_n);
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let mut mut_x = x.clone();
        mut_x.clone_from(y);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, *y);

        let mut num_x = natural_to_biguint(x);
        num_x.clone_from(&natural_to_biguint(y));
        assert_eq!(biguint_to_natural(&num_x), *y);

        let mut rug_x = natural_to_rug_integer(x);
        rug_x.clone_from(&natural_to_rug_integer(y));
        assert_eq!(rug_integer_to_natural(&rug_x), *y);
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(u, v)| {
        let x = Natural::from(u);
        let y = Natural::from(v);

        let mut mut_u = u;
        let mut mut_x = x.clone();
        mut_u.clone_from(&v);
        mut_x.clone_from(&y);
        assert_eq!(mut_x, mut_u);
    });
}
