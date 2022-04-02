use malachite_base::num::conversion::traits::ToSci;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    integer_bit_bucketer, pair_1_integer_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    integer_gen, integer_to_sci_options_pair_gen, integer_to_sci_options_pair_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_to_sci);
    register_demo!(runner, demo_integer_fmt_sci_valid);
    register_demo!(runner, demo_integer_to_sci_with_options);

    register_bench!(runner, benchmark_integer_to_sci);
    register_bench!(runner, benchmark_integer_fmt_sci_valid);
    register_bench!(runner, benchmark_integer_to_sci_with_options);
}

fn demo_integer_to_sci(gm: GenMode, config: GenConfig, limit: usize) {
    for x in integer_gen().get(gm, &config).take(limit) {
        println!("({}).to_sci() = {}", x, x.to_sci().to_string());
    }
}

fn demo_integer_fmt_sci_valid(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, options) in integer_to_sci_options_pair_gen()
        .get(gm, &config)
        .take(limit)
    {
        if x.fmt_sci_valid(options) {
            println!("{} can be converted to sci using {:?}", x, options);
        } else {
            println!("{} cannot be converted to sci using {:?}", x, options);
        }
    }
}

fn demo_integer_to_sci_with_options(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, options) in integer_to_sci_options_pair_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "to_sci_with_options({}, {:?}) = {}",
            x,
            options,
            x.to_sci_with_options(options)
        );
    }
}

fn benchmark_integer_to_sci(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.to_sci()",
        BenchmarkType::Single,
        integer_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [("Malachite", &mut |x| no_out!(x.to_sci().to_string()))],
    );
}

fn benchmark_integer_fmt_sci_valid(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.fmt_sci_valid(ToSciOptions)",
        BenchmarkType::Single,
        integer_to_sci_options_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("n"),
        &mut [("Malachite", &mut |(x, options)| {
            no_out!(x.fmt_sci_valid(options))
        })],
    );
}

fn benchmark_integer_to_sci_with_options(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_sci_with_options(ToSciOptions)",
        BenchmarkType::Single,
        integer_to_sci_options_pair_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("n"),
        &mut [("Malachite", &mut |(x, options)| {
            no_out!(x.to_sci_with_options(options).to_string())
        })],
    );
}
