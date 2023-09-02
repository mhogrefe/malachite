use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::generators::float_gen;
use malachite_float::ComparableFloatRef;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_to_string);
}

fn demo_float_to_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!("to_string({:x}) = {}", ComparableFloatRef(&f), f);
        println!("{}", rug::Float::exact_from(&f));
    }
}
