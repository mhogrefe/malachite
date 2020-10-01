use std::cmp::max;

use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::{pairs_of_integer_and_natural, pairs_of_natural_and_integer};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_lt_abs_natural);
    register_demo!(registry, demo_natural_lt_abs_integer);
    register_demo!(registry, demo_integer_gt_abs_natural);
    register_demo!(registry, demo_natural_gt_abs_integer);
    register_demo!(registry, demo_integer_le_abs_natural);
    register_demo!(registry, demo_natural_le_abs_integer);
    register_demo!(registry, demo_integer_ge_abs_natural);
    register_demo!(registry, demo_natural_ge_abs_integer);
    register_bench!(registry, Large, benchmark_integer_lt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_lt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_gt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_gt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_le_abs_natural);
    register_bench!(registry, Large, benchmark_natural_le_abs_integer);
    register_bench!(registry, Large, benchmark_integer_ge_abs_natural);
    register_bench!(registry, Large, benchmark_natural_ge_abs_integer);
}

fn demo_integer_lt_abs_natural(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        println!("{}.lt_abs(&{}) = {}", x, y, x.lt_abs(&y));
    }
}

fn demo_natural_lt_abs_integer(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        println!("{}.lt_abs(&{}) = {}", x, y, x.lt_abs(&y));
    }
}

fn demo_integer_gt_abs_natural(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        println!("{}.gt_abs(&{}) = {}", x, y, x.gt_abs(&y));
    }
}

fn demo_natural_gt_abs_integer(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        println!("{}.gt_abs(&{}) = {}", x, y, x.gt_abs(&y));
    }
}

fn demo_integer_le_abs_natural(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        println!("{}.le_abs(&{}) = {}", x, y, x.le_abs(&y));
    }
}

fn demo_natural_le_abs_integer(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        println!("{}.le_abs(&{}) = {}", x, y, x.le_abs(&y));
    }
}

fn demo_integer_ge_abs_natural(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        println!("{}.ge_abs(&{}) = {}", x, y, x.ge_abs(&y));
    }
}

fn demo_natural_ge_abs_integer(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        println!("{}.ge_abs(&{}) = {}", x, y, x.ge_abs(&y));
    }
}

fn benchmark_integer_lt_abs_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.lt_abs(&Natural)",
        BenchmarkType::Single,
        pairs_of_integer_and_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.lt_abs(&y))))],
    );
}

fn benchmark_natural_lt_abs_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.lt_abs(&Integer)",
        BenchmarkType::Single,
        pairs_of_natural_and_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.lt_abs(&y))))],
    );
}

fn benchmark_integer_gt_abs_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.gt_abs(&Natural)",
        BenchmarkType::Single,
        pairs_of_integer_and_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.gt_abs(&y))))],
    );
}

fn benchmark_natural_gt_abs_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.gt_abs(&Integer)",
        BenchmarkType::Single,
        pairs_of_natural_and_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.gt_abs(&y))))],
    );
}

fn benchmark_integer_le_abs_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.le_abs(&Natural)",
        BenchmarkType::Single,
        pairs_of_integer_and_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.le_abs(&y))))],
    );
}

fn benchmark_natural_le_abs_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.le_abs(&Integer)",
        BenchmarkType::Single,
        pairs_of_natural_and_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.le_abs(&y))))],
    );
}

fn benchmark_integer_ge_abs_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.ge_abs(&Natural)",
        BenchmarkType::Single,
        pairs_of_integer_and_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.ge_abs(&y))))],
    );
}

fn benchmark_natural_ge_abs_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.ge_abs(&Integer)",
        BenchmarkType::Single,
        pairs_of_natural_and_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("Malachite", &mut (|(x, y)| no_out!(x.ge_abs(&y))))],
    );
}
