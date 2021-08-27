use crate::bench::bucketers::{
    pair_1_natural_bit_bucketer, pair_2_pair_1_natural_bit_bucketer,
    triple_3_pair_1_natural_bit_bucketer,
};
use malachite_base::num::arithmetic::traits::{
    CeilingRoot, CeilingRootAssign, CheckedRoot, FloorRoot, FloorRootAssign, Pow, RootAssignRem,
    RootRem,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::arithmetic::root::{
    _ceiling_root_binary, _checked_root_binary, _floor_root_binary, _root_rem_binary,
};
use malachite_nz_test_util::generators::{
    natural_unsigned_pair_gen_var_7, natural_unsigned_pair_gen_var_7_nrm,
    natural_unsigned_pair_gen_var_7_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_floor_root);
    register_demo!(runner, demo_natural_floor_root_ref);
    register_demo!(runner, demo_natural_floor_root_assign);
    register_demo!(runner, demo_natural_ceiling_root);
    register_demo!(runner, demo_natural_ceiling_root_ref);
    register_demo!(runner, demo_natural_ceiling_root_assign);
    register_demo!(runner, demo_natural_checked_root);
    register_demo!(runner, demo_natural_checked_root_ref);
    register_demo!(runner, demo_natural_root_rem);
    register_demo!(runner, demo_natural_root_rem_ref);
    register_demo!(runner, demo_natural_root_assign_rem);
    register_bench!(runner, benchmark_natural_floor_root_evaluation_strategy);
    register_bench!(runner, benchmark_natural_floor_root_algorithms);
    register_bench!(runner, benchmark_natural_floor_root_library_comparison);
    register_bench!(runner, benchmark_natural_floor_root_assign);
    register_bench!(runner, benchmark_natural_ceiling_root_evaluation_strategy);
    register_bench!(runner, benchmark_natural_ceiling_root_algorithms);
    register_bench!(runner, benchmark_natural_ceiling_root_assign);
    register_bench!(runner, benchmark_natural_checked_root_evaluation_strategy);
    register_bench!(runner, benchmark_natural_checked_root_algorithms);
    register_bench!(runner, benchmark_natural_root_rem_evaluation_strategy);
    register_bench!(runner, benchmark_natural_root_rem_algorithms);
    register_bench!(runner, benchmark_natural_root_rem_library_comparison);
    register_bench!(runner, benchmark_natural_root_assign_rem);
}

fn demo_natural_floor_root(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, &config)
        .take(limit)
    {
        println!("{}.floor_root({}) = {}", x, exp, x.clone().floor_root(exp));
    }
}

fn demo_natural_floor_root_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, &config)
        .take(limit)
    {
        println!("(&{}).floor_root({}) = {}", x, exp, (&x).floor_root(exp));
    }
}

fn demo_natural_floor_root_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, &config)
        .take(limit)
    {
        let old_x = x.clone();
        x.floor_root_assign(exp);
        println!("x := {}; x.floor_root_assign({}); x = {}", old_x, exp, x);
    }
}

fn demo_natural_ceiling_root(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}.ceiling_root({}) = {}",
            x,
            exp,
            x.clone().ceiling_root(exp)
        );
    }
}

fn demo_natural_ceiling_root_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "(&{}).ceiling_root({}) = {}",
            x,
            exp,
            (&x).ceiling_root(exp)
        );
    }
}

fn demo_natural_ceiling_root_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, &config)
        .take(limit)
    {
        let old_x = x.clone();
        x.ceiling_root_assign(exp);
        println!("x := {}; x.ceiling_root_assign({}); x = {}", old_x, exp, x);
    }
}

fn demo_natural_checked_root(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}.checked_root({}) = {:?}",
            x,
            exp,
            x.clone().checked_root(exp)
        );
    }
}

fn demo_natural_checked_root_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "(&{}).checked_root({}) = {:?}",
            x,
            exp,
            (&x).checked_root(exp)
        );
    }
}

fn demo_natural_root_rem(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, &config)
        .take(limit)
    {
        println!("{}.root_rem({}) = {:?}", x, exp, x.clone().root_rem(exp));
    }
}

fn demo_natural_root_rem_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, &config)
        .take(limit)
    {
        println!("(&{}).root_rem({}) = {:?}", x, exp, (&x).root_rem(exp));
    }
}

fn demo_natural_root_assign_rem(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, exp) in natural_unsigned_pair_gen_var_7()
        .get(gm, &config)
        .take(limit)
    {
        let old_x = x.clone();
        let rem = x.root_assign_rem(exp);
        println!(
            "x := {}; x.root_assign_rem({}) = {}; x = {}",
            old_x, exp, rem, x
        );
    }
}

fn benchmark_natural_floor_root_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_root()",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_7().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.floor_root()", &mut |(x, exp)| {
                no_out!(x.floor_root(exp))
            }),
            ("(&Natural).floor_root()", &mut |(x, exp)| {
                no_out!((&x).floor_root(exp))
            }),
        ],
    );
}

fn benchmark_natural_floor_root_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_root()",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_7().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, exp)| no_out!(x.floor_root(exp))),
            ("binary", &mut |(x, exp)| {
                no_out!(_floor_root_binary(&x, exp))
            }),
        ],
    );
}

fn benchmark_natural_floor_root_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_root()",
        BenchmarkType::LibraryComparison,
        natural_unsigned_pair_gen_var_7_nrm::<u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_natural_bit_bucketer("x"),
        &mut [
            ("num", &mut |((x, exp), _, _)| {
                no_out!(x.nth_root(u32::exact_from(exp)))
            }),
            ("rug", &mut |(_, (x, exp), _)| {
                no_out!(x.root(u32::exact_from(exp)))
            }),
            ("Malachite", &mut |(_, _, (x, exp))| {
                no_out!(x.floor_root(exp))
            }),
        ],
    );
}

fn benchmark_natural_floor_root_assign(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_root_assign()",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_7().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, exp)| x.floor_root_assign(exp))],
    );
}

fn benchmark_natural_ceiling_root_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_root()",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_7().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.ceiling_root()", &mut |(x, exp)| {
                no_out!(x.ceiling_root(exp))
            }),
            ("(&Natural).ceiling_root()", &mut |(x, exp)| {
                no_out!((&x).ceiling_root(exp))
            }),
        ],
    );
}

fn benchmark_natural_ceiling_root_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_root()",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_7().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, exp)| no_out!(x.ceiling_root(exp))),
            ("binary", &mut |(x, exp)| {
                no_out!(_ceiling_root_binary(&x, exp))
            }),
        ],
    );
}

fn benchmark_natural_ceiling_root_assign(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_root_assign()",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_7().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, exp)| x.ceiling_root_assign(exp))],
    );
}

fn benchmark_natural_checked_root_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_root()",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_7().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.checked_root()", &mut |(x, exp)| {
                no_out!(x.checked_root(exp))
            }),
            ("(&Natural).checked_root()", &mut |(x, exp)| {
                no_out!((&x).checked_root(exp))
            }),
        ],
    );
}

fn benchmark_natural_checked_root_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_root()",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_7().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, exp)| no_out!(x.checked_root(exp))),
            ("binary", &mut |(x, exp)| {
                no_out!(_checked_root_binary(&x, exp))
            }),
        ],
    );
}

fn benchmark_natural_root_rem_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.root_rem()",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_7().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.root_rem()", &mut |(x, exp)| {
                no_out!(x.root_rem(exp))
            }),
            ("(&Natural).root_rem()", &mut |(x, exp)| {
                no_out!((&x).root_rem(exp))
            }),
        ],
    );
}

#[allow(clippy::no_effect)]
fn benchmark_natural_root_rem_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.root_rem()",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_7().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, exp)| no_out!(x.root_rem(exp))),
            ("floor and subtraction", &mut |(x, exp)| {
                let root = (&x).floor_root(exp);
                let pow = (&root).pow(exp);
                (root, x - pow);
            }),
            ("binary", &mut |(x, exp)| no_out!(_root_rem_binary(&x, exp))),
        ],
    );
}

fn benchmark_natural_root_rem_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.root_rem()",
        BenchmarkType::LibraryComparison,
        natural_unsigned_pair_gen_var_7_rm::<u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_natural_bit_bucketer("x"),
        &mut [
            ("rug", &mut |((x, exp), _)| {
                no_out!(x.root_rem(rug::Integer::new(), u32::exact_from(exp)))
            }),
            ("Malachite", &mut |(_, (x, exp))| no_out!(x.root_rem(exp))),
        ],
    );
}

fn benchmark_natural_root_assign_rem(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.root_assign_rem()",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_7().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, exp)| {
            no_out!(x.root_assign_rem(exp))
        })],
    );
}
