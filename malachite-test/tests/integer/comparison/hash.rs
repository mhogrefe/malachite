use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_test::common::GenerationMode;
use malachite_test::hash::hash;
use malachite_test::inputs::integer::integers;

#[test]
fn hash_properties() {
    // n.hash() == n.clone().hash()
    let one_integer = |x: Integer| {
        assert_eq!(hash(&x), hash(&x.clone()));
    };

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
