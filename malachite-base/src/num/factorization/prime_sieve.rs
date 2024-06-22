// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `primesieve.c` contributed to the GNU project by Marco Bodrato.
//
//      Copyright © 2010-2012, 2015, 2016 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::num::logic::traits::CountOnes;

pub const SIEVE_SEED_U32: u32 = 0x69128480;
// 70bits pre-sieved mask for primes 5, 7
pub const SIEVE_MASK1_U32: u32 = 0x12148960;
pub const SIEVE_MASK2_U32: u32 = 0x44a120cc;
pub const SIEVE_MASKT_U32: u32 = 0x1a;
pub const SEED_LIMIT_U32: u64 = 120;

pub const SIEVE_SEED_U64: u64 = 0x3294C9E069128480;
// 110bits pre-sieved mask for primes 5, 11
pub const SIEVE_MASK1_U64: u64 = 0x81214a1204892058;
pub const SIEVE_MASKT_U64: u64 = 0xc8130681244;
// 182bits pre-sieved mask for primes 7, 13
pub const SIEVE_2MSK1_U64: u64 = 0x9402180c40230184;
pub const SIEVE_2MSK2_U64: u64 = 0x0285021088402120;
pub const SIEVE_2MSKT_U64: u64 = 0xa41210084421;
pub const SEED_LIMIT_U64: u64 = 210;

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `bit_array.len()`.
//
// This is equivalent to `fill_bitpattern` from `primesieve.c`, GMP 6.2.1.
fn fill_bitpattern_u32(bit_array: &mut [u32], mut offset: u64) -> u64 {
    let mut len = bit_array.len();
    let (mut mask, mut mask2, mut tail) = if offset == 0 {
        (SIEVE_MASK1_U32, SIEVE_MASK2_U32, SIEVE_MASKT_U32)
    } else {
        offset %= 70;
        if offset != 0 {
            if offset <= u32::WIDTH {
                let offset_comp = u32::WIDTH - offset;
                let mut mask = SIEVE_MASK2_U32 << offset_comp;
                let mut mask2 = SIEVE_MASKT_U32 << offset_comp;
                if offset != u32::WIDTH {
                    mask |= SIEVE_MASK1_U32 >> offset;
                    mask2 |= SIEVE_MASK2_U32 >> offset;
                }
                let tail = if offset <= 70 - 2 * u32::WIDTH {
                    SIEVE_MASK1_U32 << (70 - 2 * u32::WIDTH - offset) | SIEVE_MASKT_U32 >> offset
                } else {
                    mask2 |= SIEVE_MASK1_U32 << (70 - u32::WIDTH - offset);
                    SIEVE_MASK1_U32 >> (offset + 2 * u32::WIDTH - 70)
                };
                (mask, mask2, tail)
            } else if offset < 2 * u32::WIDTH {
                let mut mask = SIEVE_MASK2_U32 >> (offset - u32::WIDTH)
                    | SIEVE_MASKT_U32 << (2 * u32::WIDTH - offset);
                if offset <= 70 - u32::WIDTH {
                    let mut tail = SIEVE_MASK2_U32 << (70 - u32::WIDTH - offset);
                    if offset != 70 - u32::WIDTH {
                        tail |= SIEVE_MASK1_U32 >> (offset + 2 * u32::WIDTH - 70);
                    }
                    (
                        mask,
                        SIEVE_MASKT_U32 >> (offset - u32::WIDTH)
                            | SIEVE_MASK1_U32 << (70 - u32::WIDTH - offset),
                        tail,
                    )
                } else {
                    mask |= SIEVE_MASK1_U32 << (70 - offset);
                    (
                        mask,
                        SIEVE_MASK2_U32 << (70 - offset)
                            | SIEVE_MASK1_U32 >> (u32::WIDTH - (70 - offset)),
                        SIEVE_MASK2_U32 >> (u32::WIDTH - (70 - offset)),
                    )
                }
            } else {
                (
                    SIEVE_MASK1_U32 << (70 - offset) | SIEVE_MASKT_U32 >> (offset - 2 * u32::WIDTH),
                    SIEVE_MASK2_U32 << (70 - offset)
                        | SIEVE_MASK1_U32 >> (offset + u32::WIDTH - 70),
                    SIEVE_MASKT_U32 << (70 - offset)
                        | SIEVE_MASK2_U32 >> (offset + u32::WIDTH - 70),
                )
            }
        } else {
            (SIEVE_MASK1_U32, SIEVE_MASK2_U32, SIEVE_MASKT_U32)
        }
    };
    for xs in bit_array.chunks_mut(2) {
        xs[0] = mask;
        len -= 1;
        if len == 0 {
            break;
        }
        xs[1] = mask2;
        let temp = mask2 >> (3 * u32::WIDTH - 70);
        mask2 = mask2 << (70 - u32::WIDTH * 2) | mask >> (3 * u32::WIDTH - 70);
        mask = mask << (70 - u32::WIDTH * 2) | tail;
        tail = temp;
        len -= 1;
        if len == 0 {
            break;
        }
    }
    2
}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `bit_array.len()`.
//
// This is equivalent to `fill_bitpattern` from `primesieve.c`, GMP 6.2.1.
fn fill_bitpattern_u64(bit_array: &mut [u64], mut offset: u64) -> u64 {
    let mut len = bit_array.len();
    let ((mut m11, mut m12), (mut m21, mut m22, mut m23)) = if offset == 0 {
        (
            (SIEVE_MASK1_U64, SIEVE_MASKT_U64),
            (SIEVE_2MSK1_U64, SIEVE_2MSK2_U64, SIEVE_2MSKT_U64),
        )
    } else {
        // correctly handle offset == 0...
        let mut m21 = offset % 110;
        let (m11, m12) = if m21 != 0 {
            if m21 < u64::WIDTH {
                let mut m11 = (SIEVE_MASK1_U64 >> m21) | (SIEVE_MASKT_U64 << (u64::WIDTH - m21));
                if m21 <= 110 - u64::WIDTH {
                    (
                        m11,
                        SIEVE_MASK1_U64 << (110 - u64::WIDTH - m21) | SIEVE_MASKT_U64 >> m21,
                    )
                } else {
                    m11 |= SIEVE_MASK1_U64 << (110 - m21);
                    (m11, SIEVE_MASK1_U64 >> (m21 + u64::WIDTH - 110))
                }
            } else {
                (
                    SIEVE_MASK1_U64 << (110 - m21) | SIEVE_MASKT_U64 >> (m21 - u64::WIDTH),
                    SIEVE_MASKT_U64 << (110 - m21) | SIEVE_MASK1_U64 >> (m21 + u64::WIDTH - 110),
                )
            }
        } else {
            (SIEVE_MASK1_U64, SIEVE_MASKT_U64)
        };
        ((m11, m12), {
            offset %= 182;
            if offset != 0 {
                if offset <= u64::WIDTH {
                    let mut m21 = SIEVE_2MSK2_U64 << (u64::WIDTH - offset);
                    let mut m22 = SIEVE_2MSKT_U64 << (u64::WIDTH - offset);
                    if offset != u64::WIDTH {
                        m21 |= SIEVE_2MSK1_U64 >> offset;
                        m22 |= SIEVE_2MSK2_U64 >> offset;
                    }
                    if offset <= 182 - 2 * u64::WIDTH {
                        (
                            m21,
                            m22,
                            SIEVE_2MSK1_U64 << (182 - 2 * u64::WIDTH - offset)
                                | SIEVE_2MSKT_U64 >> offset,
                        )
                    } else {
                        m22 |= SIEVE_2MSK1_U64 << (182 - u64::WIDTH - offset);
                        (m21, m22, SIEVE_2MSK1_U64 >> (offset + 2 * u64::WIDTH - 182))
                    }
                } else if offset < 2 * u64::WIDTH {
                    m21 = SIEVE_2MSK2_U64 >> (offset - u64::WIDTH)
                        | SIEVE_2MSKT_U64 << (2 * u64::WIDTH - offset);
                    if offset <= 182 - u64::WIDTH {
                        let mut m23 = SIEVE_2MSK2_U64 << (182 - u64::WIDTH - offset);
                        if offset != 182 - u64::WIDTH {
                            m23 |= SIEVE_2MSK1_U64 >> (offset + 2 * u64::WIDTH - 182);
                        }
                        (
                            m21,
                            SIEVE_2MSKT_U64 >> (offset - u64::WIDTH)
                                | SIEVE_2MSK1_U64 << (182 - u64::WIDTH - offset),
                            m23,
                        )
                    } else {
                        m21 |= SIEVE_2MSK1_U64 << (182 - offset);
                        (
                            m21,
                            SIEVE_2MSK2_U64 << (182 - offset)
                                | SIEVE_2MSK1_U64 >> (u64::WIDTH + offset - 182),
                            SIEVE_2MSK2_U64 >> (u64::WIDTH + offset - 182),
                        )
                    }
                } else {
                    (
                        SIEVE_2MSK1_U64 << (182 - offset)
                            | SIEVE_2MSKT_U64 >> (offset - 2 * u64::WIDTH),
                        SIEVE_2MSK2_U64 << (182 - offset)
                            | SIEVE_2MSK1_U64 >> (offset + u64::WIDTH - 182),
                        SIEVE_2MSKT_U64 << (182 - offset)
                            | SIEVE_2MSK2_U64 >> (offset + u64::WIDTH - 182),
                    )
                }
            } else {
                (SIEVE_2MSK1_U64, SIEVE_2MSK2_U64, SIEVE_2MSKT_U64)
            }
        })
    };
    for xs in bit_array.chunks_mut(2) {
        xs[0] = m11 | m21;
        len -= 1;
        if len == 0 {
            break;
        }
        let temp = m11 >> (2 * u64::WIDTH - 110);
        m11 = (m11 << (110 - u64::WIDTH)) | m12;
        m12 = temp;
        xs[1] = m11 | m22;
        let temp = m11 >> (2 * u64::WIDTH - 110);
        m11 = (m11 << (110 - u64::WIDTH)) | m12;
        m12 = temp;
        let temp = m22 >> (3 * u64::WIDTH - 182);
        m22 = m22 << (182 - u64::WIDTH * 2) | m21 >> (3 * u64::WIDTH - 182);
        m21 = m21 << (182 - u64::WIDTH * 2) | m23;
        m23 = temp;
        len -= 1;
        if len == 0 {
            break;
        }
    }
    4
}

#[doc(hidden)]
// This is equivalent to `n_to_bit` from `primesieve.c`, GMP 6.2.1.
pub const fn n_to_bit(n: u64) -> u64 {
    ((n - 5) | 1) / 3
}

#[doc(hidden)]
// This is equivalent to `id_to_n` from `primesieve.c`, GMP 6.2.1.
pub const fn id_to_n(id: u64) -> u64 {
    id * 3 + 1 + (id & 1)
}

// # Worst-case complexity
// $T(n) = O(n\log\log n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `bit_array.len()`.
//
// This is equivalent to `first_block_primesieve` from `primesieve.c`, GMP 6.2.1.
fn first_block_primesieve<T: PrimitiveUnsigned, F: Fn(&mut [T], u64) -> u64>(
    bit_array: &mut [T],
    n: u64,
    fill_bitpattern: F,
    sieve_seed: T,
    seed_limit: u64,
) {
    assert!(n > 4);
    let bits = n_to_bit(n);
    let limbs = usize::exact_from(bits >> T::LOG_WIDTH);
    let mut i = if limbs == 0 {
        0
    } else {
        fill_bitpattern(&mut bit_array[1..=limbs], 0)
    };
    bit_array[0] = sieve_seed;
    if (bits + 1) & T::WIDTH_MASK != 0 {
        bit_array[limbs] |= T::MAX << ((bits + 1) & T::WIDTH_MASK);
    }
    if n > seed_limit {
        assert!(i < T::WIDTH);
        if n_to_bit(seed_limit + 1) < T::WIDTH {
            i = 0;
        }
        let mut mask = T::power_of_2(i);
        let mut index = 0;
        for i in i + 1.. {
            if bit_array[index] & mask == T::ZERO {
                let mut step = id_to_n(i);
                // lindex = n_to_bit(id_to_n(i) * id_to_n(i));
                let mut lindex = i * (step + 1) - 1 + ((i & 1).wrapping_neg() & (i + 1));
                // lindex = i * (step + 1 + (i & 1)) - 1 + (i & 1);
                if lindex > bits {
                    break;
                }
                step <<= 1;
                let maskrot = step & T::WIDTH_MASK;
                let mut lmask = T::power_of_2(lindex & T::WIDTH_MASK);
                while lindex <= bits {
                    bit_array[usize::exact_from(lindex >> T::LOG_WIDTH)] |= lmask;
                    lmask.rotate_left_assign(maskrot);
                    lindex += step;
                }
                // lindex = n_to_bit(id_to_n(i) * bit_to_n(i));
                lindex = i * (i * 3 + 6) + (i & 1);
                lmask = T::power_of_2(lindex & T::WIDTH_MASK);
                while lindex <= bits {
                    bit_array[usize::exact_from(lindex >> T::LOG_WIDTH)] |= lmask;
                    lmask.rotate_left_assign(maskrot);
                    lindex += step;
                }
            }
            mask <<= 1;
            if mask == T::ZERO {
                mask = T::ONE;
                index += 1;
            }
        }
    }
}

// # Worst-case complexity
// $T(n) = O(n\log\log n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `bit_array.len()`.
//
// This is equivalent to `block_resieve` from `primesieve.c`, GMP 6.2.1.
fn block_resieve<T: PrimitiveUnsigned, F: Fn(&mut [T], u64) -> u64>(
    bit_array: &mut [T],
    offset: u64,
    sieve: &[T],
    fill_bitpattern: &F,
) {
    let limbs = bit_array.len();
    let off = offset;
    assert_ne!(limbs, 0);
    assert!(offset >= T::WIDTH);
    let bits = u64::exact_from(limbs << T::LOG_WIDTH) - 1;
    let i = fill_bitpattern(&mut bit_array[..limbs], offset - T::WIDTH);
    assert!(i < T::WIDTH);
    let mut mask = T::power_of_2(i);
    let mut index = 0;
    for i in i + 1.. {
        if sieve[index] & mask == T::ZERO {
            let mut step = id_to_n(i);
            // lindex = n_to_bit(id_to_n(i)*id_to_n(i));
            let mut lindex = i * (step + 1) - 1 + ((i & 1).wrapping_neg() & (i + 1));
            // lindex = i*(step+1+(i&1))-1+(i&1);
            if lindex > bits + off {
                break;
            }
            step <<= 1;
            let maskrot = step & T::WIDTH_MASK;
            if lindex < off {
                lindex += step * ((off - lindex - 1) / step + 1);
            }
            lindex -= off;
            let mut lmask = T::power_of_2(lindex & T::WIDTH_MASK);
            while lindex <= bits {
                bit_array[usize::exact_from(lindex >> T::LOG_WIDTH)] |= lmask;
                lmask.rotate_left_assign(maskrot);
                lindex += step;
            }
            // lindex = n_to_bit(id_to_n(i)*bit_to_n(i));
            lindex = i * (i * 3 + 6) + (i & 1);
            if lindex < off {
                lindex += step * ((off - lindex - 1) / step + 1);
            }
            lindex -= off;
            lmask = T::power_of_2(lindex & T::WIDTH_MASK);
            while lindex <= bits {
                bit_array[usize::exact_from(lindex >> T::LOG_WIDTH)] |= lmask;
                lmask.rotate_left_assign(maskrot);
                lindex += step;
            }
        }
        mask <<= 1;
        if mask == T::ZERO {
            mask = T::ONE;
            index += 1;
        }
    }
}

#[doc(hidden)]
#[inline]
// This is equivalent to `primesieve_size` from `primesieve.c`, GMP 6.2.1.
pub fn limbs_prime_sieve_size<T: PrimitiveUnsigned>(n: u64) -> usize {
    assert!(n >= 5);
    usize::exact_from((n_to_bit(n) >> T::LOG_WIDTH) + 1)
}

const BLOCK_SIZE: usize = 2048;

pub_test! {limbs_count_ones<T: PrimitiveUnsigned>(xs: &[T]) -> u64 {
    xs.iter().map(|&x| CountOnes::count_ones(x)).sum()
}}

// Fills bit_array with the characteristic function of composite numbers up to the parameter n. I.e.
// a bit set to "1" represent a composite, a "0" represent a prime.
//
// The primesieve_size(n) limbs pointed to by bit_array are overwritten. The returned value counts
// prime integers in the interval [4, n]. Note that n > 4.
//
// Even numbers and multiples of 3 are excluded "a priori", only numbers equivalent to +/- 1 mod 6
// have their bit in the array.
//
// Once sieved, if the bit b is ZERO it represent a prime, the represented prime is bit_to_n(b), if
// the LSbit is bit 0, or id_to_n(b), if you call "1" the first bit.
//
// # Worst-case complexity
// $T(n) = O(n\log\log n)$
//
// $M(n) = O(1)$
fn limbs_prime_sieve_generic<T: PrimitiveUnsigned, F: Fn(&mut [T], u64) -> u64>(
    bit_array: &mut [T],
    n: u64,
    fill_bitpattern: F,
    sieve_seed: T,
    seed_limit: u64,
) -> u64 {
    assert!(n > 4);
    let bits = n_to_bit(n);
    let size = usize::exact_from((bits >> T::LOG_WIDTH) + 1);
    if size > BLOCK_SIZE << 1 {
        let mut off = BLOCK_SIZE + (size % BLOCK_SIZE);
        first_block_primesieve(
            bit_array,
            id_to_n(u64::exact_from(off) << T::LOG_WIDTH),
            &fill_bitpattern,
            sieve_seed,
            seed_limit,
        );
        let (sieve, bit_array) = bit_array.split_at_mut(off);
        for xs in bit_array.chunks_mut(BLOCK_SIZE) {
            block_resieve(
                xs,
                u64::exact_from(off) << T::LOG_WIDTH,
                sieve,
                &fill_bitpattern,
            );
            off += BLOCK_SIZE;
            if off >= size {
                break;
            }
        }
    } else {
        first_block_primesieve(bit_array, n, &fill_bitpattern, sieve_seed, seed_limit);
    }
    if (bits + 1) & T::WIDTH_MASK != 0 {
        bit_array[size - 1] |= T::MAX << ((bits + 1) & T::WIDTH_MASK);
    }
    (u64::exact_from(size) << T::LOG_WIDTH) - limbs_count_ones(&bit_array[..size])
}

#[doc(hidden)]
// This is equivalent to `gmp_primesieve` from `primesieve.c`, GMP 6.2.1.
#[inline]
pub fn limbs_prime_sieve_u32(bit_array: &mut [u32], n: u64) -> u64 {
    limbs_prime_sieve_generic(
        bit_array,
        n,
        fill_bitpattern_u32,
        SIEVE_SEED_U32,
        SEED_LIMIT_U32,
    )
}

#[doc(hidden)]
// This is equivalent to `gmp_primesieve` from `primesieve.c`, GMP 6.2.1.
#[inline]
pub fn limbs_prime_sieve_u64(bit_array: &mut [u64], n: u64) -> u64 {
    limbs_prime_sieve_generic(
        bit_array,
        n,
        fill_bitpattern_u64,
        SIEVE_SEED_U64,
        SEED_LIMIT_U64,
    )
}
