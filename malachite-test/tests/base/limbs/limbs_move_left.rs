use malachite_base::limbs::limbs_move_left;

use common::test_properties;
use malachite_test::inputs::base::{pairs_of_unsigned_vec_and_small_usize_var_1, vecs_of_unsigned};

#[test]
fn limbs_move_left_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_usize_var_1,
        |&(ref limbs, amount): &(Vec<u32>, usize)| {
            let mut out_limbs = limbs.clone();
            limbs_move_left(&mut out_limbs, amount);
            let boundary = limbs.len() - amount;
            let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at(boundary);
            assert_eq!(out_limbs_lo, &limbs[amount..]);
            assert_eq!(out_limbs_hi, &limbs[boundary..]);
        },
    );

    test_properties(vecs_of_unsigned, |limbs: &Vec<u32>| {
        let mut mut_limbs = limbs.clone();
        limbs_move_left(&mut mut_limbs, 0);
        assert_eq!(&mut_limbs, limbs);

        let mut mut_limbs = limbs.clone();
        limbs_move_left(&mut mut_limbs, limbs.len());
        assert_eq!(&mut_limbs, limbs);
    });
}
