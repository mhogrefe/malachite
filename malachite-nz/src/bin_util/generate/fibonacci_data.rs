// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1999-2016 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ArithmeticCheckedShl;
use malachite_nz::platform::Limb;

// This is equivalent to `main` in `gen-fib.c`, GMP 6.3.0. The output is pasted into
// `platform_64.rs` or `platform_32.rs`, according to the limb width this is run with, and then
// formatted with `cargo fmt`.
pub(crate) fn generate_fibonacci_data() {
    println!("// This section is created by fibonacci_data.rs.");
    println!();
    // FIB_TABLE[i] is F(i - 1): the table begins with F(-1) = 1 and F(0) = 0, and extends as long
    // as F(n) fits in a limb.
    let mut xs: Vec<Limb> = vec![1, 0];
    loop {
        let len = xs.len();
        if let Some(x) = xs[len - 1].checked_add(xs[len - 2]) {
            xs.push(x);
        } else {
            break;
        }
    }
    // xs has n + 2 entries when it ends with F(n).
    let fib_limit = xs.len() - 2;
    // The largest n with L(n) = F(n) + 2 * F(n - 1) fitting in a limb. Both the shift and the add
    // can overflow.
    let mut lucnum_limit = 0;
    for n in 1..=fib_limit {
        if xs[n]
            .arithmetic_checked_shl(1u64)
            .and_then(|d: Limb| d.checked_add(xs[n + 1]))
            .is_some()
        {
            lucnum_limit = n;
        } else {
            break;
        }
    }
    println!(
        "// This is equivalent to `__gmp_fib_table` in `mpn/fib_table.c`, GMP 6.3.0. \
        `FIB_TABLE[i]` is"
    );
    println!("// F(i - 1), so the table begins with F(-1) = 1 and F(0) = 0.");
    print!("pub(crate) const FIB_TABLE: [Limb; {}] = [", xs.len());
    let mut first = true;
    for &x in &xs {
        if first {
            first = false;
        } else {
            print!(", ");
        }
        print!("{x}");
    }
    println!("];");
    println!();
    println!("// The largest n whose F(n) fits in one limb.");
    println!("//");
    println!("// This is equivalent to `FIB_TABLE_LIMIT` in `fib_table.h`, GMP 6.3.0.");
    println!("pub(crate) const FIB_TABLE_LIMIT: u64 = {fib_limit};");
    println!();
    println!("// The largest n whose L(n) = F(n) + 2 * F(n - 1) fits in one limb.");
    println!("//");
    println!("// This is equivalent to `FIB_TABLE_LUCNUM_LIMIT` in `fib_table.h`, GMP 6.3.0.");
    println!("pub(crate) const FIB_TABLE_LUCNUM_LIMIT: u64 = {lucnum_limit};");
}
