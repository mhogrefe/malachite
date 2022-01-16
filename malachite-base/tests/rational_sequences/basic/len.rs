use malachite_base::rational_sequences::RationalSequence;
use malachite_base_test_util::generators::{unsigned_rational_sequence_gen, unsigned_vec_gen};

#[test]
pub fn test_len() {
    fn test(non_repeating: &[u8], repeating: &[u8], out: Option<usize>) {
        let xs = RationalSequence::from_slices(non_repeating, repeating);
        assert_eq!(xs.len(), out);
    }
    test(&[], &[], Some(0));
    test(&[1, 2, 3], &[], Some(3));
    test(&[], &[1, 2, 3], None);
    test(&[1, 2, 3], &[4, 5, 6], None);
}

#[test]
fn len_properties() {
    unsigned_rational_sequence_gen::<u8>().test_properties(|xs| {
        assert_eq!(xs.len().is_some(), xs.is_finite());
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        assert_eq!(RationalSequence::from_slice(&xs).len(), Some(xs.len()));
    });
}
