// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn max_base_helper<T: PrimitiveUnsigned>() {
    if T::NAME == "usize" {
        return;
    }
    print!(
        "const MAX_BASE_{}: [{}; {}] = [0",
        T::WIDTH,
        T::NAME,
        T::WIDTH
    );
    for exp in 1..T::WIDTH {
        print!(", {}", T::MAX.floor_root(exp));
    }
    println!("];");
    println!();
    print!(
        "const MAX_POWER_{}: [{}; {}] = [0",
        T::WIDTH,
        T::NAME,
        T::WIDTH
    );
    for exp in 1..T::WIDTH {
        print!(", {}", T::MAX.floor_root(exp).pow(exp));
    }
    println!("];");
    println!();
}

pub(crate) fn generate_max_base() {
    println!("// This section is created by max_base.rs.");
    apply_fn_to_unsigneds!(max_base_helper);
}
