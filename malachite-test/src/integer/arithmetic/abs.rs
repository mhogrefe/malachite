use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_unsigneds;
use inputs::integer::{integers, nrm_integers};
use malachite_base::num::{Abs, AbsAssign, SignificantBits, UnsignedAbs};
use malachite_nz::natural::arithmetic::mul::_limbs_mul_to_out_toom_43_input_sizes_valid;
use num::Signed;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_abs_assign);
    register_demo!(registry, demo_integer_abs);
    register_demo!(registry, demo_integer_abs_ref);
    register_demo!(registry, demo_integer_unsigned_abs);
    register_demo!(registry, demo_integer_unsigned_abs_ref);
    register_demo!(registry, demo_integer_unsigned_abs_ref_out);
    register_bench!(registry, Large, benchmark_integer_abs_assign);
    register_bench!(registry, Large, benchmark_integer_abs_library_comparison);
    register_bench!(registry, Large, benchmark_integer_abs_evaluation_strategy);
    register_bench!(registry, Large, benchmark_integer_unsigned_abs);
    register_bench!(
        registry,
        Large,
        benchmark_integer_unsigned_abs_evaluation_strategy
    );
}

fn demo_integer_abs_assign(gm: GenerationMode, limit: usize) {
    for (xs_len, ys_len) in pairs_of_unsigneds::<u32>(gm).take(limit) {
        // s >= n
        if _limbs_mul_to_out_toom_43_input_sizes_valid(xs_len as usize, ys_len as usize) {
            let n = 1 + if 3 * xs_len >= 4 * ys_len {
                (xs_len - 1) >> 2
            } else {
                (ys_len - 1) / 3
            };
            let s = xs_len - 3 * n;
            let t = ys_len - 2 * n;
            if t + s <= n {
                println!("{} {}", xs_len, ys_len);
            }
        }
    }
    //for mut n in integers(gm).take(limit) {
    //    let n_old = n.clone();
    //    n.abs_assign();
    //    println!("n := {}; n.abs_assign(); n = {}", n_old, n);
    //}
}

fn demo_integer_abs(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("|{}| = {}", n.clone(), n.abs());
    }
}

fn demo_integer_abs_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("|&{}| = {}", n, (&n).abs());
    }
}

fn demo_integer_unsigned_abs(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("unsigned_abs({}) = {}", n.clone(), n.unsigned_abs());
    }
}

fn demo_integer_unsigned_abs_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("unsigned_abs(&{}) = {}", n, (&n).unsigned_abs());
    }
}

fn demo_integer_unsigned_abs_ref_out(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("{}.unsigned_abs_ref() = {}", n, n.unsigned_abs_ref());
    }
}

fn benchmark_integer_abs_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.abs_assign()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|mut n| n.abs_assign()))],
    );
}

fn benchmark_integer_abs_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.abs()",
        BenchmarkType::LibraryComparison,
        nrm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, n)| no_out!(n.abs()))),
            ("num", &mut (|(n, _, _)| no_out!(n.abs()))),
            ("rug", &mut (|(_, n, _)| no_out!(n.abs().cmp0()))),
        ],
    );
}

fn benchmark_integer_abs_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.abs()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Integer.abs()", &mut (|n| no_out!(n.abs()))),
            ("(&Integer).abs()", &mut (|n| no_out!((&n).abs()))),
        ],
    );
}

fn benchmark_integer_unsigned_abs(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.unsigned_abs()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.unsigned_abs())))],
    );
}

fn benchmark_integer_unsigned_abs_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.unsigned_abs()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.unsigned_abs()",
                &mut (|n| no_out!(n.unsigned_abs())),
            ),
            (
                "(&Integer).unsigned_abs()",
                &mut (|n| no_out!((&n).unsigned_abs())),
            ),
            (
                "Integer.unsigned_abs_ref()",
                &mut (|n| no_out!(n.unsigned_abs_ref())),
            ),
        ],
    );
}
