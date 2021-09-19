use malachite_base::num::logic::traits::BitAccess;
use malachite_base_test_util::bench::bucketers::{pair_2_bucketer, pair_2_pair_2_bucketer};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    unsigned_vec_unsigned_pair_gen_var_16, unsigned_vec_unsigned_pair_gen_var_17,
};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::logic::bit_access::{limbs_slice_set_bit, limbs_vec_set_bit};
use malachite_nz_test_util::generators::{
    natural_unsigned_pair_gen_var_4, natural_unsigned_pair_gen_var_4_nm,
};
use malachite_nz_test_util::natural::logic::set_bit::num_set_bit;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_slice_set_bit);
    register_demo!(runner, demo_limbs_vec_set_bit);
    register_demo!(runner, demo_natural_set_bit);

    register_bench!(runner, benchmark_limbs_slice_set_bit);
    register_bench!(runner, benchmark_limbs_vec_set_bit);
    register_bench!(runner, benchmark_natural_set_bit_library_comparison);
}

fn demo_limbs_slice_set_bit(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, index) in unsigned_vec_unsigned_pair_gen_var_17()
        .get(gm, &config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_slice_set_bit(&mut xs, index);
        println!(
            "xs := {:?}; limbs_slice_set_bit(&mut xs, {}); xs = {:?}",
            xs_old, index, xs
        );
    }
}

fn demo_limbs_vec_set_bit(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, index) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, &config)
        .take(limit)
    {
        let old_xs = xs.clone();
        limbs_vec_set_bit(&mut xs, index);
        println!(
            "xs := {:?}; limbs_vec_set_bit(&mut xs, {}); xs = {:?}",
            old_xs, index, xs
        );
    }
}

fn demo_natural_set_bit(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut n, index) in natural_unsigned_pair_gen_var_4()
        .get(gm, &config)
        .take(limit)
    {
        let n_old = n.clone();
        n.set_bit(index);
        println!("x := {}; x.set_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_limbs_slice_set_bit(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_slice_set_bit(&mut [Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_17().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut xs, index)| {
            no_out!(limbs_slice_set_bit(&mut xs, index))
        })],
    );
}

fn benchmark_limbs_vec_set_bit(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_vec_set_bit(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut xs, index)| {
            no_out!(limbs_vec_set_bit(&mut xs, index))
        })],
    );
}

fn benchmark_natural_set_bit_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.set_bit(u64)",
        BenchmarkType::LibraryComparison,
        natural_unsigned_pair_gen_var_4_nm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_2_bucketer("index"),
        &mut [
            ("Malachite", &mut |(_, (mut n, index))| n.set_bit(index)),
            ("num", &mut |((mut n, index), _)| num_set_bit(&mut n, index)),
        ],
    );
}
