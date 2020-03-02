use malachite_base::vecs::vec_delete_left;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_unsigned_vec_and_small_usize_var_1, vecs_of_unsigned};

#[test]
fn vec_delete_left_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_usize_var_1,
        |&(ref xs, delete_size): &(Vec<u32>, usize)| {
            let mut mut_xs = xs.clone();
            vec_delete_left(&mut mut_xs, delete_size);
            assert_eq!(mut_xs == *xs, delete_size == 0);
            assert_eq!(mut_xs.is_empty(), delete_size == xs.len());
            assert_eq!(mut_xs.len(), xs.len() - delete_size);
            assert_eq!(&xs[delete_size..], &*mut_xs);
        },
    );

    test_properties(vecs_of_unsigned, |xs: &Vec<u32>| {
        let mut mut_xs = xs.clone();
        vec_delete_left(&mut mut_xs, xs.len());
        assert!(mut_xs.is_empty());

        let mut mut_xs = xs.clone();
        vec_delete_left(&mut mut_xs, 0);
        assert_eq!(mut_xs, *xs);
    });
}
