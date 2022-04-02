use malachite_base::num::logic::traits::NotAssign;
use malachite_base::test_util::generators::bool_gen;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_bool_not_assign);
}

fn demo_bool_not_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for mut b in bool_gen().get(gm, &config).take(limit) {
        let b_old = b;
        b.not_assign();
        println!("b := {}; b.not_assign(); b = {}", b_old, b);
    }
}
