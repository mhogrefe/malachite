use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode};
use malachite_test::hash::hash;
use malachite_test::inputs::base::rounding_modes;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_rounding_mode_hash);
}

fn demo_rounding_mode_hash(gm: NoSpecialGenerationMode, limit: usize) {
    for rm in rounding_modes(gm).take(limit) {
        println!("hash({}) = {}", rm, hash(&rm));
    }
}
