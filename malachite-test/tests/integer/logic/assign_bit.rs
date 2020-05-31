use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz_test_util::common::{integer_to_rug_integer, rug_integer_to_integer};

use malachite_test::common::test_properties;
use malachite_test::inputs::integer::triples_of_integer_small_u64_and_bool;

#[test]
fn assign_bit_properties() {
    test_properties(
        triples_of_integer_small_u64_and_bool,
        |&(ref n, index, bit)| {
            let mut mut_n = n.clone();
            mut_n.assign_bit(index, bit);
            assert!(mut_n.is_valid());
            let result = mut_n;

            let mut rug_n = integer_to_rug_integer(n);
            rug_n.set_bit(u32::exact_from(index), bit);
            assert_eq!(rug_integer_to_integer(&rug_n), result);
        },
    );
}
