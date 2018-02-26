use common::NoSpecialGenerationMode;
use hash::hash;
use inputs::base::rounding_modes;

pub fn demo_rounding_mode_hash(gm: NoSpecialGenerationMode, limit: usize) {
    for rm in rounding_modes(gm).take(limit) {
        println!("hash({}) = {}", rm, hash(&rm));
    }
}
