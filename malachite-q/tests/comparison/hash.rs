use malachite_base_test_util::hash::hash;
use malachite_q_test_util::generators::rational_gen;

#[test]
fn hash_properties() {
    rational_gen().test_properties(|x| {
        assert_eq!(hash(&x), hash(&x.clone()));
    });
}
