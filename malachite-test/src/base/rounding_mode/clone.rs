use common::NoSpecialGenerationMode;
use inputs::base::{pairs_of_rounding_modes, rounding_modes};

#[allow(unknown_lints, clone_on_copy)]
pub fn demo_rounding_mode_clone(gm: NoSpecialGenerationMode, limit: usize) {
    for rm in rounding_modes(gm).take(limit) {
        println!("clone({}) = {}", rm, rm.clone());
    }
}

#[allow(unknown_lints, clone_on_copy)]
pub fn demo_rounding_mode_clone_from(gm: NoSpecialGenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_rounding_modes(gm).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {}; x.clone_from({}); x = {}", x_old, y, x);
    }
}
