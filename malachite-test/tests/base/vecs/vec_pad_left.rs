use malachite_base::vecs::{vec_delete_left, vec_pad_left};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_small_usize_and_unsigned, pairs_of_unsigned_vec_and_unsigned,
    triples_of_unsigned_vec_small_usize_and_unsigned,
};

#[test]
fn vec_pad_left_properties() {
    test_properties(
        triples_of_unsigned_vec_small_usize_and_unsigned,
        |&(ref xs, pad_size, pad_value): &(Vec<u32>, usize, u32)| {
            let mut mut_xs = xs.clone();
            vec_pad_left(&mut mut_xs, pad_size, pad_value);
            assert_eq!(mut_xs == *xs, pad_size == 0);
            assert_eq!(mut_xs.len(), xs.len() + pad_size);
            assert!(mut_xs[..pad_size].iter().all(|&x| x == pad_value));
            assert_eq!(&mut_xs[pad_size..], xs.as_slice());
            vec_delete_left(&mut mut_xs, pad_size);
            assert_eq!(mut_xs, *xs);
        },
    );

    test_properties(
        pairs_of_unsigned_vec_and_unsigned,
        |&(ref xs, pad_value): &(Vec<u32>, u32)| {
            let mut mut_xs = xs.clone();
            vec_pad_left(&mut mut_xs, 0, pad_value);
            assert_eq!(mut_xs, *xs);
        },
    );

    test_properties(
        pairs_of_small_usize_and_unsigned,
        |&(pad_size, pad_value): &(usize, u32)| {
            let mut xs = Vec::new();
            vec_pad_left(&mut xs, pad_size, pad_value);
            assert_eq!(xs, vec![pad_value; pad_size]);
        },
    );
}
