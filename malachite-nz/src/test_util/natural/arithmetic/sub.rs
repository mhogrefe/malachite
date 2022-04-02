use natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
use platform::Limb;

pub fn limbs_sub_same_length_in_place_with_overlap_naive(
    xs: &mut [Limb],
    right_start: usize,
) -> bool {
    let left_end = xs.len() - right_start;
    let mut x = xs[..left_end].to_vec();
    let borrow = limbs_sub_same_length_in_place_left(&mut x, &xs[right_start..]);
    xs[..left_end].copy_from_slice(&x);
    borrow
}

/// Given two slices `xs` and `ys`, computes the difference between the `Natural`s whose limbs are
/// `&xs[xs.len() - ys.len()..]` and `&ys`, and writes the limbs of the result to `&xs[..ys.len()]`.
pub fn limbs_sub_same_length_to_out_with_overlap_naive(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let y_len = ys.len();
    let mut x = xs[xs.len() - y_len..].to_vec();
    let borrow = limbs_sub_same_length_in_place_left(&mut x, ys);
    xs[..y_len].copy_from_slice(&x);
    borrow
}
