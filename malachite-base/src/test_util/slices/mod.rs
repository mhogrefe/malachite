pub fn slice_move_left_naive<T: Copy>(xs: &mut [T], amount: usize) {
    let slice = xs[amount..].to_vec();
    let limit = xs.len() - amount;
    xs[..limit].copy_from_slice(&slice);
}
