use common::test_properties;
use malachite_base::limbs::limbs_delete_left;
use malachite_test::inputs::base::{vecs_of_unsigned, pairs_of_unsigned_vec_and_small_usize_var_1};

#[test]
fn test_limbs_delete_left() {
    let test = |limbs: &[u32], delete_size: usize, out: &[u32]| {
        let mut mut_limbs = limbs.to_vec();
        limbs_delete_left(&mut mut_limbs, delete_size);
        assert_eq!(mut_limbs, out);
    };
    test(&[], 0, &[]);
    test(&[1, 2, 3, 4, 5], 0, &[1, 2, 3, 4, 5]);
    test(&[1, 2, 3, 4, 5], 3, &[4, 5]);
    test(&[1, 2, 3, 4, 5], 5, &[]);
}

#[test]
#[should_panic(expected = "assertion failed: delete_size <= limbs.len()")]
fn delete_left_fail_1() {
    let mut limbs = Vec::new();
    limbs_delete_left(&mut limbs, 1);
}

#[test]
#[should_panic(expected = "assertion failed: delete_size <= limbs.len()")]
fn delete_left_fail_2() {
    let mut limbs = vec![1, 2, 3];
    limbs_delete_left(&mut limbs, 4);
}

#[test]
fn limbs_pad_left_properties() {
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
