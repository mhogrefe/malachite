use common::test_properties;
use malachite_base::limbs::{limbs_delete_left, limbs_pad_left};
use malachite_test::inputs::base::{pairs_of_small_usize_and_unsigned,
                                   pairs_of_unsigned_vec_and_unsigned,
                                   triples_of_unsigned_vec_small_usize_and_unsigned};

#[test]
fn test_limbs_pad_left() {
    let test = |limbs: &[u32], pad_size: usize, pad_limb: u32, out: &[u32]| {
        let mut mut_limbs = limbs.to_vec();
        limbs_pad_left(&mut mut_limbs, pad_size, pad_limb);
        assert_eq!(mut_limbs, out);
    };
    test(&[], 3, 6, &[6, 6, 6]);
    test(&[1, 2, 3], 0, 10, &[1, 2, 3]);
    test(&[1, 2, 3], 5, 10, &[10, 10, 10, 10, 10, 1, 2, 3]);
}

#[test]
fn limbs_pad_left_properties() {
    test_properties(
        triples_of_unsigned_vec_small_usize_and_unsigned,
        |&(ref limbs, pad_size, pad_limb): &(Vec<u32>, usize, u32)| {
            let mut mut_limbs = limbs.clone();
            limbs_pad_left(&mut mut_limbs, pad_size, pad_limb);
            assert_eq!(mut_limbs == *limbs, pad_size == 0);
            assert_eq!(mut_limbs.len(), limbs.len() + pad_size);
            assert!(mut_limbs[0..pad_size].iter().all(|&limb| limb == pad_limb));
            assert_eq!(&mut_limbs[pad_size..], limbs.as_slice());
            limbs_delete_left(&mut mut_limbs, pad_size);
            assert_eq!(mut_limbs, *limbs);
        },
    );

    test_properties(
        pairs_of_unsigned_vec_and_unsigned,
        |&(ref limbs, pad_limb): &(Vec<u32>, u32)| {
            let mut mut_limbs = limbs.clone();
            limbs_pad_left(&mut mut_limbs, 0, pad_limb);
            assert_eq!(mut_limbs, *limbs);
        },
    );

    test_properties(
        pairs_of_small_usize_and_unsigned,
        |&(pad_size, pad_limb): &(usize, u32)| {
            let mut limbs = Vec::new();
            limbs_pad_left(&mut limbs, pad_size, pad_limb);
            assert_eq!(limbs, vec![pad_limb; pad_size]);
        },
    );
}
