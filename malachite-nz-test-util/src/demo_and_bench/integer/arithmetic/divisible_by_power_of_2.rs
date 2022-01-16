use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz_test_util::bench::bucketers::{
    pair_1_integer_bit_bucketer, pair_2_pair_1_integer_bit_bucketer,
};
use malachite_nz_test_util::generators::{
    integer_unsigned_pair_gen_var_2, integer_unsigned_pair_gen_var_2_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_divisible_by_power_of_2);

    register_bench!(
        runner,
        benchmark_integer_divisible_by_power_of_2_library_comparison
    );
    register_bench!(runner, benchmark_integer_divisible_by_power_of_2_algorithms);
}

fn demo_integer_divisible_by_power_of_2(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, pow) in integer_unsigned_pair_gen_var_2()
        .get(gm, &config)
        .take(limit)
    {
        if n.divisible_by_power_of_2(pow) {
            println!("{} is divisible by 2^{}", n, pow);
        } else {
            println!("{} is not divisible by 2^{}", n, pow);
        }
    }
}

fn benchmark_integer_divisible_by_power_of_2_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.divisible_by_power_of_2(u64)",
        BenchmarkType::LibraryComparison,
        integer_unsigned_pair_gen_var_2_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (n, pow))| {
                no_out!(n.divisible_by_power_of_2(pow))
            }),
            ("rug", &mut |((n, pow), _)| {
                n.is_divisible_2pow(u32::exact_from(pow));
            }),
        ],
    );
}

fn benchmark_integer_divisible_by_power_of_2_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.divisible_by_power_of_2(u64)",
        BenchmarkType::Algorithms,
        integer_unsigned_pair_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.divisible_by_power_of_2(u64)", &mut |(n, pow)| {
                no_out!(n.divisible_by_power_of_2(pow))
            }),
            (
                "Integer.trailing_zeros().map_or(true, |z| z >= u64)",
                &mut |(n, pow)| no_out!(n.trailing_zeros().map_or(true, |z| z >= pow)),
            ),
        ],
    );
}
