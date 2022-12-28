use malachite_base::test_util::generators::bool_gen;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_from_bool);
}

fn demo_rational_from_bool(gm: GenMode, config: GenConfig, limit: usize) {
    for b in bool_gen().get(gm, &config).take(limit) {
        println!("Rational::from({}) = {}", b, Rational::from(b));
    }
}
