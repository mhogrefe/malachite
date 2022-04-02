use malachite_base::rational_sequences::RationalSequence;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::unsigned_vec_gen;

#[test]
pub fn test_from_vec_and_from_slice() {
    fn test(non_repeating: &[u8], out: &str) {
        assert_eq!(RationalSequence::from_slice(non_repeating).to_string(), out);
        assert_eq!(
            RationalSequence::from_vec(non_repeating.to_vec()).to_string(),
            out
        );
    }
    test(&[], "[]");
    test(&[1, 2, 3], "[1, 2, 3]");
}

#[test]
fn from_vec_and_from_slice_properties() {
    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let rs = RationalSequence::from_slice(&xs);
        assert!(rs.is_valid());
        assert_eq!(rs.to_string(), xs.to_debug_string());
        assert!(rs.is_finite());
        assert_eq!(RationalSequence::from_vec(xs), rs);
    });
}
