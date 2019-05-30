use std::cmp::{min, Ordering};

use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::comparison::ord::{limbs_cmp, limbs_cmp_same_length};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{pairs_of_unsigned_vec_var_1, pairs_of_unsigned_vec_var_2};
use inputs::natural::{nrm_pairs_of_naturals, pairs_of_naturals};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_cmp_same_length);
    register_demo!(registry, demo_limbs_cmp);
    register_demo!(registry, demo_natural_cmp);
    register_bench!(registry, Small, benchmark_limbs_cmp_same_length);
    register_bench!(registry, Small, benchmark_limbs_cmp);
    register_bench!(registry, Large, benchmark_natural_cmp_library_comparison);
}

fn demo_limbs_cmp_same_length(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_1(gm).take(limit) {
        println!(
            "limbs_cmp_same_length({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_cmp_same_length(&xs, &ys),
        );
    }
}

fn demo_limbs_cmp(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_2(gm).take(limit) {
        println!("limbs_cmp({:?}, {:?}) = {:?}", xs, ys, limbs_cmp(&xs, &ys));
    }
}

fn demo_natural_cmp(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        match x.cmp(&y) {
            Ordering::Less => println!("{} < {}", x, y),
            Ordering::Equal => println!("{} = {}", x, y),
            Ordering::Greater => println!("{} > {}", x, y),
        }
    }
}

fn benchmark_limbs_cmp_same_length(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_cmp_same_length(&[u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(xs, ys)| no_out!(limbs_cmp_same_length(&xs, &ys))),
        )],
    );
}

fn benchmark_limbs_cmp(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_cmp(&[u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| min(xs.len(), ys.len())),
        "min(xs.len(), ys.len())",
        &mut [("malachite", &mut (|(xs, ys)| no_out!(limbs_cmp(&xs, &ys))))],
    );
}

fn benchmark_natural_cmp_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.cmp(&Natural)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| {
            usize::checked_from(min(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "min(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.cmp(&y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(x.cmp(&y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.cmp(&y)))),
        ],
    );
}
