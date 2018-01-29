use common::LARGE_LIMIT;
use malachite_nz::natural::Natural;
use malachite_test::common::GenerationMode;
use malachite_test::hash::hash;
use malachite_test::inputs::natural::naturals;

#[test]
fn hash_properties() {
    // n.hash() == n.clone().hash()
    let one_natural = |x: Natural| {
        assert_eq!(hash(&x), hash(&x.clone()));
    };

    for n in naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
