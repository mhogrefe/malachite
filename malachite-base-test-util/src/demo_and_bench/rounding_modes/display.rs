use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::rounding_mode_gen;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rounding_mode_display);
}

fn demo_rounding_mode_display(gm: GenMode, config: GenConfig, limit: usize) {
    for rm in rounding_mode_gen().get(gm, &config).take(limit) {
        println!("{}", rm);
    }
}
