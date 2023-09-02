use malachite_base::num::conversion::string::from_string::digit_from_display_byte;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{unsigned_gen, unsigned_gen_var_10};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_digit_from_display_byte);
    register_demo!(runner, demo_digit_from_display_byte_targeted);
}

fn demo_digit_from_display_byte(gm: GenMode, config: &GenConfig, limit: usize) {
    for b in unsigned_gen().get(gm, config).take(limit) {
        println!(
            "digit_from_display_byte({}) = {:?}",
            b,
            digit_from_display_byte(b)
        );
    }
}

fn demo_digit_from_display_byte_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for b in unsigned_gen_var_10().get(gm, config).take(limit) {
        println!(
            "digit_from_display_byte({}) = {}",
            b,
            digit_from_display_byte(b).unwrap()
        );
    }
}
