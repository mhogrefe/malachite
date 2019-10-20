use malachite_nz::natural::arithmetic::div_exact::{
    _limbs_modular_div_mod_schoolbook, _limbs_modular_div_schoolbook,
};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    quadruples_of_three_unsigned_vecs_and_unsigned_var_3,
    quadruples_of_three_unsigned_vecs_and_unsigned_var_4,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_modular_div_mod_schoolbook);
    register_demo!(registry, demo_limbs_modular_div_schoolbook);
    register_bench!(registry, Small, benchmark_limbs_modular_div_mod_schoolbook);
    register_bench!(registry, Small, benchmark_limbs_modular_div_schoolbook);
}

fn demo_limbs_modular_div_mod_schoolbook(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_unsigned_vecs_and_unsigned_var_4(gm).take(limit)
    {
        let qs_old = qs.clone();
        let ns_old = ns.clone();
        let borrow = _limbs_modular_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; \
             _limbs_modular_div_mod_schoolbook(&mut qs, &mut ns, {:?}, {}) = {}; qs = {:?}; \
             ns = {:?}",
            qs_old, ns_old, ds, inverse, borrow, qs, ns
        );
    }
}

fn demo_limbs_modular_div_schoolbook(gm: GenerationMode, limit: usize) {
    for (mut qs, mut ns, ds, inverse) in
        quadruples_of_three_unsigned_vecs_and_unsigned_var_3(gm).take(limit)
    {
        let qs_old = qs.clone();
        let ns_old = ns.clone();
        _limbs_modular_div_schoolbook(&mut qs, &mut ns, &ds, inverse);
        println!(
            "qs := {:?}; ns := {:?}; _limbs_modular_div_schoolbook(&mut qs, &mut ns, {:?}, {}); \
             qs = {:?}",
            qs_old, ns_old, ds, inverse, qs
        );
    }
}

fn benchmark_limbs_modular_div_mod_schoolbook(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_modular_div_mod_schoolbook(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _, _)| ns.len()),
        "ns.len()",
        &mut [(
            "malachite",
            &mut (|(mut qs, mut ns, ds, inverse)| {
                no_out!(_limbs_modular_div_mod_schoolbook(
                    &mut qs, &mut ns, &ds, inverse
                ))
            }),
        )],
    );
}

fn benchmark_limbs_modular_div_schoolbook(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_modular_div_schoolbook(&mut [Limb], &mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        quadruples_of_three_unsigned_vecs_and_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ns, _, _)| ns.len()),
        "ns.len()",
        &mut [(
            "malachite",
            &mut (|(mut qs, mut ns, ds, inverse)| {
                _limbs_modular_div_schoolbook(&mut qs, &mut ns, &ds, inverse)
            }),
        )],
    );
}
