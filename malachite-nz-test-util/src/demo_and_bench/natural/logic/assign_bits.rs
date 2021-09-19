use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base_test_util::bench::bucketers::quadruple_3_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::large_type_gen_var_3;
use malachite_base_test_util::num::logic::bit_block_access::assign_bits_naive;
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::logic::bit_block_access::limbs_assign_bits;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::generators::natural_unsigned_unsigned_natural_quadruple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_assign_bits);
    register_demo!(runner, demo_natural_assign_bits);
    register_bench!(runner, benchmark_limbs_assign_bits);
    register_bench!(runner, benchmark_natural_assign_bits_algorithms);
}

fn demo_limbs_assign_bits(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, start, end, bits) in large_type_gen_var_3().get(gm, &config).take(limit) {
        let old_xs = xs.clone();
        limbs_assign_bits(&mut xs, start, end, &bits);
        println!(
            "xs := {:?}; limbs_assign_bits(&mut xs, {}, {}, &{:?}); limbs = {:?}",
            old_xs, start, end, bits, xs
        );
    }
}

fn demo_natural_assign_bits(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut n, start, end, bits) in natural_unsigned_unsigned_natural_quadruple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        let old_n = n.clone();
        n.assign_bits(start, end, &bits);
        println!(
            "n := {}; n.assign_bits({}, {}, &{}); n = {}",
            old_n, start, end, bits, n
        );
    }
}

fn benchmark_limbs_assign_bits(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_assign_bits(&mut Vec<Limb>, u64, u64, &[Limb])",
        BenchmarkType::Single,
        large_type_gen_var_3().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("end"),
        &mut [("limbs_assign_bits", &mut |(
            ref mut limbs,
            start,
            end,
            ref bits,
        )| {
            limbs_assign_bits(limbs, start, end, bits)
        })],
    );
}

fn benchmark_natural_assign_bits_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.assign_bits(u64, u64, &Natural)",
        BenchmarkType::Algorithms,
        natural_unsigned_unsigned_natural_quadruple_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("end"),
        &mut [
            ("default", &mut |(mut n, start, end, bits)| {
                n.assign_bits(start, end, &bits)
            }),
            ("naive", &mut |(mut n, start, end, bits)| {
                assign_bits_naive::<Natural, Natural>(&mut n, start, end, &bits)
            }),
        ],
    );
}
