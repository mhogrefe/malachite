use malachite_base::num::logic::traits::BitAccess;
use malachite_base_test_util::bench::bucketers::pair_2_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_vec_unsigned_pair_gen_var_20;
use malachite_base_test_util::runner::Runner;
use malachite_nz::integer::logic::bit_access::limbs_set_bit_neg;
use malachite_nz_test_util::generators::integer_unsigned_pair_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_set_bit_neg);
    register_demo!(runner, demo_integer_set_bit);

    register_bench!(runner, benchmark_limbs_set_bit_neg);
    register_bench!(runner, benchmark_integer_set_bit);
}

fn demo_limbs_set_bit_neg(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, index) in unsigned_vec_unsigned_pair_gen_var_20()
        .get(gm, &config)
        .take(limit)
    {
        let old_xs = xs.clone();
        limbs_set_bit_neg(&mut xs, index);
        println!(
            "xs := {:?}; limbs_set_bit_neg(&mut xs, {}); xs = {:?}",
            old_xs, index, xs
        );
    }
}

fn demo_integer_set_bit(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut n, index) in integer_unsigned_pair_gen_var_2()
        .get(gm, &config)
        .take(limit)
    {
        let n_old = n.clone();
        n.set_bit(index);
        println!("x := {}; x.set_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_limbs_set_bit_neg(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_set_bit_neg(&mut [Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_20().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut xs, index)| {
            no_out!(limbs_set_bit_neg(&mut xs, index))
        })],
    );
}

fn benchmark_integer_set_bit(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.set_bit(u64)",
        BenchmarkType::Single,
        integer_unsigned_pair_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut n, index)| n.set_bit(index))],
    );
}
