use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::slices::slice_test_zero;
use natural::arithmetic::add::limbs_add_to_out_aliased;
use natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use natural::arithmetic::gcd::matrix_2_2::limbs_matrix_2_2_mul;
use natural::arithmetic::mul::limb::{limbs_mul_limb_to_out, limbs_slice_mul_limb_in_place};
use natural::arithmetic::mul::limbs_mul_to_out;
use platform::Limb;
use std::cmp::max;

//TODO remove
pub fn half_gcd_matrix_create_string(m: &HalfGcdMatrix) -> String {
    format!("half_gcd_matrix_create({}, {}, vec!{:?})", m.s, m.n, m.data)
}

//TODO remove
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

#[doc(hidden)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HalfGcdMatrix {
    pub data: Vec<Limb>,
    pub s: usize,
    pub two_s: usize,
    pub three_s: usize,
    pub n: usize,
}

impl HalfGcdMatrix {
    #[inline]
    #[doc(hidden)]
    pub fn get(&self, row: u8, column: u8) -> &[Limb] {
        match (row, column) {
            (0, 0) => &self.data[..self.s],
            (0, 1) => &self.data[self.s..self.two_s],
            (1, 0) => &self.data[self.two_s..self.three_s],
            (1, 1) => &self.data[self.three_s..],
            _ => panic!(),
        }
    }

    #[inline]
    fn get_mut(&mut self, row: u8, column: u8) -> &mut [Limb] {
        match (row, column) {
            (0, 0) => &mut self.data[..self.s],
            (0, 1) => &mut self.data[self.s..self.two_s],
            (1, 0) => &mut self.data[self.two_s..self.three_s],
            (1, 1) => &mut self.data[self.three_s..],
            _ => panic!(),
        }
    }

    #[inline]
    fn get_two_mut(
        &mut self,
        row_1: u8,
        column_1: u8,
        row_2: u8,
        column_2: u8,
    ) -> (&mut [Limb], &mut [Limb]) {
        match (row_1, column_1, row_2, column_2) {
            (0, 0, 0, 1) => self.data[..self.two_s].split_at_mut(self.s),
            (0, 1, 0, 0) => {
                let (xs, ys) = self.data[..self.two_s].split_at_mut(self.s);
                (ys, xs)
            }
            (1, 0, 1, 1) => self.data[self.two_s..].split_at_mut(self.s),
            (1, 1, 1, 0) => {
                let (xs, ys) = self.data[self.two_s..].split_at_mut(self.s);
                (ys, xs)
            }
            _ => panic!(),
        }
    }

    #[inline]
    fn get_four_mut(&mut self) -> (&mut [Limb], &mut [Limb], &mut [Limb], &mut [Limb]) {
        split_into_chunks_mut!(self.data, self.s, [x00, x01, x10], x11);
        (x00, x01, x10, x11)
    }

    #[doc(hidden)]
    pub const fn min_init_scratch(n: usize) -> usize {
        (((n + 1) >> 1) + 1) << 2
    }

    /// For input of size n, matrix elements are of size at most ceil(n / 2) - 1, but we need two
    /// limbs extra.
    ///
    /// This is mpn_hgcd_matrix_init from mpn/generic/hgcd_matrix.c, GMP 6.2.1, where the matrix is
    /// returned.
    #[doc(hidden)]
    pub fn init(n: usize, p: Vec<Limb>) -> HalfGcdMatrix {
        let s = (n + 1) / 2 + 1;
        let two_s = s << 1;
        let three_s = two_s + s;
        let mut m = HalfGcdMatrix {
            data: p,
            s,
            two_s,
            three_s,
            n: 1,
        };
        m.get_mut(0, 0)[0] = 1;
        m.get_mut(1, 1)[0] = 1;
        m
    }

    #[doc(hidden)]
    pub const fn update_q_scratch_len(m: &HalfGcdMatrix, qs_len: usize) -> usize {
        qs_len + m.n
    }

    /// Update column `column`, adding in Q * column (1-`col`). Temporary storage:
    /// qn + n <= `self.s`, where n is the size of the largest element in column 1 - `column`.
    ///
    /// This is mpn_hgcd_matrix_update_q from mpn/generic/hgcd_matrix.c, GMP 6.2.1.
    #[doc(hidden)]
    pub fn update_q(&mut self, qs: &[Limb], column: u8, scratch: &mut [Limb]) {
        let qs_len = qs.len();
        assert!(qs_len + self.n <= self.s);
        assert!(column < 2);
        if qs_len == 1 {
            let q = qs[0];
            let n = self.n;
            let (m_0_a, m_0_b) = self.get_two_mut(0, column, 0, 1 - column);
            let carry_0 =
                limbs_slice_add_mul_limb_same_length_in_place_left(&mut m_0_a[..n], &m_0_b[..n], q);
            let (m_1_a, m_1_b) = self.get_two_mut(1, column, 1, 1 - column);
            let carry_1 =
                limbs_slice_add_mul_limb_same_length_in_place_left(&mut m_1_a[..n], &m_1_b[..n], q);
            self.get_mut(0, column)[n] = carry_0;
            self.get_mut(1, column)[n] = carry_1;
            if carry_0 != 0 || carry_1 != 0 {
                self.n += 1;
            }
        } else {
            // Carries for the unlikely case that we get both high words from the multiplication
            // and carries from the addition.
            let mut carries = [0; 2];
            // The matrix will not necessarily grow in size by qn, so we need normalization in
            // order not to overflow self.
            let mut n = self.n;
            while n + qs_len > self.n {
                assert_ne!(n, 0);
                if self.get(0, 1 - column)[n - 1] > 0 || self.get(1, 1 - column)[n - 1] > 0 {
                    break;
                }
                n -= 1;
            }
            assert!(qs_len + n <= self.s);
            if n != 0 {
                for row in 0..2 {
                    limbs_mul_to_out(scratch, &self.get(row, 1 - column)[..n], &qs[..qs_len]);
                    assert!(n + qs_len >= self.n);
                    let self_n = self.n;
                    if limbs_add_to_out_aliased(
                        self.get_mut(row, column),
                        self_n,
                        &scratch[..n + qs_len],
                    ) {
                        carries[usize::wrapping_from(row)] = 1;
                    }
                }
            }
            n += qs_len;
            if carries[0] != 0 || carries[1] != 0 {
                self.get_mut(0, column)[n] = carries[0];
                self.get_mut(1, column)[n] = carries[1];
                n += 1;
            } else if self.get(0, column)[n - 1] == 0 && self.get(1, column)[n - 1] == 0 {
                n -= 1;
            }
            self.n = n;
        }
        assert!(self.n <= self.s);
    }

    /// Multiply M by M1 from the right. Since the M1 elements fit in
    /// GMP_NUMB_BITS - 1 bits, M grows by at most one limb. Needs
    /// temporary space M->n
    ///
    /// This is mpn_hgcd_matrix_mul_1 from mpn/generic/hgcd_matrix.c, GMP 6.2.1.
    #[doc(hidden)]
    pub fn mul_matrix_1(&mut self, m_1: &HalfGcdMatrix1, scratch: &mut [Limb]) {
        let n = self.n;
        let scratch = &mut scratch[..n];
        scratch.copy_from_slice(&self.get(0, 0)[..n]);
        let (m_0_0, m_0_1) = self.get_two_mut(0, 0, 0, 1);
        let n0 = m_1.mul_vector(m_0_0, scratch, m_0_1);
        scratch.copy_from_slice(&self.get(1, 0)[..n]);
        let (m_1_0, m_1_1) = self.get_two_mut(1, 0, 1, 1);
        let n1 = m_1.mul_vector(m_1_0, scratch, m_1_1);
        self.n = max(n0, n1);
        assert!(self.n <= self.s);
    }

    //TODO clean
    /// Multiply M by M1 from the right. Needs 3*(M->n + M1->n) + 5 limbs
    /// of temporary storage (see mpn_matrix22_mul_itch).
    ///
    /// This is mpn_hgcd_matrix_mul from mpn/generic/hgcd_matrix.c, GMP 6.2.1.
    #[doc(hidden)]
    pub fn mul_matrix(&mut self, m1: &HalfGcdMatrix, tp: &mut [Limb]) {
        // About the new size of M:s elements. Since M1's diagonal elements
        // are > 0, no element can decrease. The new elements are of size
        // M->n + M1->n, one limb more or less. The computation of the
        // matrix product produces elements of size M->n + M1->n + 1. But
        // the true size, after normalization, may be three limbs smaller.
        //
        // The reason that the product has normalized size >= M->n + M1->n -
        // 2 is subtle. It depends on the fact that M and M1 can be factored
        // as products of (1,1; 0,1) and (1,0; 1,1), and that we can't have
        // M ending with a large power and M1 starting with a large power of
        // the same matrix.
        assert!(self.n + m1.n < self.s);
        let last_index = self.n - 1;
        assert!(
            self.get(0, 0)[last_index] != 0
                || self.get(0, 1)[last_index] != 0
                || self.get(1, 0)[last_index] != 0
                || self.get(1, 1)[last_index] != 0
        );
        let m1_n = m1.n;
        let last_index = m1_n - 1;
        assert!(
            m1.get(0, 0)[last_index] != 0
                || m1.get(0, 1)[last_index] != 0
                || m1.get(1, 0)[last_index] != 0
                || m1.get(1, 1)[last_index] != 0
        );
        let n = self.n;
        let (x00, x01, x10, x11) = self.get_four_mut();
        limbs_matrix_2_2_mul(
            x00,
            x01,
            x10,
            x11,
            n,
            &m1.get(0, 0)[..m1_n],
            &m1.get(0, 1)[..m1_n],
            &m1.get(1, 0)[..m1_n],
            &m1.get(1, 1)[..m1_n],
            tp,
        );
        // Index of last potentially non-zero limb, size is one greater.
        let mut n = self.n + m1_n;
        if self.get(0, 0)[n] == 0
            && self.get(0, 1)[n] == 0
            && self.get(1, 0)[n] == 0
            && self.get(1, 1)[n] == 0
        {
            n -= 1;
        }
        if self.get(0, 0)[n] == 0
            && self.get(0, 1)[n] == 0
            && self.get(1, 0)[n] == 0
            && self.get(1, 1)[n] == 0
        {
            n -= 1;
        }
        if self.get(0, 0)[n] == 0
            && self.get(0, 1)[n] == 0
            && self.get(1, 0)[n] == 0
            && self.get(1, 1)[n] == 0
        {
            n -= 1;
        }
        assert!(
            self.get(0, 0)[n] != 0
                || self.get(0, 1)[n] != 0
                || self.get(1, 0)[n] != 0
                || self.get(1, 1)[n] != 0
        );
        self.n = n + 1;
    }
}

#[doc(hidden)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HalfGcdMatrix1 {
    pub data: [[Limb; 2]; 2],
}

impl HalfGcdMatrix1 {
    /// Sets (r;b) = (a;b) M, with M = (u00, u01; u10, u11). Vector must
    /// have space for n + 1 limbs. Uses three buffers to avoid a copy
    ///
    /// This is mpn_hgcd_mul_matrix1_vector from mpn/generic/hgcd2.c, GMP 6.2.1.
    #[doc(hidden)]
    pub fn mul_vector(&self, out: &mut [Limb], xs: &[Limb], ys: &mut [Limb]) -> usize {
        let n = xs.len();
        assert!(ys.len() > n);
        assert!(out.len() > n);
        let (out_lo, out_hi) = out.split_at_mut(n);
        let (ys_lo, ys_hi) = ys.split_at_mut(n);
        let mut a_high = limbs_mul_limb_to_out(out_lo, xs, self.data[0][0]);
        a_high +=
            limbs_slice_add_mul_limb_same_length_in_place_left(out_lo, ys_lo, self.data[1][0]);
        let mut b_high = limbs_slice_mul_limb_in_place(ys_lo, self.data[1][1]);
        b_high += limbs_slice_add_mul_limb_same_length_in_place_left(ys_lo, xs, self.data[0][1]);
        out_hi[0] = a_high;
        ys_hi[0] = b_high;
        if a_high == 0 && b_high == 0 {
            n
        } else {
            n + 1
        }
    }
}
