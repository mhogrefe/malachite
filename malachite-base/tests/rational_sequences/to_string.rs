use malachite_base::rational_sequences::RationalSequence;
use malachite_base::strings::string_is_subset;
use malachite_base::strings::ToDebugString;
use malachite_base_test_util::generators::{unsigned_rational_sequence_gen, unsigned_vec_gen};

#[test]
pub fn test_to_string() {
    fn test(non_repeating: &[u8], repeating: &[u8], out: &str) {
        let xs = RationalSequence::from_slices(non_repeating, repeating);
        assert_eq!(xs.to_string(), out);
        assert_eq!(xs.to_debug_string(), out);
    }
    test(&[], &[], "[]");
    test(&[1, 2, 3], &[], "[1, 2, 3]");
    test(&[], &[1, 2, 3], "[[1, 2, 3]]");
    test(&[1, 2, 3], &[4, 5, 6], "[1, 2, 3, [4, 5, 6]]");
}

#[test]
fn to_string_properties() {
    unsigned_rational_sequence_gen::<u8>().test_properties(|xs| {
        let s = xs.to_string();
        assert_eq!(xs.to_debug_string(), s);
        assert!(string_is_subset(&s, " ,0123456789[]"));
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        assert_eq!(
            RationalSequence::from_slice(&xs).to_string(),
            xs.to_debug_string()
        );
    });
}
