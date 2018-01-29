use common::GenerationMode;
use inputs::base::rounding_modes;

pub fn demo_rounding_mode_neg(gm: GenerationMode, limit: usize) {
    for rm in rounding_modes(gm).take(limit) {
        println!("-{} = {}", rm, -rm);
    }
}
