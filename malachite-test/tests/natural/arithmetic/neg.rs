use malachite_nz::integer::Integer;
use malachite_nz_test_util::common::{
    bigint_to_integer, natural_to_biguint, natural_to_rug_integer, rug_integer_to_integer,
};
use malachite_nz_test_util::natural::arithmetic::neg::neg_num;

use malachite_test::common::test_properties;
use malachite_test::inputs::natural::naturals;

#[test]
fn neg_properties() {
    test_properties(naturals, |x| {
        let neg = -x.clone();
        assert!(neg.is_valid());

        let neg_alt = -x;
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        assert_eq!(rug_integer_to_integer(&(-natural_to_rug_integer(x))), neg);
        assert_eq!(bigint_to_integer(&neg_num(natural_to_biguint(x))), neg);

        assert_eq!(-Integer::from(x), neg);
        assert_eq!(neg == *x, *x == 0);
        assert_eq!(-neg, *x);
    });
}
