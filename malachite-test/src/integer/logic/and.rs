use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{pairs_of_integers, rm_pairs_of_integers};
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use std::cmp::max;
use std::iter::repeat;
use std::u32;

pub fn integer_and_alt_1(x: &Integer, y: &Integer) -> Integer {
    let x_negative = *x < 0;
    let y_negative = *y < 0;
    let bit_zip: Box<Iterator<Item = (bool, bool)>> =
        if x.twos_complement_bits().count() >= y.twos_complement_bits().count() {
            Box::new(
                x.twos_complement_bits()
                    .zip(y.twos_complement_bits().chain(repeat(y_negative))),
            )
        } else {
            Box::new(
                x.twos_complement_bits()
                    .chain(repeat(x_negative))
                    .zip(y.twos_complement_bits()),
            )
        };
    let mut and_bits = Vec::new();
    for (b, c) in bit_zip {
        and_bits.push(b && c);
    }
    and_bits.push(x_negative && y_negative);
    Integer::from_twos_complement_bits_asc(&and_bits)
}

pub fn integer_and_alt_2(x: &Integer, y: &Integer) -> Integer {
    let x_extension = if *x < 0 { u32::MAX } else { 0 };
    let y_extension = if *y < 0 { u32::MAX } else { 0 };
    let limb_zip: Box<Iterator<Item = (u32, u32)>> =
        if x.twos_complement_limbs().count() >= y.twos_complement_limbs().count() {
            Box::new(
                x.twos_complement_limbs()
                    .zip(y.twos_complement_limbs().chain(repeat(y_extension))),
            )
        } else {
            Box::new(
                x.twos_complement_limbs()
                    .chain(repeat(x_extension))
                    .zip(y.twos_complement_limbs()),
            )
        };
    let mut and_limbs = Vec::new();
    for (x, y) in limb_zip {
        and_limbs.push(x & y);
    }
    and_limbs.push(x_extension & y_extension);
    Integer::from_owned_twos_complement_limbs_asc(and_limbs)
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_and_assign);
    register_demo!(registry, demo_integer_and_assign_ref);
    register_demo!(registry, demo_integer_and);
    register_demo!(registry, demo_integer_and_val_ref);
    register_demo!(registry, demo_integer_and_ref_val);
    register_demo!(registry, demo_integer_and_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_and_assign_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_and_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_and_library_comparison);
    register_bench!(registry, Large, benchmark_integer_and_algorithms);
    register_bench!(registry, Large, benchmark_integer_and_evaluation_strategy);
}

fn demo_integer_and_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x &= y.clone();
        println!("x := {}; x &= {}; x = {}", x_old, y, x);
    }
}

fn demo_integer_and_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x &= &y;
        println!("x := {}; x &= &{}; x = {}", x_old, y, x);
    }
}

fn demo_integer_and(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} & {} = {}", x_old, y_old, x & y);
    }
}

fn demo_integer_and_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        println!("{} & &{} = {}", x_old, y, x & &y);
    }
}

fn demo_integer_and_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} & {} = {}", x, y_old, &x & y);
    }
}

fn demo_integer_and_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        println!("&{} & &{} = {}", x, y, &x & &y);
    }
}

fn benchmark_integer_and_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer &= Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x &= y)),
            ("rug", &mut (|((mut x, y), _)| x &= y)),
        ],
    );
}

fn benchmark_integer_and_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer &= Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Integer &= Integer", &mut (|(mut x, y)| no_out!(x &= y))),
            ("Integer &= &Integer", &mut (|(mut x, y)| no_out!(x &= &y))),
        ],
    );
}

fn benchmark_integer_and_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer & Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x & y))),
            ("rug", &mut (|((x, y), _)| no_out!(x & y))),
        ],
    );
}

fn benchmark_integer_and_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer & Integer",
        BenchmarkType::Algorithms,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("default", &mut (|(ref x, ref y)| no_out!(x & y))),
            (
                "using bits explicitly",
                &mut (|(ref x, ref y)| no_out!(integer_and_alt_1(x, y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(ref x, ref y)| no_out!(integer_and_alt_2(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_and_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer & Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Integer & Integer", &mut (|(x, y)| no_out!(x & y))),
            ("Integer & &Integer", &mut (|(x, y)| no_out!(x & &y))),
            ("&Integer & Integer", &mut (|(x, y)| no_out!(&x & y))),
            ("&Integer & &Integer", &mut (|(x, y)| no_out!(&x & &y))),
        ],
    );
}
