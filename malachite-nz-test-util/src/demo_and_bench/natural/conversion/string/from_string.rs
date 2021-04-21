use malachite_base::num::conversion::traits::{FromStringBase, WrappingFrom};
use malachite_base_test_util::bench::bucketers::{pair_2_string_len_bucketer, string_len_bucketer};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    string_gen, string_gen_var_3, unsigned_string_pair_gen_var_1, unsigned_string_pair_gen_var_2,
};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::natural::conversion::string::from_string::_from_string_base_naive;
use num::{BigUint, Num};
use std::str::FromStr;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_from_str);
    register_demo!(runner, demo_natural_from_str_targeted);
    register_demo!(runner, demo_natural_from_string_base);
    register_demo!(runner, demo_natural_from_string_base_targeted);
    register_bench!(runner, benchmark_natural_from_str_library_comparison);
    register_bench!(runner, benchmark_natural_from_str_algorithms);
    register_bench!(
        runner,
        benchmark_natural_from_string_base_library_comparison
    );
    register_bench!(runner, benchmark_natural_from_string_base_algorithms);
}

fn demo_natural_from_string_base(gm: GenMode, config: GenConfig, limit: usize) {
    for (base, s) in unsigned_string_pair_gen_var_2()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "Natural::from_string_base({}, {}) = {:?}",
            base,
            s,
            Natural::from_string_base(base, &s)
        );
    }
}

fn demo_natural_from_string_base_targeted(gm: GenMode, config: GenConfig, limit: usize) {
    for (base, s) in unsigned_string_pair_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "Natural::from_string_base({}, {}) = {}",
            base,
            s,
            Natural::from_string_base(base, &s).unwrap()
        );
    }
}

fn demo_natural_from_str(gm: GenMode, config: GenConfig, limit: usize) {
    for s in string_gen().get(gm, &config).take(limit) {
        println!("Natural::from_str({}) = {:?}", s, Natural::from_str(&s));
    }
}

fn demo_natural_from_str_targeted(gm: GenMode, config: GenConfig, limit: usize) {
    for s in string_gen_var_3().get(gm, &config).take(limit) {
        println!(
            "Natural::from_str({}) = {}",
            s,
            Natural::from_str(&s).unwrap()
        );
    }
}

fn benchmark_natural_from_str_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::from_str(&str)",
        BenchmarkType::LibraryComparison,
        string_gen_var_3().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [
            (
                "Malachite",
                &mut |s| no_out!(Natural::from_str(&s).unwrap()),
            ),
            ("num", &mut |s| no_out!(BigUint::from_str(&s).unwrap())),
            ("rug", &mut |s| no_out!(rug::Integer::from_str(&s).unwrap())),
        ],
    );
}

fn benchmark_natural_from_str_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::from_str(&str)",
        BenchmarkType::Algorithms,
        string_gen_var_3().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [
            ("default", &mut |s| no_out!(Natural::from_str(&s).unwrap())),
            ("naive", &mut |s| {
                no_out!(_from_string_base_naive(10, &s).unwrap())
            }),
        ],
    );
}

fn benchmark_natural_from_string_base_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::from_string_base(u64, &str)",
        BenchmarkType::LibraryComparison,
        unsigned_string_pair_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_string_len_bucketer("s"),
        &mut [
            ("Malachite", &mut |(base, s)| {
                no_out!(Natural::from_string_base(base, &s).unwrap())
            }),
            ("num", &mut |(base, s)| {
                no_out!(BigUint::from_str_radix(&s, u32::wrapping_from(base)).unwrap())
            }),
            ("rug", &mut |(base, s)| {
                no_out!(rug::Integer::from_str_radix(&s, i32::wrapping_from(base)).unwrap())
            }),
        ],
    );
}

fn benchmark_natural_from_string_base_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::from_string_base(u64, &str)",
        BenchmarkType::Algorithms,
        unsigned_string_pair_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_string_len_bucketer("s"),
        &mut [
            ("default", &mut |(base, s)| {
                no_out!(Natural::from_string_base(base, &s).unwrap())
            }),
            ("naive", &mut |(base, s)| {
                no_out!(_from_string_base_naive(base, &s).unwrap())
            }),
        ],
    );
}
