use malachite_base_test_util::slices::slice_move_left::slice_move_left_naive;

use malachite_base::slices::slice_move_left::slice_move_left;

#[test]
fn test_slice_move_left() {
    let test = |xs_in: &[u32], amount, xs_out: &[u32]| {
        let mut xs = xs_in.to_vec();
        slice_move_left(&mut xs, amount);
        assert_eq!(xs, xs_out);

        let mut xs = xs_in.to_vec();
        slice_move_left_naive::<u32>(&mut xs, amount);
        assert_eq!(xs, xs_out);
    };
    test(&[], 0, &[]);
    test(&[1], 0, &[1]);
    test(&[1], 1, &[1]);
    test(&[1, 2, 3], 0, &[1, 2, 3]);
    test(&[1, 2, 3], 1, &[2, 3, 3]);
    test(&[1, 2, 3], 2, &[3, 2, 3]);
    test(&[1, 2, 3], 3, &[1, 2, 3]);
    test(&[1, 2, 3, 4, 5, 6], 2, &[3, 4, 5, 6, 5, 6])
}

#[test]
#[should_panic]
fn slice_move_left_fail_1() {
    let xs = &mut [];
    slice_move_left::<u32>(xs, 1);
}

#[test]
#[should_panic]
fn slice_move_left_fail_2() {
    let xs = &mut [1, 2, 3];
    slice_move_left::<u32>(xs, 4);
}
