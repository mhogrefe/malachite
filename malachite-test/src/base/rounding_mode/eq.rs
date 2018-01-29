use common::GenerationMode;
use inputs::base::pairs_of_rounding_modes;

pub fn demo_rounding_mode_eq(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_rounding_modes(gm).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}
