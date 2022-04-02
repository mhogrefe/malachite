use malachite_base::test_util::hash::hash;
use malachite_q::test_util::generators::rational_gen;

#[test]
fn hash_properties() {
    rational_gen().test_properties(|x| {
        assert_eq!(hash(&x), hash(&x.clone()));
    });
}
