use malachite_base::vecs::{vec_delete_left, vec_pad_left};

#[test]
fn test_vec_pad_left() {
    let test = |xs: &[u32], pad_size: usize, pad_value: u32, out: &[u32]| {
        let mut mut_xs = xs.to_vec();
        vec_pad_left(&mut mut_xs, pad_size, pad_value);
        assert_eq!(mut_xs, out);
    };
    test(&[], 3, 6, &[6, 6, 6]);
    test(&[1, 2, 3], 0, 10, &[1, 2, 3]);
    test(&[1, 2, 3], 5, 10, &[10, 10, 10, 10, 10, 1, 2, 3]);
}

#[test]
fn test_vec_delete_left() {
    let test = |xs: &[u32], delete_size: usize, out: &[u32]| {
        let mut mut_xs = xs.to_vec();
        vec_delete_left(&mut mut_xs, delete_size);
        assert_eq!(mut_xs, out);
    };
    test(&[], 0, &[]);
    test(&[1, 2, 3, 4, 5], 0, &[1, 2, 3, 4, 5]);
    test(&[1, 2, 3, 4, 5], 3, &[4, 5]);
    test(&[1, 2, 3, 4, 5], 5, &[]);
}

#[test]
#[should_panic]
fn vec_delete_left_fail_1() {
    let mut xs: Vec<u32> = Vec::new();
    vec_delete_left(&mut xs, 1);
}

#[test]
#[should_panic]
fn vec_delete_left_fail_2() {
    let mut xs: Vec<u32> = vec![1, 2, 3];
    vec_delete_left(&mut xs, 4);
}

pub mod exhaustive {
    pub mod exhaustive_fixed_length_vecs_from_single;
    pub mod exhaustive_fixed_length_vecs_m_inputs;
    pub mod exhaustive_length_n_vecs;
}
pub mod random_values_from_vec;
