use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::rounding_mode_gen;
use malachite_base_test_util::hash::hash;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rounding_mode_hash);
}

fn demo_rounding_mode_hash(gm: GenMode, config: GenConfig, limit: usize) {
    for rm in rounding_mode_gen().get(gm, &config).take(limit) {
        println!("hash({}) = {}", rm, hash(&rm));
    }
}
