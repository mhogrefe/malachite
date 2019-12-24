use malachite_base::limbs::{limbs_leading_zero_limbs, limbs_test_zero, limbs_trailing_zero_limbs};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::vecs_of_unsigned;

#[test]
fn limbs_trailing_zero_limbs_properties() {
    test_properties(vecs_of_unsigned, |limbs: &Vec<u32>| {
        let trailing_zero_limbs = limbs_trailing_zero_limbs(limbs);
        assert!(trailing_zero_limbs <= limbs.len());
        assert_eq!(trailing_zero_limbs == limbs.len(), limbs_test_zero(limbs));
        let mut new_limbs = limbs.clone();
        new_limbs.reverse();
        assert_eq!(limbs_leading_zero_limbs(&new_limbs), trailing_zero_limbs);
    });
}
