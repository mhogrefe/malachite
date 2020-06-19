use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::common::{natural_to_rug_integer, rug_integer_to_natural};

use malachite_test::common::test_properties;
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;

#[test]
fn flip_bit_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, index)| {
        let mut mut_n = n.clone();
        mut_n.flip_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;
        assert_ne!(result, *n);

        let mut rug_n = natural_to_rug_integer(n);
        rug_n.toggle_bit(u32::exact_from(index));
        assert_eq!(rug_integer_to_natural(&rug_n), result);

        let mut mut_result = result.clone();
        mut_result.flip_bit(index);
        assert_eq!(mut_result, *n);

        assert_eq!(n ^ Natural::power_of_two(index), result);
    });
}
