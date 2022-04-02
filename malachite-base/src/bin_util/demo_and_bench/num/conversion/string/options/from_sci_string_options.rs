use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    from_sci_string_options_gen, from_sci_string_options_rounding_mode_pair_gen,
    from_sci_string_options_unsigned_pair_gen_var_1,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_from_sci_string_options_to_debug_string);
    register_demo!(runner, demo_from_sci_string_options_get_base);
    register_demo!(runner, demo_from_sci_string_options_get_rounding_mode);
    register_demo!(runner, demo_from_sci_string_options_set_base);
    register_demo!(runner, demo_from_sci_string_options_set_rounding_mode);
}

fn demo_from_sci_string_options_to_debug_string(gm: GenMode, config: GenConfig, limit: usize) {
    for options in from_sci_string_options_gen().get(gm, &config).take(limit) {
        println!("{:?}", options);
    }
}

fn demo_from_sci_string_options_get_base(gm: GenMode, config: GenConfig, limit: usize) {
    for options in from_sci_string_options_gen().get(gm, &config).take(limit) {
        println!("get_base({:?}) = {}", options, options.get_base());
    }
}

fn demo_from_sci_string_options_get_rounding_mode(gm: GenMode, config: GenConfig, limit: usize) {
    for options in from_sci_string_options_gen().get(gm, &config).take(limit) {
        println!(
            "get_rounding_mode({:?}) = {}",
            options,
            options.get_rounding_mode()
        );
    }
}

fn demo_from_sci_string_options_set_base(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut options, base) in from_sci_string_options_unsigned_pair_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        let old_options = options;
        options.set_base(base);
        println!(
            "options := {:?}; options.set_base({}); options = {:?}",
            old_options, base, options
        );
    }
}

fn demo_from_sci_string_options_set_rounding_mode(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut options, rounding_mode) in from_sci_string_options_rounding_mode_pair_gen()
        .get(gm, &config)
        .take(limit)
    {
        let old_options = options;
        options.set_rounding_mode(rounding_mode);
        println!(
            "options := {:?}; options.set_rounding_mode({}); options = {:?}",
            old_options, rounding_mode, options
        );
    }
}
