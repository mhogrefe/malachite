use common::GenerationMode;
use inputs::integer::pairs_of_integer_and_small_u32;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_integer_mod_power_of_two_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        n.mod_power_of_two_assign(u);
        println!(
            "x := {}; x.mod_power_of_two_assign({}); x = {}",
            n_old, u, n
        );
    }
}

pub fn demo_integer_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.mod_power_of_two({}) = {}",
            n_old,
            u,
            n.mod_power_of_two(u)
        );
    }
}

pub fn demo_integer_mod_power_of_two_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        println!(
            "{}.mod_power_of_two_ref({}) = {}",
            n,
            u,
            n.mod_power_of_two_ref(u)
        );
    }
}

pub fn demo_integer_rem_power_of_two_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        n.rem_power_of_two_assign(u);
        println!(
            "x := {}; x.rem_power_of_two_assign({}); x = {}",
            n_old, u, n
        );
    }
}

pub fn demo_integer_rem_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.rem_power_of_two({}) = {}",
            n_old,
            u,
            n.rem_power_of_two(u)
        );
    }
}

pub fn demo_integer_rem_power_of_two_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        println!(
            "{}.rem_power_of_two_ref({}) = {}",
            n,
            u,
            n.rem_power_of_two_ref(u)
        );
    }
}

pub fn demo_integer_ceiling_mod_power_of_two_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        n.ceiling_mod_power_of_two_assign(u);
        println!(
            "x := {}; x.ceiling_mod_power_of_two_assign({}); x = {}",
            n_old, u, n
        );
    }
}

pub fn demo_integer_ceiling_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_mod_power_of_two({}) = {}",
            n_old,
            u,
            n.ceiling_mod_power_of_two(u)
        );
    }
}

pub fn demo_integer_ceiling_mod_power_of_two_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_u32(gm).take(limit) {
        println!(
            "{}.ceiling_mod_power_of_two_ref({}) = {}",
            n,
            u,
            n.ceiling_mod_power_of_two_ref(u)
        );
    }
}

pub fn benchmark_integer_mod_power_of_two_assign(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.mod_power_of_two_assign(u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_integer_and_small_u32(gm),
        function_f: &(|(mut n, u): (Integer, u32)| n.mod_power_of_two_assign(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.mod\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_mod_power_of_two(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.mod_power_of_two(u32)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_integer_and_small_u32(gm),
        function_f: &(|(n, u): (Integer, u32)| n.mod_power_of_two(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.mod\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_mod_power_of_two_ref(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.mod_power_of_two_ref(u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_integer_and_small_u32(gm),
        function_f: &(|(n, u): (Integer, u32)| n.mod_power_of_two_ref(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.mod\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_rem_power_of_two_assign(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.rem_power_of_two_assign(u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_integer_and_small_u32(gm),
        function_f: &(|(mut n, u): (Integer, u32)| n.rem_power_of_two_assign(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.rem\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_rem_power_of_two(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.rem_power_of_two(u32)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_integer_and_small_u32(gm),
        function_f: &(|(n, u): (Integer, u32)| n.rem_power_of_two(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.rem\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_rem_power_of_two_ref(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.rem_power_of_two_ref(u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_integer_and_small_u32(gm),
        function_f: &(|(n, u): (Integer, u32)| n.rem_power_of_two_ref(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.rem\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_ceiling_mod_power_of_two_assign(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.ceiling_mod_power_of_two_assign(u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_integer_and_small_u32(gm),
        function_f: &(|(mut n, u): (Integer, u32)| n.ceiling_mod_power_of_two_assign(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.ceiling\\\\_mod\\\\_power\\\\_of\\\\_2\\\\_assign(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_ceiling_mod_power_of_two(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.ceiling_mod_power_of_two(u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_integer_and_small_u32(gm),
        function_f: &(|(n, u): (Integer, u32)| n.ceiling_mod_power_of_two(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.ceiling\\\\_mod\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_ceiling_mod_power_of_two_ref(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.ceiling_mod_power_of_two_ref(u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_integer_and_small_u32(gm),
        function_f: &(|(n, u): (Integer, u32)| n.ceiling_mod_power_of_two_ref(u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: "Integer.ceiling\\\\_mod\\\\_power\\\\_of\\\\_2\\\\_ref(u32)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
