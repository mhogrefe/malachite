use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{rounding_mode_gen, rounding_mode_pair_gen};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rounding_mode_clone);
    register_demo!(runner, demo_rounding_mode_clone_from);
}

fn demo_rounding_mode_clone(gm: GenMode, config: GenConfig, limit: usize) {
    for rm in rounding_mode_gen().get(gm, &config).take(limit) {
        println!("clone({}) = {}", rm, rm.clone());
    }
}

fn demo_rounding_mode_clone_from(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, y) in rounding_mode_pair_gen().get(gm, &config).take(limit) {
        let x_old = x;
        x.clone_from(&y);
        println!("x := {}; x.clone_from({}); x = {}", x_old, y, x);
    }
}
