use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    to_sci_options_bool_pair_gen, to_sci_options_gen, to_sci_options_rounding_mode_pair_gen,
    to_sci_options_signed_pair_gen_var_1, to_sci_options_unsigned_pair_gen_var_1,
    to_sci_options_unsigned_pair_gen_var_2, to_sci_options_unsigned_pair_gen_var_3,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_to_sci_options_to_debug_string);
    register_demo!(runner, demo_to_sci_options_get_base);
    register_demo!(runner, demo_to_sci_options_get_rounding_mode);
    register_demo!(runner, demo_to_sci_options_get_size_options);
    register_demo!(runner, demo_to_sci_options_get_neg_exp_threshold);
    register_demo!(runner, demo_to_sci_options_get_lowercase);
    register_demo!(runner, demo_to_sci_options_get_e_lowercase);
    register_demo!(runner, demo_to_sci_options_get_force_exponent_plus_sign);
    register_demo!(runner, demo_to_sci_options_get_include_trailing_zeros);
    register_demo!(runner, demo_to_sci_options_set_base);
    register_demo!(runner, demo_to_sci_options_set_rounding_mode);
    register_demo!(runner, demo_to_sci_options_set_size_complete);
    register_demo!(runner, demo_to_sci_options_set_precision);
    register_demo!(runner, demo_to_sci_options_set_scale);
    register_demo!(runner, demo_to_sci_options_set_neg_exp_threshold);
    register_demo!(runner, demo_to_sci_options_set_lowercase);
    register_demo!(runner, demo_to_sci_options_set_uppercase);
    register_demo!(runner, demo_to_sci_options_set_e_lowercase);
    register_demo!(runner, demo_to_sci_options_set_e_uppercase);
    register_demo!(runner, demo_to_sci_options_set_force_exponent_plus_sign);
    register_demo!(runner, demo_to_sci_options_set_include_trailing_zeros);
}

fn demo_to_sci_options_to_debug_string(gm: GenMode, config: GenConfig, limit: usize) {
    for options in to_sci_options_gen().get(gm, &config).take(limit) {
        println!("{:?}", options);
    }
}

fn demo_to_sci_options_get_base(gm: GenMode, config: GenConfig, limit: usize) {
    for options in to_sci_options_gen().get(gm, &config).take(limit) {
        println!("get_base({:?}) = {}", options, options.get_base());
    }
}

fn demo_to_sci_options_get_rounding_mode(gm: GenMode, config: GenConfig, limit: usize) {
    for options in to_sci_options_gen().get(gm, &config).take(limit) {
        println!(
            "get_rounding_mode({:?}) = {}",
            options,
            options.get_rounding_mode()
        );
    }
}

fn demo_to_sci_options_get_size_options(gm: GenMode, config: GenConfig, limit: usize) {
    for options in to_sci_options_gen().get(gm, &config).take(limit) {
        println!(
            "get_size_options({:?}) = {:?}",
            options,
            options.get_size_options()
        );
    }
}

fn demo_to_sci_options_get_neg_exp_threshold(gm: GenMode, config: GenConfig, limit: usize) {
    for options in to_sci_options_gen().get(gm, &config).take(limit) {
        println!(
            "get_neg_exp_threshold({:?}) = {}",
            options,
            options.get_neg_exp_threshold()
        );
    }
}

fn demo_to_sci_options_get_lowercase(gm: GenMode, config: GenConfig, limit: usize) {
    for options in to_sci_options_gen().get(gm, &config).take(limit) {
        println!("get_lowercase({:?}) = {}", options, options.get_lowercase());
    }
}

fn demo_to_sci_options_get_e_lowercase(gm: GenMode, config: GenConfig, limit: usize) {
    for options in to_sci_options_gen().get(gm, &config).take(limit) {
        println!(
            "get_e_lowercase({:?}) = {}",
            options,
            options.get_e_lowercase()
        );
    }
}

fn demo_to_sci_options_get_force_exponent_plus_sign(gm: GenMode, config: GenConfig, limit: usize) {
    for options in to_sci_options_gen().get(gm, &config).take(limit) {
        println!(
            "get_force_exponent_plus_sign({:?}) = {}",
            options,
            options.get_force_exponent_plus_sign()
        );
    }
}

fn demo_to_sci_options_get_include_trailing_zeros(gm: GenMode, config: GenConfig, limit: usize) {
    for options in to_sci_options_gen().get(gm, &config).take(limit) {
        println!(
            "get_include_trailing_zeros({:?}) = {}",
            options,
            options.get_include_trailing_zeros()
        );
    }
}

fn demo_to_sci_options_set_base(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut options, base) in to_sci_options_unsigned_pair_gen_var_1()
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

fn demo_to_sci_options_set_rounding_mode(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut options, rounding_mode) in to_sci_options_rounding_mode_pair_gen()
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

fn demo_to_sci_options_set_size_complete(gm: GenMode, config: GenConfig, limit: usize) {
    for mut options in to_sci_options_gen().get(gm, &config).take(limit) {
        let old_options = options;
        options.set_size_complete();
        println!(
            "options := {:?}; options.set_size_complete(); options = {:?}",
            old_options, options
        );
    }
}

fn demo_to_sci_options_set_precision(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut options, precision) in to_sci_options_unsigned_pair_gen_var_3()
        .get(gm, &config)
        .take(limit)
    {
        let old_options = options;
        options.set_precision(precision);
        println!(
            "options := {:?}; options.set_precision({}); options = {:?}",
            old_options, precision, options
        );
    }
}

fn demo_to_sci_options_set_scale(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut options, scale) in to_sci_options_unsigned_pair_gen_var_2()
        .get(gm, &config)
        .take(limit)
    {
        let old_options = options;
        options.set_scale(scale);
        println!(
            "options := {:?}; options.set_scale({}); options = {:?}",
            old_options, scale, options
        );
    }
}

fn demo_to_sci_options_set_neg_exp_threshold(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut options, neg_exp_threshold) in to_sci_options_signed_pair_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        let old_options = options;
        options.set_neg_exp_threshold(neg_exp_threshold);
        println!(
            "options := {:?}; options.set_neg_exp_threshold({}); options = {:?}",
            old_options, neg_exp_threshold, options
        );
    }
}

fn demo_to_sci_options_set_lowercase(gm: GenMode, config: GenConfig, limit: usize) {
    for mut options in to_sci_options_gen().get(gm, &config).take(limit) {
        let old_options = options;
        options.set_lowercase();
        println!(
            "options := {:?}; options.set_lowercase(); options = {:?}",
            old_options, options
        );
    }
}

fn demo_to_sci_options_set_uppercase(gm: GenMode, config: GenConfig, limit: usize) {
    for mut options in to_sci_options_gen().get(gm, &config).take(limit) {
        let old_options = options;
        options.set_uppercase();
        println!(
            "options := {:?}; options.set_uppercase(); options = {:?}",
            old_options, options
        );
    }
}

fn demo_to_sci_options_set_e_lowercase(gm: GenMode, config: GenConfig, limit: usize) {
    for mut options in to_sci_options_gen().get(gm, &config).take(limit) {
        let old_options = options;
        options.set_e_lowercase();
        println!(
            "options := {:?}; options.set_e_lowercase(); options = {:?}",
            old_options, options
        );
    }
}

fn demo_to_sci_options_set_e_uppercase(gm: GenMode, config: GenConfig, limit: usize) {
    for mut options in to_sci_options_gen().get(gm, &config).take(limit) {
        let old_options = options;
        options.set_e_uppercase();
        println!(
            "options := {:?}; options.set_e_uppercase(); options = {:?}",
            old_options, options
        );
    }
}

fn demo_to_sci_options_set_force_exponent_plus_sign(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut options, force_exponent_plus_sign) in
        to_sci_options_bool_pair_gen().get(gm, &config).take(limit)
    {
        let old_options = options;
        options.set_force_exponent_plus_sign(force_exponent_plus_sign);
        println!(
            "options := {:?}; options.set_force_exponent_plus_sign({}); options = {:?}",
            old_options, force_exponent_plus_sign, options
        );
    }
}

fn demo_to_sci_options_set_include_trailing_zeros(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut options, include_trailing_zeros) in
        to_sci_options_bool_pair_gen().get(gm, &config).take(limit)
    {
        let old_options = options;
        options.set_include_trailing_zeros(include_trailing_zeros);
        println!(
            "options := {:?}; options.set_include_trailing_zeros({}); options = {:?}",
            old_options, include_trailing_zeros, options
        );
    }
}
