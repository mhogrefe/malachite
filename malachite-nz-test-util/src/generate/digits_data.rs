use malachite_base::num::arithmetic::traits::{ModPowerOfTwoAssign, Parity, Pow, ShrRound, Square};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use std::cmp::Ordering;

// Calculate r satisfying r*d == 1 mod 2^n.
// This is mpz_invert_2exp from bootstrap.c, GMP 6.2.1.
fn invert_mod_power_of_two(a: &Natural, n: u64) -> Natural {
    assert!(a.odd());
    let mut inv = Natural::ONE;
    for i in 0..n {
        let prod = &inv * a;
        if prod.get_bit(i) {
            inv.set_bit(i);
        }
    }
    let mut prod = &inv * a;
    prod.mod_power_of_two_assign(n);
    assert_eq!(prod, 1);
    inv
}

// This is ulog2 from gen-bases.c, GMP 6.2.1.
fn ulog2(mut x: u64) -> u64 {
    let mut i = 0;
    while x != 0 {
        x >>= 1;
        i += 1;
    }
    i
}

//TODO use real sqrt
fn simple_sqrt(x: &Natural) -> Natural {
    let mut low = Natural::ZERO;
    let mut high = x.clone();
    loop {
        if high <= low {
            return low;
        }
        let mid = (&low + &high).shr_round(1u64, RoundingMode::Ceiling);
        let mid_squared = (&mid).square();
        match mid_squared.cmp(x) {
            Ordering::Equal => return mid,
            Ordering::Less => low = mid,
            Ordering::Greater => high = mid - Natural::ONE,
        }
    }
}

const EXTRA: u64 = 16;

// Compute log(2)/log(b) as a fixnum.
//
// This is mp_2logb from gen-bases.c, GMP 6.2.1.
fn mp_2logb(bi: u64, prec: u64) -> Natural {
    let mut t = Natural::ONE;
    t <<= prec + EXTRA;
    let mut two = Natural::TWO;
    two <<= prec + EXTRA;
    let mut r = Natural::ZERO;
    let mut b = Natural::from(bi);
    b <<= prec + EXTRA;
    let mut i = prec - 1;
    loop {
        b <<= prec + EXTRA;
        b = simple_sqrt(&b);
        let mut t2 = &t * &b;
        t2 >>= prec + EXTRA;
        if t2 < two {
            r.set_bit(i);
            t = t2;
        }
        if i == 0 {
            break;
        } else {
            i -= 1;
        }
    }
    r
}

#[derive(Default)]
struct State {
    chars_per_limb: u64,
    big_base_trailing_zeros: u64,
    big_base: Natural,
    normalization_steps: u64,
    big_base_inverted: Natural,
    big_base_inverted_mod_b: Natural,
}

// This is generate from gen-bases.c, GMP 6.2.1
fn generate(state: &mut State, base: u64) {
    let mut t = Natural::ONE;
    t <<= Limb::WIDTH;
    state.big_base = Natural::ONE;
    state.chars_per_limb = 0;
    loop {
        state.big_base *= Natural::from(base);
        if state.big_base > t {
            break;
        }
        state.chars_per_limb += 1;
    }
    state.big_base = Natural::from(base).pow(state.chars_per_limb);
    state.normalization_steps = Limb::WIDTH.wrapping_sub(state.big_base.significant_bits());
    t = Natural::ONE;
    t <<= (Limb::WIDTH << 1).wrapping_sub(state.normalization_steps);
    state.big_base_inverted = t / &state.big_base;
    t = Natural::ONE;
    t <<= Limb::WIDTH;
    state.big_base_inverted -= t; //TODO use clear_bit

    state.big_base_trailing_zeros = state.big_base.trailing_zeros().unwrap();
    let big_base_odd = &state.big_base >> state.big_base_trailing_zeros;
    state.big_base_inverted_mod_b = invert_mod_power_of_two(&big_base_odd, Limb::WIDTH);
}

// This is header from gen-bases.c, GMP 6.2.1.
fn header() {
    let mut state = State::default();
    generate(&mut state, 10);
    println!("// mp_bases[10] data, as literal values");
    println!(
        "pub const MP_BASES_CHARS_PER_LIMB_10: usize = {};",
        state.chars_per_limb
    );
    println!(
        "pub const MP_BASES_BIG_BASE_CTZ_10: usize = {};",
        state.big_base_trailing_zeros
    );
    println!(
        "pub const MP_BASES_BIG_BASE_10: Limb = {:#x};",
        Limb::exact_from(state.big_base)
    );
    println!(
        "pub const MP_BASES_BIG_BASE_INVERTED_10: Limb = {:#x};",
        Limb::exact_from(state.big_base_inverted)
    );
    println!(
        "pub const MP_BASES_BIG_BASE_BINVERTED_10: Limb = {:#x};",
        Limb::exact_from(state.big_base_inverted_mod_b)
    );
    println!(
        "pub const MP_BASES_NORMALIZATION_STEPS_10: u64 = {};",
        state.normalization_steps
    );
}

// This is table from gen-bases.c, GMP 6.2.1.
fn table() {
    println!("// Format is (chars_per_limb, logb2, log2b, big_base, big_base_inverted)");
    println!("pub const BASES: [(usize, Limb, Limb, Limb, Limb); 257] = [");
    println!("    (0, 0, 0, 0, 0), // 0");
    println!("    (0, 0, 0, 0, 0), // 1");
    for base in 2..=256 {
        let mut state = State::default();
        generate(&mut state, base);
        let mut r = mp_2logb(base, Limb::WIDTH + 8);
        let logb2 = &r >> 8;
        let mut t = Natural::ONE;
        t <<= (Limb::WIDTH << 1) + 5;
        t -= Natural::ONE;
        r += Natural::ONE;
        let log2b = t / r;
        if base.is_power_of_two() {
            state.big_base = Natural::from(ulog2(base) - 1);
            state.big_base_inverted = Natural::ZERO;
        }
        println!(
            "    ({}, {:#x}, {:#x}, {:#x}, {:#x}), // {}",
            state.chars_per_limb,
            Limb::exact_from(logb2),
            Limb::exact_from(log2b),
            Limb::exact_from(state.big_base),
            Limb::exact_from(state.big_base_inverted),
            base
        );
    }
    println!("];");
}

pub(crate) fn generate_string_data() {
    println!("// This section is generated by digits_data.rs.");
    println!();
    header();
    println!();
    table();
}
