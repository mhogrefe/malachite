use common::test_properties;
use malachite_base::limbs::{limbs_set_zero, limbs_test_zero};
use malachite_test::inputs::base::vecs_of_unsigned;

#[test]
fn test_limbs_test_zero() {
    let test = |limbs: &[u32], out: &[u32]| {
        let mut mut_limbs = limbs.to_vec();
        limbs_set_zero(&mut mut_limbs);
        assert_eq!(mut_limbs, out);
    };
    test(&[], &[]);
    test(&[0], &[0]);
    test(&[0, 0, 0], &[0, 0, 0]);
    test(&[123], &[0]);
    test(&[123, 0], &[0, 0]);
    test(&[0, 123, 0, 0, 0], &[0, 0, 0, 0, 0]);
}

#[test]
fn limbs_set_zero_properties() {
    test_properties(vecs_of_unsigned, |limbs: &Vec<u32>| {
        let mut mut_limbs = limbs.clone();
        limbs_set_zero(&mut mut_limbs);
        assert_eq!(limbs.len(), mut_limbs.len());
        assert!(limbs_test_zero(&mut_limbs));
    });
}
