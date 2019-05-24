use common::test_properties;
use malachite_base::num::traits::Assign;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{
    integer_to_rug_integer, natural_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::integer::pairs_of_integer_and_natural;
use rug;
use rug::Assign as rug_assign;
use std::str::FromStr;

#[test]
fn test_assign_natural() {
    let test = |u, v, out| {
        // assign Integer by value
        let mut x = Integer::from_str(u).unwrap();
        x.assign(&Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = rug::Integer::from_str(u).unwrap();
        x.assign(rug::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);

        // assign Integer by reference
        let mut x = Integer::from_str(u).unwrap();
        x.assign(&Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = rug::Integer::from_str(u).unwrap();
        x.assign(&rug::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), out);
    };
    test("-123", "456", "456");
    test("-123", "1000000000000", "1000000000000");
    test("1000000000000", "123", "123");
    test("1000000000000", "2000000000000", "2000000000000");
}

#[test]
fn assign_natural_properties() {
    test_properties(pairs_of_integer_and_natural, |&(ref x, ref y)| {
        let mut mut_x = x.clone();
        mut_x.assign(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, *y);

        let mut rug_x = integer_to_rug_integer(x);
        rug_x.assign(natural_to_rug_integer(y));
        assert_eq!(rug_integer_to_integer(&rug_x), *y);

        let mut mut_x = x.clone();
        mut_x.assign(y);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, *y);

        let mut rug_x = integer_to_rug_integer(x);
        rug_x.assign(&natural_to_rug_integer(y));
        assert_eq!(rug_integer_to_integer(&rug_x), *y);
    });
}
