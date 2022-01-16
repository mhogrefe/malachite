use malachite_base::rational_sequences::RationalSequence;
use malachite_base_test_util::generators::{unsigned_rational_sequence_gen, unsigned_vec_gen};

#[test]
pub fn test_component_len() {
    fn test(non_repeating: &[u8], repeating: &[u8], out: usize) {
        let xs = RationalSequence::from_slices(non_repeating, repeating);
        assert_eq!(xs.component_len(), out);
    }
    test(&[], &[], 0);
    test(&[1, 2, 3], &[], 3);
    test(&[], &[1, 2, 3], 3);
    test(&[1, 2, 3], &[4, 5, 6], 6);
}

#[test]
fn component_len_properties() {
    unsigned_rational_sequence_gen::<u8>().test_properties(|xs| {
        xs.component_len();
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        assert_eq!(RationalSequence::from_slice(&xs).component_len(), xs.len());
    });
}
