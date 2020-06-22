use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignMod, CeilingDivMod, CeilingMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
    DivRound, Mod,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use num::Integer;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_nonzero_integer, pairs_of_integer_and_nonzero_integer,
    rm_pairs_of_integer_and_nonzero_integer,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_div_assign_mod);
    register_demo!(registry, demo_integer_div_assign_mod_ref);
    register_demo!(registry, demo_integer_div_mod);
    register_demo!(registry, demo_integer_div_mod_val_ref);
    register_demo!(registry, demo_integer_div_mod_ref_val);
    register_demo!(registry, demo_integer_div_mod_ref_ref);
    register_demo!(registry, demo_integer_div_assign_rem);
    register_demo!(registry, demo_integer_div_assign_rem_ref);
    register_demo!(registry, demo_integer_div_rem);
    register_demo!(registry, demo_integer_div_rem_val_ref);
    register_demo!(registry, demo_integer_div_rem_ref_val);
    register_demo!(registry, demo_integer_div_rem_ref_ref);
    register_demo!(registry, demo_integer_ceiling_div_assign_mod);
    register_demo!(registry, demo_integer_ceiling_div_assign_mod_ref);
    register_demo!(registry, demo_integer_ceiling_div_mod);
    register_demo!(registry, demo_integer_ceiling_div_mod_val_ref);
    register_demo!(registry, demo_integer_ceiling_div_mod_ref_val);
    register_demo!(registry, demo_integer_ceiling_div_mod_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_assign_mod_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_mod_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_div_mod_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_mod_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_assign_rem_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_rem_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_div_rem_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_rem_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_assign_mod_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_mod_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_mod_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_mod_evaluation_strategy
    );
}

fn demo_integer_div_assign_mod(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let remainder = x.div_assign_mod(y);
        println!(
            "x := {}; x.div_assign_mod({}) = {}; x = {}",
            x_old, y_old, remainder, x
        );
    }
}

fn demo_integer_div_assign_mod_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        let remainder = x.div_assign_mod(&y);
        println!(
            "x := {}; x.div_assign_mod(&{}) = {}; x = {}",
            x_old, y, remainder, x
        );
    }
}

fn demo_integer_div_mod(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.div_mod({}) = {:?}", x_old, y_old, x.div_mod(y));
    }
}

fn demo_integer_div_mod_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        println!("{}.div_mod(&{}) = {:?}", x_old, y, x.div_mod(&y));
    }
}

fn demo_integer_div_mod_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let y_old = y.clone();
        println!("(&{}).div_mod({}) = {:?}", x, y_old, (&x).div_mod(y));
    }
}

fn demo_integer_div_mod_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        println!("(&{}).div_mod(&{}) = {:?}", x, y, (&x).div_mod(&y));
    }
}

fn demo_integer_div_assign_rem(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let remainder = x.div_assign_rem(y);
        println!(
            "x := {}; x.div_assign_rem({}) = {}; x = {}",
            x_old, y_old, remainder, x
        );
    }
}

fn demo_integer_div_assign_rem_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        let remainder = x.div_assign_rem(&y);
        println!(
            "x := {}; x.div_assign_rem(&{}) = {}; x = {}",
            x_old, y, remainder, x
        );
    }
}

fn demo_integer_div_rem(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.div_rem({}) = {:?}", x_old, y_old, x.div_rem(y));
    }
}

fn demo_integer_div_rem_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        println!("{}.div_rem(&{}) = {:?}", x_old, y, x.div_rem(&y));
    }
}

fn demo_integer_div_rem_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let y_old = y.clone();
        println!("(&{}).div_rem({}) = {:?}", x, y_old, (&x).div_rem(y));
    }
}

fn demo_integer_div_rem_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        println!("(&{}).div_rem(&{}) = {:?}", x, y, (&x).div_rem(&y));
    }
}

fn demo_integer_ceiling_div_assign_mod(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let remainder = x.ceiling_div_assign_mod(y);
        println!(
            "x := {}; x.ceiling_div_assign_mod({}) = {}; x = {}",
            x_old, y_old, remainder, x
        );
    }
}

fn demo_integer_ceiling_div_assign_mod_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        let remainder = x.ceiling_div_assign_mod(&y);
        println!(
            "x := {}; x.ceiling_div_assign_mod(&{}) = {}; x = {}",
            x_old, y, remainder, x
        );
    }
}

fn demo_integer_ceiling_div_mod(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{}.ceiling_div_mod({}) = {:?}",
            x_old,
            y_old,
            x.ceiling_div_mod(y)
        );
    }
}

fn demo_integer_ceiling_div_mod_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        println!(
            "{}.ceiling_div_mod(&{}) = {:?}",
            x_old,
            y,
            x.ceiling_div_mod(&y)
        );
    }
}

fn demo_integer_ceiling_div_mod_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{}).ceiling_div_mod({}) = {:?}",
            x,
            y_old,
            (&x).ceiling_div_mod(y)
        );
    }
}

fn demo_integer_ceiling_div_mod_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        println!(
            "(&{}).ceiling_div_mod(&{}) = {:?}",
            x,
            y,
            (&x).ceiling_div_mod(&y)
        );
    }
}

fn benchmark_integer_div_assign_mod_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_assign_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_assign_mod(Integer)",
                &mut (|(mut x, y)| no_out!(x.div_assign_mod(y))),
            ),
            (
                "Integer.div_mod(&Integer)",
                &mut (|(mut x, y)| no_out!(x.div_assign_mod(&y))),
            ),
        ],
    );
}

fn benchmark_integer_div_mod_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_mod(Integer)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_mod(y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(x.div_mod_floor(&y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.div_rem_floor(y)))),
        ],
    );
}

fn benchmark_integer_div_mod_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_mod(Integer)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.div_mod(y)))),
            (
                "using div_round and mod_op",
                &mut (|(x, y)| no_out!(((&x).div_round(&y, RoundingMode::Floor), x.mod_op(y)))),
            ),
        ],
    );
}

fn benchmark_integer_div_mod_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_mod(Integer)",
                &mut (|(x, y)| no_out!(x.div_mod(y))),
            ),
            (
                "Integer.div_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.div_mod(&y))),
            ),
            (
                "(&Integer).div_mod(Integer)",
                &mut (|(x, y)| no_out!((&x).div_mod(y))),
            ),
            (
                "(&Integer).div_mod(&Integer)",
                &mut (|(x, y)| no_out!((&x).div_mod(&y))),
            ),
        ],
    );
}

fn benchmark_integer_div_assign_rem_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_assign_rem(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_assign_rem(Integer)",
                &mut (|(mut x, y)| no_out!(x.div_assign_rem(y))),
            ),
            (
                "Integer.div_assign_rem(&Integer)",
                &mut (|(mut x, y)| no_out!(x.div_assign_rem(&y))),
            ),
        ],
    );
}

fn benchmark_integer_div_rem_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_rem(Integer)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_rem(y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(x.div_rem(&y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.div_rem(y)))),
        ],
    );
}

fn benchmark_integer_div_rem_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_rem(Integer)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.div_rem(y)))),
            ("using / and %", &mut (|(x, y)| no_out!((&x / &y, x % y)))),
        ],
    );
}

fn benchmark_integer_div_rem_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_rem(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_rem(Integer)",
                &mut (|(x, y)| no_out!(x.div_rem(y))),
            ),
            (
                "Integer.div_rem(&Integer)",
                &mut (|(x, y)| no_out!(x.div_rem(&y))),
            ),
            (
                "(&Integer).div_rem(Integer)",
                &mut (|(x, y)| no_out!((&x).div_rem(y))),
            ),
            (
                "(&Integer).div_rem(&Integer)",
                &mut (|(x, y)| no_out!((&x).div_rem(&y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_div_assign_mod_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_assign_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.ceiling_div_assign_mod(Integer)",
                &mut (|(mut x, y)| no_out!(x.ceiling_div_assign_mod(y))),
            ),
            (
                "Integer.ceiling_div_assign_mod(&Integer)",
                &mut (|(mut x, y)| no_out!(x.ceiling_div_assign_mod(&y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_div_mod_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_mod(Integer)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (x, y))| no_out!(x.ceiling_div_mod(y))),
            ),
            ("rug", &mut (|((x, y), _)| no_out!(x.div_rem_ceil(y)))),
        ],
    );
}

fn benchmark_integer_ceiling_div_mod_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.ceiling_div_mod(Integer)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.ceiling_div_mod(y)))),
            (
                "using div_round and ceiling_mod",
                &mut (|(x, y)| {
                    ((&x).div_round(&y, RoundingMode::Ceiling), x.ceiling_mod(y));
                }),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_div_mod_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.ceiling_div_mod(Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(y))),
            ),
            (
                "Integer.ceiling_div_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(&y))),
            ),
            (
                "(&Integer).ceiling_div_mod(Integer)",
                &mut (|(x, y)| no_out!((&x).ceiling_div_mod(y))),
            ),
            (
                "(&Integer).ceiling_div_mod(&Integer)",
                &mut (|(x, y)| no_out!((&x).ceiling_div_mod(&y))),
            ),
        ],
    );
}
