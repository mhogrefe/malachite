use malachite_base::rational_sequences::RationalSequence;
use malachite_base_test_util::generators::large_type_gen_var_22;

#[test]
pub fn test_mutate() {
    fn test(
        non_repeating: &[u8],
        repeating: &[u8],
        index: usize,
        new_value: u8,
        out: u8,
        non_repeating_out: &[u8],
        repeating_out: &[u8],
    ) {
        let mut xs = RationalSequence::from_slices(non_repeating, repeating);
        assert_eq!(
            xs.mutate(index, |x| {
                *x = new_value;
                out
            }),
            out
        );
        assert_eq!(
            xs,
            RationalSequence::from_slices(non_repeating_out, repeating_out)
        );
    }
    test(&[1, 2, 3], &[], 0, 5, 6, &[5, 2, 3], &[]);
    test(&[1, 2, 3], &[], 1, 5, 6, &[1, 5, 3], &[]);
    test(&[1, 2, 3], &[], 2, 5, 6, &[1, 2, 5], &[]);
    test(
        &[1, 2, 3],
        &[4, 5, 6],
        3,
        100,
        6,
        &[1, 2, 3, 100],
        &[5, 6, 4],
    );
    test(
        &[1, 2, 3],
        &[4, 5, 6],
        10,
        100,
        6,
        &[1, 2, 3, 4, 5, 6, 4, 5, 6, 4, 100],
        &[6, 4, 5],
    );
}

#[test]
#[should_panic]
fn mutate_fail_1() {
    RationalSequence::<u8>::from_vec(vec![]).mutate(0, |_| {});
}

#[test]
#[should_panic]
fn mutate_fail_2() {
    RationalSequence::from_vec(vec![1, 2, 3]).mutate(3, |_| {})
}

#[test]
fn mutate_properties() {
    large_type_gen_var_22::<u8>().test_properties(|(mut xs, index, y, z)| {
        let xs_old = xs.clone();
        let x_old = xs[index];
        assert_eq!(
            xs.mutate(index, |x| {
                *x = y;
                z
            }),
            z
        );
        assert_eq!(xs[index], y);
        xs.mutate(index, |x| {
            *x = x_old;
        });
        assert_eq!(xs, xs_old);
    });
}
