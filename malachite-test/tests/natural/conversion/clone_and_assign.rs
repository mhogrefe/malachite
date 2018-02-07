use common::test_properties;
use malachite_base::num::Assign;
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, natural_to_biguint, natural_to_rug_integer,
                             rug_integer_to_natural};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals};
use num::BigUint;
use rug;
use rug::Assign as rug_assign;
use std::str::FromStr;

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
fn test_clone_clone_from_and_assign() {
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

        // assign Natural by value
        let mut x = Natural::from_str(u).unwrap();
        x.assign(Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = rug::Integer::from_str(u).unwrap();
        x.assign(rug::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);

        // assign Natural by reference
        let mut x = Natural::from_str(u).unwrap();
        x.assign(&Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = rug::Integer::from_str(u).unwrap();
        x.assign(&rug::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
    };
    test("123", "456");
    test("123", "1000000000000");
    test("1000000000000", "123");
    test("1000000000000", "2000000000000");
}

#[test]
fn clone_clone_from_and_assign_properties() {
    test_properties(naturals, |x| {
        let x_mut = x.clone();
        assert!(x_mut.is_valid());
        assert_eq!(x_mut, *x);

        assert_eq!(biguint_to_natural(&natural_to_biguint(x).clone()), *x);
        assert_eq!(
            rug_integer_to_natural(&natural_to_rug_integer(x).clone()),
            *x
        );
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

        let mut mut_x = x.clone();
        mut_x.assign(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, *y);
        let mut rug_x = natural_to_rug_integer(x);
        rug_x.assign(natural_to_rug_integer(y));
        assert_eq!(rug_integer_to_natural(&rug_x), *y);

        let mut mut_x = x.clone();
        mut_x.assign(y);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, *y);
        let mut rug_x = natural_to_rug_integer(x);
        rug_x.assign(&natural_to_rug_integer(y));
        assert_eq!(rug_integer_to_natural(&rug_x), *y);
    });
}
