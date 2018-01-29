use common::LARGE_LIMIT;
use malachite_base::round::RoundingMode;
use malachite_test::common::GenerationMode;
use malachite_test::hash::hash;
use malachite_test::inputs::base::rounding_modes;

#[test]
#[allow(unknown_lints, clone_on_copy)]
fn hash_properties() {
    // n.hash() == n.clone().hash()
    let one_rounding_mode = |rm: RoundingMode| {
        assert_eq!(hash(&rm), hash(&rm.clone()));
    };

    for rm in rounding_modes(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_rounding_mode(rm);
    }

    for rm in rounding_modes(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_rounding_mode(rm);
    }
}
