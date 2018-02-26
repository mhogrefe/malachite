use common::NoSpecialGenerationMode;
use inputs::base::rounding_modes;

pub fn demo_rounding_mode_neg(gm: NoSpecialGenerationMode, limit: usize) {
    for rm in rounding_modes(gm).take(limit) {
        println!("-{} = {}", rm, -rm);
    }
}
