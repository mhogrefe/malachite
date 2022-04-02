use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    pair_2_pair_integer_max_bit_bucketer, pair_integer_max_bit_bucketer,
    triple_3_pair_integer_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    integer_pair_gen, integer_pair_gen_nrm, integer_pair_gen_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_add);
    register_demo!(runner, demo_integer_add_val_ref);
    register_demo!(runner, demo_integer_add_ref_val);
    register_demo!(runner, demo_integer_add_ref_ref);
    register_demo!(runner, demo_integer_add_assign);
    register_demo!(runner, demo_integer_add_assign_ref);

    register_bench!(runner, benchmark_integer_add_library_comparison);
    register_bench!(runner, benchmark_integer_add_evaluation_strategy);
    register_bench!(runner, benchmark_integer_add_assign_library_comparison);
    register_bench!(runner, benchmark_integer_add_assign_evaluation_strategy);
}

fn demo_integer_add(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} + {} = {}", x_old, y_old, x + y);
    }
}

fn demo_integer_add_val_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        println!("{} + &{} = {}", x_old, y, x + &y);
    }
}

fn demo_integer_add_ref_val(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, &config).take(limit) {
        let y_old = y.clone();
        println!("&{} + {} = {}", x, y_old, &x + y);
    }
}

fn demo_integer_add_ref_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, &config).take(limit) {
        println!("&{} + &{} = {}", x, y, &x + &y);
    }
}

fn demo_integer_add_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        x += y.clone();
        println!("x := {}; x += {}; x = {}", x_old, y, x);
    }
}

fn demo_integer_add_assign_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        x += &y;
        println!("x := {}; x += &{}; x = {}", x_old, y, x);
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_integer_add_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer + Integer",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x + y)),
            ("num", &mut |((x, y), _, _)| no_out!(x + y)),
            ("rug", &mut |(_, (x, y), _)| no_out!(x + y)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_add_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer + Integer",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Integer + Integer", &mut |(x, y)| no_out!(x + y)),
            ("Integer + &Integer", &mut |(x, y)| no_out!(x + &y)),
            ("&Integer + Integer", &mut |(x, y)| no_out!(&x + y)),
            ("&Integer + &Integer", &mut |(x, y)| no_out!(&x + &y)),
        ],
    );
}

fn benchmark_integer_add_assign_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer += Integer",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_integer_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(_, (mut x, y))| x += y), ("rug", &mut |((mut x, y), _)| x += y)],
    );
}

fn benchmark_integer_add_assign_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer += Integer",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Integer += Integer", &mut |(mut x, y)| no_out!(x += y)),
            ("Integer += &Integer", &mut |(mut x, y)| no_out!(x += &y)),
        ],
    );
}
