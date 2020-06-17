use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz_test_util::common::{natural_to_rug_integer, rug_integer_to_natural};

use malachite_test::common::test_properties;
use malachite_test::inputs::natural::triples_of_natural_small_u64_and_bool;

#[test]
fn assign_bit_properties() {
    test_properties(
        triples_of_natural_small_u64_and_bool,
        |&(ref n, index, bit)| {
            let mut mut_n = n.clone();
            mut_n.assign_bit(index, bit);
            assert!(mut_n.is_valid());
            let result = mut_n;

            let mut rug_n = natural_to_rug_integer(n);
            rug_n.set_bit(u32::exact_from(index), bit);
            assert_eq!(rug_integer_to_natural(&rug_n), result);
        },
    );
}
