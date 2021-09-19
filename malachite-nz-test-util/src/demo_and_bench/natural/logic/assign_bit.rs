use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base_test_util::bench::bucketers::pair_2_triple_2_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz_test_util::generators::{
    natural_unsigned_bool_triple_gen_var_1, natural_unsigned_bool_triple_gen_var_1_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_assign_bit);
    register_bench!(runner, benchmark_natural_assign_bit_library_comparison);
}

fn demo_natural_assign_bit(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut n, index, bit) in natural_unsigned_bool_triple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        let n_old = n.clone();
        n.assign_bit(index, bit);
        println!(
            "x := {}; x.assign_bit({}, {}); x = {}",
            n_old, index, bit, n
        );
    }
}

fn benchmark_natural_assign_bit_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.assign_bit(u64, bool)",
        BenchmarkType::LibraryComparison,
        natural_unsigned_bool_triple_gen_var_1_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_2_bucketer("index"),
        &mut [
            ("Malachite", &mut |(_, (mut n, index, bit))| {
                n.assign_bit(index, bit)
            }),
            ("rug", &mut |((mut n, index, bit), _)| {
                no_out!(n.set_bit(u32::exact_from(index), bit))
            }),
        ],
    );
}
