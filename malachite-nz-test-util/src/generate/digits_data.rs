use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwo, Parity, Pow, PowerOfTwo, ShrRound, Square,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, LowMask, SignificantBits};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use std::cmp::Ordering;

/// Calculate r satisfying r*d == 1 mod 2^n.
///
/// This is mpz_invert_2exp from bootstrap.c, GMP 6.2.1.
fn invert_mod_power_of_two(x: &Natural, pow: u64) -> Natural {
    assert!(x.odd());
    let mut inverse = Natural::ONE;
    for i in 0..pow {
        if (&inverse * x).get_bit(i) {
            inverse.set_bit(i);
        }
    }
    assert_eq!((&inverse * x).mod_power_of_two(pow), 1);
    inverse
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

// Compute log(2) / log(b) as a fixnum.
//
// This is mp_2logb from gen-bases.c, GMP 6.2.1.
fn get_log_base_of_2(base: u64, precision: u64) -> Natural {
    let extended_precision = precision + 16;
    let mut t = Natural::power_of_two(extended_precision);
    let two = Natural::power_of_two(extended_precision + 1);
    let mut log = Natural::ZERO;
    let mut base = Natural::from(base) << extended_precision;
    for i in (0..precision).rev() {
        base = simple_sqrt(&(&base << extended_precision));
        let next_t = (&t * &base) >> extended_precision;
        if next_t < two {
            log.set_bit(i);
            t = next_t;
        }
    }
    log
}

struct BaseData {
    chars_per_limb: u64,
    big_base_trailing_zeros: u64,
    big_base: Natural,
    normalization_steps: u64,
    big_base_inverted: Natural,
    big_base_inverted_mod_b: Natural,
}

// This is generate from gen-bases.c, GMP 6.2.1
fn generate(base: u64) -> BaseData {
    let limit = Natural::power_of_two(Limb::WIDTH);
    let mut big_base = Natural::ONE;
    let mut chars_per_limb = 0;
    while big_base <= limit {
        chars_per_limb += 1;
        big_base *= Natural::from(base);
    }
    chars_per_limb -= 1;
    big_base = Natural::from(base).pow(chars_per_limb);
    let normalization_steps = Limb::WIDTH.wrapping_sub(big_base.significant_bits());
    let mut big_base_inverted =
        Natural::power_of_two((Limb::WIDTH << 1).wrapping_sub(normalization_steps)) / &big_base;
    big_base_inverted.clear_bit(Limb::WIDTH);
    let big_base_trailing_zeros = big_base.trailing_zeros().unwrap();
    let big_base_odd = &big_base >> big_base_trailing_zeros;
    let big_base_inverted_mod_b = invert_mod_power_of_two(&big_base_odd, Limb::WIDTH);
    BaseData {
        chars_per_limb,
        big_base_trailing_zeros,
        big_base,
        normalization_steps,
        big_base_inverted,
        big_base_inverted_mod_b,
    }
}

// This is header from gen-bases.c, GMP 6.2.1.
fn header() {
    let data = generate(10);
    println!("// mp_bases[10] data, as literal values");
    println!(
        "pub const MP_BASES_CHARS_PER_LIMB_10: usize = {};",
        data.chars_per_limb
    );
    println!(
        "pub const MP_BASES_BIG_BASE_CTZ_10: usize = {};",
        data.big_base_trailing_zeros
    );
    println!(
        "pub const MP_BASES_BIG_BASE_10: Limb = {:#x};",
        Limb::exact_from(data.big_base)
    );
    println!(
        "pub const MP_BASES_BIG_BASE_INVERTED_10: Limb = {:#x};",
        Limb::exact_from(data.big_base_inverted)
    );
    println!(
        "pub const MP_BASES_BIG_BASE_BINVERTED_10: Limb = {:#x};",
        Limb::exact_from(data.big_base_inverted_mod_b)
    );
    println!(
        "pub const MP_BASES_NORMALIZATION_STEPS_10: u64 = {};",
        data.normalization_steps
    );
}

// This is table from gen-bases.c, GMP 6.2.1.
fn table() {
    println!("// Format is (chars_per_limb, logb2, log2b, big_base, big_base_inverted)");
    println!("pub const BASES: [(usize, Limb, Limb, Limb, Limb); 257] = [");
    println!("    (0, 0, 0, 0, 0), // 0");
    println!("    (0, 0, 0, 0, 0), // 1");
    for base in 2..=256 {
        let mut data = generate(base);
        let raw = get_log_base_of_2(base, Limb::WIDTH + 8);
        let log_base_of_2 = &raw >> 8;
        let log_2_of_base = Natural::low_mask((Limb::WIDTH << 1) + 5) / (raw + Natural::ONE);
        if base.is_power_of_two() {
            data.big_base = Natural::from(base.significant_bits() - 1);
            data.big_base_inverted = Natural::ZERO;
        }
        println!(
            "    ({}, {:#x}, {:#x}, {:#x}, {:#x}), // {}",
            data.chars_per_limb,
            Limb::exact_from(log_base_of_2),
            Limb::exact_from(log_2_of_base),
            Limb::exact_from(data.big_base),
            Limb::exact_from(data.big_base_inverted),
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
