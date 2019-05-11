use malachite_base::limbs::limbs_test_zero;

use common::test_properties;
use malachite_test::inputs::base::vecs_of_unsigned;

#[test]
fn limbs_test_zero_properties() {
    test_properties(vecs_of_unsigned, |limbs: &Vec<u32>| {
        limbs_test_zero(limbs);
    });
}
