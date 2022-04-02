use malachite_base::test_util::generators::rounding_mode_gen;
use malachite_base::test_util::hash::hash;

#[test]
fn hash_properties() {
    rounding_mode_gen().test_properties(|rm| {
        assert_eq!(hash(&rm), hash(&rm.clone()));
    });
}
