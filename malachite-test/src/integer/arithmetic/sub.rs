use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{nrm_pairs_of_integers, pairs_of_integers, rm_pairs_of_integers};
use malachite_base::num::SignificantBits;
use std::cmp::max;

pub fn demo_integer_sub_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x -= y.clone();
        println!("x := {}; x -= {}; x = {}", x_old, y, x);
    }
}

pub fn demo_integer_sub_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x -= &y;
        println!("x := {}; x -= &{}; x = {}", x_old, y, x);
    }
}

pub fn demo_integer_sub(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} = {}", x_old, y_old, x - y);
    }
}

pub fn demo_integer_sub_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {}", x_old, y, x - &y);
    }
}

pub fn demo_integer_sub_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} - {} = {}", x, y_old, &x - y);
    }
}

pub fn demo_integer_sub_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        println!("&{} - &{} = {}", x, y, &x - &y);
    }
}

pub fn benchmark_integer_sub_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer -= Integer",
        BenchmarkType::Ordinary,
        rm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &[
            ("malachite", &mut (|(_, (mut x, y))| x -= y)),
            ("rug", &mut (|((mut x, y), _)| x -= y)),
        ],
    );
}

pub fn benchmark_integer_sub_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer -= Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &[
            ("Integer -= Integer", &mut (|(mut x, y)| no_out!(x -= y))),
            ("Integer -= &Integer", &mut (|(mut x, y)| no_out!(x -= &y))),
        ],
    );
}

pub fn benchmark_integer_sub(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer - Integer",
        BenchmarkType::Ordinary,
        nrm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &[
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x - y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x - y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x - y))),
        ],
    );
}

pub fn benchmark_integer_sub_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer - Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &[
            ("Integer - Integer", &mut (|(x, y)| no_out!(x - y))),
            ("Integer - &Integer", &mut (|(x, y)| no_out!(x - &y))),
            ("&Integer - Integer", &mut (|(x, y)| no_out!(&x - y))),
            ("&Integer - &Integer", &mut (|(x, y)| no_out!(&x - &y))),
        ],
    );
}
