use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{integers, nrm_integers};
use malachite_base::num::SignificantBits;
use malachite_base::num::AbsAssign;
use num::Signed;

pub fn demo_integer_abs_assign(gm: GenerationMode, limit: usize) {
    for mut n in integers(gm).take(limit) {
        let n_old = n.clone();
        n.abs_assign();
        println!("n := {}; n.abs_assign(); n = {}", n_old, n);
    }
}

pub fn demo_integer_abs(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("abs({}) = {}", n.clone(), n.abs());
    }
}

pub fn demo_integer_abs_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("abs_ref(&{}) = {}", n, n.abs_ref());
    }
}

pub fn demo_integer_natural_abs(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("natural_abs({}) = {}", n.clone(), n.natural_abs());
    }
}

pub fn demo_integer_natural_abs_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("natural_abs_ref(&{}) = {}", n, n.natural_abs_ref());
    }
}

pub fn benchmark_integer_abs_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.abs_assign()",
        BenchmarkType::Ordinary,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[("malachite", &mut (|mut n| n.abs_assign()))],
    );
}

pub fn benchmark_integer_abs(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.abs()",
        BenchmarkType::Ordinary,
        nrm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(_, _, n)| no_out!(n.abs()))),
            ("num", &mut (|(n, _, _)| no_out!(n.abs()))),
            ("rug", &mut (|(_, n, _)| no_out!(n.abs().cmp0()))),
        ],
    );
}

pub fn benchmark_integer_abs_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.abs()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("Integer.abs()", &mut (|n| no_out!(n.abs()))),
            ("Integer.abs_ref()", &mut (|n| no_out!(n.abs_ref()))),
        ],
    );
}

pub fn benchmark_integer_natural_abs(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.natural_abs()",
        BenchmarkType::Ordinary,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[("malachite", &mut (|n| no_out!(n.natural_abs())))],
    );
}

pub fn benchmark_integer_natural_abs_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.natural_abs()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("Integer.natural_abs()", &mut (|n| no_out!(n.natural_abs()))),
            (
                "Integer.natural_abs_ref()",
                &mut (|n| no_out!(n.natural_abs_ref())),
            ),
        ],
    );
}
