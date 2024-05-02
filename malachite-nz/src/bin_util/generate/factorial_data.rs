// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 2002, 2010-2018 Free Software Foundation, Inc.
//
//      `mpz/bin_uiui.c` contributed to the GNU project by Torbjörn Granlund and Marco Bodrato.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, BinomialCoefficient, CheckedDoubleFactorial, DivExactAssign, FloorRoot,
    ModPowerOf2Inverse, OverflowingMulAssign, WrappingMulAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::CountOnes;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

#[allow(clippy::useless_conversion)]
fn odd_factorial_table() -> (u64, u64) {
    let mut xs = vec![1, 1, 1];
    let mut x: Limb = 1;
    let mut limit = 0;
    for b in Limb::from(3u32).. {
        if x.overflowing_mul_assign(b >> b.trailing_zeros()) {
            limit = b;
            break;
        } else {
            xs.push(x);
        }
    }
    xs.push(x);
    let mut y = Limb::binomial_coefficient(limit, limit >> 1);
    let mut ext_limit = 0;
    for b in limit + 1.. {
        let a = b >> b.trailing_zeros();
        if a == b {
            y.div_exact_assign((a >> 1) + 1);
            if y.overflowing_mul_assign(a) {
                ext_limit = b;
                break;
            }
        } else if let Some(shifted) = y.arithmetic_checked_shl(1) {
            y = shifted;
        } else {
            ext_limit = b;
            break;
        }
        x.wrapping_mul_assign(a);
        xs.push(x);
    }
    println!(
        "// This is equivalent to `__gmp_oddfac_table` in `mpn/comb_tables.c`, GMP 6.2.1, which \
        is the"
    );
    println!(
        "// combination of `ONE_LIMB_ODD_FACTORIAL_TABLE` and `ONE_LIMB_ODD_FACTORIAL_EXTTABLE` in"
    );
    println!("// `fac_table.h`, GMP 6.2.1.");
    print!(
        "pub const ONE_LIMB_ODD_FACTORIAL_TABLE: [Limb; {}] = [",
        xs.len()
    );
    let mut first = true;
    for &x in &xs {
        if first {
            first = false;
        } else {
            print!(", ");
        }
        print!("{x:#x}");
    }
    println!("];");
    limit -= 1;
    ext_limit -= 1;
    println!("// This is equivalent to `ODD_FACTORIAL_TABLE_LIMIT` in `fac_table.h`, GMP 6.2.1.");
    println!("pub const ODD_FACTORIAL_TABLE_LIMIT: usize = {limit};");
    println!(
        "// This is equivalent to `ODD_FACTORIAL_EXTTABLE_LIMIT` in `fac_table.h`, GMP 6.2.1."
    );
    println!("pub const ODD_FACTORIAL_EXTTABLE_LIMIT: usize = {ext_limit};");
    println!("// This is equivalent to `ODD_FACTORIAL_TABLE_MAX` in `fac_table.h`, GMP 6.2.1.");
    println!(
        "pub const ODD_FACTORIAL_TABLE_MAX: Limb = {:#x};",
        xs[usize::wrapping_from(limit)]
    );
    (u64::from(limit), u64::from(ext_limit))
}

fn odd_double_factorial_table() {
    let mut xs = vec![];
    let mut limit = 0;
    for n in 0.. {
        let n = (n << 1) | 1;
        if let Some(x) = Limb::checked_double_factorial(n) {
            xs.push(x);
        } else {
            limit = n;
            break;
        }
    }
    println!(
        "// This is equivalent to `__gmp_odd2fac_table` in `mpn/comb_tables.c`, GMP 6.2.1, and"
    );
    println!("// `ONE_LIMB_ODD_DOUBLEFACTORIAL_TABLE` from `fac_table.h`, GMP 6.2.1.");
    print!(
        "pub const ONE_LIMB_ODD_DOUBLEFACTORIAL_TABLE: [Limb; {}] = [",
        xs.len()
    );
    let mut first = true;
    for &x in &xs {
        if first {
            first = false;
        } else {
            print!(", ");
        }
        print!("{x:#x}");
    }
    println!("];");
    limit -= 2;
    println!(
        "// This is equivalent to `ODD_DOUBLEFACTORIAL_TABLE_LIMIT` in `fac_table.h`, GMP 6.2.1."
    );
    println!("pub const ODD_DOUBLEFACTORIAL_TABLE_LIMIT: usize = {limit};");
    println!(
        "// This is equivalent to `ODD_DOUBLEFACTORIAL_TABLE_MAX` in `fac_table.h`, GMP 6.2.1."
    );
    println!(
        "pub const ODD_DOUBLEFACTORIAL_TABLE_MAX: Limb = {:#x};",
        xs[usize::wrapping_from(limit >> 1)]
    );
}

fn nth_root_mask_table() {
    println!(
        "// This is equivalent to `__gmp_limbroots_table` in `mpn/comb_tables.c`, GMP 6.2.1, and"
    );
    println!("// `NTH_ROOT_NUMB_MASK_TABLE` from `fac_table.h`, GMP 6.2.1.");
    print!("pub const NTH_ROOT_NUMB_MASK_TABLE: [Limb; 8] = [Limb::MAX");
    for b in 2..=8 {
        print!(", {:#x}", Limb::MAX.floor_root(b));
    }
    println!("];");
}

fn factorial_inverse_table(limit: u64) {
    println!(
        "// This is equivalent to `ONE_LIMB_ODD_FACTORIAL_INVERSES_TABLE` from `fac_table.h`, \
        GMP 6.2.1."
    );
    print!(
        "pub const ONE_LIMB_ODD_FACTORIAL_INVERSES_TABLE: [Limb; {}] = [0x1",
        limit - 3
    );
    let limit = Limb::wrapping_from(limit);
    let mut x: Limb = 1;
    for b in 3..=limit - 2 {
        x.wrapping_mul_assign(b >> b.trailing_zeros());
        print!(", {:#x}", x.mod_power_of_2_inverse(Limb::WIDTH).unwrap());
    }
    println!("];");
}

fn factorial_2_count_table(limit: u64) {
    println!("// https://oeis.org/A005187, skipping the initial 0");
    println!("//");
    println!(
        "// This is equivalent to `__gmp_fac2cnt_table` in `mpn/comb_tables.c`, GMP 6.2.1, and"
    );
    println!("// `TABLE_2N_MINUS_POPC_2N` from `fac_table.h`, GMP 6.2.1.");
    let limit = ((limit >> 4) + 1) << 4;
    let mut b = 4;
    print!(
        "pub const TABLE_2N_MINUS_POPC_2N: [u8; {}] = [1",
        ((limit - b) >> 1) + 2
    );
    while b <= limit {
        print!(", {}", b - CountOnes::count_ones(b));
        b += 2;
    }
    println!("];");
    println!();
    println!(
        "pub const TABLE_LIMIT_2N_MINUS_POPC_2N: u64 = {};",
        limit + 1
    );
}

fn odd_central_binomial_table(limit: u64) -> u64 {
    println!("pub const ODD_CENTRAL_BINOMIAL_OFFSET: usize = {limit};");
    println!();
    println!("// This table contains binomial(2k, k) / 2 ^ t.");
    println!("//");
    println!("// This is equivalent to `bin2kk` in `mpz/bin_uiui.c`, GMP 6.2.1, and");
    println!("// `ONE_LIMB_ODD_CENTRAL_BINOMIAL_TABLE` from `fac_table.h`, GMP 6.2.1.");
    let mut xs = Vec::new();
    let mut binomial_limit = 0;
    for b in limit.. {
        binomial_limit = b;
        let mut x = Natural::binomial_coefficient(Natural::from(b) << 1u32, Natural::from(b));
        x >>= x.trailing_zeros().unwrap();
        if let Ok(x) = Limb::try_from(&x) {
            xs.push(x);
        } else {
            break;
        }
    }
    print!(
        "pub const ONE_LIMB_ODD_CENTRAL_BINOMIAL_TABLE: [Limb; {}] = [",
        xs.len()
    );
    let mut first = true;
    for &x in &xs {
        if first {
            first = false;
        } else {
            print!(", ");
        }
        print!("{x:#x}");
    }
    println!("];");
    u64::wrapping_from(binomial_limit)
}

fn odd_central_binomial_inverse_table(limit: u64, binomial_limit: u64) {
    let binomial_limit = binomial_limit - 1;
    println!("pub const ODD_CENTRAL_BINOMIAL_TABLE_LIMIT: usize = {binomial_limit};");
    println!();
    println!("// This table contains the inverses of elements in the previous table.");
    println!("//");
    println!("// This is equivalent to `bin2kkinv` in `mpz/bin_uiui.c`, GMP 6.2.1, and");
    println!("// `ONE_LIMB_ODD_CENTRAL_BINOMIAL_INVERSE_TABLE` from `fac_table.h`, GMP 6.2.1.");
    let mut xs = Vec::new();
    for b in limit..=binomial_limit {
        let mut x = Natural::binomial_coefficient(Natural::from(b) << 1u32, Natural::from(b));
        x >>= x.trailing_zeros().unwrap();
        xs.push(Limb::exact_from(
            &(x.mod_power_of_2_inverse(Limb::WIDTH).unwrap()),
        ));
    }
    print!(
        "pub const ONE_LIMB_ODD_CENTRAL_BINOMIAL_INVERSE_TABLE: [Limb; {}] = [",
        xs.len()
    );
    let mut first = true;
    for &x in &xs {
        if first {
            first = false;
        } else {
            print!(", ");
        }
        print!("{x:#x}");
    }
    println!("];");
}

fn central_binomial_2_fac_table(limit: u64, binomial_limit: u64) {
    let binomial_limit = binomial_limit - 1;
    println!("// This table contains the values t in the formula binomial(2k, k) / 2 ^ t.");
    println!("//");
    println!("// This is equivalent to `fac2bin` in `mpz/bin_uiui.c`, GMP 6.2.1, and");
    println!("// `CENTRAL_BINOMIAL_2FAC_TABLE` from `fac_table.h`, GMP 6.2.1.");
    let mut xs = Vec::new();
    for b in limit..=binomial_limit {
        xs.push(
            Natural::binomial_coefficient(Natural::from(b) << 1u32, Natural::from(b))
                .trailing_zeros()
                .unwrap(),
        );
    }
    print!(
        "pub const CENTRAL_BINOMIAL_2FAC_TABLE: [u64; {}] = [",
        xs.len()
    );
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
}

pub(crate) fn generate_factorial_data() {
    println!("// This section is created by factorial_data.rs.");
    println!();
    let (mut of_limit, ext_limit) = odd_factorial_table();
    println!();
    odd_double_factorial_table();
    println!();
    nth_root_mask_table();
    println!();
    factorial_inverse_table(ext_limit);
    println!();
    of_limit = (of_limit + 1) >> 1;
    let binomial_limit = odd_central_binomial_table(of_limit);
    println!();
    odd_central_binomial_inverse_table(of_limit, binomial_limit);
    println!();
    central_binomial_2_fac_table(of_limit, binomial_limit);
    println!();
    factorial_2_count_table(u64::exact_from(ext_limit));
}
