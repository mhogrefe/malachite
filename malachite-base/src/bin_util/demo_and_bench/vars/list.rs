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
use malachite_base::test_util::generators::string_gen;
use malachite_base::test_util::runner::Runner;
use malachite_base::vars::VarScheme;
use malachite_base::vars::list::ListVars;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_list_vars_var);
    register_demo!(runner, demo_list_vars_parse_var);
}

// A name the caller supplies may be anything that is not empty and holds no reserved character,
// which is a much wider set than the other schemes draw on; this shows what each such name is set
// as.
fn demo_list_vars_var(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen().get(gm, config).take(limit) {
        if s.is_empty() || s.chars().any(malachite_base::vars::char_is_reserved) {
            continue;
        }
        let vars = ListVars::new([s.clone()]);
        println!(
            "ListVars::new([{:?}]).var(0) = {}, {}, {}",
            s,
            vars.var(0),
            vars.var(0).to_latex(),
            vars.var(0).to_typst()
        );
    }
}

fn demo_list_vars_parse_var(gm: GenMode, config: &GenConfig, limit: usize) {
    let vars = ListVars::new(["t", "price", "α"]);
    for s in string_gen().get(gm, config).take(limit) {
        println!(
            "ListVars::new([\"t\", \"price\", \"α\"]).parse_var({:?}) = {:?}",
            s,
            vars.parse_var(&s)
        );
    }
}
