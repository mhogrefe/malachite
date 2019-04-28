use malachite_base::limbs::limbs_test_zero;

use common::test_properties;
use malachite_test::inputs::base::vecs_of_unsigned;

#[test]
fn test_limbs_test_zero() {
    let test = |limbs: &[u32], out| {
        assert_eq!(limbs_test_zero(limbs), out);
    };
    test(&[], true);
    test(&[0], true);
    test(&[0, 0, 0], true);
    test(&[123], false);
    test(&[123, 0], false);
    test(&[0, 123, 0, 0, 0], false);
}

#[test]
fn limbs_test_zero_properties() {
    test_properties(vecs_of_unsigned, |limbs: &Vec<u32>| {
        limbs_test_zero(limbs);
    });
}
