use common::LARGE_LIMIT;
use malachite_base::round::RoundingMode;
use malachite_test::common::GenerationMode;
use malachite_test::base::rounding_mode::hash::{hash, select_inputs};

#[test]
#[allow(unknown_lints, clone_on_copy)]
fn hash_properties() {
    // n.hash() == n.clone().hash()
    let one_rounding_mode = |rm: RoundingMode| {
        assert_eq!(hash(&rm), hash(&rm.clone()));
    };

    for rm in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_rounding_mode(rm);
    }

    for rm in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_rounding_mode(rm);
    }
}
