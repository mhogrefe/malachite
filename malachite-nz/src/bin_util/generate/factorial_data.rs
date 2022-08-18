use crate::malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, CheckedDoubleFactorial, DivExact, DivExactAssign, Factorial, FloorRoot,
    OverflowingMulAssign, WrappingMulAssign,
};
use crate::malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::CountOnes;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

// TODO replace
fn simple_binomial(n: Limb, k: Limb) -> Limb {
    let n = u64::wrapping_from(n);
    let k = u64::wrapping_from(k);
    Limb::exact_from(
        &Natural::factorial(n).div_exact(Natural::factorial(k) * Natural::factorial(n - k)),
    )
}

#[allow(clippy::useless_conversion)]
fn odd_factorial_table() -> Limb {
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
    let mut y = simple_binomial(limit, limit >> 1);
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
        print!("{:#x}", x);
    }
    println!("];");
    limit -= 1;
    ext_limit -= 1;
    println!("// This is equivalent to `ODD_FACTORIAL_TABLE_LIMIT` in `fac_table.h`, GMP 6.2.1.");
    println!("pub const ODD_FACTORIAL_TABLE_LIMIT: usize = {};", limit);
    println!(
        "// This is equivalent to `ODD_FACTORIAL_EXTTABLE_LIMIT` in `fac_table.h`, GMP 6.2.1."
    );
    println!(
        "pub const ODD_FACTORIAL_EXTTABLE_LIMIT: usize = {};",
        ext_limit
    );
    println!("// This is equivalent to `ODD_FACTORIAL_TABLE_MAX` in `fac_table.h`, GMP 6.2.1.");
    println!(
        "pub const ODD_FACTORIAL_TABLE_MAX: Limb = {:#x};",
        xs[usize::wrapping_from(limit)]
    );
    ext_limit
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
        print!("{:#x}", x);
    }
    println!("];");
    limit -= 2;
    println!(
        "// This is equivalent to `ODD_DOUBLEFACTORIAL_TABLE_LIMIT` in `fac_table.h`, GMP 6.2.1."
    );
    println!(
        "pub const ODD_DOUBLEFACTORIAL_TABLE_LIMIT: usize = {};",
        limit
    );
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

pub(crate) fn generate_factorial_data() {
    println!("// This section is created by factorial_data.rs.");
    println!();
    let ext_limit = odd_factorial_table();
    println!();
    odd_double_factorial_table();
    println!();
    nth_root_mask_table();
    println!();
    factorial_2_count_table(u64::exact_from(ext_limit));
}
