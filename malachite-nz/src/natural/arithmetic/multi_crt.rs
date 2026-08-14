// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010 Fredrik Johansson
//
//      Copyright © 2021 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{
    AddMul, BalancedMod, DivMod, Mod, ModInverse, SubMul,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;

// One step of a compiled Chinese-remainder program: combine operands `b` and `c` into slot `a_idx`
// as `b * c_modulus + c * b_modulus`. A nonnegative operand index names a working slot holding an
// earlier step's result; a negative index `-1 - i` names the input residue `i`, which is multiplied
// by its fraction modulus and reduced before use.
//
// This is _fmpz_multi_CRT_instr from fmpz_types.h, FLINT 3.6.0.
#[derive(Clone, Debug, Eq, PartialEq)]
struct MultiCrtInstr {
    a_idx: usize,
    b_idx: isize,
    c_idx: isize,
    b_modulus: Natural,
    c_modulus: Natural,
}

/// A precomputed context for combining many congruences by the Chinese remainder theorem.
///
/// Building the context from a list of moduli compiles a balanced subproduct tree and a
/// partial-fraction decomposition of its root into a linear program; each subsequent
/// [`apply`](MultiCrt::apply) or [`apply_balanced`](MultiCrt::apply_balanced) runs the program on a
/// list of residues, which costs less than combining the congruences one at a time when the moduli
/// are many or large. The moduli must be nonzero and pairwise coprime, and, when there are at least
/// two of them, none may be 1.
///
/// This is fmpz_multi_CRT_t from fmpz_types.h, FLINT 3.6.0, with `fmpz_multi_CRT_init`,
/// `fmpz_multi_CRT_precompute`, and `fmpz_multi_CRT_clear` folded into construction and drop.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultiCrt {
    prog: Vec<MultiCrtInstr>,
    moduli: Vec<Natural>,
    fracmoduli: Vec<Natural>,
    final_modulus: Natural,
    localsize: usize,
}

// Fills `w` with the partial-fraction decomposition of `a / (v[j] * v[j + 1])` down the tree: at
// each node, `a / (v[j] * v[j + 1]) = w[j] / v[j] + w[j + 1] / v[j + 1] (mod 1)`. Returns `false`
// if a modulus is 0 or 1, or two moduli are not coprime.
//
// This is _fill_pfrac from fmpz/multi_CRT.c, FLINT 3.6.0, with the gcd-and-inverse call replaced by
// `mod_inverse`: the cofactor is only used when the GCD is 1, and then it is the unique inverse, so
// the two agree.
fn fill_pfrac(
    link: &mut [isize],
    v: &mut [Natural],
    w: &mut [Natural],
    mut j: isize,
    mut a: Natural,
) -> bool {
    while j >= 0 {
        let ju = usize::exact_from(j);
        let cmp = v[ju].cmp(&v[ju + 1]);
        if v[ju] == 0u32
            || v[ju + 1] == 0u32
            || v[ju] == 1u32
            || v[ju + 1] == 1u32
            || cmp == Ordering::Equal
        {
            return false;
        }
        // mod_inverse requires its first argument reduced, and the smaller node must be visited
        // first below, so order the pair.
        if cmp == Ordering::Greater {
            v.swap(ju, ju + 1);
            link.swap(ju, ju + 1);
        }
        let Some(s) = (&v[ju]).mod_inverse(&v[ju + 1]) else {
            return false;
        };
        w[ju + 1] = &a * s % &v[ju + 1];
        // w[j] = (a - v[j] * w[j + 1]) / v[j + 1] mod v[j]; the division is exact, but the
        // numerator may be negative, so it runs through Integer.
        let t = Integer::from(&a).sub_mul(Integer::from(&v[ju]), Integer::from(&w[ju + 1]));
        let (q, rem) = t.div_mod(Integer::from(&v[ju + 1]));
        assert_eq!(rem, 0u32, "division should be exact");
        w[ju] = Natural::exact_from(q.mod_op(Integer::from(&v[ju])));
        if !fill_pfrac(link, v, w, link[ju], w[ju].clone()) {
            return false;
        }
        a = w[ju + 1].clone();
        j = link[ju + 1];
    }
    true
}

// Linearizes the tree into the instruction program, working slots numbered so that each
// instruction's operands are already computed when it runs and slots are reused once consumed.
//
// This is _fill_prog from fmpz/multi_CRT.c, FLINT 3.6.0.
struct ProgBuilder<'a> {
    link: &'a [isize],
    v: &'a [Natural],
    w: &'a [Natural],
    prog: Vec<MultiCrtInstr>,
    moduli: Vec<Natural>,
    fracmoduli: Vec<Natural>,
    localsize: usize,
}

impl ProgBuilder<'_> {
    fn fill(&mut self, j: isize, ret_idx: usize) {
        assert!(j >= 0);
        let ju = usize::exact_from(j);
        let mut next_ret_idx = ret_idx;
        let b_idx = if self.link[ju] >= 0 {
            next_ret_idx += 1;
            let b_idx = isize::exact_from(next_ret_idx);
            self.fill(self.link[ju], next_ret_idx);
            b_idx
        } else {
            let leaf = usize::exact_from(-self.link[ju] - 1);
            self.moduli[leaf] = self.v[ju].clone();
            self.fracmoduli[leaf] = self.w[ju].clone();
            -1 - isize::exact_from(leaf)
        };
        let c_idx = if self.link[ju + 1] >= 0 {
            next_ret_idx += 1;
            let c_idx = isize::exact_from(next_ret_idx);
            self.fill(self.link[ju + 1], next_ret_idx);
            c_idx
        } else {
            let leaf = usize::exact_from(-self.link[ju + 1] - 1);
            self.moduli[leaf] = self.v[ju + 1].clone();
            self.fracmoduli[leaf] = self.w[ju + 1].clone();
            -1 - isize::exact_from(leaf)
        };
        self.prog.push(MultiCrtInstr {
            a_idx: ret_idx,
            b_idx,
            c_idx,
            b_modulus: self.v[ju].clone(),
            c_modulus: self.v[ju + 1].clone(),
        });
        self.localsize = self.localsize.max(next_ret_idx + 1);
    }
}

impl MultiCrt {
    /// Compiles a Chinese-remainder context from a list of moduli, returning `None` if the list is
    /// unusable.
    ///
    /// A single modulus is usable as long as it is nonzero. Two or more moduli are usable if and
    /// only if none is 0 or 1 and they are pairwise coprime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of significant bits of
    /// the product of the moduli.
    ///
    /// # Panics
    /// Panics if `moduli` is empty.
    ///
    /// # Examples
/// (The examples are not compiled: this module is public only under the `test_build` feature, so
/// its paths do not resolve in an ordinary build.)
///
    /// ```rust,ignore
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural::arithmetic::multi_crt::MultiCrt;
    ///
    /// let moduli = [
    ///     Natural::from(3u32),
    ///     Natural::from(5u32),
    ///     Natural::from(7u32),
    /// ];
    /// let crt = MultiCrt::new(&moduli).unwrap();
    /// assert_eq!(crt.modulus(), &Natural::from(105u32));
    ///
    /// // The moduli 4 and 6 are not coprime.
    /// assert!(MultiCrt::new(&[Natural::from(4u32), Natural::from(6u32)]).is_none());
    /// ```
    pub fn new(moduli: &[Natural]) -> Option<Self> {
        let r = moduli.len();
        assert_ne!(r, 0, "moduli must be nonempty");
        if r < 2 {
            return if moduli[0] == 0u32 {
                None
            } else {
                Some(Self {
                    prog: Vec::new(),
                    moduli: vec![moduli[0].clone()],
                    fracmoduli: vec![Natural::ONE],
                    final_modulus: moduli[0].clone(),
                    localsize: 1,
                })
            };
        }
        let n = (r << 1) - 2;
        let mut link = vec![0; n];
        // One buffer split in half, as FLINT lays it out: the tree nodes, then the fractions.
        let mut vw = vec![Natural::ZERO; n << 1];
        let (v, w) = vw.split_at_mut(n);
        for (i, m) in moduli.iter().enumerate() {
            v[i] = m.clone();
            link[i] = -1 - isize::exact_from(i);
        }
        // Build the tree by repeatedly multiplying the two smallest remaining nodes, which keeps it
        // balanced by size.
        let mut i = r;
        let mut j = 0;
        while j < n - 2 {
            for target in [j, j + 1] {
                let mut minp = target;
                for s in target + 1..i {
                    if v[s] < v[minp] {
                        minp = s;
                    }
                }
                v.swap(target, minp);
                link.swap(target, minp);
            }
            v[i] = &v[j] * &v[j + 1];
            link[i] = isize::exact_from(j);
            i += 1;
            j += 2;
        }
        let final_modulus = &v[n - 2] * &v[n - 1];
        let root = isize::exact_from(n - 2);
        if !fill_pfrac(&mut link, v, w, root, Natural::ONE) {
            return None;
        }
        let mut builder = ProgBuilder {
            link: &link,
            v,
            w,
            prog: Vec::new(),
            moduli: vec![Natural::ZERO; r],
            fracmoduli: vec![Natural::ZERO; r],
            localsize: 1,
        };
        builder.fill(root, 0);
        Some(Self {
            prog: builder.prog,
            moduli: builder.moduli,
            fracmoduli: builder.fracmoduli,
            final_modulus,
            localsize: builder.localsize,
        })
    }

    /// Returns the product of the moduli: the modulus the combined residue is reduced by.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```rust,ignore
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural::arithmetic::multi_crt::MultiCrt;
    ///
    /// let crt = MultiCrt::new(&[Natural::from(3u32), Natural::from(5u32)]).unwrap();
    /// assert_eq!(crt.modulus(), &Natural::from(15u32));
    /// ```
    #[inline]
    pub const fn modulus(&self) -> &Natural {
        &self.final_modulus
    }

    /// Returns the number of moduli, which is the number of residues [`apply`](MultiCrt::apply)
    /// expects.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```rust,ignore
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural::arithmetic::multi_crt::MultiCrt;
    ///
    /// let crt = MultiCrt::new(&[Natural::from(3u32), Natural::from(5u32)]).unwrap();
    /// assert_eq!(crt.moduli_count(), 2);
    /// ```
    #[inline]
    pub const fn moduli_count(&self) -> usize {
        self.moduli.len()
    }

    /// Combines residues into the unique number below the moduli product that is congruent to each
    /// residue modulo the corresponding modulus. The residues must be already reduced.
    ///
    /// $f(\mathrm{self}, (r_1, \ldots, r_k)) = x$, where $x < \prod_i m_i$ and $x \equiv r_i \mod
    /// m_i$ for all $i$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of significant bits of
    /// the product of the moduli.
    ///
    /// # Panics
    /// Panics if the number of values differs from the number of moduli, or if any value is greater
    /// than or equal to its modulus.
    ///
    /// # Examples
    /// ```rust,ignore
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural::arithmetic::multi_crt::MultiCrt;
    ///
    /// let moduli = [
    ///     Natural::from(3u32),
    ///     Natural::from(5u32),
    ///     Natural::from(7u32),
    /// ];
    /// let values = [
    ///     Natural::from(2u32),
    ///     Natural::from(3u32),
    ///     Natural::from(2u32),
    /// ];
    /// let crt = MultiCrt::new(&moduli).unwrap();
    /// // 23 is 2 mod 3, 3 mod 5, and 2 mod 7.
    /// assert_eq!(crt.apply(&values), Natural::from(23u32));
    /// ```
    // The flagged divisors are indexed by the instruction's leaf, so they vary across the loop.
    #[cfg_attr(dylint_lib = "malachite_lints", allow(use_div_mod_precomputed))]
    pub fn apply(&self, values: &[Natural]) -> Natural {
        self.check_values(values);
        // A single modulus compiles to no instructions, and equal residues modulo coprime moduli
        // are their own combination.
        if self.prog.is_empty() || values.iter().all(|v| *v == values[0]) {
            return &values[0] % &self.final_modulus;
        }
        let mut outs = vec![Natural::ZERO; self.localsize];
        for instr in &self.prog {
            // Each working slot is written by one instruction and consumed by exactly one later
            // instruction, so taking it out is safe.
            let b_val = if instr.b_idx < 0 {
                let leaf = usize::exact_from(-instr.b_idx - 1);
                &values[leaf] * &self.fracmoduli[leaf] % &self.moduli[leaf]
            } else {
                take(&mut outs[usize::exact_from(instr.b_idx)])
            };
            let c_val = if instr.c_idx < 0 {
                let leaf = usize::exact_from(-instr.c_idx - 1);
                &values[leaf] * &self.fracmoduli[leaf] % &self.moduli[leaf]
            } else {
                take(&mut outs[usize::exact_from(instr.c_idx)])
            };
            outs[instr.a_idx] = (b_val * &instr.c_modulus).add_mul(c_val, &instr.b_modulus);
        }
        take(&mut outs[0]) % &self.final_modulus
    }

    /// Combines residues into the balanced representative: the unique [`Integer`] $x$ with $-P/2 <
    /// x \leq P/2$, where $P$ is the moduli product, that is congruent to each residue modulo the
    /// corresponding modulus. The residues must be already reduced.
    ///
    /// $f(\mathrm{self}, (r_1, \ldots, r_k)) = x$, where $-P/2 < x \leq P/2$, $P = \prod_i m_i$,
    /// and $x \equiv r_i \mod m_i$ for all $i$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of significant bits of
    /// the product of the moduli.
    ///
    /// # Panics
    /// Panics if the number of values differs from the number of moduli, or if any value is greater
    /// than or equal to its modulus.
    ///
    /// # Examples
    /// ```rust,ignore
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural::arithmetic::multi_crt::MultiCrt;
    ///
    /// let crt = MultiCrt::new(&[Natural::from(3u32), Natural::from(5u32)]).unwrap();
    /// // 8 is 2 mod 3 and 3 mod 5, and its balanced representative mod 15 is -7.
    /// assert_eq!(
    ///     crt.apply_balanced(&[Natural::from(2u32), Natural::from(3u32)]),
    ///     Integer::from(-7)
    /// );
    /// ```
    pub fn apply_balanced(&self, values: &[Natural]) -> Integer {
        self.check_values(values);
        if self.prog.is_empty() || values.iter().all(|v| *v == values[0]) {
            return Integer::from(&values[0]).balanced_mod(Integer::from(&self.final_modulus));
        }
        let mut outs = vec![Integer::ZERO; self.localsize];
        for instr in &self.prog {
            let b_val = if instr.b_idx < 0 {
                let leaf = usize::exact_from(-instr.b_idx - 1);
                Integer::from(&values[leaf] * &self.fracmoduli[leaf])
                    .balanced_mod(Integer::from(&self.moduli[leaf]))
            } else {
                take(&mut outs[usize::exact_from(instr.b_idx)])
            };
            let c_val = if instr.c_idx < 0 {
                let leaf = usize::exact_from(-instr.c_idx - 1);
                Integer::from(&values[leaf] * &self.fracmoduli[leaf])
                    .balanced_mod(Integer::from(&self.moduli[leaf]))
            } else {
                take(&mut outs[usize::exact_from(instr.c_idx)])
            };
            outs[instr.a_idx] = (b_val * Integer::from(&instr.c_modulus))
                .add_mul(c_val, Integer::from(&instr.b_modulus));
        }
        take(&mut outs[0]).balanced_mod(Integer::from(&self.final_modulus))
    }

    fn check_values(&self, values: &[Natural]) {
        assert_eq!(
            values.len(),
            self.moduli.len(),
            "one value per modulus is required"
        );
        for (v, m) in values.iter().zip(self.moduli.iter()) {
            assert!(
                v < m,
                "values must be reduced modulo the moduli, but {v} >= {m}"
            );
        }
    }
}

impl Natural {
    /// Combines residues modulo pairwise-coprime moduli into the unique number below the moduli
    /// product that is congruent to each residue, returning `None` if the moduli are unusable. The
    /// residues must be already reduced.
    ///
    /// For the representative of smallest absolute value instead, use
    /// [`Integer::multi_balanced_crt`](crate::integer::Integer::multi_balanced_crt).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of significant bits of
    /// the product of the moduli.
    ///
    /// # Panics
    /// Panics if `moduli` is empty, if the number of values differs from the number of moduli, or
    /// if any value is greater than or equal to its modulus.
    ///
    /// # Examples
    /// ```rust,ignore
    /// use malachite_nz::natural::Natural;
    ///
    /// let moduli = [
    ///     Natural::from(3u32),
    ///     Natural::from(5u32),
    ///     Natural::from(7u32),
    /// ];
    /// let values = [
    ///     Natural::from(2u32),
    ///     Natural::from(3u32),
    ///     Natural::from(2u32),
    /// ];
    /// // 23 is 2 mod 3, 3 mod 5, and 2 mod 7.
    /// assert_eq!(
    ///     Natural::multi_crt(&moduli, &values),
    ///     Some(Natural::from(23u32))
    /// );
    /// assert_eq!(
    ///     Natural::multi_crt(&[Natural::from(4u32), Natural::from(6u32)], &values[..2]),
    ///     None
    /// );
    /// ```
    ///
    /// This is fmpz_multi_CRT from fmpz/multi_CRT.c, FLINT 3.6.0, with sign = 0 and the residues
    /// required to be reduced.
    pub fn multi_crt(moduli: &[Self], values: &[Self]) -> Option<Self> {
        Some(MultiCrt::new(moduli)?.apply(values))
    }
}
