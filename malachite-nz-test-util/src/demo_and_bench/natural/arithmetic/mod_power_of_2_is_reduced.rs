use crate::bench::bucketers::pair_1_natural_bit_bucketer;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModPowerOf2IsReduced, PowerOf2};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::generators::natural_unsigned_pair_gen_var_4;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_power_of_2_is_reduced);

    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_is_reduced_algorithms
    );
}

fn demo_natural_mod_power_of_2_is_reduced(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, log_base) in natural_unsigned_pair_gen_var_4()
        .get(gm, &config)
        .take(limit)
    {
        if n.mod_power_of_2_is_reduced(log_base) {
            println!("{} is reduced mod 2^{}", n, log_base);
        } else {
            println!("{} is not reduced mod 2^{}", n, log_base);
        }
    }
}

fn benchmark_natural_mod_power_of_2_is_reduced_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_power_of_2_add_limb(&[Limb], Limb, u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_4().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            ("default", &mut |(n, log_base)| {
                no_out!(n.mod_power_of_2_is_reduced(log_base))
            }),
            ("using mod_is_reduced", &mut |(n, log_base)| {
                no_out!(n.mod_is_reduced(&Natural::power_of_2(log_base)))
            }),
        ],
    );
}
