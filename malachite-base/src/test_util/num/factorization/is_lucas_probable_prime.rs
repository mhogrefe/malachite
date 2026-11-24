// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{FloorSqrt, Parity};
use crate::num::logic::traits::{BitAccess, SignificantBits};
use crate::num::modular::traits::{ModAdd, ModMul, ModMulAssign, ModSquare, ModSub};
use crate::num::number_theory::traits::JacobiSymbol;

// x / 2 mod an odd n, for x < n.
fn half_mod(x: u128, n: u128) -> u128 {
    if x.even() {
        x >> 1
    } else {
        // (x + n) / 2, without overflow
        (x >> 1) + (n >> 1) + 1
    }
}

// D mod n, for a small signed D.
fn signed_mod(d: i64, n: u128) -> u128 {
    let r = u128::from(d.unsigned_abs()) % n;
    if d < 0 && r != 0 { n - r } else { r }
}

/// Whether n is a Lucas probable prime with Selfridge's parameters: D is the first of 5, -7, 9,
/// -11, ... with (D / n) = -1, P = 1, Q = (1 - D) / 4, and U_{n+1}(P, Q) = 0 mod n. Composites
/// sharing a factor with D or Q, even numbers other than 2, squares, and 0 and 1 fail.
///
/// U_{n+1} is computed directly with the doubling formulas for U_k, V_k, and Q^k.
pub fn is_lucas_probable_prime_naive(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n.even() {
        return n == 2;
    }
    let root = n.floor_sqrt();
    if root * root == n {
        return false;
    }
    let mut d: i64 = 5;
    let q = loop {
        let r = signed_mod(d, n);
        match r.jacobi_symbol(n) {
            -1 => {
                // D = 1 mod 4, so the shift is exact.
                let q = (1 - d) >> 2;
                let mut a = n;
                let mut b = u128::from(q.unsigned_abs());
                while b != 0 {
                    (a, b) = (b, a % b);
                }
                // a = n happens only for tiny n, which divide Q; then the next D is tried.
                if a == 1 {
                    break q;
                } else if a != n {
                    return false;
                }
            }
            0 if r != 0 => return false,
            _ => {}
        }
        d = if d > 0 { -(d + 2) } else { -(d - 2) };
    };
    let q_mod = signed_mod(q, n);
    let d_mod = signed_mod(d, n);
    // n + 1, as bits, without overflow: n is odd, so n + 1 = 2 * ((n >> 1) + 1)
    let k_half = (n >> 1) + 1;
    // Start from U_1 = 1, V_1 = P = 1, Q^1 = Q, at the top bit of k_half, then double once more at
    // the end for the final factor of 2.
    let mut u = 1 % n;
    let mut v = 1 % n;
    let mut qk = q_mod;
    let bits = k_half.significant_bits();
    let step_double = |u: u128, v: u128, qk: u128| {
        (
            u.mod_mul(v, n),
            v.mod_square(n).mod_sub(qk.mod_add(qk, n), n),
            qk.mod_square(n),
        )
    };
    for bit in (0..bits - 1).rev() {
        (u, v, qk) = step_double(u, v, qk);
        if k_half.get_bit(bit) {
            let (u2, v2) = (u, v);
            u = half_mod(u2.mod_add(v2, n), n);
            v = half_mod(d_mod.mod_mul(u2, n).mod_add(v2, n), n);
            qk.mod_mul_assign(q_mod, n);
        }
    }
    step_double(u, v, qk).0 == 0
}
