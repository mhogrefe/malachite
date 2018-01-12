use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_test::common::GenerationMode;
use malachite_test::integer::comparison::hash::{hash, select_inputs};

#[test]
fn hash_properties() {
    // n.hash() == n.clone().hash()
    let one_integer = |x: Integer| {
        assert_eq!(hash(&x), hash(&x.clone()));
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
