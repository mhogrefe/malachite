use malachite_base::slices::{
    slice_leading_zeros, slice_move_left, slice_set_zero, slice_test_zero, slice_trailing_zeros,
};

#[test]
fn test_slice_test_zero() {
    let test = |xs: &[u32], out| {
        assert_eq!(slice_test_zero(xs), out);
    };
    test(&[], true);
    test(&[0], true);
    test(&[0, 0, 0], true);
    test(&[123], false);
    test(&[123, 0], false);
    test(&[0, 123, 0, 0, 0], false);
}

#[test]
fn test_slice_set_zero() {
    let test = |xs: &[u32], out: &[u32]| {
        let mut mut_xs = xs.to_vec();
        slice_set_zero(&mut mut_xs);
        assert_eq!(mut_xs, out);
    };
    test(&[], &[]);
    test(&[0], &[0]);
    test(&[0, 0, 0], &[0, 0, 0]);
    test(&[123], &[0]);
    test(&[123, 0], &[0, 0]);
    test(&[0, 123, 0, 0, 0], &[0, 0, 0, 0, 0]);
}

fn slice_move_left_naive<T: Copy>(xs: &mut [u32], amount: usize) {
    let slice = xs[amount..].to_vec();
    let limit = xs.len() - amount;
    xs[..limit].copy_from_slice(&slice);
}

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

#[test]
fn test_slice_leading_zeros() {
    let test = |xs: &[u32], out| {
        assert_eq!(slice_leading_zeros(xs), out);
    };
    test(&[], 0);
    test(&[0], 1);
    test(&[0, 0, 0], 3);
    test(&[123], 0);
    test(&[123, 0], 0);
    test(&[0, 123, 0, 0, 0], 1);
    test(&[1, 2, 3], 0);
    test(&[0, 0, 0, 1, 2, 3], 3);
}

#[test]
fn test_slice_trailing_zeros() {
    let test = |xs: &[u32], out| {
        assert_eq!(slice_trailing_zeros(xs), out);
    };
    test(&[], 0);
    test(&[0], 1);
    test(&[0, 0, 0], 3);
    test(&[123], 0);
    test(&[123, 0], 1);
    test(&[0, 123, 0, 0, 0], 3);
    test(&[1, 2, 3], 0);
    test(&[1, 2, 3, 0, 0, 0], 3);
}
