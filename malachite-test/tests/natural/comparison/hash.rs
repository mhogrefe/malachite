use common::LARGE_LIMIT;
use malachite_nz::natural::Natural;
use malachite_test::common::GenerationMode;
use malachite_test::natural::comparison::hash::{hash, select_inputs};

#[test]
fn hash_properties() {
    // n.hash() == n.clone().hash()
    let one_natural = |x: Natural| {
        assert_eq!(hash(&x), hash(&x.clone()));
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
