use malachite_base::num::arithmetic::traits::DivMod;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::platform::{SignedDoubleLimb, SignedLimb};

use malachite_test::common::test_properties;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::pairs_of_signeds;
use malachite_test::inputs::integer::{integers, pairs_of_integers, triples_of_integers};
use malachite_test::inputs::natural::pairs_of_naturals;

#[test]
fn mul_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let product_val_val = x.clone() * y.clone();
        let product_val_ref = x.clone() * y;
        let product_ref_val = x * y.clone();
        let product = x * y;
        assert!(product_val_val.is_valid());
        assert!(product_val_ref.is_valid());
        assert!(product_ref_val.is_valid());
        assert!(product.is_valid());
        assert_eq!(product_val_val, product);
        assert_eq!(product_val_ref, product);
        assert_eq!(product_ref_val, product);

        let mut mut_x = x.clone();
        mut_x *= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x *= y;
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        let mut mut_x = integer_to_rug_integer(x);
        mut_x *= integer_to_rug_integer(y);
        assert_eq!(rug_integer_to_integer(&mut_x), product);

        assert_eq!(
            bigint_to_integer(&(integer_to_bigint(x) * integer_to_bigint(y))),
            product
        );
        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(x) * integer_to_rug_integer(y))),
            product
        );
        assert_eq!(y * x, product);
        if *x != 0 {
            let (q, r) = (&product).div_mod(x);
            assert_eq!(q, *y);
            assert_eq!(r, 0);
        }
        if *y != 0 {
            let (q, r) = (&product).div_mod(y);
            assert_eq!(q, *x);
            assert_eq!(r, 0);
        }

        assert_eq!(-x * y, -&product);
        assert_eq!(x * -y, -product);
    });

    #[allow(unknown_lints, erasing_op)]
    test_properties(integers, |x| {
        assert_eq!(x * Integer::ZERO, 0);
        assert_eq!(Integer::ZERO * x, 0);
        assert_eq!(x * Integer::ONE, *x);
        assert_eq!(Integer::ONE * x, *x);
        assert_eq!(x * Integer::NEGATIVE_ONE, -x);
        assert_eq!(Integer::NEGATIVE_ONE * x, -x);
        //TODO assert_eq!(x * x, x.pow(2));
    });

    test_properties(triples_of_integers, |&(ref x, ref y, ref z)| {
        assert_eq!((x * y) * z, x * (y * z));
        assert_eq!(x * (y + z), x * y + x * z);
        assert_eq!((x + y) * z, x * z + y * z);
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(x * y, Integer::from(x) * Integer::from(y));
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(x, y)| {
        assert_eq!(
            Integer::from(SignedDoubleLimb::from(x) * SignedDoubleLimb::from(y)),
            Integer::from(x) * Integer::from(y)
        );
    });
}
