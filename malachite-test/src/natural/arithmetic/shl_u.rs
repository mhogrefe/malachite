use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_unsigned_vec_and_small_u64, pairs_of_unsigned_vec_and_u32_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_u32_var_5,
};
use inputs::natural::{
    pairs_of_natural_and_small_unsigned, rm_pairs_of_natural_and_small_unsigned,
};
use malachite_base::misc::Named;
use malachite_base::num::PrimitiveInteger;
use malachite_nz::natural::arithmetic::shl_u::{
    limbs_shl, limbs_shl_to_out, limbs_slice_shl_in_place, limbs_vec_shl_in_place,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_shl);
    register_demo!(registry, demo_limbs_shl_to_out);
    register_demo!(registry, demo_limbs_slice_shl_in_place);
    register_demo!(registry, demo_limbs_vec_shl_in_place);

    register_demo!(registry, demo_natural_shl_assign_u8);
    register_demo!(registry, demo_natural_shl_assign_u16);
    register_demo!(registry, demo_natural_shl_assign_u32);
    register_demo!(registry, demo_natural_shl_assign_u64);

    register_demo!(registry, demo_natural_shl_u8);
    register_demo!(registry, demo_natural_shl_u16);
    register_demo!(registry, demo_natural_shl_u32);
    register_demo!(registry, demo_natural_shl_u64);

    register_demo!(registry, demo_natural_shl_u8_ref);
    register_demo!(registry, demo_natural_shl_u16_ref);
    register_demo!(registry, demo_natural_shl_u32_ref);
    register_demo!(registry, demo_natural_shl_u64_ref);

    register_bench!(registry, Small, benchmark_limbs_shl);
    register_bench!(registry, Small, benchmark_limbs_shl_to_out);
    register_bench!(registry, Small, benchmark_limbs_slice_shl_in_place);
    register_bench!(registry, Small, benchmark_limbs_vec_shl_in_place);

    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_assign_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_u32_library_comparison
    );

    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_u64_evaluation_strategy
    );
}

fn demo_limbs_shl(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_small_u64(gm).take(limit) {
        println!(
            "limbs_shl({:?}, {}) = {:?}",
            limbs,
            bits,
            limbs_shl(&limbs, bits)
        );
    }
}

fn demo_limbs_shl_to_out(gm: GenerationMode, limit: usize) {
    for (out_limbs, in_limbs, bits) in
        triples_of_unsigned_vec_unsigned_vec_and_u32_var_5(gm).take(limit)
    {
        let mut out_limbs = out_limbs.to_vec();
        let mut out_limbs_old = out_limbs.clone();
        let carry = limbs_shl_to_out(&mut out_limbs, &in_limbs, bits);
        println!(
            "out_limbs := {:?}; limbs_shl_to_out(&mut out_limbs, {:?}, {}) = {}; out_limbs = {:?}",
            out_limbs_old, in_limbs, bits, carry, out_limbs
        );
    }
}

fn demo_limbs_slice_shl_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_u32_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        let carry = limbs_slice_shl_in_place(&mut limbs, bits);
        println!(
            "limbs := {:?}; limbs_slice_shl_in_place(&mut limbs, {}) = {}; limbs = {:?}",
            limbs_old, bits, carry, limbs
        );
    }
}

fn demo_limbs_vec_shl_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_small_u64(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        limbs_vec_shl_in_place(&mut limbs, bits);
        println!(
            "limbs := {:?}; limbs_vec_shl_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, bits, limbs
        );
    }
}

macro_rules! demos_and_benches {
    (
        $t:ident,
        $demo_natural_shl_assign_u:ident,
        $demo_natural_shl_u:ident,
        $demo_natural_shl_u_ref:ident,
        $benchmark_natural_shl_u_evaluation_strategy:ident
    ) => {
        fn $demo_natural_shl_assign_u(gm: GenerationMode, limit: usize) {
            for (mut n, u) in pairs_of_natural_and_small_unsigned::<$t>(gm).take(limit) {
                let n_old = n.clone();
                n <<= u;
                println!("x := {}; x <<= {}; x = {}", n_old, u, n);
            }
        }

        fn $demo_natural_shl_u(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_natural_and_small_unsigned::<$t>(gm).take(limit) {
                let n_old = n.clone();
                println!("{} << {} = {}", n_old, u, n << u);
            }
        }

        fn $demo_natural_shl_u_ref(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_natural_and_small_unsigned::<$t>(gm).take(limit) {
                println!("&{} << {} = {}", n, u, &n << u);
            }
        }

        fn $benchmark_natural_shl_u_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Natural << {}", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                pairs_of_natural_and_small_unsigned::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other)| other as usize),
                "other",
                &mut [
                    (
                        &format!("Natural << {}", $t::NAME),
                        &mut (|(x, y)| no_out!(x << y)),
                    ),
                    (
                        &format!("&Natural << {}", $t::NAME),
                        &mut (|(x, y)| no_out!(&x << y)),
                    ),
                ],
            );
        }
    };
}
demos_and_benches!(
    u8,
    demo_natural_shl_assign_u8,
    demo_natural_shl_u8,
    demo_natural_shl_u8_ref,
    benchmark_natural_shl_u8_evaluation_strategy
);
demos_and_benches!(
    u16,
    demo_natural_shl_assign_u16,
    demo_natural_shl_u16,
    demo_natural_shl_u16_ref,
    benchmark_natural_shl_u16_evaluation_strategy
);
demos_and_benches!(
    u32,
    demo_natural_shl_assign_u32,
    demo_natural_shl_u32,
    demo_natural_shl_u32_ref,
    benchmark_natural_shl_u32_evaluation_strategy
);
demos_and_benches!(
    u64,
    demo_natural_shl_assign_u64,
    demo_natural_shl_u64,
    demo_natural_shl_u64_ref,
    benchmark_natural_shl_u64_evaluation_strategy
);

fn benchmark_limbs_shl(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_shl(&[u32], u32)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_u64(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits)| limbs.len() + (bits / u64::from(u32::WIDTH)) as usize),
        "limbs.len() + bits / u32::WIDTH",
        &mut [(
            "malachite",
            &mut (|(limbs, bits)| no_out!(limbs_shl(&limbs, bits))),
        )],
    );
}

fn benchmark_limbs_shl_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_shl_to_out(&mut [u32], &[u32], u32)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_u32_var_5(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out_limbs, in_limbs, bits)| {
                no_out!(limbs_shl_to_out(&mut out_limbs, &in_limbs, bits))
            }),
        )],
    );
}

fn benchmark_limbs_slice_shl_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_slice_shl_in_place(&mut [u32], u32)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, bits)| no_out!(limbs_slice_shl_in_place(&mut limbs, bits))),
        )],
    );
}

fn benchmark_limbs_vec_shl_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_vec_shl_in_place(&mut Vec<u32>, u32)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_u64(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits)| limbs.len() + (bits / u64::from(u32::WIDTH)) as usize),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, bits)| limbs_vec_shl_in_place(&mut limbs, bits)),
        )],
    );
}

fn benchmark_natural_shl_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural <<= u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_small_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| other as usize),
        "other",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x <<= y)),
            ("rug", &mut (|((mut x, y), _)| x <<= y)),
        ],
    );
}

fn benchmark_natural_shl_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural << u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_small_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| other as usize),
        "other",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x << y))),
            ("rug", &mut (|((x, y), _)| no_out!(x << y))),
        ],
    );
}
