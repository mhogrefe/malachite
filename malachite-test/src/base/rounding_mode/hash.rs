use common::GenerationMode;
use hash::hash;
use inputs::base::rounding_modes;

pub fn demo_rounding_mode_hash(gm: GenerationMode, limit: usize) {
    for rm in rounding_modes(gm).take(limit) {
        println!("hash({}) = {}", rm, hash(&rm));
    }
}
