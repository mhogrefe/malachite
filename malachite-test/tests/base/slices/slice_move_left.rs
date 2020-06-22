use malachite_base::slices::slice_move_left;
use malachite_base_test_util::slices::slice_move_left_naive;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_unsigned_vec_and_small_usize_var_1, vecs_of_unsigned};

#[test]
fn slice_move_left_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_usize_var_1,
        |&(ref xs, amount): &(Vec<u32>, usize)| {
            let mut out_xs = xs.clone();
            slice_move_left(&mut out_xs, amount);
            let boundary = xs.len() - amount;
            let (out_xs_lo, out_xs_hi) = out_xs.split_at(boundary);
            assert_eq!(out_xs_lo, &xs[amount..]);
            assert_eq!(out_xs_hi, &xs[boundary..]);

            let mut out_xs_alt = xs.clone();
            slice_move_left_naive(&mut out_xs_alt, amount);
            assert_eq!(out_xs_alt, out_xs);
        },
    );

    test_properties(vecs_of_unsigned, |xs: &Vec<u32>| {
        let mut mut_xs = xs.clone();
        slice_move_left(&mut mut_xs, 0);
        assert_eq!(&mut_xs, xs);

        let mut mut_xs = xs.clone();
        slice_move_left(&mut mut_xs, xs.len());
        assert_eq!(&mut_xs, xs);
    });
}
