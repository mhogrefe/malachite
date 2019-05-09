use std::cmp::{max, min, Ordering};

use malachite_base::comparison::Max;
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::integers::PrimitiveInteger;
use malachite_base::num::traits::{JoinHalves, SplitInHalf, WrappingAddAssign, WrappingSubAssign};

use natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use natural::arithmetic::mul::mul_mod::{
    _limbs_mul_mod_limb_width_to_n_minus_1_next_size,
    _limbs_mul_mod_limb_width_to_n_minus_1_scratch_size,
};
use natural::arithmetic::mul::{limbs_mul_greater_to_out, limbs_mul_to_out};
use natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::arithmetic::sub_mul_limb::mpn_submul_1;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::logic::not::limbs_not_to_out;
use platform::{DoubleLimb, Limb};

// will remove
fn sub_ddmmss(sh: &mut Limb, sl: &mut Limb, ah: Limb, al: Limb, bh: Limb, bl: Limb) {
    let (hi, lo) = DoubleLimb::join_halves(ah, al)
        .wrapping_sub(DoubleLimb::join_halves(bh, bl))
        .split_in_half();
    *sh = hi;
    *sl = lo;
}

// will remove
fn udiv_qrnnd(q: &mut Limb, r: &mut Limb, n_hi: Limb, n_lo: Limb, d: Limb) {
    let n = DoubleLimb::join_halves(n_hi, n_lo);
    let d = DoubleLimb::from(d);
    *r = (n % d).lower_half();
    *q = (n / d).lower_half();
}

// will remove
fn invert_limb(invxl: &mut Limb, xl: Limb) {
    assert_ne!(xl, 0);
    let mut _dummy = 0;
    udiv_qrnnd(invxl, &mut _dummy, !xl, Limb::MAX, xl);
}

// will remove
fn umul_ppmm(ph: &mut Limb, pl: &mut Limb, m1: Limb, m2: Limb) {
    let (hi, lo) = (DoubleLimb::from(m1) * DoubleLimb::from(m2)).split_in_half();
    *ph = hi;
    *pl = lo;
}

//TODO test
// checked
// docs preserved
// invert_pi1 from gmp-impl.h
fn invert_pi1(dinv: &mut Limb, d1: Limb, d0: Limb) {
    let mut v = 0;
    invert_limb(&mut v, d1);
    let mut p = d1.wrapping_mul(v);
    p.wrapping_add_assign(d0);
    if p < d0 {
        v.wrapping_sub_assign(1);
        let mask = if p >= d1 { Limb::MAX } else { 0 };
        p.wrapping_sub_assign(d1);
        v.wrapping_add_assign(mask);
        p.wrapping_sub_assign(mask & d1);
    }
    let mut t1 = 0;
    let mut t0 = 0;
    umul_ppmm(&mut t1, &mut t0, d0, v);
    p.wrapping_add_assign(t1);
    if p < t1 {
        v.wrapping_sub_assign(1);
        if p >= d1 && (p > d1 || t0 >= d0) {
            v.wrapping_sub_assign(1);
        }
    }
    *dinv = v;
}

// will remove
fn add_ssaaaa(sh: &mut Limb, sl: &mut Limb, ah1: Limb, al1: Limb, ah2: Limb, al2: Limb) {
    let (hi, lo) = DoubleLimb::join_halves(ah1, al1)
        .wrapping_add(DoubleLimb::join_halves(ah2, al2))
        .split_in_half();
    *sh = hi;
    *sl = lo;
}

//TODO test
// checked
// docs preserved
// Compute quotient the quotient and remainder for n / d. Requires d >= B^2 / 2 and n < d B. di is
// the inverse of (?)
//
// floor((B^3 - 1) / (d0 + d1 B)) - B.
//
// NOTE: Output variables are updated multiple times.
// udiv_qr_3by2 from gmp-impl.h
#[allow(clippy::too_many_arguments)]
fn udiv_qr_3by2(
    q: &mut Limb,
    r1: &mut Limb,
    r0: &mut Limb,
    n2: Limb,
    n1: Limb,
    n0: Limb,
    d1: Limb,
    d0: Limb,
    dinv: Limb,
) {
    let mut q0 = 0;
    umul_ppmm(q, &mut q0, n2, dinv);
    let old_q = *q;
    let old_q0 = q0;
    add_ssaaaa(q, &mut q0, old_q, old_q0, n2, n1);

    // Compute the two most significant limbs of n - q'd
    *r1 = n1.wrapping_sub(d1.wrapping_mul(*q));
    let old_r1 = *r1;
    sub_ddmmss(r1, r0, old_r1, n0, d1, d0);
    let mut t1 = 0;
    let mut t0 = 0;
    umul_ppmm(&mut t1, &mut t0, d0, *q);
    let old_r1 = *r1;
    let old_r0 = *r0;
    sub_ddmmss(r1, r0, old_r1, old_r0, t1, t0);
    q.wrapping_add_assign(1);

    // Conditionally adjust q and the remainders
    let mask = if *r1 >= q0 { Limb::MAX } else { 0 };
    q.wrapping_add_assign(mask);
    let old_r1 = *r1;
    let old_r0 = *r0;
    add_ssaaaa(r1, r0, old_r1, old_r0, mask & d1, mask & d0);
    if *r1 >= d1 && (*r1 > d1 || *r0 >= d0) {
        q.wrapping_add_assign(1);
        let old_r1 = *r1;
        let old_r0 = *r0;
        sub_ddmmss(r1, r0, old_r1, old_r0, d1, d0);
    }
}

//TODO test
// checked
// docs preserved
// Divide numerator (np) by denominator (dp) and write the np.len() - 2 least significant quotient
// limbs at qp and the 2-long remainder at np. Return the most significant limb of the quotient;
// this is always 0 or 1.
//
// Preconditions:
// 1. dp.len() == 2.
// 2. The most significant bit of the divisor must be set.
// 3. np.len() >= 2.
//
// mpn_divrem_2 from mpn/generic/divrem_2.c
pub fn mpn_divrem_2(qp: &mut [Limb], np: &mut [Limb], dp: &[Limb]) -> Limb {
    let nn = np.len();

    assert_eq!(dp.len(), 2);
    assert!(nn >= 2);
    assert!(dp[1].get_highest_bit());

    let mut np_offset = 0;
    np_offset += nn - 2;
    let d1 = dp[1];
    let d0 = dp[0];
    let mut r1 = np[np_offset + 1];
    let mut r0 = np[np_offset];

    let most_significant_q_limb = if r1 >= d1 && (r1 > d1 || r0 >= d0) {
        let old_r1 = r1;
        let old_r0 = r0;
        sub_ddmmss(&mut r1, &mut r0, old_r1, old_r0, d1, d0);
        1
    } else {
        0
    };

    let mut di = 0;
    invert_pi1(&mut di, d1, d0);

    for i in (0..(nn - 2)).rev() {
        let n0 = np[np_offset - 1];
        let mut q = 0;
        let old_r1 = r1;
        let old_r0 = r0;
        udiv_qr_3by2(&mut q, &mut r1, &mut r0, old_r1, old_r0, n0, d1, d0, di);
        np_offset -= 1;
        qp[i] = q;
    }

    np[np_offset + 1] = r1;
    np[np_offset] = r0;

    most_significant_q_limb
}

//TODO test
// checked
// docs preserved
// Schoolbook division using the Möller-Granlund 3/2 division algorithm.
// mpn_sbpi1_div_qr from mpn/generic/sbpi1_div_qr.c
pub fn mpn_sbpi1_div_qr(qp: &mut [Limb], np: &mut [Limb], dp: &[Limb], dinv: Limb) -> bool {
    let nn = np.len();
    let mut dn = dp.len();

    assert!(dn > 2);
    assert!(nn >= dn);
    assert!(dp[dn - 1].get_highest_bit());

    let mut np_offset = nn;

    let qh = limbs_cmp_same_length(&np[np_offset - dn..np_offset], dp) >= Ordering::Equal;
    if qh {
        limbs_sub_same_length_in_place_left(&mut np[np_offset - dn..np_offset], dp);
    }

    let mut qp_offset = nn - dn;

    dn -= 2; // offset dn by 2 for main division loops, saving two iterations in mpn_submul_1.
    let d1 = dp[dn + 1];
    let d0 = dp[dn];

    np_offset -= 2;

    let mut n1 = np[np_offset + 1];

    for _ in 1..(nn - dn - 1) {
        np_offset -= 1;
        let mut q = 0;
        if n1 == d1 && np[np_offset + 1] == d0 {
            q = Limb::MAX;
            mpn_submul_1(&mut np[np_offset - dn..], &dp[..dn + 2], q);
            n1 = np[np_offset + 1]; // update n1, last loop's value will now be invalid
        } else {
            let mut n0 = 0;
            let old_n1 = n1;
            udiv_qr_3by2(
                &mut q,
                &mut n1,
                &mut n0,
                old_n1,
                np[np_offset + 1],
                np[np_offset],
                d1,
                d0,
                dinv,
            );
            let mut cy = mpn_submul_1(&mut np[np_offset - dn..], &dp[..dn], q);
            let cy1 = if n0 < cy { 1 } else { 0 };
            n0.wrapping_sub_assign(cy);
            cy = if n1 < cy1 { 1 } else { 0 };
            n1.wrapping_sub_assign(cy1);
            np[np_offset] = n0;

            if cy != 0 {
                n1.wrapping_add_assign(d1.wrapping_add(
                    if limbs_slice_add_same_length_in_place_left(
                        &mut np[np_offset - dn..],
                        &dp[..=dn],
                    ) {
                        1
                    } else {
                        0
                    },
                ));
                q.wrapping_sub_assign(1);
            }
        }
        qp_offset -= 1;
        qp[qp_offset] = q;
    }
    np[np_offset + 1] = n1;
    qh
}

//TODO tune
const DC_DIV_QR_THRESHOLD: usize = 51;

//TODO test
// checked
// docs preserved
// Recursive divide-and-conquer division for arbitrary size operands.
// mpn_dcpi1_div_qr_n from mpn/generic/dcpi1_div_qr.c
pub fn mpn_dcpi1_div_qr_n(
    qp: &mut [Limb],
    np: &mut [Limb],
    dp: &[Limb],
    dinv: Limb,
    tp: &mut [Limb],
) -> Limb {
    let n = dp.len();
    let lo = n >> 1; // floor(n/2)
    let hi = n - lo; // ceil(n/2)

    let mut qh = if hi < DC_DIV_QR_THRESHOLD {
        if mpn_sbpi1_div_qr(
            &mut qp[lo..],
            &mut np[2 * lo..2 * (lo + hi)],
            &dp[lo..lo + hi],
            dinv,
        ) {
            1
        } else {
            0
        }
    } else {
        mpn_dcpi1_div_qr_n(
            &mut qp[lo..],
            &mut np[2 * lo..2 * lo + hi],
            &dp[lo..lo + hi],
            dinv,
            tp,
        )
    };

    limbs_mul_greater_to_out(tp, &qp[lo..lo + hi], &dp[..lo]);
    let mut cy = if limbs_sub_same_length_in_place_left(&mut np[lo..lo + n], &tp[..n]) {
        1
    } else {
        0
    };
    if qh != 0 {
        cy += if limbs_sub_same_length_in_place_left(&mut np[n..n + lo], &dp[..lo]) {
            1
        } else {
            0
        };
    }

    while cy != 0 {
        qh.wrapping_sub_assign(if limbs_sub_limb_in_place(&mut qp[lo..lo + hi], 1) {
            1
        } else {
            0
        });
        cy.wrapping_sub_assign(
            if limbs_slice_add_same_length_in_place_left(&mut np[lo..lo + n], &dp[..n]) {
                1
            } else {
                0
            },
        );
    }

    let ql = if lo < DC_DIV_QR_THRESHOLD {
        if mpn_sbpi1_div_qr(qp, &mut np[hi..hi + 2 * lo], &dp[hi..hi + lo], dinv) {
            1
        } else {
            0
        }
    } else {
        mpn_dcpi1_div_qr_n(qp, &mut np[hi..hi + 2 * lo], &dp[hi..hi + lo], dinv, tp)
    };

    limbs_mul_greater_to_out(tp, &dp[..hi], &qp[..lo]);
    let mut cy = if limbs_sub_same_length_in_place_left(&mut np[..n], &tp[..n]) {
        1
    } else {
        0
    };
    if ql != 0 {
        cy += if limbs_sub_same_length_in_place_left(&mut np[lo..lo + hi], &dp[..hi]) {
            1
        } else {
            0
        };
    }

    while cy != 0 {
        limbs_sub_limb_in_place(&mut qp[..lo], 1);
        cy -= if limbs_slice_add_same_length_in_place_left(&mut np[..n], &dp[..n]) {
            1
        } else {
            0
        };
    }
    qh
}

//TODO test
// checked
// docs preserved
// mpn_dcpi1_div_qr from mpn/generic/dcpi1_div_qr.c
#[allow(clippy::cyclomatic_complexity)]
pub fn mpn_dcpi1_div_qr(qp: &mut [Limb], np: &mut [Limb], dp: &[Limb], dinv: Limb) -> Limb {
    let nn = np.len();
    let dn = dp.len();
    assert!(dn >= 6); // to adhere to mpn_sbpi1_div_qr's limits
    assert!(nn - dn >= 3); // to adhere to mpn_sbpi1_div_qr's limits
    assert!(dp[dn - 1].get_highest_bit());
    let mut tp = vec![0; dn];
    let mut qn = nn - dn;
    let mut qp_offset = qn;
    let mut np_offset = nn;
    let dp_offset = dn;
    let mut qh;
    if qn > dn {
        // Reduce qn mod dn without division, optimizing small operations.
        loop {
            qn -= dn;
            if qn <= dn {
                break;
            }
        }
        qp_offset -= qn; // point at low limb of next quotient block
        np_offset -= qn; // point in the middle of partial remainder

        // Perform the typically smaller block first.
        if qn == 1 {
            // Handle qh up front, for simplicity.
            qh = if limbs_cmp_same_length(&np[np_offset - dn + 1..=np_offset], &dp[..dn])
                >= Ordering::Equal
            {
                1
            } else {
                0
            };
            if qh != 0 {
                //TODO
                assert!(!limbs_sub_same_length_in_place_left(
                    &mut np[np_offset - dn + 1..=np_offset],
                    &dp[dp_offset - dn..dp_offset]
                ));
            }

            // A single iteration of schoolbook: One 3/2 division, followed by the bignum update and
            // adjustment.
            let n2 = np[np_offset];
            let mut n1 = np[np_offset - 1];
            let mut n0 = np[np_offset - 2];
            let d1 = dp[dp_offset - 1];
            let d0 = dp[dp_offset - 2];

            assert!(n2 < d1 || (n2 == d1 && n1 <= d0));

            let mut q = 0;
            if n2 == d1 && n1 == d0 {
                q = Limb::MAX;
                let cy = mpn_submul_1(&mut np[np_offset - dn..], &dp[..dn], q);
                assert_eq!(cy, n2);
            } else {
                let old_n1 = n1;
                let old_n0 = n0;
                udiv_qr_3by2(&mut q, &mut n1, &mut n0, n2, old_n1, old_n0, d1, d0, dinv);

                if dn > 2 {
                    let mut cy = mpn_submul_1(
                        &mut np[np_offset - dn..],
                        &dp[dp_offset - dn..dp_offset - 2],
                        q,
                    );
                    let cy1 = if n0 < cy { 1 } else { 0 };
                    n0.wrapping_sub_assign(cy);
                    cy = if n1 < cy1 { 1 } else { 0 };
                    n1.wrapping_sub_assign(cy1);
                    np[np_offset - 2] = n0;

                    if cy != 0 {
                        n1.wrapping_add_assign(d1.wrapping_add(
                            if limbs_slice_add_same_length_in_place_left(
                                &mut np[np_offset - dn..np_offset - 1],
                                &dp[dp_offset - dn..dp_offset - 1],
                            ) {
                                1
                            } else {
                                0
                            },
                        ));
                        qh.wrapping_sub_assign(if q == 0 { 1 } else { 0 });
                        q.wrapping_sub_assign(1);
                    }
                } else {
                    np[np_offset - 2] = n0;
                }
                np[np_offset - 1] = n1;
            }
            qp[qp_offset] = q;
        } else {
            // Do a 2qn / qn division
            qh = if qn == 2 {
                mpn_divrem_2(
                    &mut qp[qp_offset..],
                    &mut np[np_offset - 2..np_offset + 2],
                    &dp[dp_offset - 2..],
                )
            } else if qn < DC_DIV_QR_THRESHOLD {
                if mpn_sbpi1_div_qr(
                    &mut qp[qp_offset..],
                    &mut np[np_offset - qn..np_offset + qn],
                    &dp[dp_offset - qn..dp_offset],
                    dinv,
                ) {
                    1
                } else {
                    0
                }
            } else {
                mpn_dcpi1_div_qr_n(
                    &mut qp[qp_offset..],
                    &mut np[np_offset - qn..np_offset],
                    &dp[dp_offset - qn..dp_offset],
                    dinv,
                    &mut tp,
                )
            };

            if qn != dn {
                limbs_mul_to_out(
                    &mut tp,
                    &qp[qp_offset..qp_offset + qn],
                    &dp[dp_offset - dn..dp_offset - qn],
                );

                let mut cy = if limbs_sub_same_length_in_place_left(
                    &mut np[np_offset - dn..np_offset],
                    &tp[..dn],
                ) {
                    1
                } else {
                    0
                };
                if qh != 0 {
                    cy += if limbs_sub_same_length_in_place_left(
                        &mut np[np_offset - dn + qn..np_offset],
                        &dp[dp_offset - dn..dp_offset - qn],
                    ) {
                        1
                    } else {
                        0
                    };
                }

                while cy != 0 {
                    qh -= if limbs_sub_limb_in_place(&mut qp[qp_offset..qp_offset + qn], 1) {
                        1
                    } else {
                        0
                    };
                    cy -= if limbs_slice_add_same_length_in_place_left(
                        &mut np[np_offset - dn..np_offset],
                        &dp[dp_offset - dn..dp_offset],
                    ) {
                        1
                    } else {
                        0
                    };
                }
            }
        }

        let mut qn = isize::checked_from(nn - dn - qn).unwrap();
        assert!(qn >= 0);
        loop {
            qp_offset -= dn;
            np_offset -= dn;
            mpn_dcpi1_div_qr_n(
                &mut qp[qp_offset..],
                &mut np[np_offset - dn..np_offset],
                &dp[dp_offset - dn..dp_offset],
                dinv,
                &mut tp,
            );
            qn -= isize::checked_from(dn).unwrap();
            if qn <= 0 {
                break;
            }
        }
    } else {
        qp_offset -= qn; // point at low limb of next quotient block
        np_offset -= qn; // point in the middle of partial remainder

        qh = if qn < DC_DIV_QR_THRESHOLD {
            if mpn_sbpi1_div_qr(
                &mut qp[qp_offset..],
                &mut np[np_offset - qn..np_offset + qn],
                &dp[dp_offset - qn..dp_offset],
                dinv,
            ) {
                1
            } else {
                0
            }
        } else {
            mpn_dcpi1_div_qr_n(
                &mut qp[qp_offset..],
                &mut np[np_offset - qn..np_offset],
                &dp[dp_offset - qn..dp_offset],
                dinv,
                &mut tp,
            )
        };

        if qn != dn {
            limbs_mul_to_out(
                &mut tp,
                &qp[qp_offset..qp_offset + qn],
                &dp[dp_offset - dn..dp_offset - qn],
            );
            let mut cy = if limbs_sub_same_length_in_place_left(
                &mut np[np_offset - dn..np_offset],
                &tp[..dn],
            ) {
                1
            } else {
                0
            };
            if qh != 0 {
                cy += if limbs_sub_same_length_in_place_left(
                    &mut np[np_offset - dn + qn..np_offset],
                    &dp[dp_offset - dn..dp_offset - qn],
                ) {
                    1
                } else {
                    0
                };
            }

            while cy != 0 {
                qh -= if limbs_sub_limb_in_place(&mut qp[qp_offset..qp_offset + qn], 1) {
                    1
                } else {
                    0
                };
                cy -= if limbs_slice_add_same_length_in_place_left(
                    &mut np[np_offset - dn..np_offset],
                    &dp[dp_offset - dn..dp_offset],
                ) {
                    1
                } else {
                    0
                };
            }
        }
    }
    qh
}

//TODO test
// checked
// docs preserved
// In case k == 0 (automatic choice), we distinguish 3 cases:
// (a) dn < qn:           in = ceil(qn / ceil(qn / dn))
// (b) dn / 3 < qn <= dn: in = ceil(qn / 2)
// (c) qn < dn / 3:       in = qn
// In all cases we have in <= dn.
// mpn_mulmod_bnm1_itch from mpn/generic/mu_div_qr.c
pub fn mpn_mu_div_qr_choose_in(qn: usize, dn: usize, k: Limb) -> usize {
    if k == 0 {
        if qn > dn {
            // Compute an inverse size that is a nice partition of the quotient.
            let b = (qn - 1) / dn + 1; // ceil(qn / dn), number of blocks
            (qn - 1) / b + 1 // ceil(qn / b) = ceil(qn / ceil(qn / dn))
        } else if 3 * qn > dn {
            (qn - 1) / 2 + 1 // b = 2
        } else {
            (qn - 1) + 1 // b = 1
        }
    } else {
        let xn = min(dn, qn);
        (xn - 1) / usize::checked_from(k).unwrap() + 1
    }
}

//TODO test
// checked
// docs preserved
// mpn_preinv_mu_div_qr_itch from mpn/generic/mu_div_qr.c
pub fn mpn_preinv_mu_div_qr_itch(dn: usize, in_size: usize) -> usize {
    let itch_local = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(dn + 1);
    let itch_out = _limbs_mul_mod_limb_width_to_n_minus_1_scratch_size(itch_local, dn, in_size);
    itch_local + itch_out
}

//TODO test
// checked
// docs preserved
// mpn_invertappr_itch from gmp-impl.h
pub fn mpn_invertappr_itch(n: usize) -> usize {
    2 * n
}

//TODO test
// checked
// docs preserved
// mpn_mu_div_qr_itch from mpn/generic/mu_div_qr.c
pub fn mpn_mu_div_qr_itch(nn: usize, dn: usize, mua_k: Limb) -> usize {
    let in_size = mpn_mu_div_qr_choose_in(nn - dn, dn, mua_k);
    let itch_preinv = mpn_preinv_mu_div_qr_itch(dn, in_size);
    let itch_invapp = mpn_invertappr_itch(in_size + 1) + in_size + 2; // 3 * in_size + 4

    assert!(itch_preinv >= itch_invapp);
    in_size + max(itch_invapp, itch_preinv)
}

//TODO test
// checked
// docs preserved
// Schoolbook division using the Möller-Granlund 3/2 division algorithm, returning approximate
// quotient. The quotient returned is either correct, or one too large.
// mpn_sbpi1_divappr_q from mpn/generic/sbpi1_divappr_q.c
pub fn mpn_sbpi1_divappr_q(qp: &mut [Limb], np: &mut [Limb], dp: &[Limb], dinv: Limb) -> Limb {
    let nn = np.len();
    let mut dn = dp.len();

    assert!(dn > 2);
    assert!(nn >= dn);
    assert!(dp[dn - 1].get_highest_bit());

    let mut np_offset = nn;
    let qn = nn - dn;
    let mut dp_offset = 0;
    if qn + 1 < dn {
        dp_offset += dn - (qn + 1);
        dn = qn + 1;
    }

    let qh = if limbs_cmp_same_length(
        &np[np_offset - dn..np_offset],
        &dp[dp_offset..dp_offset + dn],
    ) >= Ordering::Equal
    {
        1
    } else {
        0
    };
    if qh != 0 {
        limbs_sub_same_length_in_place_left(
            &mut np[np_offset - dn..np_offset],
            &dp[dp_offset..dp_offset + dn],
        );
    }
    let mut qp_offset = qn;
    let dn_was_at_least_2 = dn >= 2;
    dn -= 2; // offset dn by 2 for main division loops, saving two iterations in mpn_submul_1.
    let d1 = dp[dp_offset + dn + 1];
    let d0 = dp[dp_offset + dn];
    np_offset -= 2;
    let mut n1 = np[np_offset + 1];
    let mut q = 0;
    let mut n0 = 0;
    for _ in 0..(qn - dn - 1) {
        np_offset -= 1;
        if n1 == d1 && np[np_offset + 1] == d0 {
            q = Limb::MAX;
            mpn_submul_1(
                &mut np[np_offset - dn..],
                &dp[dp_offset..dp_offset + dn + 2],
                q,
            );
            n1 = np[np_offset + 1]; // update n1, last loop's value will now be invalid
        } else {
            let old_n1 = n1;
            udiv_qr_3by2(&mut q, &mut n1, &mut n0, old_n1, np[1], np[0], d1, d0, dinv);
            let mut cy = mpn_submul_1(&mut np[np_offset - dn..], &dp[dp_offset..dp_offset + dn], q);
            let cy1 = if n0 < cy { 1 } else { 0 };
            n0.wrapping_sub_assign(cy);
            cy = if n1 < cy1 { 1 } else { 0 };
            n1.wrapping_sub_assign(cy1);
            np[np_offset] = n0;

            if cy != 0 {
                n1.wrapping_add_assign(d1.wrapping_add(
                    if limbs_slice_add_same_length_in_place_left(
                        &mut np[np_offset - dn..=np_offset],
                        &dp[dp_offset..=dp_offset + dn],
                    ) {
                        1
                    } else {
                        0
                    },
                ));
                q -= 1;
            }
        }
        qp_offset -= 1;
        qp[qp_offset] = q;
    }

    let mut flag = Limb::MAX;
    if dn_was_at_least_2 {
        let limit = dn;
        for _ in 0..limit {
            np_offset -= 1;
            if n1 >= (d1 & flag) {
                q = Limb::MAX;
                let cy = mpn_submul_1(
                    &mut np[np_offset - dn..],
                    &dp[dp_offset..dp_offset + dn + 2],
                    q,
                );
                if n1 != cy {
                    if n1 < (cy & flag) {
                        q.wrapping_sub_assign(1);
                        limbs_slice_add_same_length_in_place_left(
                            &mut np[np_offset - dn..np_offset + 2],
                            &dp[dp_offset..dp_offset + dn + 2],
                        );
                    } else {
                        flag = 0;
                    }
                }
                n1 = np[np_offset + 1];
            } else {
                let old_n1 = n1;
                udiv_qr_3by2(
                    &mut q,
                    &mut n1,
                    &mut n0,
                    old_n1,
                    np[np_offset + 1],
                    np[np_offset],
                    d1,
                    d0,
                    dinv,
                );

                let mut cy = mpn_submul_1(
                    &mut np[np_offset - dn..np_offset],
                    &dp[dp_offset..dp_offset + dn],
                    q,
                );
                let cy1 = if n0 < cy { 1 } else { 0 };
                n0.wrapping_sub_assign(cy);
                cy = if n1 < cy1 { 1 } else { 0 };
                n1.wrapping_sub_assign(cy1);
                np[np_offset] = n0;

                if cy != 0 {
                    n1.wrapping_add_assign(d1.wrapping_add(
                        if limbs_slice_add_same_length_in_place_left(
                            &mut np[np_offset - dn..=np_offset],
                            &dp[dp_offset..=dp_offset + dn],
                        ) {
                            1
                        } else {
                            0
                        },
                    ));
                    q.wrapping_sub_assign(1);
                }
            }
            qp_offset -= 1;
            qp[qp_offset] = q;

            // Truncate operands.
            dn -= 1;
            dp_offset += 1;
        }

        np_offset -= 1;
        if n1 >= (d1 & flag) {
            q = Limb::MAX;
            let cy = mpn_submul_1(&mut np[np_offset..], &dp[dp_offset..dp_offset + 2], q);

            if n1 != cy && n1 < (cy & flag) {
                q.wrapping_sub_assign(1);
                let old_np0 = np[np_offset];
                let old_np1 = np[np_offset + 1];
                let (np_lo, np_hi) = np.split_at_mut(np_offset + 1);
                add_ssaaaa(
                    &mut np_hi[0],
                    &mut np_lo[np_offset],
                    old_np1,
                    old_np0,
                    dp[dp_offset + 1],
                    dp[dp_offset],
                );
            }
            n1 = np[np_offset + 1];
        } else {
            let old_n1 = n1;
            udiv_qr_3by2(
                &mut q,
                &mut n1,
                &mut n0,
                old_n1,
                np[np_offset + 1],
                np[np_offset],
                d1,
                d0,
                dinv,
            );

            np[np_offset + 1] = n1;
            np[np_offset] = n0;
        }
        qp_offset -= 1;
        qp[qp_offset] = q;
    }
    assert_eq!(np[np_offset + 1], n1);
    qh
}

//TODO tune
const MAYBE_DCP1_DIVAPPR: bool = true;
const DC_DIVAPPR_Q_THRESHOLD: usize = 171;

//TODO test
// docs preserved
// checked
// mpn_dcpi1_divappr_q_n from mpn/generic/dcpi1_divappr_q.c
pub fn mpn_dcpi1_divappr_q_n(
    qp: &mut [Limb],
    np: &mut [Limb],
    dp: &[Limb],
    dinv: Limb,
    tp: &mut [Limb],
) -> Limb {
    let n = dp.len();
    assert_eq!(np.len(), n);
    let lo = n >> 1; // floor(n / 2)
    let hi = n - lo; // ceil(n / 2)

    let mut qh = if hi < DC_DIV_QR_THRESHOLD {
        if mpn_sbpi1_div_qr(
            &mut qp[lo..],
            &mut np[2 * lo..2 * (lo + hi)],
            &dp[lo..lo + hi],
            dinv,
        ) {
            1
        } else {
            0
        }
    } else {
        mpn_dcpi1_div_qr_n(
            &mut qp[lo..],
            &mut np[2 * lo..2 * lo + hi],
            &dp[lo..lo + hi],
            dinv,
            tp,
        )
    };
    limbs_mul_greater_to_out(tp, &qp[lo..lo + hi], &dp[..lo]);
    let mut cy = if limbs_sub_same_length_in_place_left(&mut np[lo..lo + n], &tp[..n]) {
        1
    } else {
        0
    };
    if qh != 0 {
        cy += if limbs_sub_same_length_in_place_left(&mut np[n..n + lo], &dp[..lo]) {
            1
        } else {
            0
        };
    }

    while cy != 0 {
        qh.wrapping_sub_assign(if limbs_sub_limb_in_place(&mut qp[lo..lo + hi], 1) {
            1
        } else {
            0
        });
        cy.wrapping_sub_assign(
            if limbs_slice_add_same_length_in_place_left(&mut np[lo..lo + n], &dp[..n]) {
                1
            } else {
                0
            },
        );
    }

    let ql = if lo < DC_DIVAPPR_Q_THRESHOLD {
        mpn_sbpi1_divappr_q(qp, &mut np[hi..hi + 2 * lo], &dp[hi..hi + lo], dinv)
    } else {
        mpn_dcpi1_divappr_q_n(qp, &mut np[hi..hi + lo], &dp[hi..hi + lo], dinv, tp)
    };
    if ql != 0 {
        for q in qp[..lo].iter_mut() {
            *q = Limb::MAX;
        }
    }
    qh
}

//TODO test
// docs preserved
// checked
// divide-and-conquer division, returning approximate quotient. The quotient returned is either
// correct, or one too large.
// mpn_dcpi1_divappr_q from mpn/generic/dcpi1_divappr_q.c
#[allow(clippy::cyclomatic_complexity)]
pub fn mpn_dcpi1_divappr_q(qp: &mut [Limb], np: &mut [Limb], dp: &[Limb], dinv: Limb) -> Limb {
    let nn = np.len();
    let dn = dp.len();
    assert!(dn >= 6);
    assert!(nn > dn);
    assert!(dp[dn - 1].get_highest_bit());

    let mut qn = nn - dn;
    let mut qp_offset = qn;
    let mut np_offset = nn;
    let dp_offset = dn;
    let mut qh;
    if qn >= dn {
        qn += 1; // Pretend we'll need an extra limb
                 // Reduce qn mod dn without division, optimizing small operations.
        loop {
            qn -= dn;
            if qn <= dn {
                break;
            }
        }

        qp_offset -= qn; // point at low limb of next quotient block
        np_offset -= qn; // point in the middle of partial remainder
        let mut tp = vec![0; dn];
        // Perform the typically smaller block first.
        if qn == 1 {
            // Handle qh up front, for simplicity.
            qh = if limbs_cmp_same_length(
                &np[np_offset - dn + 1..=np_offset],
                &dp[dp_offset - dn..dp_offset],
            ) >= Ordering::Equal
            {
                1
            } else {
                0
            };
            if qh != 0 {
                assert!(!limbs_sub_same_length_in_place_left(
                    &mut np[np_offset - dn + 1..=np_offset],
                    &dp[dp_offset - dn..dp_offset]
                ));
            }

            // A single iteration of schoolbook: One 3/2 division, followed by the bignum update and
            // adjustment.
            let n2 = np[np_offset];
            let mut n1 = np[np_offset - 1];
            let mut n0 = np[np_offset - 2];
            let d1 = dp[dp_offset - 1];
            let d0 = dp[dp_offset - 2];
            assert!(n2 < d1 || (n2 == d1 && n1 <= d0));
            let mut q = 0;
            if n2 == d1 && n1 == d0 {
                q = Limb::MAX;
                let cy = mpn_submul_1(&mut np[np_offset - dn..], &dp[dp_offset - dn..dp_offset], q);
                assert_eq!(cy, n2);
            } else {
                let old_n1 = n1;
                let old_n0 = n0;
                udiv_qr_3by2(&mut q, &mut n1, &mut n0, n2, old_n1, old_n0, d1, d0, dinv);

                if dn > 2 {
                    //mp_limb_t cy, cy1;
                    let mut cy = mpn_submul_1(
                        &mut np[np_offset - dn..],
                        &dp[dp_offset - dn..dp_offset - 2],
                        q,
                    );
                    let cy1 = if n0 < cy { 1 } else { 0 };
                    n0.wrapping_sub_assign(cy);
                    cy = if n1 < cy1 { 1 } else { 0 };
                    n1.wrapping_sub_assign(cy1);
                    np[np_offset - 2] = n0;

                    if cy != 0 {
                        n1.wrapping_add_assign(d1.wrapping_add(
                            if limbs_slice_add_same_length_in_place_left(
                                &mut np[np_offset - dn..np_offset - 1],
                                &dp[dp_offset - dn..dp_offset - 1],
                            ) {
                                1
                            } else {
                                0
                            },
                        ));
                        qh.wrapping_sub_assign(if q == 0 { 1 } else { 0 });
                        q.wrapping_sub_assign(1);
                    }
                } else {
                    np[np_offset - 2] = n0;
                }
                np[np_offset - 1] = n1;
            }
            qp[qp_offset] = q;
        } else {
            qh = if qn == 2 {
                mpn_divrem_2(
                    &mut qp[qp_offset..],
                    &mut np[np_offset - 2..np_offset + 2],
                    &dp[dp_offset - 2..],
                )
            } else if qn < DC_DIV_QR_THRESHOLD {
                if mpn_sbpi1_div_qr(
                    &mut qp[qp_offset..],
                    &mut np[np_offset - qn..np_offset + qn],
                    &dp[dp_offset - qn..dp_offset],
                    dinv,
                ) {
                    1
                } else {
                    0
                }
            } else {
                mpn_dcpi1_div_qr_n(
                    &mut qp[qp_offset..],
                    &mut np[np_offset - qn..np_offset],
                    &dp[dp_offset - qn..dp_offset],
                    dinv,
                    &mut tp,
                )
            };

            if qn != dn {
                limbs_mul_to_out(
                    &mut tp,
                    &qp[qp_offset..qp_offset + qn],
                    &dp[dp_offset - dn..dp_offset - qn],
                );

                let mut cy = if limbs_sub_same_length_in_place_left(
                    &mut np[np_offset - dn..np_offset],
                    &tp[..dn],
                ) {
                    1
                } else {
                    0
                };
                if qh != 0 {
                    cy += if limbs_sub_same_length_in_place_left(
                        &mut np[np_offset - dn + qn..np_offset],
                        &dp[dp_offset - dn..dp_offset - qn],
                    ) {
                        1
                    } else {
                        0
                    };
                }

                while cy != 0 {
                    qh -= if limbs_sub_limb_in_place(&mut qp[qp_offset..qp_offset + qn], 1) {
                        1
                    } else {
                        0
                    };
                    cy -= if limbs_slice_add_same_length_in_place_left(
                        &mut np[np_offset - dn..np_offset],
                        &dp[dp_offset - dn..dp_offset],
                    ) {
                        1
                    } else {
                        0
                    };
                }
            }
        }
        qn = nn - dn - qn + 1;
        while qn > dn {
            qp_offset -= dn;
            np_offset -= dn;
            mpn_dcpi1_div_qr_n(
                &mut qp[qp_offset..],
                &mut np[np_offset - dn..np_offset],
                &dp[dp_offset - dn..dp_offset],
                dinv,
                &mut tp,
            );
            qn -= dn;
        }

        // Since we pretended we'd need an extra quotient limb before, we now have made sure the
        // code above left just dp.len() - 1 = qp.len() quotient limbs to develop. Develop that plus
        // a guard limb.
        qn -= 1;
        qp_offset -= qn;
        np_offset -= dn;
        let qsave = qp[qp_offset + qn];
        mpn_dcpi1_divappr_q_n(
            &mut qp[qp_offset..],
            &mut np[np_offset - dn..np_offset],
            &dp[dp_offset - dn..dp_offset],
            dinv,
            &mut tp,
        );
        //TODO use copy_within when stable
        for i in qp_offset..qn + qp_offset {
            qp[i + 1] = qp[i];
        }
        qp[qp_offset + qn] = qsave;
    } else {
        // qp.len() < dp.len()
        qp_offset -= qn; // point at low limb of next quotient block
        np_offset -= qn; // point in the middle of partial remainder
        let mut q2p = vec![0; qn + 1];
        // Should we at all check DC_DIVAPPR_Q_THRESHOLD here, or rely on callers not to be silly?
        if qn < DC_DIVAPPR_Q_THRESHOLD {
            qh = mpn_sbpi1_divappr_q(
                &mut q2p,
                &mut np[np_offset - qn - 2..np_offset + qn],
                &dp[dp_offset - qn - 1..dp_offset],
                dinv,
            );
        } else {
            // It is tempting to use qp for recursive scratch and put quotient in tp, but the
            // recursive scratch needs one limb too many.
            let mut tp = vec![0; qn + 1];
            qh = mpn_dcpi1_divappr_q_n(
                &mut q2p,
                &mut np[np_offset - qn - 2..np_offset - 1],
                &dp[dp_offset - qn - 1..dp_offset],
                dinv,
                &mut tp,
            );
        }
        qp[qp_offset..qp_offset + qn].copy_from_slice(&q2p[1..=qn]);
    }
    qh
}

//TODO test
// docs preserved
// mpn_bc_invertappr (ip, dp, scratch), takes the strictly normalised value dp (i.e., most
// significant bit must be set) as an input, and computes ip of length n: the approximate reciprocal
// of dp.
//
// Let e = mpn_bc_invertappr(ip, dp, scratch) be the returned value; the following conditions are
// satisfied by the output:
//   a) 0 <= e <= 1
//   b) dp * (B ^ n + ip) < B ^ {2n} <= dp * (B ^ n + ip + 1 + e)
//      i.e. e=0 means that the result ip equals the one given by mpn_invert. e=1 means that the
//      result may be one less than expected. e=1 most of the time.
//
// When the strict result is needed, i.e., e = 0 in the relation above:
//   dp * (B ^ n + ip) < B ^ {2n} <= dp * (B ^ n + ip + 1)
// the function mpn_invert(ip, dp, scratch) should be used instead.
// mpn_bc_invertappr from mpn/generic/invertappr.c
pub fn mpn_bc_invertappr(ip: &mut [Limb], dp: &[Limb], xp: &mut [Limb]) -> Limb {
    let n = dp.len();
    assert_ne!(n, 0);
    assert!(dp[n - 1].get_highest_bit());

    // Compute a base value of r limbs.
    if n == 1 {
        invert_limb(&mut ip[0], dp[0]);
    } else {
        // n > 1 here
        let mut i = n;
        loop {
            i -= 1;
            xp[i] = Limb::MAX;
            if i == 0 {
                break;
            }
        }
        limbs_not_to_out(&mut xp[n..], &dp[..n]);

        // Now xp contains B ^ 2n - dp * B ^ n - 1
        if n == 2 {
            mpn_divrem_2(ip, &mut xp[..4], dp);
        } else {
            let mut inv = 0;
            invert_pi1(&mut inv, dp[n - 1], dp[n - 2]);
            if !MAYBE_DCP1_DIVAPPR || n < DC_DIVAPPR_Q_THRESHOLD {
                mpn_sbpi1_divappr_q(ip, &mut xp[..2 * n], &dp[..n], inv);
            } else {
                mpn_dcpi1_divappr_q(ip, &mut xp[..2 * n], &dp[..n], inv);
                limbs_sub_limb_in_place(&mut ip[..n], 1);
                return 1;
            }
        }
    }
    0
}

//TODO PASTE E
