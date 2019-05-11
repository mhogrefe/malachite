use malachite_base::limbs::limbs_pad_left;

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
