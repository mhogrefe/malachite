use malachite_base_test_util::hash::hash;
use malachite_nz_test_util::generators::natural_gen;

#[test]
fn hash_properties() {
    natural_gen().test_properties(|x| {
        assert_eq!(hash(&x), hash(&x.clone()));
    });
}
