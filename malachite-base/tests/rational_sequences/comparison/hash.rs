use malachite_base::test_util::generators::unsigned_rational_sequence_gen;
use malachite_base::test_util::hash::hash;

#[test]
fn hash_properties() {
    unsigned_rational_sequence_gen::<u8>().test_properties(|xs| {
        assert_eq!(hash(&xs), hash(&xs.clone()));
    });
}
