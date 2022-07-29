use crate::malachite_base::num::arithmetic::traits::{DivisibleBy, FloorSqrt, Parity};
use crate::malachite_base::num::basic::integers::PrimitiveInt;
use crate::malachite_base::num::conversion::traits::ExactFrom;
use crate::malachite_base::num::logic::traits::{BitAccess, NotAssign};
use crate::natural::factorization::prime_sieve::{id_to_n, n_to_bit};
use crate::natural::logic::bit_access::{limbs_clear_bit, limbs_slice_set_bit};
use crate::natural::logic::bit_scan::limbs_index_of_next_true_bit;
use crate::natural::logic::count_ones::limbs_count_ones;
use crate::platform::Limb;

// Replace with good primality test once we have one
fn is_prime_simple(n: u64) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        n if n.even() => false,
        n => {
            for f in 3..=n.floor_sqrt() {
                if n.divisible_by(f) {
                    return false;
                }
            }
            true
        }
    }
}

pub fn limbs_prime_sieve_naive_1(bit_array: &mut [Limb], n: u64) -> u64 {
    assert!(n > 4);
    let mut f = 5;
    let mut b = false;
    'outer: for x in bit_array.iter_mut() {
        *x = Limb::MAX;
        for i in 0..Limb::WIDTH {
            if is_prime_simple(f) {
                x.clear_bit(i);
            }
            f += if b { 4 } else { 2 };
            if f > n {
                break 'outer;
            }
            b.not_assign();
        }
    }
    (u64::exact_from(bit_array.len()) << Limb::LOG_WIDTH) - limbs_count_ones(bit_array)
}

pub fn limbs_prime_sieve_naive_2(bit_array: &mut [Limb], n: u64) -> u64 {
    assert!(n > 4);
    for x in bit_array.iter_mut() {
        *x = Limb::MAX;
    }
    let mut p = 0;
    loop {
        let id = limbs_index_of_next_true_bit(bit_array, if p == 0 { 0 } else { n_to_bit(p) + 1 });
        if let Some(id) = id {
            p = id_to_n(id + 1);
        } else {
            break;
        }
        let mut m = p * 5;
        if m > n {
            break;
        }
        let two_p = p << 1;
        while m <= n {
            if m.odd() && !m.divisible_by(3) {
                limbs_clear_bit(bit_array, n_to_bit(m));
            }
            m += two_p;
        }
    }
    for x in bit_array.iter_mut() {
        x.not_assign();
    }
    let bit_len = u64::exact_from(bit_array.len()) << Limb::LOG_WIDTH;
    for i in (0..bit_len).rev() {
        if id_to_n(i + 1) <= n {
            break;
        }
        limbs_slice_set_bit(bit_array, i);
    }
    bit_len - limbs_count_ones(bit_array)
}
