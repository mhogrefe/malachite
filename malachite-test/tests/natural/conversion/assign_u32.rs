use common::test_properties;
use malachite_base::num::Assign;
use malachite_nz::natural::Natural;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::pairs_of_unsigneds;
use malachite_test::inputs::natural::pairs_of_natural_and_unsigned;
use malachite_test::natural::conversion::assign_u32::num_assign_u32;
use num::BigUint;
use rug;
use rug::Assign as rug_assign;
use std::str::FromStr;
use std::u32;

#[test]
fn test_assign_u32() {
    let test = |u, v: u32, out| {
        let mut x = Natural::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = BigUint::from_str(u).unwrap();
        num_assign_u32(&mut x, v);
        assert_eq!(x.to_string(), out);

        let mut x = rug::Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
    };
    test("123", 456, "456");
    test("123", u32::MAX, "4294967295");
    test("1000000000000", 123, "123");
}

#[test]
fn assign_u32_properties() {
    test_properties(pairs_of_natural_and_unsigned::<u32>, |&(ref n, u)| {
        let mut mut_n = n.clone();
        mut_n.assign(u);
        assert!(mut_n.is_valid());
        assert_eq!(mut_n, u);

        let mut mut_n = n.clone();
        mut_n.assign(Natural::from(u));
        assert_eq!(mut_n, u);

        let mut num_n = natural_to_biguint(n);
        num_assign_u32(&mut num_n, u);
        assert_eq!(biguint_to_natural(&num_n), u);

        let mut rug_n = natural_to_rug_integer(n);
        rug_n.assign(u);
        assert_eq!(rug_integer_to_natural(&rug_n), u);
    });

    test_properties(pairs_of_unsigneds::<u32>, #[allow(unused_assignments)]
    |&(u, v)| {
        let mut mut_u = u;
        let mut mut_n = Natural::from(u);
        mut_u = v;
        mut_n.assign(v);
        assert_eq!(mut_u, mut_n);
    });
}
