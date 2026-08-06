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
use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::multi_crt::MultiCrt;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{ModInverse, ModMul, XXDivModYToQR};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, SplitInHalf};

// The better the addmul kernel is, the larger this can be.
//
// This is FMPZ_CRT_UI_CUTOFF from fmpz/comb_init.c, FLINT 3.6.0, in limbs.
const CRT_UI_CUTOFF: usize = 50;

// The better big-by-word reduction is, the larger this can be.
//
// This is FMPZ_MOD_UI_CUTOFF from fmpz/comb_init.c, FLINT 3.6.0, in limbs.
const MOD_UI_CUTOFF: usize = 75;

// The merge thresholds below which a trailing chunk is folded into its predecessor.
const CRT_UI_MERGE_CUTOFF: usize = (CRT_UI_CUTOFF * 3) >> 2;
const MOD_UI_MERGE_CUTOFF: usize = (MOD_UI_CUTOFF * 3) >> 2;

// The minimal number of chunks in which to partition, unless everything fits in one.
//
// This is FMPZ_CRT_UI_MULTIPLE_CUTOFF and FMPZ_MOD_UI_MULTIPLE_CUTOFF from fmpz/comb_init.c, FLINT
// 3.6.0.
const MULTIPLE_CUTOFF: usize = 4;

// One step of a compiled multi-reduction program: reduce slot `in_idx` (0 is the original input)
// modulo `modulus`, into working slot `out_idx` if nonnegative and into final output `-1 - i`
// otherwise.
//
// This is _fmpz_multi_mod_instr from fmpz.h, FLINT 3.6.0.
#[derive(Clone, Debug, Eq, PartialEq)]
struct MultiModInstr {
    in_idx: usize,
    out_idx: isize,
    modulus: Natural,
}

// A compiled program reducing one number modulo many moduli through a subproduct tree. The moduli
// need not be coprime; only zero moduli are unusable.
//
// This is fmpz_multi_mod_t from fmpz.h, FLINT 3.6.0, with `fmpz_multi_mod_init`,
// `fmpz_multi_mod_precompute`, and `fmpz_multi_mod_clear` folded into construction and drop.
#[derive(Clone, Debug, Eq, PartialEq)]
struct MultiMod {
    prog: Vec<MultiModInstr>,
    localsize: usize,
    moduli_count: usize,
}

// Orders each tree pair so that the smaller node is visited first, and rejects zero moduli. The
// moduli are not required to be coprime, unlike the combination direction.
//
// This is _fill_sort from fmpz/multi_mod.c, FLINT 3.6.0. Through `CrtComb`, currently the only
// caller, the swap and the zero rejection are unreachable: the chunk products are nonzero by
// construction, and the chunk merging leaves either one chunk or at least four, in which case the
// pairing-by-smallest tree construction has already ordered every pair. Both branches are kept for
// FLINT fidelity and for future direct users of the reduction program.
fn mod_fill_sort(link: &mut [isize], v: &mut [Natural], mut j: isize) -> bool {
    while j >= 0 {
        let ju = usize::exact_from(j);
        if v[ju] == 0u32 || v[ju + 1] == 0u32 {
            return false;
        }
        if v[ju] > v[ju + 1] {
            v.swap(ju, ju + 1);
            link.swap(ju, ju + 1);
        }
        if !mod_fill_sort(link, v, link[ju]) {
            return false;
        }
        j = link[ju + 1];
    }
    true
}

// Linearizes the tree into the reduction program. Sibling reductions share the working slot one
// past their parent's, since the left subtree is completely evaluated before the right reduction
// runs.
//
// This is _fill_prog from fmpz/multi_mod.c, FLINT 3.6.0.
struct ModProgBuilder<'a> {
    link: &'a [isize],
    v: &'a [Natural],
    prog: Vec<MultiModInstr>,
    localsize: usize,
}

impl ModProgBuilder<'_> {
    fn fill(&mut self, j: isize, a_idx: usize) {
        assert!(j >= 0);
        let ju = usize::exact_from(j);
        let b_idx = if self.link[ju] >= 0 {
            isize::exact_from(a_idx + 1)
        } else {
            self.link[ju]
        };
        self.prog.push(MultiModInstr {
            in_idx: a_idx,
            out_idx: b_idx,
            modulus: self.v[ju].clone(),
        });
        if self.link[ju] >= 0 {
            self.fill(self.link[ju], a_idx + 1);
        }
        let c_idx = if self.link[ju + 1] >= 0 {
            isize::exact_from(a_idx + 1)
        } else {
            self.link[ju + 1]
        };
        self.prog.push(MultiModInstr {
            in_idx: a_idx,
            out_idx: c_idx,
            modulus: self.v[ju + 1].clone(),
        });
        if self.link[ju + 1] >= 0 {
            self.fill(self.link[ju + 1], a_idx + 1);
        }
        self.localsize = max(self.localsize, a_idx + 1);
    }
}

impl MultiMod {
    // This is fmpz_multi_mod_precompute from fmpz/multi_mod.c, FLINT 3.6.0, where the moduli are
    // nonnegative.
    fn new(moduli: &[Natural]) -> Option<Self> {
        let r = moduli.len();
        assert_ne!(r, 0, "moduli must be nonempty");
        if r < 2 {
            return if moduli[0] == 0u32 {
                None
            } else {
                Some(Self {
                    prog: vec![MultiModInstr {
                        in_idx: 0,
                        out_idx: -1,
                        modulus: moduli[0].clone(),
                    }],
                    localsize: 1,
                    moduli_count: 1,
                })
            };
        }
        let n = (r << 1) - 2;
        let mut link = vec![0; n];
        let mut v = vec![Natural::ZERO; n];
        for (i, m) in moduli.iter().enumerate() {
            v[i] = m.clone();
            link[i] = -1 - isize::exact_from(i);
        }
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
        let root = isize::exact_from(n - 2);
        if !mod_fill_sort(&mut link, &mut v, root) {
            return None;
        }
        let mut builder = ModProgBuilder {
            link: &link,
            v: &v,
            prog: Vec::new(),
            localsize: 1,
        };
        builder.fill(root, 0);
        Some(Self {
            prog: builder.prog,
            localsize: builder.localsize,
            moduli_count: r,
        })
    }

    // Runs the program, filling `outputs[i]` with `input mod moduli[i]`.
    //
    // This is _fmpz_multi_mod_precomp from fmpz/multi_mod.c, FLINT 3.6.0, where the input is
    // nonnegative, so the truncating and canonical reductions coincide. A `None` working slot
    // records that its value equals the input, FLINT's copy-avoidance for small inputs.
    fn apply_into(&self, outputs: &mut [Natural], input: &Natural) {
        let mut t: Vec<Option<Natural>> = vec![None; self.localsize + 1];
        for instr in &self.prog {
            let a = instr.in_idx;
            let from_input = a == 0 || t[a].is_none();
            if instr.out_idx < 0 {
                let o = usize::exact_from(-instr.out_idx - 1);
                outputs[o] = if from_input {
                    input % &instr.modulus
                } else {
                    t[a].as_ref().unwrap() % &instr.modulus
                };
            } else {
                let b = usize::exact_from(instr.out_idx);
                t[b] = if from_input {
                    if instr.modulus > *input {
                        None
                    } else {
                        Some(input % &instr.modulus)
                    }
                } else {
                    Some(t[a].as_ref().unwrap() % &instr.modulus)
                };
            }
        }
    }
}

// A group of one, two, or three consecutive word moduli whose product fits in a word, with the
// premultiplied combination idempotents: the group's combined residue is `r0 * i0 + r1 * i1 + r2 *
// i2 mod m`, and after construction each idempotent also carries the factor lifting the group into
// its chunk.
//
// This is crt_lut_entry from fmpz.h, FLINT 3.6.0, with the nmod_t reduced to its modulus.
#[derive(Clone, Debug, Eq, PartialEq)]
struct CrtLutEntry {
    m: Limb,
    i0: Limb,
    i1: Limb,
    i2: Limb,
}

// The reduction-direction counterpart: the group product and the group's individual moduli, with
// zero marking an absent second or third member.
//
// This is mod_lut_entry from fmpz.h, FLINT 3.6.0, with the nmod_ts reduced to their moduli.
#[derive(Clone, Debug, Eq, PartialEq)]
struct ModLutEntry {
    m: Limb,
    m0: Limb,
    m1: Limb,
    m2: Limb,
}

/// A precomputed context for reducing a number modulo many word-sized moduli at once, and for
/// combining word residues back into a number, by the Chinese remainder theorem.
///
/// The moduli must be pairwise coprime and at least 2; they are usually primes. Consecutive moduli
/// are packed in groups of up to three whose product fits in a word, groups are packed into
/// multi-limb chunks with premultiplied combination multipliers, and the chunks are handled by
/// compiled subproduct-tree programs in both directions, so that [`reduce`](CrtComb::reduce) and
/// [`combine`](CrtComb::combine) cost less than handling each modulus separately.
///
/// This is fmpz_comb_t from fmpz.h, FLINT 3.6.0, with `fmpz_comb_init` and `fmpz_comb_clear` folded
/// into construction and drop, and the temporary space of `fmpz_comb_temp_t` allocated per call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrtComb {
    primes: Vec<Limb>,
    crt_p: MultiCrt,
    crt_chunks: Vec<Natural>,
    packed_multipliers: Vec<Limb>,
    step: Vec<isize>,
    crt_lu: Vec<CrtLutEntry>,
    crt_offsets: Vec<usize>,
    mod_p: MultiMod,
    mod_lu: Vec<ModLutEntry>,
    mod_offsets: Vec<usize>,
}

// The multiplicative inverse of `x mod m`, where `x` need not be reduced; `None` if they are not
// coprime. This stands in for FLINT's n_gcdinv success checks.
fn word_inverse(x: Limb, m: Limb) -> Option<Limb> {
    let x = x % m;
    if x == 0 { None } else { x.mod_inverse(m) }
}

impl CrtComb {
    /// Compiles a comb from a list of word-sized moduli, returning `None` if any modulus is less
    /// than 2 or two moduli are not coprime.
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
    /// Panics if `primes` is empty.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::arithmetic::crt_comb::CrtComb;
    /// use malachite_nz::natural::Natural;
    ///
    /// let comb = CrtComb::new(&[3, 5, 7]).unwrap();
    /// assert_eq!(comb.modulus(), &Natural::from(105u32));
    /// assert!(CrtComb::new(&[4, 6]).is_none());
    /// ```
    pub fn new(primes: &[Limb]) -> Option<Self> {
        let len = primes.len();
        assert_ne!(len, 0, "primes must be nonempty");
        if primes.iter().any(|&p| p < 2) {
            return None;
        }
        // The combination side: group, chunk, and compute the idempotents.
        let mut crt_lu: Vec<CrtLutEntry> = Vec::new();
        let mut crt_offsets: Vec<usize> = Vec::new();
        let mut crt_chunks: Vec<Natural> = Vec::new();
        // The per-group multipliers, each the chunk product divided by the group product; FLINT's
        // Mm.
        let mut mm: Vec<Natural> = Vec::new();
        let mut l = 0;
        while l < len {
            let chunk_start = crt_lu.len();
            let mut chunk = Natural::ONE;
            while l < len && usize::exact_from(chunk.limb_count()) <= CRT_UI_CUTOFF {
                let p0 = primes[l];
                let mut entry = CrtLutEntry {
                    m: p0,
                    i0: 0,
                    i1: 0,
                    i2: 0,
                };
                if let Some(p01) = (l + 1 < len)
                    .then(|| p0.checked_mul(primes[l + 1]))
                    .flatten()
                {
                    let p1 = primes[l + 1];
                    if let Some(p012) = (l + 2 < len)
                        .then(|| p01.checked_mul(primes[l + 2]))
                        .flatten()
                    {
                        let p2 = primes[l + 2];
                        entry.m = p012;
                        entry.i0 = word_inverse(p1 * p2, p0)? * (p1 * p2);
                        entry.i1 = word_inverse(p0 * p2, p1)? * (p0 * p2);
                        entry.i2 = word_inverse(p0 * p1, p2)? * (p0 * p1);
                        l += 3;
                    } else {
                        entry.m = p01;
                        entry.i0 = word_inverse(p1, p0)? * p1;
                        entry.i1 = word_inverse(p0, p1)? * p0;
                        l += 2;
                    }
                } else {
                    entry.i0 = 1;
                    l += 1;
                }
                chunk *= Natural::from(entry.m);
                crt_lu.push(entry);
            }
            crt_offsets.push(crt_lu.len());
            for lu in &mut crt_lu[chunk_start..] {
                let m = lu.m;
                let mm_i = &chunk / Natural::from(m);
                let tt = word_inverse(Limb::exact_from(&(&mm_i % Natural::from(m))), m)?;
                lu.i0 = tt.mod_mul(lu.i0 % m, m);
                lu.i1 = tt.mod_mul(lu.i1 % m, m);
                lu.i2 = tt.mod_mul(lu.i2 % m, m);
                mm.push(mm_i);
            }
            crt_chunks.push(chunk);
        }
        // Avoid a small last chunk, and have at least MULTIPLE_CUTOFF chunks or one big chunk.
        let mut k = crt_chunks.len();
        while k > 1
            && (k < MULTIPLE_CUTOFF
                || usize::exact_from(crt_chunks[k - 1].limb_count()) <= CRT_UI_MERGE_CUTOFF)
        {
            k -= 1;
            let start = if k >= 2 { crt_offsets[k - 2] } else { 0 };
            for i in start..crt_offsets[k] {
                let last = i >= crt_offsets[k - 1];
                let other = &crt_chunks[k - usize::from(last)];
                mm[i] *= other;
                let m = crt_lu[i].m;
                let tt = word_inverse(Limb::exact_from(&(other % Natural::from(m))), m)?;
                crt_lu[i].i0 = tt.mod_mul(crt_lu[i].i0, m);
                crt_lu[i].i1 = tt.mod_mul(crt_lu[i].i1, m);
                crt_lu[i].i2 = tt.mod_mul(crt_lu[i].i2, m);
            }
            crt_offsets[k - 1] = crt_offsets[k];
            crt_offsets.truncate(k);
            let last_chunk = crt_chunks.pop().unwrap();
            crt_chunks[k - 1] *= last_chunk;
        }
        let crt_p = MultiCrt::new(&crt_chunks)?;
        // Choose each chunk's stride, taking the packed path when every group in the chunk is a
        // single modulus: its idempotent premultiplies into the packed multiplier, encoded by a
        // negative step.
        let mut step: Vec<isize> = Vec::with_capacity(crt_chunks.len());
        let mut i = 0;
        for (k, offset) in crt_offsets.iter().enumerate() {
            let mut all_large = true;
            let mut s = 1;
            for j in i..*offset {
                if crt_lu[j].i1 != 0 || crt_lu[j].i2 != 0 {
                    all_large = false;
                }
                debug_assert!(mm[j] <= crt_chunks[k]);
                s = max(s, usize::exact_from(mm[j].limb_count()));
            }
            if all_large {
                s = 1;
                for (mm_j, lu) in mm[i..*offset].iter_mut().zip(crt_lu[i..*offset].iter()) {
                    *mm_j *= Natural::from(lu.i0);
                    s = max(s, usize::exact_from(mm_j.limb_count()));
                }
                step.push(-1 - isize::exact_from(s));
            } else {
                step.push(isize::exact_from(s));
            }
            i = *offset;
        }
        let mut packed_multipliers = Vec::new();
        let mut i = 0;
        for (k, offset) in crt_offsets.iter().enumerate() {
            let raw = step[k];
            let s = usize::exact_from(if raw < 0 { -raw - 1 } else { raw });
            for mm_j in &mm[i..*offset] {
                let mut limbs = mm_j.to_limbs_asc();
                limbs.resize(s, 0);
                packed_multipliers.extend_from_slice(&limbs);
            }
            i = *offset;
        }
        // The reduction side: an independent walk with its own cutoff.
        let mut mod_lu: Vec<ModLutEntry> = Vec::new();
        let mut mod_offsets: Vec<usize> = Vec::new();
        let mut mod_chunks: Vec<Natural> = Vec::new();
        let mut l = 0;
        while l < len {
            let mut chunk = Natural::ONE;
            while l < len && usize::exact_from(chunk.limb_count()) <= MOD_UI_CUTOFF {
                let p0 = primes[l];
                let mut entry = ModLutEntry {
                    m: p0,
                    m0: p0,
                    m1: 0,
                    m2: 0,
                };
                if let Some(p01) = (l + 1 < len)
                    .then(|| p0.checked_mul(primes[l + 1]))
                    .flatten()
                {
                    entry.m = p01;
                    entry.m1 = primes[l + 1];
                    if let Some(p012) = (l + 2 < len)
                        .then(|| p01.checked_mul(primes[l + 2]))
                        .flatten()
                    {
                        entry.m = p012;
                        entry.m2 = primes[l + 2];
                        l += 3;
                    } else {
                        l += 2;
                    }
                } else {
                    l += 1;
                }
                chunk *= Natural::from(entry.m);
                mod_lu.push(entry);
            }
            mod_offsets.push(mod_lu.len());
            mod_chunks.push(chunk);
        }
        let mut k = mod_chunks.len();
        while k > 1
            && (k < MULTIPLE_CUTOFF
                || usize::exact_from(mod_chunks[k - 1].limb_count()) < MOD_UI_MERGE_CUTOFF)
        {
            k -= 1;
            mod_offsets[k - 1] = mod_offsets[k];
            mod_offsets.truncate(k);
            let last_chunk = mod_chunks.pop().unwrap();
            mod_chunks[k - 1] *= last_chunk;
        }
        let mod_p = MultiMod::new(&mod_chunks)?;
        Some(Self {
            primes: primes.to_vec(),
            crt_p,
            crt_chunks,
            packed_multipliers,
            step,
            crt_lu,
            crt_offsets,
            mod_p,
            mod_lu,
            mod_offsets,
        })
    }

    /// Returns the number of moduli, which is the number of residues [`reduce`](CrtComb::reduce)
    /// produces and [`combine`](CrtComb::combine) expects.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::arithmetic::crt_comb::CrtComb;
    ///
    /// let comb = CrtComb::new(&[3, 5, 7]).unwrap();
    /// assert_eq!(comb.prime_count(), 3);
    /// ```
    #[inline]
    pub const fn prime_count(&self) -> usize {
        self.primes.len()
    }

    /// Returns the product of the moduli: the modulus [`combine`](CrtComb::combine)'s result is
    /// reduced by.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::arithmetic::crt_comb::CrtComb;
    /// use malachite_nz::natural::Natural;
    ///
    /// let comb = CrtComb::new(&[3, 5, 7]).unwrap();
    /// assert_eq!(comb.modulus(), &Natural::from(105u32));
    /// ```
    #[inline]
    pub const fn modulus(&self) -> &Natural {
        self.crt_p.modulus()
    }

    /// Reduces a number modulo each of the comb's moduli, returning the residues in order.
    ///
    /// The input may be any size; it does not need to be reduced modulo the moduli product.
    ///
    /// $f(\mathrm{self}, x) = (x \bmod m_1, \ldots, x \bmod m_k)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// self.modulus().significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural::arithmetic::crt_comb::CrtComb;
    ///
    /// let comb = CrtComb::new(&[3, 5, 7]).unwrap();
    /// assert_eq!(comb.reduce(&Natural::from(1000u32)), &[1, 0, 6]);
    /// ```
    ///
    /// This is fmpz_multi_mod_ui from fmpz/multi_mod.c, FLINT 3.6.0, where the input is
    /// nonnegative.
    pub fn reduce(&self, x: &Natural) -> Vec<Limb> {
        let klen = self.mod_p.moduli_count;
        let mut chunk_vals = Vec::new();
        if klen != 1 {
            chunk_vals.resize(klen, Natural::ZERO);
            self.mod_p.apply_into(&mut chunk_vals, x);
        }
        let mut out = vec![0; self.primes.len()];
        let mut l = 0;
        let mut i = 0;
        for (k, offset) in self.mod_offsets.iter().enumerate() {
            // With a single chunk, the input stands in for its own reduction, as FLINT arranges by
            // aliasing.
            let a_k = if klen == 1 { x } else { &chunk_vals[k] };
            for lu in &self.mod_lu[i..*offset] {
                let t = Limb::exact_from(&(a_k % Natural::from(lu.m)));
                if lu.m2 != 0 {
                    out[l] = t % lu.m0;
                    out[l + 1] = t % lu.m1;
                    out[l + 2] = t % lu.m2;
                    l += 3;
                } else if lu.m1 != 0 {
                    out[l] = t % lu.m0;
                    out[l + 1] = t % lu.m1;
                    l += 2;
                } else {
                    out[l] = t;
                    l += 1;
                }
            }
            i = *offset;
        }
        assert_eq!(l, self.primes.len());
        out
    }

    // Accumulates the residues into one value per chunk, reduced modulo the chunk product.
    //
    // This is the per-chunk accumulation of fmpz_multi_CRT_ui from fmpz/multi_CRT.c, FLINT 3.6.0,
    // always producing canonical chunk values; the final sign is applied by the caller. The flagged
    // divisor is indexed by the chunk, so it varies across the loop.
    #[cfg_attr(dylint_lib = "malachite_lints", allow(use_div_mod_precomputed))]
    fn chunk_values(&self, residues: &[Limb]) -> Vec<Natural> {
        assert_eq!(
            residues.len(),
            self.primes.len(),
            "one residue per modulus is required"
        );
        for (r, p) in residues.iter().zip(self.primes.iter()) {
            assert!(
                r < p,
                "residues must be reduced modulo the moduli, but {r} >= {p}"
            );
        }
        let mut a = Vec::with_capacity(self.crt_chunks.len());
        let mut md_pos = 0;
        let mut l = 0;
        let mut i = 0;
        for (k, offset) in self.crt_offsets.iter().enumerate() {
            let raw = self.step[k];
            let s = usize::exact_from(if raw < 0 { -raw - 1 } else { raw });
            let mut ad = vec![0; s + 2];
            if raw < 0 {
                // Every group in this chunk has one modulus, and its idempotent is premultiplied
                // into the packed multiplier, so each residue multiplies in directly.
                let mut hi: Limb = 0;
                let mut lo: Limb = 0;
                for _ in i..*offset {
                    let md = &self.packed_multipliers[md_pos..md_pos + s];
                    let carry = limbs_slice_add_mul_limb_same_length_in_place_left(
                        &mut ad[..s],
                        md,
                        residues[l],
                    );
                    let (new_lo, overflow) = lo.overflowing_add(carry);
                    lo = new_lo;
                    hi += Limb::from(overflow);
                    md_pos += s;
                    l += 1;
                }
                ad[s] = lo;
                ad[s + 1] = hi;
                i = *offset;
            } else {
                for lu in &self.crt_lu[i..*offset] {
                    // The group's combined residue: each idempotent is below the group product, so
                    // the sum fits in two words with the high word below the group product.
                    let mut acc = DoubleLimb::from(residues[l]) * DoubleLimb::from(lu.i0);
                    l += 1;
                    if lu.i2 != 0 {
                        acc += DoubleLimb::from(residues[l]) * DoubleLimb::from(lu.i1);
                        l += 1;
                        acc += DoubleLimb::from(residues[l]) * DoubleLimb::from(lu.i2);
                        l += 1;
                    } else if lu.i1 != 0 {
                        acc += DoubleLimb::from(residues[l]) * DoubleLimb::from(lu.i1);
                        l += 1;
                    }
                    let (hi, lo) = acc.split_in_half();
                    debug_assert!(hi < lu.m);
                    let t = Limb::xx_div_mod_y_to_qr(hi, lo, lu.m).1;
                    let md = &self.packed_multipliers[md_pos..md_pos + s];
                    let carry =
                        limbs_slice_add_mul_limb_same_length_in_place_left(&mut ad[..s], md, t);
                    let (new_lo, overflow) = ad[s].overflowing_add(carry);
                    ad[s] = new_lo;
                    ad[s + 1] += Limb::from(overflow);
                    md_pos += s;
                }
                i = *offset;
            }
            a.push(Natural::from_owned_limbs_asc(ad) % &self.crt_chunks[k]);
        }
        assert_eq!(l, self.primes.len());
        a
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
    /// Panics if the number of residues differs from the number of moduli, or if any residue is
    /// greater than or equal to its modulus.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural::arithmetic::crt_comb::CrtComb;
    ///
    /// let comb = CrtComb::new(&[3, 5, 7]).unwrap();
    /// // 55 is 1 mod 3, 0 mod 5, and 6 mod 7.
    /// assert_eq!(comb.combine(&[1, 0, 6]), Natural::from(55u32));
    /// ```
    ///
    /// This is fmpz_multi_CRT_ui from fmpz/multi_CRT.c, FLINT 3.6.0, with sign = 0.
    #[inline]
    pub fn combine(&self, residues: &[Limb]) -> Natural {
        self.crt_p.apply(&self.chunk_values(residues))
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
    /// Panics if the number of residues differs from the number of moduli, or if any residue is
    /// greater than or equal to its modulus.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::arithmetic::crt_comb::CrtComb;
    ///
    /// let comb = CrtComb::new(&[3, 5, 7]).unwrap();
    /// // 55 is 1 mod 3, 0 mod 5, and 6 mod 7, and its balanced representative mod 105 is -50.
    /// assert_eq!(comb.combine_balanced(&[1, 0, 6]), Integer::from(-50));
    /// ```
    ///
    /// This is fmpz_multi_CRT_ui from fmpz/multi_CRT.c, FLINT 3.6.0, with sign = 1. FLINT keeps its
    /// intermediate chunk values balanced; combining canonical chunk values and balancing at the
    /// end is equivalent, since the two differ by chunk-modulus multiples.
    #[inline]
    pub fn combine_balanced(&self, residues: &[Limb]) -> Integer {
        self.crt_p.apply_balanced(&self.chunk_values(residues))
    }
}
