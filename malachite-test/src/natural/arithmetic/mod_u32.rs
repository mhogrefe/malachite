use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_unsigned_vec_and_positive_unsigned_var_1;
use inputs::natural::{
    nrm_pairs_of_natural_and_positive_unsigned, pairs_of_natural_and_positive_unsigned,
    pairs_of_unsigned_and_positive_natural,
};
use malachite_base::num::{DivMod, Mod, ModAssign, NegMod, NegModAssign, SignificantBits};
use malachite_nz::natural::arithmetic::mod_u32::limbs_mod_limb;
use num::{BigUint, ToPrimitive};

// For `Natural`s, `mod` is equivalent to `rem`.

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_mod_limb);
    register_demo!(registry, demo_natural_rem_assign_u32);
    register_demo!(registry, demo_natural_rem_u32);
    register_demo!(registry, demo_natural_rem_u32_ref);
    register_demo!(registry, demo_natural_mod_assign_u32);
    register_demo!(registry, demo_natural_mod_u32);
    register_demo!(registry, demo_natural_mod_u32_ref);
    register_demo!(registry, demo_natural_neg_mod_assign_u32);
    register_demo!(registry, demo_natural_neg_mod_u32);
    register_demo!(registry, demo_natural_neg_mod_u32_ref);
    register_demo!(registry, demo_u32_rem_natural);
    register_demo!(registry, demo_u32_rem_natural_ref);
    register_demo!(registry, demo_u32_rem_assign_natural);
    register_demo!(registry, demo_u32_rem_assign_natural_ref);
    register_demo!(registry, demo_u32_mod_natural);
    register_demo!(registry, demo_u32_mod_natural_ref);
    register_demo!(registry, demo_u32_mod_assign_natural);
    register_demo!(registry, demo_u32_mod_assign_natural_ref);
    register_demo!(registry, demo_u32_neg_mod_natural);
    register_demo!(registry, demo_u32_neg_mod_natural_ref);
    register_bench!(registry, Small, benchmark_limbs_mod_limb);
    register_bench!(registry, Large, benchmark_natural_rem_assign_u32);
    register_bench!(
        registry,
        Large,
        benchmark_natural_rem_u32_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_rem_u32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_rem_u32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mod_assign_u32);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_u32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_neg_mod_assign_u32);
    register_bench!(
        registry,
        Large,
        benchmark_natural_neg_mod_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_rem_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_rem_assign_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_mod_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_mod_assign_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_neg_mod_natural_evaluation_strategy
    );
}

pub fn num_rem_u32(x: BigUint, u: u32) -> u32 {
    (x % u).to_u32().unwrap()
}

fn demo_limbs_mod_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_mod_limb({:?}, {}) = {}",
            limbs,
            limb,
            limbs_mod_limb(&limbs, limb)
        );
    }
}

fn demo_natural_rem_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n %= u;
        println!("x := {}; x %= {}; x = {}", n_old, u, n);
    }
}

fn demo_natural_rem_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % {} = {}", n_old, u, n % u);
    }
}

fn demo_natural_rem_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        println!("&{} % {} = {}", n, u, &n % u);
    }
}

fn demo_natural_mod_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n.mod_assign(u);
        println!("x := {}; x.mod_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_natural_mod_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod({}) = {}", n_old, u, n.mod_op(u));
    }
}

fn demo_natural_mod_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        println!("(&{}).mod({}) = {}", n, u, (&n).mod_op(u));
    }
}

fn demo_natural_neg_mod_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n.neg_mod_assign(u);
        println!("x := {}; x.neg_mod_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_natural_neg_mod_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.neg_mod({}) = {}", n_old, u, n.neg_mod(u));
    }
}

fn demo_natural_neg_mod_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        println!("(&{}).neg_mod({}) = {}", n, u, (&n).neg_mod(u));
    }
}

fn demo_u32_rem_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % {} = {}", u, n_old, u % n);
    }
}

fn demo_u32_rem_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % &{} = {}", u, n_old, u % &n);
    }
}

fn demo_u32_rem_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        u %= n;
        println!("x := {}; x %= {}; x = {}", u_old, n_old, u);
    }
}

fn demo_u32_rem_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let u_old = u;
        u %= &n;
        println!("x := {}; x %= &{}; x = {}", u_old, n, u);
    }
}

fn demo_u32_mod_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod({}) = {:?}", u, n_old, u.mod_op(n));
    }
}

fn demo_u32_mod_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod(&{}) = {:?}", u, n_old, u.mod_op(&n));
    }
}

fn demo_u32_mod_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        u.mod_assign(n);
        println!("x := {}; x.mod_assign({}); x = {}", u_old, n_old, u);
    }
}

fn demo_u32_mod_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let u_old = u;
        u.mod_assign(&n);
        println!("x := {}; x.mod_assign(&{}); x = {}", u_old, n, u);
    }
}

fn demo_u32_neg_mod_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.neg_mod({}) = {:?}", u, n_old, u.neg_mod(n));
    }
}

fn demo_u32_neg_mod_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.neg_mod(&{}) = {:?}", u, n_old, u.neg_mod(&n));
    }
}

fn benchmark_limbs_mod_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_mod_limb(&[u32], u32)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_mod_limb(&limbs, limb))),
        )],
    );
}

fn benchmark_natural_rem_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural %= u32",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x %= y))],
    );
}

fn benchmark_natural_rem_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural % u32",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x % y))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_rem_u32(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x % y))),
        ],
    );
}

fn benchmark_natural_rem_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural % u32",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x % y))),
            ("naive", &mut (|(x, y)| no_out!(x._mod_u32_naive(y)))),
            ("using div_mod", &mut (|(x, y)| no_out!(x.div_mod(y).1))),
        ],
    );
}

fn benchmark_natural_rem_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural % u32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Natural % u32", &mut (|(x, y)| no_out!(x % y))),
            ("&Natural % u32", &mut (|(x, y)| no_out!(&x % y))),
        ],
    );
}

fn benchmark_natural_mod_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.mod_assign(u32)",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.mod_assign(y)))],
    );
}

fn benchmark_natural_mod_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.mod(u32)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.mod_op(y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_rem_u32(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x % y))),
        ],
    );
}

fn benchmark_natural_mod_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod(u32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Natural.mod(u32)", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            (
                "(&Natural).mod(u32)",
                &mut (|(x, y)| no_out!((&x).mod_op(y))),
            ),
        ],
    );
}

fn benchmark_natural_neg_mod_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.neg_mod_assign(u32)",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.neg_mod_assign(y)))],
    );
}

fn benchmark_natural_neg_mod_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.neg_mod(u32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Natural.neg_mod(u32)",
                &mut (|(x, y)| no_out!(x.neg_mod(y))),
            ),
            (
                "(&Natural).neg_mod(u32)",
                &mut (|(x, y)| no_out!((&x).neg_mod(y))),
            ),
        ],
    );
}

fn benchmark_u32_rem_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32 % Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("u32 % Natural", &mut (|(x, y)| no_out!(x % y))),
            ("u32 % &Natural", &mut (|(x, y)| no_out!(x % &y))),
        ],
    );
}

fn benchmark_u32_rem_assign_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32 %= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("u32 %= Natural", &mut (|(mut x, y)| x %= y)),
            ("u32 %= &Natural", &mut (|(mut x, y)| x %= &y)),
        ],
    );
}

fn benchmark_u32_mod_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("u32.mod(Natural)", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            ("u32.mod(&Natural)", &mut (|(x, y)| no_out!(x.mod_op(&y)))),
        ],
    );
}

fn benchmark_u32_mod_assign_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.mod_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.mod_assign(Natural)",
                &mut (|(mut x, y)| x.mod_assign(y)),
            ),
            (
                "u32.mod_assign(&Natural)",
                &mut (|(mut x, y)| x.mod_assign(&y)),
            ),
        ],
    );
}

fn benchmark_u32_neg_mod_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.neg_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.neg_mod(Natural)",
                &mut (|(x, y)| no_out!(x.neg_mod(y))),
            ),
            (
                "u32.neg_mod(&Natural)",
                &mut (|(x, y)| no_out!(x.neg_mod(&y))),
            ),
        ],
    );
}
