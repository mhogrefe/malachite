// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1996, 1998, 2000-2004, 2008, 2012, 2019 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::gcd::half_gcd::{
    limbs_half_gcd_matrix_mul_matrix_1, limbs_half_gcd_matrix_update_q, HalfGcdMatrix,
    HalfGcdMatrix1,
};
use crate::natural::Natural;
use crate::platform::{DoubleLimb, Limb};
use malachite_base::num::arithmetic::traits::{DivMod, Parity, XXSubYYToZZ};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};
use malachite_base::num::logic::traits::LeadingZeros;
use malachite_base::slices::slice_test_zero;

pub fn gcd_euclidean_nz(x: Natural, y: Natural) -> Natural {
    if y == 0 {
        x
    } else {
        let r = x % &y;
        gcd_euclidean_nz(y, r)
    }
}

// recursive implementation overflows stack, so using a loop instead
pub fn gcd_binary_nz(mut x: Natural, mut y: Natural) -> Natural {
    let mut twos = 0;
    loop {
        if x == y {
            return x << twos;
        } else if x == 0 {
            return y << twos;
        } else if y == 0 {
            return x << twos;
        } else if x.even() {
            x >>= 1;
            if y.even() {
                y >>= 1;
                twos += 1;
            }
        } else if y.even() {
            y >>= 1;
        } else if x > y {
            x -= &y;
            x >>= 1;
        } else {
            y -= &x;
            y >>= 1;
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedHalfGcdMatrix {
    pub data: Vec<Limb>,
    pub s: usize,
    pub two_s: usize,
    pub three_s: usize,
    pub n: usize,
}

fn from_owned(m: &mut OwnedHalfGcdMatrix) -> HalfGcdMatrix {
    HalfGcdMatrix {
        s: m.s,
        two_s: m.two_s,
        three_s: m.three_s,
        n: m.n,
        data: &mut m.data,
    }
}

impl OwnedHalfGcdMatrix {
    pub fn init(n: usize, mut p: Vec<Limb>) -> OwnedHalfGcdMatrix {
        let m = HalfGcdMatrix::init(n, &mut p);
        OwnedHalfGcdMatrix {
            s: m.s,
            two_s: m.two_s,
            three_s: m.three_s,
            n: m.n,
            data: p,
        }
    }

    pub const fn update_q_scratch_len(&self, qs_len: usize) -> usize {
        self.n + qs_len
    }

    pub fn mul_matrix_1(&mut self, m_1: &HalfGcdMatrix1, scratch: &mut [Limb]) {
        let mut om = from_owned(self);
        limbs_half_gcd_matrix_mul_matrix_1(&mut om, m_1, scratch);
        self.n = om.n;
    }

    pub fn update_q(&mut self, qs: &[Limb], column: u8, scratch: &mut [Limb]) {
        let mut om = from_owned(self);
        limbs_half_gcd_matrix_update_q(&mut om, qs, column, scratch);
        self.n = om.n;
    }

    pub fn get(&self, row: u8, column: u8) -> &[Limb] {
        match (row, column) {
            (0, 0) => &self.data[..self.s],
            (0, 1) => &self.data[self.s..self.two_s],
            (1, 0) => &self.data[self.two_s..self.three_s],
            (1, 1) => &self.data[self.three_s..],
            _ => panic!(),
        }
    }
}

pub fn half_gcd_matrix_create(s: usize, n: usize, data: Vec<Limb>) -> OwnedHalfGcdMatrix {
    assert!(n <= s);
    assert_eq!(data.len(), s << 2);
    OwnedHalfGcdMatrix {
        data,
        s,
        two_s: s << 1,
        three_s: s * 3,
        n,
    }
}

pub fn half_gcd_matrix_to_naturals(m: &OwnedHalfGcdMatrix) -> (Natural, Natural, Natural, Natural) {
    let n = m.n;
    (
        Natural::from_limbs_asc(&m.get(0, 0)[..n]),
        Natural::from_limbs_asc(&m.get(0, 1)[..n]),
        Natural::from_limbs_asc(&m.get(1, 0)[..n]),
        Natural::from_limbs_asc(&m.get(1, 1)[..n]),
    )
}

pub fn half_gcd_matrix_1_to_naturals(m_1: &HalfGcdMatrix1) -> (Natural, Natural, Natural, Natural) {
    (
        Natural::from(m_1.data[0][0]),
        Natural::from(m_1.data[0][1]),
        Natural::from(m_1.data[1][0]),
        Natural::from(m_1.data[1][1]),
    )
}

pub fn half_gcd_matrix_create_string(m: &HalfGcdMatrix) -> String {
    format!("half_gcd_matrix_create({}, {}, vec!{:?})", m.s, m.n, m.data)
}

pub fn half_gcd_matrix_all_elements_nonzero(m: &HalfGcdMatrix) -> bool {
    for i in 0..2 {
        for j in 0..2 {
            if slice_test_zero(m.get(i, j)) {
                return false;
            }
        }
    }
    true
}

/// This is equivalent to `div2` from `mpn/generic/hgcd2.c`, GMP 6.2.1, where `HGCD2_DIV2_METHOD ==
/// 2`.
pub fn limbs_gcd_div_alt(
    mut n1: Limb,
    mut n0: Limb,
    mut d1: Limb,
    mut d0: Limb,
) -> (Limb, Limb, Limb) {
    let mut q = 0;
    let n_zeros = LeadingZeros::leading_zeros(n1);
    let mut d_zeros = LeadingZeros::leading_zeros(d1);
    assert!(d_zeros >= n_zeros);
    d_zeros -= n_zeros;
    d1 = (d1 << d_zeros) + (d0 >> 1 >> (Limb::WIDTH - 1 - d_zeros));
    d0 <<= d_zeros;
    for _ in 0..=d_zeros {
        q <<= 1;
        if n1 == d1 && n0 >= d0 || n1 != d1 && n1 > d1 {
            q |= 1;
            (n1, n0) = Limb::xx_sub_yy_to_zz(n1, n0, d1, d0);
        }
        d0 = (d1 << (Limb::WIDTH - 1)) | (d0 >> 1);
        d1 >>= 1;
    }
    (q, n1, n0)
}

pub fn limbs_gcd_div_naive(n1: Limb, n0: Limb, d1: Limb, d0: Limb) -> (Limb, Limb, Limb) {
    let (q, r) = DoubleLimb::join_halves(n1, n0).div_mod(DoubleLimb::join_halves(d1, d0));
    let (q1, q0) = q.split_in_half();
    assert_eq!(q1, 0);
    let (r1, r0) = r.split_in_half();
    (q0, r1, r0)
}
