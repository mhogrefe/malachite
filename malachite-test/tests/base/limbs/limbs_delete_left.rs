use malachite_base::limbs::limbs_delete_left;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_unsigned_vec_and_small_usize_var_1, vecs_of_unsigned};

#[test]
fn limbs_delete_left_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_usize_var_1,
        |&(ref limbs, delete_size): &(Vec<u32>, usize)| {
            let mut mut_limbs = limbs.clone();
            limbs_delete_left(&mut mut_limbs, delete_size);
            assert_eq!(mut_limbs == *limbs, delete_size == 0);
            assert_eq!(mut_limbs.is_empty(), delete_size == limbs.len());
            assert_eq!(mut_limbs.len(), limbs.len() - delete_size);
            assert_eq!(&limbs[delete_size..], &*mut_limbs);
        },
    );

    test_properties(vecs_of_unsigned, |limbs: &Vec<u32>| {
        let mut mut_limbs = limbs.clone();
        limbs_delete_left(&mut mut_limbs, limbs.len());
        assert!(mut_limbs.is_empty());

        let mut mut_limbs = limbs.clone();
        limbs_delete_left(&mut mut_limbs, 0);
        assert_eq!(mut_limbs, *limbs);
    });
}
