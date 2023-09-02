use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::rounding_mode_pair_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rounding_mode_eq);
}

fn demo_rounding_mode_eq(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rounding_mode_pair_gen().get(gm, config).take(limit) {
        if x == y {
            println!("{x} = {y}");
        } else {
            println!("{x} ≠ {y}");
        }
    }
}
