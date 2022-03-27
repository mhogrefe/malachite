use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::rounding_mode_gen;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rounding_mode_neg_assign);
    register_demo!(runner, demo_rounding_mode_neg);
}

fn demo_rounding_mode_neg_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for mut rm in rounding_mode_gen().get(gm, &config).take(limit) {
        let rm_old = rm;
        rm.neg_assign();
        println!("rm := {}; r.neg_assign(); rm = {}", rm_old, rm);
    }
}

fn demo_rounding_mode_neg(gm: GenMode, config: GenConfig, limit: usize) {
    for rm in rounding_mode_gen().get(gm, &config).take(limit) {
        println!("-{} = {}", rm, -rm);
    }
}
