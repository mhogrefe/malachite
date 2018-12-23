use malachite_base::num::{
    JoinHalves, PrimitiveInteger, SplitInHalf, WrappingAddAssign, WrappingSubAssign,
};
use natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use natural::arithmetic::div_mod_u32::limbs_div_limb_to_out_mod;
use natural::arithmetic::mul::{mpn_mul, mpn_mulmod_bnm1_itch, mpn_mulmod_bnm1_next_size};
use natural::arithmetic::shl_u::limbs_shl_to_out;
use natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
use natural::arithmetic::sub_mul_u32::mpn_submul_1;
use natural::arithmetic::sub_u32::limbs_sub_limb_in_place;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::logic::not::limbs_not_to_out;
use std::cmp::{max, min, Ordering};
use std::mem::size_of;
use std::u32;

// will remove
fn sub_ddmmss(sh: &mut u32, sl: &mut u32, ah: u32, al: u32, bh: u32, bl: u32) {
    let (hi, lo) = u64::join_halves(ah, al)
        .wrapping_sub(u64::join_halves(bh, bl))
        .split_in_half();
    *sh = hi;
    *sl = lo;
}

// will remove
fn udiv_qrnnd(q: &mut u32, r: &mut u32, n_hi: u32, n_lo: u32, d: u32) {
    let n = u64::join_halves(n_hi, n_lo);
    let d = u64::from(d);
    *r = (n % d).lower_half();
    *q = (n / d).lower_half();
}

// will remove
fn invert_limb(invxl: &mut u32, xl: u32) {
    assert_ne!(xl, 0);
    let mut _dummy = 0;
    udiv_qrnnd(invxl, &mut _dummy, !xl, u32::MAX, xl);
}

// will remove
fn umul_ppmm(ph: &mut u32, pl: &mut u32, m1: u32, m2: u32) {
    let (hi, lo) = (u64::from(m1) * u64::from(m2)).split_in_half();
    *ph = hi;
    *pl = lo;
}

// checked
// docs preserved
// invert_pi1 from gmp-impl.h
fn invert_pi1(dinv: &mut u32, d1: u32, d0: u32) {
    let mut v = 0;
    invert_limb(&mut v, d1);
    let mut p = d1.wrapping_mul(v);
    p.wrapping_add_assign(d0);
    if p < d0 {
        v.wrapping_sub_assign(1);
        let mask = if p >= d1 { u32::MAX } else { 0 };
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
        if p >= d1 {
            if p > d1 || t0 >= d0 {
                v.wrapping_sub_assign(1);
            }
        }
    }
    *dinv = v;
}

// will remove
fn add_ssaaaa(sh: &mut u32, sl: &mut u32, ah1: u32, al1: u32, ah2: u32, al2: u32) {
    let (hi, lo) = u64::join_halves(ah1, al1)
        .wrapping_add(u64::join_halves(ah2, al2))
        .split_in_half();
    *sh = hi;
    *sl = lo;
}

// checked
// docs preserved
// Compute quotient the quotient and remainder for n / d. Requires d >= B^2 / 2 and n < d B. di is
// the inverse of (?)
//
// floor((B^3 - 1) / (d0 + d1 B)) - B.
//
// NOTE: Output variables are updated multiple times.
// udiv_qr_3by2 from gmp-impl.h
fn udiv_qr_3by2(
    q: &mut u32,
    r1: &mut u32,
    r0: &mut u32,
    n2: u32,
    n1: u32,
    n0: u32,
    d1: u32,
    d0: u32,
    dinv: u32,
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
    let mask = if *r1 >= q0 { u32::MAX } else { 0 };
    q.wrapping_add_assign(mask);
    let old_r1 = *r1;
    let old_r0 = *r0;
    add_ssaaaa(r1, r0, old_r1, old_r0, mask & d1, mask & d0);
    if *r1 >= d1 {
        if *r1 > d1 || *r0 >= d0 {
            q.wrapping_add_assign(1);
            let old_r1 = *r1;
            let old_r0 = *r0;
            sub_ddmmss(r1, r0, old_r1, old_r0, d1, d0);
        }
    }
}

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
pub fn mpn_divrem_2(qp: &mut [u32], np: &mut [u32], dp: &[u32]) -> u32 {
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

    let mut most_significant_q_limb = 0;
    if r1 >= d1 && (r1 > d1 || r0 >= d0) {
        let old_r1 = r1;
        let old_r0 = r0;
        sub_ddmmss(&mut r1, &mut r0, old_r1, old_r0, d1, d0);
        most_significant_q_limb = 1;
    }

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

// checked
// docs preserved
// Schoolbook division using the Möller-Granlund 3/2 division algorithm.
// mpn_sbpi1_div_qr from mpn/generic/sbpi1_div_qr.c
pub fn mpn_sbpi1_div_qr(qp: &mut [u32], np: &mut [u32], dp: &[u32], dinv: u32) -> bool {
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
    let d0 = dp[dn + 0];

    np_offset -= 2;

    let mut n1 = np[np_offset + 1];

    for _ in 1..(nn - dn - 1) {
        np_offset -= 1;
        let mut q = 0;
        if n1 == d1 && np[np_offset + 1] == d0 {
            q = u32::MAX;
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
                        &dp[..dn + 1],
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
const DC_DIV_QR_THRESHOLD: usize = 56;

// checked
// docs preserved
// Recursive divide-and-conquer division for arbitrary size operands.
// mpn_dcpi1_div_qr_n from mpn/generic/dcpi1_div_qr.c
pub fn mpn_dcpi1_div_qr_n(
    qp: &mut [u32],
    np: &mut [u32],
    dp: &[u32],
    dinv: u32,
    tp: &mut [u32],
) -> u32 {
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

    mpn_mul(tp, &qp[lo..lo + hi], &dp[..lo]);
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

    mpn_mul(tp, &dp[..hi], &qp[..lo]);
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

// checked
// docs preserved
// mpn_dcpi1_div_qr from mpn/generic/dcpi1_div_qr.c
pub fn mpn_dcpi1_div_qr(qp: &mut [u32], np: &mut [u32], dp: &[u32], dinv: u32) -> u32 {
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
            qh = if limbs_cmp_same_length(&np[np_offset - dn + 1..np_offset + 1], &dp[..dn])
                >= Ordering::Equal
            {
                1
            } else {
                0
            };
            if qh != 0 {
                //TODO
                assert!(!limbs_sub_same_length_in_place_left(
                    &mut np[np_offset - dn + 1..np_offset + 1],
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
                q = u32::MAX;
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
                if qn > dn - qn {
                    mpn_mul(
                        &mut tp,
                        &qp[qp_offset..qp_offset + qn],
                        &dp[dp_offset - dn..dp_offset - qn],
                    );
                } else {
                    mpn_mul(
                        &mut tp,
                        &dp[dp_offset - dn..dp_offset - qn],
                        &qp[qp_offset..qp_offset + qn],
                    );
                }

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

        let mut qn = (nn - dn - qn) as isize;
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
            qn -= dn as isize;
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
            if qn > dn - qn {
                mpn_mul(
                    &mut tp,
                    &qp[qp_offset..qp_offset + qn],
                    &dp[dp_offset - dn..dp_offset - qn],
                );
            } else {
                mpn_mul(
                    &mut tp,
                    &dp[dp_offset - dn..dp_offset - qn],
                    &qp[qp_offset..qp_offset + qn],
                );
            }

            let mut cy = if limbs_sub_same_length_in_place_left(
                &mut np[np_offset - dn..np_offset],
                &mut tp[..dn],
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

// checked
// docs preserved
// In case k == 0 (automatic choice), we distinguish 3 cases:
// (a) dn < qn:           in = ceil(qn / ceil(qn / dn))
// (b) dn / 3 < qn <= dn: in = ceil(qn / 2)
// (c) qn < dn / 3:       in = qn
// In all cases we have in <= dn.
// mpn_mulmod_bnm1_itch from mpn/generic/mu_div_qr.c
pub fn mpn_mu_div_qr_choose_in(qn: usize, dn: usize, k: u32) -> usize {
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
        (xn - 1) / (k as usize) + 1
    }
}

// checked
// docs preserved
// mpn_preinv_mu_div_qr_itch from mpn/generic/mu_div_qr.c
pub fn mpn_preinv_mu_div_qr_itch(dn: usize, in_size: usize) -> usize {
    let itch_local = mpn_mulmod_bnm1_next_size(dn + 1);
    let itch_out = mpn_mulmod_bnm1_itch(itch_local, dn, in_size);
    itch_local + itch_out
}

pub fn mpn_invertappr_itch(n: usize) -> usize {
    2 * n
}

pub fn mpn_mu_div_qr_itch(nn: usize, dn: usize, mua_k: u32) -> usize {
    let in_size = mpn_mu_div_qr_choose_in(nn - dn, dn, mua_k);
    let itch_preinv = mpn_preinv_mu_div_qr_itch(dn, in_size);
    let itch_invapp = mpn_invertappr_itch(in_size + 1) + in_size + 2; /* 3in + 4 */

    assert!(itch_preinv >= itch_invapp);
    in_size + max(itch_invapp, itch_preinv)
}

pub fn mpn_sbpi1_divappr_q(qp: &mut [u32], np: &mut [u32], dp: &[u32], dinv: u32) -> u32 {
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
            &mut np[np_offset - dn..],
            &dp[dp_offset..dp_offset + dn],
        );
    }

    let mut qp_offset = qn;
    let dn_at_least_2 = dn >= 2;
    dn -= 2; /* offset dn by 2 for main division loops,
             saving two iterations in mpn_submul_1.  */
    let d1 = dp[dp_offset + dn + 1];
    let d0 = dp[dp_offset + dn];

    np_offset -= 2;

    let mut n1 = np[np_offset + 1];

    let mut q = 0;
    let mut n0 = 0;
    for _ in 0..(qn - dn - 1) {
        np_offset -= 1;
        if n1 == d1 && np[np_offset + 1] == d0 {
            q = u32::MAX;
            mpn_submul_1(
                &mut np[np_offset - dn..],
                &dp[dp_offset..dp_offset + dn + 2],
                q,
            );
            n1 = np[np_offset + 1]; /* update n1, last loop's value will now be invalid */
        } else {
            let old_n1 = n1;
            udiv_qr_3by2(&mut q, &mut n1, &mut n0, old_n1, np[1], np[0], d1, d0, dinv);
            let mut cy = mpn_submul_1(&mut np[np_offset - dn..], &dp[dp_offset..dp_offset + dn], q);
            let cy1 = if n0 < cy { 1 } else { 0 };
            n0.wrapping_sub_assign(cy);
            cy = if n1 < cy1 { 1 } else { 0 };
            n1 -= cy1;
            np[np_offset] = n0;

            if cy != 0 {
                n1.wrapping_add_assign(d1.wrapping_add(
                    if limbs_slice_add_same_length_in_place_left(
                        &mut np[np_offset - dn..],
                        &dp[dp_offset..dp_offset + dn + 1],
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

    let mut flag = u32::MAX;

    if dn_at_least_2 {
        for _ in 0..dn {
            np_offset -= 1;
            if n1 >= (d1 & flag) {
                q = u32::MAX;
                let cy = mpn_submul_1(
                    &mut np[np_offset - dn..],
                    &dp[dp_offset..dp_offset + dn + 2],
                    q,
                );
                if n1 != cy {
                    if n1 < (cy & flag) {
                        q.wrapping_add_assign(1);
                        limbs_slice_add_same_length_in_place_left(
                            &mut np[np_offset - dn..],
                            &dp[dp_offset..dp_offset + dn + 2],
                        );
                    } else {
                        flag = 0;
                    }
                }
                n1 = np[np_offset + 1];
            } else {
                let old_n1 = n1;
                udiv_qr_3by2(&mut q, &mut n1, &mut n0, old_n1, np[1], np[0], d1, d0, dinv);

                let mut cy = mpn_submul_1(
                    &mut np[np_offset..np_offset + dn],
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
                            &mut np[np_offset - dn..np_offset],
                            &dp[dp_offset..dp_offset + dn + 1],
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

            /* Truncate operands.  */
            dn -= 1;
            dp_offset += 1;
        }

        np_offset -= 1;
        if n1 >= (d1 & flag) {
            q = u32::MAX;
            let cy = mpn_submul_1(np, &dp[dp_offset..dp_offset + 2], q);

            if n1 != cy {
                if n1 < (cy & flag) {
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
const DC_DIVAPPR_Q_THRESHOLD: usize = 68;

// docs preserved
// mpn_dcpi1_divappr_q_n from mpn/generic/dcpi1_divappr_q.c
pub fn mpn_dcpi1_divappr_q_n(
    qp: &mut [u32],
    np: &mut [u32],
    dp: &[u32],
    dinv: u32,
    tp: &mut [u32],
) -> u32 {
    let n = dp.len();
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
    mpn_mul(tp, &qp[lo..lo + hi], &dp[..lo]);
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
        mpn_dcpi1_divappr_q_n(qp, &mut np[hi..], &dp[hi..hi + lo], dinv, tp)
    };
    if ql != 0 {
        for q in qp[..lo].iter_mut() {
            *q = u32::MAX;
        }
    }
    qh
}

// docs preserved
// divide-and-conquer division, returning approximate quotient. The quotient returned is either
// correct, or one too large.
// mpn_dcpi1_divappr_q from mpn/generic/dcpi1_divappr_q.c
pub fn mpn_dcpi1_divappr_q(qp: &mut [u32], np: &mut [u32], dp: &[u32], dinv: u32) -> u32 {
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
                &np[np_offset - dn + 1..np_offset - 1],
                &dp[dp_offset - dn..dp_offset],
            ) >= Ordering::Equal
            {
                1
            } else {
                0
            };
            if qh != 0 {
                assert!(!limbs_sub_same_length_in_place_left(
                    &mut np[np_offset - dn + 1..np_offset + 1],
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
                q = u32::MAX;
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
                if qn > dn - qn {
                    mpn_mul(
                        &mut tp,
                        &qp[qp_offset..qp_offset + qn],
                        &dp[dp_offset - dn..dp_offset - qn],
                    );
                } else {
                    mpn_mul(
                        &mut tp,
                        &dp[dp_offset - dn..dp_offset - qn],
                        &qp[qp_offset..qp_offset + qn],
                    );
                }

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
                &mut np[np_offset - dn..],
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
            &mut np[np_offset - dn..],
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
        // Should we at all check DC_DIVAPPR_Q_THRESHOLD here, or reply on callers not to be silly?
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
        qp[qp_offset..qp_offset + qn].copy_from_slice(&q2p[1..qn + 1]);
    }
    qh
}

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
pub fn mpn_bc_invertappr(ip: &mut [u32], dp: &[u32], xp: &mut [u32]) -> u32 {
    let n = dp.len();
    assert!(n > 0);
    assert!(dp[n - 1].get_highest_bit());

    // Compute a base value of r limbs.
    if n == 1 {
        invert_limb(&mut ip[0], dp[0]);
    } else {
        // n > 1 here
        let mut i = n;
        loop {
            i -= 1;
            xp[i] = u32::MAX;
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

/*
const NPOWS: usize = (size_of::<u32>() > 6 ? 48 : 8*sizeof(mp_size_t)) - LOG2C (INV_NEWTON_THRESHOLD)

// Computes the approximate reciprocal using Newton's iterations (at least one).
//
// Inspired by Algorithm "ApproximateReciprocal", published in "Modern Computer Arithmetic" by
// Richard P. Brent and Paul Zimmermann, algorithm 3.5, page 121 in version 0.4 of the book.
//
// Some adaptations were introduced, to allow product mod B ^ m - 1 and return the value e.
//
// We introduced a correction in such a way that "the value of B ^ {n + h} - T computed at step 8
// cannot exceed B ^ n - 1" (the book reads "2B ^ n - 1").
//
// Maximum scratch needed by this branch <= 2 * n, but have to fit 3 * rn in the scratch, i.e.
// 3 * rn <= 2 * n: we require n > 4.
//
// We use a wrapped product modulo B ^ m - 1. NOTE: is there any normalisation problem for the [0]
// class? It shouldn't: we compute 2 * |A * X_h - B ^ {n + h}| < B ^ m - 1. We may get [0] if and
// only if we get AX_h = B ^ {n + h}. This can happen only if A = B ^ {n} / 2, but this implies
// X_h = B ^ {h} * 2 - 1 i.e., AX_h = B ^ {n + h} - A, then we get into the "negative" branch, where
// X_h is not incremented (because A < B ^ n).
// mpn_ni_invertappr from mpn/generic/invertappr.c
pub fn mpn_ni_invertappr(ip: &mut [u32], dp: &[u32], scratch: &mut [u32]) -> u32
{
    let n = dp.len();
  //mp_limb_t cy;
  //mp_size_t rn, mn;
  //mp_size_t sizes[NPOWS], *sizp;
  //mp_ptr tp;

  assert!(n > 4);
  assert!(dp[n-1].get_highest_bit());

  // Compute the computation precisions from highest to lowest, leaving the base case size in 'rn'.
  sizp = sizes;
  rn = n;
  do {
    *sizp = rn;
    rn = (rn >> 1) + 1;
    ++sizp;
  } while (ABOVE_THRESHOLD (rn, INV_NEWTON_THRESHOLD));

  /* We search the inverse of 0.{dp,n}, we compute it as 1.{ip,n} */
  dp += n;
  ip += n;

  /* Compute a base value of rn limbs. */
  mpn_bc_invertappr (ip - rn, dp - rn, rn, scratch);

  TMP_MARK;

  if (ABOVE_THRESHOLD (n, INV_MULMOD_BNM1_THRESHOLD))
    {
      mn = mpn_mulmod_bnm1_next_size (n + 1);
      tp = TMP_ALLOC_LIMBS (mpn_mulmod_bnm1_itch (mn, n, (n >> 1) + 1));
    }
  /* Use Newton's iterations to get the desired precision.*/

  while (1) {
    n = *--sizp;
    /*
      v    n  v
      +----+--+
      ^ rn ^
    */

    /* Compute i_jd . */
    if (BELOW_THRESHOLD (n, INV_MULMOD_BNM1_THRESHOLD)
    || ((mn = mpn_mulmod_bnm1_next_size (n + 1)) > (n + rn))) {
      /* FIXME: We do only need {xp,n+1}*/
      mpn_mul (xp, dp - n, n, ip - rn, rn);
      mpn_add_n (xp + rn, xp + rn, dp - n, n - rn + 1);
      cy = CNST_LIMB(1); /* Remember we truncated, Mod B^(n+1) */
      /* We computed (truncated) {xp,n+1} <- 1.{ip,rn} * 0.{dp,n} */
    } else { /* Use B^mn-1 wraparound */
      mpn_mulmod_bnm1 (xp, mn, dp - n, n, ip - rn, rn, tp);
      /* We computed {xp,mn} <- {ip,rn} * {dp,n} mod (B^mn-1) */
      /* We know that 2*|ip*dp + dp*B^rn - B^{rn+n}| < B^mn-1 */
      /* Add dp*B^rn mod (B^mn-1) */
      ASSERT (n >= mn - rn);
      cy = mpn_add_n (xp + rn, xp + rn, dp - n, mn - rn);
      cy = mpn_add_nc (xp, xp, dp - (n - (mn - rn)), n - (mn - rn), cy);
      /* Subtract B^{rn+n}, maybe only compensate the carry*/
      xp[mn] = CNST_LIMB (1); /* set a limit for DECR_U */
      MPN_DECR_U (xp + rn + n - mn, 2 * mn + 1 - rn - n, CNST_LIMB (1) - cy);
      MPN_DECR_U (xp, mn, CNST_LIMB (1) - xp[mn]); /* if DECR_U eroded xp[mn] */
      cy = CNST_LIMB(0); /* Remember we are working Mod B^mn-1 */
    }

    if (xp[n] < CNST_LIMB (2)) { /* "positive" residue class */
      cy = xp[n]; /* 0 <= cy <= 1 here. */
      if (cy++ && !mpn_sub_n (xp, xp, dp - n, n)) {
    ASSERT_CARRY (mpn_sub_n (xp, xp, dp - n, n));
    ++cy;
      }
      /* 1 <= cy <= 3 here. */
      if (mpn_cmp (xp, dp - n, n) > 0) {
    ASSERT_NOCARRY (mpn_sub_n (xp, xp, dp - n, n));
    ++cy;
      }
      ASSERT_NOCARRY (mpn_sub_nc (xp + 2 * n - rn, dp - rn, xp + n - rn, rn, mpn_cmp (xp, dp - n, n - rn) > 0));
      MPN_DECR_U(ip - rn, rn, cy); /* 1 <= cy <= 4 here. */
    } else { /* "negative" residue class */
      ASSERT (xp[n] >= GMP_NUMB_MAX - CNST_LIMB(1));
      MPN_DECR_U(xp, n + 1, cy);
      if (xp[n] != GMP_NUMB_MAX) {
    MPN_INCR_U(ip - rn, rn, CNST_LIMB (1));
    ASSERT_CARRY (mpn_add_n (xp, xp, dp - n, n));
      }
      mpn_com (xp + 2 * n - rn, xp + n - rn, rn);
    }

    /* Compute x_ju_j. FIXME:We need {xp+rn,rn}, mulhi? */
    mpn_mul_n (xp, xp + 2 * n - rn, ip - rn, rn);
    cy = mpn_add_n (xp + rn, xp + rn, xp + 2 * n - rn, 2 * rn - n);
    cy = mpn_add_nc (ip - n, xp + 3 * rn - n, xp + n + rn, n - rn, cy);
    MPN_INCR_U (ip - rn, rn, cy);
    if (sizp == sizes) { /* Get out of the cycle */
      /* Check for possible carry propagation from below. */
      cy = xp[3 * rn - n - 1] > GMP_NUMB_MAX - CNST_LIMB (7); /* Be conservative. */
      /*    cy = mpn_add_1 (xp + rn, xp + rn, 2*rn - n, 4); */
      break;
    }
    rn = n;
  }
  TMP_FREE;

  return cy;
}*/

/*
mp_limb_t
mpn_invertappr (mp_ptr ip, mp_srcptr dp, mp_size_t n, mp_ptr scratch)
{
  ASSERT (n > 0);
  ASSERT (dp[n-1] & GMP_NUMB_HIGHBIT);
  ASSERT (! MPN_OVERLAP_P (ip, n, dp, n));
  ASSERT (! MPN_OVERLAP_P (ip, n, scratch, mpn_invertappr_itch(n)));
  ASSERT (! MPN_OVERLAP_P (dp, n, scratch, mpn_invertappr_itch(n)));

  if (BELOW_THRESHOLD (n, INV_NEWTON_THRESHOLD))
    return mpn_bc_invertappr (ip, dp, n, scratch);
  else
    return mpn_ni_invertappr (ip, dp, n, scratch);
} */

/*
pub fn mpn_mu_div_qr2 (qp: &mut [u32],
        rp: &mut [u32],
        np: &[u32],
        dp: &[u32],
        scratch: &mut [u32]) -> u32
{
  //mp_size_t qn, in;
  //mp_limb_t cy, qh;
  //mp_ptr ip, tp;
  let nn = np.len();
  let dn = dp.len();

  assert!(dn > 1);

  let qn = nn - dn;

  /* Compute the inverse size.  */
  let in_size = mpn_mu_div_qr_choose_in (qn, dn, 0);
  assert!(in_size <= dn);
  let (ip, tp) = scratch.split_at_mut(in_size + 1);

  /* compute an approximate inverse on (in+1) limbs */
  if dn == in_size
    {
    tp[1..].copy_from_slice(&dp[..in_size]);
      tp[0] = 1;
      //TODO
      mpn_invertappr(ip, tp, in + 1, tp + in + 1);
      MPN_COPY_INCR (ip, ip + 1, in);
    }
  else
    {
      cy = mpn_add_1 (tp, dp + dn - (in + 1), in + 1, 1);
      if (UNLIKELY (cy != 0))
    MPN_ZERO (ip, in);
      else
    {
      mpn_invertappr (ip, tp, in + 1, tp + in + 1);
      MPN_COPY_INCR (ip, ip + 1, in);
    }
    }

  qh = mpn_preinv_mu_div_qr (qp, rp, np, nn, dp, dn, ip, in, scratch + in);

  return qh;
}*/

/*pub fn mpn_mu_div_qr (qp: &mut [u32],
           rp: &mut [u32],
           np: &[u32],
           dp: &[u32],
           scratch: &mut [u32]) -> u32 {
           let nn = np.len();
           let dn = dp.len();
  //mp_size_t qn;
  //mp_limb_t cy, qh;

  let qn = nn - dn;
  let qh;
  if qn + MU_DIV_QR_SKEW_THRESHOLD < dn
    {
    //TODO
      qh = mpn_mu_div_qr2 (qp, rp + nn - (2 * qn + 1),
               np + nn - (2 * qn + 1), 2 * qn + 1,
               dp + dn - (qn + 1), qn + 1,
               scratch);

      /* Multiply the quotient by the divisor limbs ignored above.  */
      if (dn - (qn + 1) > qn)
    mpn_mul (scratch, dp, dn - (qn + 1), qp, qn);  /* prod is dn-1 limbs */
      else
    mpn_mul (scratch, qp, qn, dp, dn - (qn + 1));  /* prod is dn-1 limbs */

      if (qh)
    cy = mpn_add_n (scratch + qn, scratch + qn, dp, dn - (qn + 1));
      else
    cy = 0;
      scratch[dn - 1] = cy;

      cy = mpn_sub_n (rp, np, scratch, nn - (2 * qn + 1));
      cy = mpn_sub_nc (rp + nn - (2 * qn + 1),
               rp + nn - (2 * qn + 1),
               scratch + nn - (2 * qn + 1),
               qn + 1, cy);
      if (cy)
    {
      qh -= mpn_sub_1 (qp, qp, qn, 1);
      mpn_add_n (rp, rp, dp, dn);
    }
    }
  else
    {
      qh = mpn_mu_div_qr2 (qp, rp, np, nn, dp, dn, scratch);
    }

  return qh;
}*/

/*
pub fn mpn_tdiv_qr (qp: &mut [u32], rp: &mut [u32],
         np: &[u32], dp: &[u32])
{
  let nn = np.size();
  let dn = dp.size();
  assert!(nn >= 0);
  assert!(dn >= 0);
  assert!(dn == 0 || dp[dn - 1] != 0);

  match dn
    {
    0 => panic!("division by zero"),

    1 =>
      {
    rp[0] = limbs_div_limb_to_out_mod(qp, np, dp[0]);
    return;
      }

    2 =>
      {
    //mp_ptr n2p, d2p;
    //mp_limb_t qhl, cy;
    //TMP_DECL;
    //TMP_MARK;
    if !dp[1].get_highest_bit()
      {
        //int cnt;
        //mp_limb_t dtmp[2];
        let cnt = dp[1].leading_zeros();
        let dtmp = vec![0; 2];
        let mut d2p = dtmp;
        d2p[1] = (dp[1] << cnt) | (dp[0] >> (u32::WIDTH - cnt));
        d2p[0] = dp[0] << cnt;
        let mut n2p = vec![0; nn + 1];
        let cy = limbs_shl_to_out(&mut n2p, np, cnt);
        n2p[nn] = cy;
        let qhl = mpn_divrem_2 (qp, &mut n2p, &d2p);
        if cy == 0 {
            qp[nn - 2] = qhl;    /* always store nn-2+1 quotient limbs */
        }
        rp[0] = (n2p[0] >> cnt) | (n2p[1] << (u32::WIDTH - cnt));
        rp[1] = (n2p[1] >> cnt);
      }
    else
      {
        let d2p = dp;
        let mut n2p = vec![0; nn];
        n2p.copy_from_slice(np);
        let qhl = mpn_divrem_2(qp, &mut n2p, d2p);
        qp[nn - 2] = qhl;    /* always store nn-2+1 quotient limbs */
        rp[0] = n2p[0];
        rp[1] = n2p[1];
      }
    return;
      }

    _ => {
    //int adjust;
    //gmp_pi1_t dinv;
    let adjust = if np[nn - 1] >= dp[dn - 1] {1} else {0};    /* conservative tests for quotient size */
    if nn + adjust >= 2 * dn
      {
        //mp_ptr n2p, d2p;
        //mp_limb_t cy;
        //int cnt;

        qp[nn - dn] = 0;              /* zero high quotient limb */
        if !dp[dn - 1].get_highest_bit() /* normalize divisor */
          {
        let cnt = dp[dn - 1].leading_zeros();
        let mut d2p = vec![0; dn];
        limbs_shl_to_out(&mut d2p, dp, cnt);
        let mut n2p = vec![0; nn + 1];
        let cy = limbs_shl_to_out(&mut n2p, np, cnt);
        n2p[nn] = cy;
        nn += adjust;
          }
        else
          {
        let cnt = 0;
        let d2p = dp;
        let mut n2p = vec![0; nn + 1];
        n2p[0..nn].copy_from_slice(np);
        n2p[nn] = 0;
        nn += adjust;
          }

        invert_pi1(&mut dinv, d2p[dn - 1], d2p[dn - 2]);
        if dn < DC_DIV_QR_THRESHOLD {
            mpn_sbpi1_div_qr(qp, &mut n2p[0..nn], d2p, dinv);
        } else if dn < MUPI_DIV_QR_THRESHOLD ||   /* fast condition */
             nn < 2 * MU_DIV_QR_THRESHOLD || /* fast condition */
             (2 * (MU_DIV_QR_THRESHOLD - MUPI_DIV_QR_THRESHOLD)) as f64 * dn /* slow... */
             + MUPI_DIV_QR_THRESHOLD as f64 * nn > dn as f64 * nn {
          /* ...condition */
          mpn_dcpi1_div_qr(qp, &mut n2p[..nn], &d2p[..dn], dinv);
      } else
          {
        let itch = mpn_mu_div_qr_itch (nn, dn, 0);
        let mut scratch = vec![0; itch];
        //TODO
        mpn_mu_div_qr (qp, rp, n2p, nn, d2p, dn, scratch);
        n2p = rp;
          }

        if (cnt != 0)
          mpn_rshift (rp, n2p, dn, cnt);
        else
          MPN_COPY (rp, n2p, dn);
        TMP_FREE;
        return;
      }

    /* When we come here, the numerator/partial remainder is less
       than twice the size of the denominator.  */

      {
        /* Problem:

           Divide a numerator N with nn limbs by a denominator D with dn
           limbs forming a quotient of qn=nn-dn+1 limbs.  When qn is small
           compared to dn, conventional division algorithms perform poorly.
           We want an algorithm that has an expected running time that is
           dependent only on qn.

           Algorithm (very informally stated):

           1) Divide the 2 x qn most significant limbs from the numerator
          by the qn most significant limbs from the denominator.  Call
          the result qest.  This is either the correct quotient, but
          might be 1 or 2 too large.  Compute the remainder from the
          division.  (This step is implemented by an mpn_divrem call.)

           2) Is the most significant limb from the remainder < p, where p
          is the product of the most significant limb from the quotient
          and the next(d)?  (Next(d) denotes the next ignored limb from
          the denominator.)  If it is, decrement qest, and adjust the
          remainder accordingly.

           3) Is the remainder >= qest?  If it is, qest is the desired
          quotient.  The algorithm terminates.

           4) Subtract qest x next(d) from the remainder.  If there is
          borrow out, decrement qest, and adjust the remainder
          accordingly.

           5) Skip one word from the denominator (i.e., let next(d) denote
          the next less significant limb.  */

        mp_size_t qn;
        mp_ptr n2p, d2p;
        mp_ptr tp;
        mp_limb_t cy;
        mp_size_t in, rn;
        mp_limb_t quotient_too_large;
        unsigned int cnt;

        qn = nn - dn;
        qp[qn] = 0;                /* zero high quotient limb */
        qn += adjust;            /* qn cannot become bigger */

        if (qn == 0)
          {
        MPN_COPY (rp, np, dn);
        TMP_FREE;
        return;
          }

        in = dn - qn;        /* (at least partially) ignored # of limbs in ops */
        /* Normalize denominator by shifting it to the left such that its
           most significant bit is set.  Then shift the numerator the same
           amount, to mathematically preserve quotient.  */
        if ((dp[dn - 1] & GMP_NUMB_HIGHBIT) == 0)
          {
        count_leading_zeros (cnt, dp[dn - 1]);
        cnt -= GMP_NAIL_BITS;

        d2p = TMP_ALLOC_LIMBS (qn);
        mpn_lshift (d2p, dp + in, qn, cnt);
        d2p[0] |= dp[in - 1] >> (GMP_NUMB_BITS - cnt);

        n2p = TMP_ALLOC_LIMBS (2 * qn + 1);
        cy = mpn_lshift (n2p, np + nn - 2 * qn, 2 * qn, cnt);
        if (adjust)
          {
            n2p[2 * qn] = cy;
            n2p++;
          }
        else
          {
            n2p[0] |= np[nn - 2 * qn - 1] >> (GMP_NUMB_BITS - cnt);
          }
          }
        else
          {
        cnt = 0;
        d2p = (mp_ptr) dp + in;

        n2p = TMP_ALLOC_LIMBS (2 * qn + 1);
        MPN_COPY (n2p, np + nn - 2 * qn, 2 * qn);
        if (adjust)
          {
            n2p[2 * qn] = 0;
            n2p++;
          }
          }

        /* Get an approximate quotient using the extracted operands.  */
        if (qn == 1)
          {
        mp_limb_t q0, r0;
        udiv_qrnnd (q0, r0, n2p[1], n2p[0] << GMP_NAIL_BITS, d2p[0] << GMP_NAIL_BITS);
        n2p[0] = r0 >> GMP_NAIL_BITS;
        qp[0] = q0;
          }
        else if (qn == 2)
          mpn_divrem_2 (qp, 0L, n2p, 4L, d2p); /* FIXME: obsolete function */
        else
          {
        invert_pi1 (dinv, d2p[qn - 1], d2p[qn - 2]);
        if (BELOW_THRESHOLD (qn, DC_DIV_QR_THRESHOLD))
          mpn_sbpi1_div_qr (qp, n2p, 2 * qn, d2p, qn, dinv.inv32);
        else if (BELOW_THRESHOLD (qn, MU_DIV_QR_THRESHOLD))
          mpn_dcpi1_div_qr (qp, n2p, 2 * qn, d2p, qn, &dinv);
        else
          {
            mp_size_t itch = mpn_mu_div_qr_itch (2 * qn, qn, 0);
            mp_ptr scratch = TMP_ALLOC_LIMBS (itch);
            mp_ptr r2p = rp;
            if (np == r2p)    /* If N and R share space, put ... */
              r2p += nn - qn;    /* intermediate remainder at N's upper end. */
            mpn_mu_div_qr (qp, r2p, n2p, 2 * qn, d2p, qn, scratch);
            MPN_COPY (n2p, r2p, qn);
          }
          }

        rn = qn;
        /* Multiply the first ignored divisor limb by the most significant
           quotient limb.  If that product is > the partial remainder's
           most significant limb, we know the quotient is too large.  This
           test quickly catches most cases where the quotient is too large;
           it catches all cases where the quotient is 2 too large.  */
        {
          mp_limb_t dl, x;
          mp_limb_t h, dummy;

          if (in - 2 < 0)
        dl = 0;
          else
        dl = dp[in - 2];

          x = (dp[in - 1] << cnt) | ((dl >> 1) >> ((~cnt) % GMP_LIMB_BITS));
          umul_ppmm (h, dummy, x, qp[qn - 1] << GMP_NAIL_BITS);

          if (n2p[qn - 1] < h)
        {
          mp_limb_t cy;

          mpn_decr_u (qp, (mp_limb_t) 1);
          cy = mpn_add_n (n2p, n2p, d2p, qn);
          if (cy)
            {
              /* The partial remainder is safely large.  */
              n2p[qn] = cy;
              ++rn;
            }
        }
        }

        quotient_too_large = 0;
        if (cnt != 0)
          {
        mp_limb_t cy1, cy2;

        /* Append partially used numerator limb to partial remainder.  */
        cy1 = mpn_lshift (n2p, n2p, rn, GMP_NUMB_BITS - cnt);
        n2p[0] |= np[in - 1] & (GMP_NUMB_MASK >> cnt);

        /* Update partial remainder with partially used divisor limb.  */
        cy2 = mpn_submul_1 (n2p, qp, qn, dp[in - 1] & (GMP_NUMB_MASK >> cnt));
        if (qn != rn)
          {
            ASSERT_ALWAYS (n2p[qn] >= cy2);
            n2p[qn] -= cy2;
          }
        else
          {
            n2p[qn] = cy1 - cy2; /* & GMP_NUMB_MASK; */

            quotient_too_large = (cy1 < cy2);
            ++rn;
          }
        --in;
          }
        /* True: partial remainder now is neutral, i.e., it is not shifted up.  */

        tp = TMP_ALLOC_LIMBS (dn);

        if (in < qn)
          {
        if (in == 0)
          {
            MPN_COPY (rp, n2p, rn);
            ASSERT_ALWAYS (rn == dn);
            goto foo;
          }
        mpn_mul (tp, qp, qn, dp, in);
          }
        else
          mpn_mul (tp, dp, in, qp, qn);

        cy = mpn_sub (n2p, n2p, rn, tp + in, qn);
        MPN_COPY (rp + in, n2p, dn - in);
        quotient_too_large |= cy;
        cy = mpn_sub_n (rp, np, tp, in);
        cy = mpn_sub_1 (rp + in, rp + in, rn, cy);
        quotient_too_large |= cy;
      foo:
        if (quotient_too_large)
          {
        mpn_decr_u (qp, (mp_limb_t) 1);
        mpn_add_n (rp, rp, dp, dn);
          }
      }
    TMP_FREE;
    return;
      }
    }
}*/
