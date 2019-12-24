use malachite_base::limbs::{limbs_set_zero, limbs_test_zero};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::vecs_of_unsigned;

#[test]
fn limbs_test_zero_properties() {
    test_properties(vecs_of_unsigned, |limbs: &Vec<u32>| {
        let limbs_are_zero = limbs_test_zero(limbs);
        let mut new_limbs = limbs.clone();
        limbs_set_zero(&mut new_limbs);
        assert_eq!(*limbs == new_limbs, limbs_are_zero);
    });
}
