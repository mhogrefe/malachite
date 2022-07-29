use crate::malachite_base::num::arithmetic::traits::{Parity, PowerOf2};
use crate::malachite_base::num::basic::integers::PrimitiveInt;
use crate::malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::natural::logic::count_ones::limbs_count_ones;
#[cfg(feature = "32_bit_limbs")]
use crate::platform::SIEVE_MASK2;
use crate::platform::{Limb, SEED_LIMIT, SIEVE_MASK1, SIEVE_MASKT, SIEVE_SEED};
#[cfg(not(feature = "32_bit_limbs"))]
use crate::platform::{SIEVE_2MSK1, SIEVE_2MSK2, SIEVE_2MSKT};

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `bit_array.len()`.
//
// This is equivalent to `fill_bitpattern` from `primesieve.c`, GMP 6.2.1.
#[cfg(feature = "32_bit_limbs")]
fn fill_bitpattern(bit_array: &mut [Limb], mut offset: u64) -> u64 {
    let mut len = bit_array.len();
    let (mut mask, mut mask2, mut tail) = if offset == 0 {
        (SIEVE_MASK1, SIEVE_MASK2, SIEVE_MASKT)
    } else {
        offset %= 70;
        if offset != 0 {
            if offset <= Limb::WIDTH {
                let offset_comp = Limb::WIDTH - offset;
                let mut mask = SIEVE_MASK2 << offset_comp;
                let mut mask2 = SIEVE_MASKT << offset_comp;
                if offset != Limb::WIDTH {
                    mask |= SIEVE_MASK1 >> offset;
                    mask2 |= SIEVE_MASK2 >> offset;
                }
                let tail = if offset <= 70 - 2 * Limb::WIDTH {
                    SIEVE_MASK1 << (70 - 2 * Limb::WIDTH - offset) | SIEVE_MASKT >> offset
                } else {
                    mask2 |= SIEVE_MASK1 << (70 - Limb::WIDTH - offset);
                    SIEVE_MASK1 >> (offset + 2 * Limb::WIDTH - 70)
                };
                (mask, mask2, tail)
            } else if offset < 2 * Limb::WIDTH {
                let mut mask = SIEVE_MASK2 >> (offset - Limb::WIDTH)
                    | SIEVE_MASKT << (2 * Limb::WIDTH - offset);
                if offset <= 70 - Limb::WIDTH {
                    let mut tail = SIEVE_MASK2 << (70 - Limb::WIDTH - offset);
                    if offset != 70 - Limb::WIDTH {
                        tail |= SIEVE_MASK1 >> (offset + 2 * Limb::WIDTH - 70);
                    }
                    (
                        mask,
                        SIEVE_MASKT >> (offset - Limb::WIDTH)
                            | SIEVE_MASK1 << (70 - Limb::WIDTH - offset),
                        tail,
                    )
                } else {
                    mask |= SIEVE_MASK1 << (70 - offset);
                    (
                        mask,
                        SIEVE_MASK2 << (70 - offset) | SIEVE_MASK1 >> (Limb::WIDTH - (70 - offset)),
                        SIEVE_MASK2 >> (Limb::WIDTH - (70 - offset)),
                    )
                }
            } else {
                (
                    SIEVE_MASK1 << (70 - offset) | SIEVE_MASKT >> (offset - 2 * Limb::WIDTH),
                    SIEVE_MASK2 << (70 - offset) | SIEVE_MASK1 >> (offset + Limb::WIDTH - 70),
                    SIEVE_MASKT << (70 - offset) | SIEVE_MASK2 >> (offset + Limb::WIDTH - 70),
                )
            }
        } else {
            (SIEVE_MASK1, SIEVE_MASK2, SIEVE_MASKT)
        }
    };
    for xs in bit_array.chunks_mut(2) {
        xs[0] = mask;
        len -= 1;
        if len == 0 {
            break;
        }
        xs[1] = mask2;
        let temp = mask2 >> (3 * Limb::WIDTH - 70);
        mask2 = mask2 << (70 - Limb::WIDTH * 2) | mask >> (3 * Limb::WIDTH - 70);
        mask = mask << (70 - Limb::WIDTH * 2) | tail;
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
#[cfg(not(feature = "32_bit_limbs"))]
fn fill_bitpattern(bit_array: &mut [Limb], mut offset: u64) -> u64 {
    let mut len = bit_array.len();
    let ((mut m11, mut m12), (mut m21, mut m22, mut m23)) = if offset == 0 {
        (
            (SIEVE_MASK1, SIEVE_MASKT),
            (SIEVE_2MSK1, SIEVE_2MSK2, SIEVE_2MSKT),
        )
    } else {
        // correctly handle offset == 0...
        let mut m21 = offset % 110;
        (
            if m21 != 0 {
                if m21 < Limb::WIDTH {
                    let mut m11 = (SIEVE_MASK1 >> m21) | (SIEVE_MASKT << (Limb::WIDTH - m21));
                    if m21 <= 110 - Limb::WIDTH {
                        (
                            m11,
                            SIEVE_MASK1 << (110 - Limb::WIDTH - m21) | SIEVE_MASKT >> m21,
                        )
                    } else {
                        m11 |= SIEVE_MASK1 << (110 - m21);
                        (m11, SIEVE_MASK1 >> (m21 + Limb::WIDTH - 110))
                    }
                } else {
                    (
                        SIEVE_MASK1 << (110 - m21) | SIEVE_MASKT >> (m21 - Limb::WIDTH),
                        SIEVE_MASKT << (110 - m21) | SIEVE_MASK1 >> (m21 + Limb::WIDTH - 110),
                    )
                }
            } else {
                (SIEVE_MASK1, SIEVE_MASKT)
            },
            {
                offset %= 182;
                if offset != 0 {
                    if offset <= Limb::WIDTH {
                        let mut m21 = SIEVE_2MSK2 << (Limb::WIDTH - offset);
                        let mut m22 = SIEVE_2MSKT << (Limb::WIDTH - offset);
                        if offset != Limb::WIDTH {
                            m21 |= SIEVE_2MSK1 >> offset;
                            m22 |= SIEVE_2MSK2 >> offset;
                        }
                        if offset <= 182 - 2 * Limb::WIDTH {
                            (
                                m21,
                                m22,
                                SIEVE_2MSK1 << (182 - 2 * Limb::WIDTH - offset)
                                    | SIEVE_2MSKT >> offset,
                            )
                        } else {
                            m22 |= SIEVE_2MSK1 << (182 - Limb::WIDTH - offset);
                            (m21, m22, SIEVE_2MSK1 >> (offset + 2 * Limb::WIDTH - 182))
                        }
                    } else if offset < 2 * Limb::WIDTH {
                        m21 = SIEVE_2MSK2 >> (offset - Limb::WIDTH)
                            | SIEVE_2MSKT << (2 * Limb::WIDTH - offset);
                        if offset <= 182 - Limb::WIDTH {
                            let mut m23 = SIEVE_2MSK2 << (182 - Limb::WIDTH - offset);
                            if offset != 182 - Limb::WIDTH {
                                m23 |= SIEVE_2MSK1 >> (offset + 2 * Limb::WIDTH - 182);
                            }
                            (
                                m21,
                                SIEVE_2MSKT >> (offset - Limb::WIDTH)
                                    | SIEVE_2MSK1 << (182 - Limb::WIDTH - offset),
                                m23,
                            )
                        } else {
                            m21 |= SIEVE_2MSK1 << (182 - offset);
                            (
                                m21,
                                SIEVE_2MSK2 << (182 - offset)
                                    | SIEVE_2MSK1 >> (Limb::WIDTH + offset - 182),
                                SIEVE_2MSK2 >> (Limb::WIDTH + offset - 182),
                            )
                        }
                    } else {
                        (
                            SIEVE_2MSK1 << (182 - offset)
                                | SIEVE_2MSKT >> (offset - 2 * Limb::WIDTH),
                            SIEVE_2MSK2 << (182 - offset)
                                | SIEVE_2MSK1 >> (offset + Limb::WIDTH - 182),
                            SIEVE_2MSKT << (182 - offset)
                                | SIEVE_2MSK2 >> (offset + Limb::WIDTH - 182),
                        )
                    }
                } else {
                    (SIEVE_2MSK1, SIEVE_2MSK2, SIEVE_2MSKT)
                }
            },
        )
    };
    for xs in bit_array.chunks_mut(2) {
        xs[0] = m11 | m21;
        len -= 1;
        if len == 0 {
            break;
        }
        let temp = m11 >> (2 * Limb::WIDTH - 110);
        m11 = (m11 << (110 - Limb::WIDTH)) | m12;
        m12 = temp;
        xs[1] = m11 | m22;
        let temp = m11 >> (2 * Limb::WIDTH - 110);
        m11 = (m11 << (110 - Limb::WIDTH)) | m12;
        m12 = temp;
        let temp = m22 >> (3 * Limb::WIDTH - 182);
        m22 = m22 << (182 - Limb::WIDTH * 2) | m21 >> (3 * Limb::WIDTH - 182);
        m21 = m21 << (182 - Limb::WIDTH * 2) | m23;
        m23 = temp;
        len -= 1;
        if len == 0 {
            break;
        }
    }
    4
}

// This is equivalent to `n_to_bit` from `primesieve.c`, GMP 6.2.1.
pub(crate) const fn n_to_bit(n: u64) -> u64 {
    ((n - 5) | 1) / 3
}

// This is equivalent to `id_to_n` from `primesieve.c`, GMP 6.2.1.
pub(crate) const fn id_to_n(id: u64) -> u64 {
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
fn first_block_primesieve(bit_array: &mut [Limb], n: u64) {
    assert!(n > 4);
    let bits = n_to_bit(n);
    let limbs = usize::exact_from(bits >> Limb::LOG_WIDTH);
    let mut i = if limbs == 0 {
        0
    } else {
        fill_bitpattern(&mut bit_array[1..limbs + 1], 0)
    };
    bit_array[0] = SIEVE_SEED;
    if (bits + 1) & Limb::WIDTH_MASK != 0 {
        bit_array[limbs] |= Limb::MAX << ((bits + 1) & Limb::WIDTH_MASK);
    }
    if n > SEED_LIMIT {
        assert!(i < Limb::WIDTH);
        if n_to_bit(SEED_LIMIT + 1) < Limb::WIDTH {
            i = 0;
        }
        let mut mask = Limb::power_of_2(i);
        let mut index = 0;
        for i in i + 1.. {
            if bit_array[index] & mask == 0 {
                let mut step = id_to_n(i);
                // lindex = n_to_bit(id_to_n(i)*id_to_n(i));
                let mut lindex = i * (step + 1) - 1 + ((i & 1).wrapping_neg() & (i + 1));
                // lindex = i*(step+1+(i&1))-1+(i&1);
                if lindex > bits {
                    break;
                }
                step <<= 1;
                let maskrot = u32::wrapping_from(step & Limb::WIDTH_MASK);
                let mut lmask = Limb::power_of_2(lindex & Limb::WIDTH_MASK);
                while lindex <= bits {
                    bit_array[usize::exact_from(lindex >> Limb::LOG_WIDTH)] |= lmask;
                    lmask = lmask.rotate_left(maskrot);
                    lindex += step;
                }
                // lindex = n_to_bit(id_to_n(i)*bit_to_n(i));
                lindex = i * (i * 3 + 6) + (i & 1);
                lmask = Limb::power_of_2(lindex & Limb::WIDTH_MASK);
                while lindex <= bits {
                    bit_array[usize::exact_from(lindex >> Limb::LOG_WIDTH)] |= lmask;
                    lmask = lmask.rotate_left(maskrot);
                    lindex += step
                }
            }
            mask = mask.rotate_left(1);
            if mask.odd() {
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
fn block_resieve(bit_array: &mut [Limb], offset: u64, sieve: &[Limb]) {
    let limbs = bit_array.len();
    let off = offset;
    assert_ne!(limbs, 0);
    assert!(offset >= Limb::WIDTH);
    let bits = u64::exact_from(limbs << Limb::LOG_WIDTH) - 1;
    let i = fill_bitpattern(&mut bit_array[..limbs], offset - Limb::WIDTH);
    assert!(i < Limb::WIDTH);
    let mut mask = Limb::power_of_2(i);
    let mut index = 0;
    for i in i + 1.. {
        if sieve[index] & mask == 0 {
            let mut step = id_to_n(i);
            // lindex = n_to_bit(id_to_n(i)*id_to_n(i));
            let mut lindex = i * (step + 1) - 1 + ((i & 1).wrapping_neg() & (i + 1));
            // lindex = i*(step+1+(i&1))-1+(i&1);
            if lindex > bits + off {
                break;
            }
            step <<= 1;
            let maskrot = u32::wrapping_from(step & Limb::WIDTH_MASK);
            if lindex < off {
                lindex += step * ((off - lindex - 1) / step + 1);
            }
            lindex -= off;
            let mut lmask = Limb::power_of_2(lindex & Limb::WIDTH_MASK);
            while lindex <= bits {
                bit_array[usize::exact_from(lindex >> Limb::LOG_WIDTH)] |= lmask;
                lmask = lmask.rotate_left(maskrot);
                lindex += step;
            }
            // lindex = n_to_bit(id_to_n(i)*bit_to_n(i));
            lindex = i * (i * 3 + 6) + (i & 1);
            if lindex < off {
                lindex += step * ((off - lindex - 1) / step + 1);
            }
            lindex -= off;
            lmask = Limb::power_of_2(lindex & Limb::WIDTH_MASK);
            while lindex <= bits {
                bit_array[usize::exact_from(lindex >> Limb::LOG_WIDTH)] |= lmask;
                lmask = lmask.rotate_left(maskrot);
                lindex += step;
            }
        }
        mask = mask.rotate_left(1);
        if mask.odd() {
            index += 1;
        }
    }
}

// This is equivalent to `primesieve_size` from `primesieve.c`, GMP 6.2.1.
pub fn limbs_prime_sieve_size(n: u64) -> usize {
    usize::exact_from((n_to_bit(n) >> Limb::LOG_WIDTH) + 1)
}

const BLOCK_SIZE: usize = 2048;

// Fills bit_array with the characteristic function of composite
// numbers up to the parameter n. I.e. a bit set to "1" represent a
// composite, a "0" represent a prime.
//
// The primesieve_size(n) limbs pointed to by bit_array are
// overwritten. The returned value counts prime integers in the
// interval [4, n]. Note that n > 4.
//
// Even numbers and multiples of 3 are excluded "a priori", only
// numbers equivalent to +/- 1 mod 6 have their bit in the array.
//
// Once sieved, if the bit b is ZERO it represent a prime, the
// represented prime is bit_to_n(b), if the LSbit is bit 0, or
// id_to_n(b), if you call "1" the first bit.
//
// # Worst-case complexity
// $T(n) = O(n\log\log n)$
//
// $M(n) = O(1)$
//
// This is equivalent to `gmp_primesieve` from `primesieve.c`, GMP 6.2.1.
pub fn limbs_prime_sieve(bit_array: &mut [Limb], n: u64) -> u64 {
    assert!(n > 4);
    let bits = n_to_bit(n);
    let size = usize::exact_from((bits >> Limb::LOG_WIDTH) + 1);
    if size > BLOCK_SIZE << 1 {
        let mut off = BLOCK_SIZE + (size % BLOCK_SIZE);
        first_block_primesieve(bit_array, id_to_n(u64::exact_from(off) << Limb::LOG_WIDTH));
        loop {
            let (bit_array_lo, bit_array_hi) = bit_array.split_at_mut(off);
            block_resieve(
                &mut bit_array_hi[..BLOCK_SIZE],
                u64::exact_from(off) << Limb::LOG_WIDTH,
                bit_array_lo,
            );
            off += BLOCK_SIZE;
            if off >= size {
                break;
            }
        }
    } else {
        first_block_primesieve(bit_array, n);
    }
    if (bits + 1) & Limb::WIDTH_MASK != 0 {
        bit_array[size - 1] |= Limb::MAX << ((bits + 1) & Limb::WIDTH_MASK);
    }
    (u64::exact_from(size) << Limb::LOG_WIDTH) - limbs_count_ones(&bit_array[..size])
}
