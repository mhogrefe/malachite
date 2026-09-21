// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::latex::ToLatex;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{string_gen, unsigned_gen};
use malachite_base::test_util::runner::Runner;
use malachite_base::vars::VarScheme;
use malachite_base::vars::indexed::{IndexedCapsVars, IndexedVars};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_indexed_vars_var);
    register_demo!(runner, demo_indexed_vars_var_latex);
    register_demo!(runner, demo_indexed_vars_var_typst);
    register_demo!(runner, demo_indexed_caps_vars_var);
    register_demo!(runner, demo_indexed_vars_parse_var);
}

fn demo_indexed_vars_var(gm: GenMode, config: &GenConfig, limit: usize) {
    for i in unsigned_gen::<usize>().get(gm, config).take(limit) {
        println!("IndexedVars.var({}) = {}", i, IndexedVars.var(i));
    }
}

fn demo_indexed_vars_var_latex(gm: GenMode, config: &GenConfig, limit: usize) {
    for i in unsigned_gen::<usize>().get(gm, config).take(limit) {
        println!(
            "IndexedVars.var({}).to_latex() = {}",
            i,
            IndexedVars.var(i).to_latex()
        );
    }
}

fn demo_indexed_vars_var_typst(gm: GenMode, config: &GenConfig, limit: usize) {
    for i in unsigned_gen::<usize>().get(gm, config).take(limit) {
        println!(
            "IndexedVars.var({}).to_typst() = {}",
            i,
            IndexedVars.var(i).to_typst()
        );
    }
}

fn demo_indexed_caps_vars_var(gm: GenMode, config: &GenConfig, limit: usize) {
    for i in unsigned_gen::<usize>().get(gm, config).take(limit) {
        println!("IndexedCapsVars.var({}) = {}", i, IndexedCapsVars.var(i));
    }
}

fn demo_indexed_vars_parse_var(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen().get(gm, config).take(limit) {
        println!(
            "IndexedVars.parse_var({:?}) = {:?}",
            s,
            IndexedVars.parse_var(&s)
        );
    }
}
