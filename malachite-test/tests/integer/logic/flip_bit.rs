use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz::integer::Integer;
use malachite_nz_test_util::common::{integer_to_rug_integer, rug_integer_to_integer};

use malachite_test::common::test_properties;
use malachite_test::inputs::integer::pairs_of_integer_and_small_unsigned;

#[test]
fn flip_bit_properties() {
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, index)| {
        let mut mut_n = n.clone();
        mut_n.flip_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;

        assert_ne!(result, *n);

        let mut rug_n = integer_to_rug_integer(n);
        rug_n.toggle_bit(u32::exact_from(index));
        assert_eq!(rug_integer_to_integer(&rug_n), result);

        let mut mut_result = result.clone();
        mut_result.flip_bit(index);
        assert_eq!(mut_result, *n);

        assert_eq!(n ^ Integer::power_of_2(index), result);
    });
}
