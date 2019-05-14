use common::{DemoBenchRegistry, NoSpecialGenerationMode};
use hash::hash;
use inputs::base::rounding_modes;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_rounding_mode_hash);
}

fn demo_rounding_mode_hash(gm: NoSpecialGenerationMode, limit: usize) {
    for rm in rounding_modes(gm).take(limit) {
        println!("hash({}) = {}", rm, hash(&rm));
    }
}
