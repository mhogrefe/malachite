use malachite_base::limbs::{limbs_set_zero, limbs_test_zero};
use malachite_base::num::{NotAssign, PrimitiveInteger};
use natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_add_to_out, limbs_slice_add_greater_in_place_left,
    limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_mul_u32::mpn_addmul_1;
use natural::arithmetic::add_u32::{limbs_add_limb_to_out, limbs_slice_add_limb_in_place};
use natural::arithmetic::mul_u32::mpn_mul_1;
use natural::arithmetic::shl_u32::{mpn_lshift, mpn_lshift_in_place};
use natural::arithmetic::shr_u32::mpn_rshift_in_place;
use natural::arithmetic::sub::{limbs_sub_same_length_to_out, limbs_sub_same_length_in_place_right, limbs_sub_same_length_in_place_left, limbs_sub_to_out};
use natural::arithmetic::sub_u32::limbs_sub_limb_in_place;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::Natural::{self, Large, Small};
use std::cmp::Ordering;
use std::ops::{Mul, MulAssign};

//TODO use better algorithms

const MUL_BASECASE_MAX_UN: usize = 500;
const MUL_TOOM22_THRESHOLD: usize = 30;
const MUL_TOOM33_THRESHOLD: usize = 100;
const MUL_TOOM44_THRESHOLD: usize = 300;
const MUL_TOOM6H_THRESHOLD: usize = 350;
const MUL_TOOM8H_THRESHOLD: usize = 450;
const MUL_TOOM32_TO_TOOM43_THRESHOLD: usize = 100;
const MUL_TOOM32_TO_TOOM53_THRESHOLD: usize = 110;
const MUL_TOOM42_TO_TOOM53_THRESHOLD: usize = 100;
const MUL_TOOM42_TO_TOOM63_THRESHOLD: usize = 110;
const MUL_TOOM33_THRESHOLD_LIMIT: usize = MUL_TOOM33_THRESHOLD;

fn toom44_ok(an: usize, bn: usize) -> bool {
    12 + 3 * an < 4 * bn
}

fn mpn_toom22_mul_itch(an: usize) -> usize {
    2 * (an + u32::WIDTH as usize)
}

fn mpn_toom33_mul_itch(an: usize) -> usize {
    3 * an + u32::WIDTH as usize
}

fn mpn_toom32_mul_itch(an: usize, bn: usize) -> usize {
    let n = 1 + (if 2 * an >= 3 * bn {
        (an - 1) / 3
    } else {
        (bn - 1) >> 1
    });
    2 * n + 1
}

fn mpn_toom42_mul_itch(an: usize, bn: usize) -> usize {
    let n = if an >= 2 * bn {
        (an + 3) >> 2
    } else {
        (bn + 1) >> 1
    };
    6 * n + 3
}

fn mpn_toom43_mul_itch(an: usize, bn: usize) -> usize {
    let n = 1 + (if 3 * an >= 4 * bn {
        (an - 1) >> 2
    } else {
        (bn - 1) / 3
    });
    6 * n + 4
}

fn mpn_toom44_mul_itch(an: usize) -> usize {
    3 * an + u32::WIDTH as usize
}

fn mpn_toom53_mul_itch(an: usize, bn: usize) -> usize {
    let n = 1 + (if 3 * an >= 5 * bn {
        (an - 1) / 5
    } else {
        (bn - 1) / 3
    });
    10 * n + 10
}

fn mpn_toom63_mul_itch(an: usize, bn: usize) -> usize {
    let n = 1 + (if an >= 2 * bn {
        (an - 1) / 6
    } else {
        (bn - 1) / 3
    });
    9 * n + 3
}

fn mpn_toom6h_mul_itch(an: usize, bn: usize) -> usize {
    println!("{}", an);
    println!("{}", bn);
    unimplemented!();
}

fn mpn_toom8h_mul_itch(an: usize, bn: usize) -> usize {
    println!("{}", an);
    println!("{}", bn);
    unimplemented!();
}

fn mpn_toom6_mul_n_itch(an: usize) -> usize {
    println!("{}", an);
    unimplemented!();
}

fn mpn_toom8_mul_n_itch(an: usize) -> usize {
    println!("{}", an);
    unimplemented!();
}

#[allow(unknown_lints, eq_op)]
pub fn mpn_mul_n(p: &mut [u32], a: &[u32], b: &[u32]) {
    let n = a.len();
    assert_eq!(n, b.len());
    assert!(n >= 1);

    if n < MUL_TOOM22_THRESHOLD {
        mpn_mul_basecase(p, a, b);
    } else if n < MUL_TOOM33_THRESHOLD {
        let mut ws = vec![0; mpn_toom22_mul_itch(MUL_TOOM33_THRESHOLD_LIMIT - 1)];
        assert!(MUL_TOOM33_THRESHOLD <= MUL_TOOM33_THRESHOLD_LIMIT);
        mpn_toom22_mul(p, a, b, &mut ws);
    } else if n < MUL_TOOM44_THRESHOLD {
        let mut ws = vec![0; mpn_toom33_mul_itch(n)];
        mpn_toom33_mul(p, a, b, &mut ws);
    } else if n < MUL_TOOM6H_THRESHOLD {
        let mut ws = vec![0; mpn_toom44_mul_itch(n)];
        mpn_toom44_mul(p, a, b, &mut ws);
    } else if n < MUL_TOOM8H_THRESHOLD {
        let mut ws = vec![0; mpn_toom6_mul_n_itch(n)];
        mpn_toom6h_mul(p, a, b, &mut ws);
    } else {
        let mut ws = vec![0; mpn_toom8_mul_n_itch(n)];
        mpn_toom8h_mul(p, a, b, &mut ws);
    }
}

fn toom22_mul_n_rec(p: &mut [u32], a: &[u32], b: &[u32], ws: &mut [u32]) {
    let a_len = a.len();
    assert_eq!(a_len, b.len());
    if a_len < MUL_TOOM22_THRESHOLD {
        mpn_mul_basecase(p, a, b);
    } else {
        mpn_toom22_mul(p, a, b, ws);
    }
}

// Normally, this calls mul_basecase or toom22_mul. But when when the fraction
// MUL_TOOM33_THRESHOLD / MUL_TOOM22_THRESHOLD is large, an initially small
// relative unbalance will become a larger and larger relative unbalance with
// each recursion (the difference s-t will be invariant over recursive calls).
// Therefore, we need to call toom32_mul.
fn toom22_mul_rec(p: &mut [u32], a: &[u32], b: &[u32], ws: &mut [u32]) {
    if b.len() < MUL_TOOM22_THRESHOLD {
        mpn_mul_basecase(p, a, b);
    } else if 4 * a.len() < 5 * b.len() {
        mpn_toom22_mul(p, a, b, ws);
    } else {
        mpn_toom32_mul(p, a, b, ws);
    }
}

// Evaluate in: -1, 0, +inf

// <-s--><--n-->
//  ____ ______
// |_a1_|___a0_|
//  |b1_|___b0_|
//  <-t-><--n-->

// v0  =  a0     * b0       #   A(0)*B(0)
// vm1 = (a0- a1)*(b0- b1)  #  A(-1)*B(-1)
// vinf=      a1 *     b1   # A(inf)*B(inf)
#[allow(unknown_lints, many_single_char_names)]
fn mpn_toom22_mul(p: &mut [u32], a: &[u32], b: &[u32], scratch: &mut [u32]) {
    let an = a.len();
    let bn = b.len();
    let s = an >> 1;
    let n = an - s;
    let t = bn - n;

    assert!(an >= bn);
    assert!(0 < s && s <= n && s >= n - 1);
    assert!(0 < t && t <= s);

    let mut vm1_neg = false;

    // Compute asm1.
    if s == n {
        if limbs_cmp_same_length(&a[..n], &a[n..2 * n]) == Ordering::Less {
            limbs_sub_same_length_to_out(p, &a[n..2 * n], &a[..n]);
            vm1_neg = true;
        } else {
            limbs_sub_same_length_to_out(p, &a[..n], &a[n..2 * n]);
        }
    } else if a[s] == 0 && limbs_cmp_same_length(&a[..s], &a[n..n + s]) == Ordering::Less {
        // n - s == 1
        limbs_sub_same_length_to_out(p, &a[n..n + s], &a[..s]);
        p[s] = 0;
        vm1_neg = true;
    } else {
        p[s] = a[s] - if limbs_sub_same_length_to_out(p, &a[..s], &a[n..n + s]) {
            1
        } else {
            0
        };
    }

    // Compute bsm1.
    if t == n {
        if limbs_cmp_same_length(&b[..n], &b[n..2 * n]) == Ordering::Less {
            limbs_sub_same_length_to_out(&mut p[n..], &b[n..2 * n], &b[..n]);
            vm1_neg.not_assign();
        } else {
            limbs_sub_same_length_to_out(&mut p[n..], &b[..n], &b[n..2 * n]);
        }
    } else if limbs_test_zero(&b[t..n])
        && limbs_cmp_same_length(&b[..t], &b[n..n + t]) == Ordering::Less
    {
        limbs_sub_same_length_to_out(&mut p[n..], &b[n..n + t], &b[..t]);
        limbs_set_zero(&mut p[n + t..2 * n]);
        vm1_neg.not_assign();
    } else {
        limbs_sub_to_out(&mut p[n..], &b[..n], &b[n..n + t]);
    }

    // vm1, 2n limbs
    {
        let (p1, p2) = p.split_at_mut(n);
        let (scratch1, scratch2) = scratch.split_at_mut(2 * n);
        toom22_mul_n_rec(scratch1, &p1[..n], &p2[..n], scratch2);

        if s > t {
            toom22_mul_rec(&mut p2[n..], &a[n..n + s], &b[n..n + t], scratch2);
        } else {
            toom22_mul_n_rec(&mut p2[n..], &a[n..n + s], &b[n..n + s], scratch2);
        }
    }

    // v0, 2n limbs
    toom22_mul_n_rec(p, &a[..n], &b[..n], &mut scratch[2 * n..]);

    // H(v0) + L(vinf)
    let mut cy = {
        let (p1, p2) = p.split_at_mut(2 * n);
        if limbs_slice_add_same_length_in_place_left(&mut p2[..n], &p1[n..2 * n]) {
            1
        } else {
            0
        }
    };

    // L(v0) + H(v0)
    let cy2 = {
        let (p1, p23) = p.split_at_mut(n);
        let (p2, p3) = p23.split_at_mut(n);
        cy + if limbs_add_same_length_to_out(p2, p3, p1) {
            1
        } else {
            0
        }
    };

    // L(vinf) + H(vinf)
    {
        let (p1, p2) = p.split_at_mut(3 * n);
        cy += if limbs_slice_add_greater_in_place_left(&mut p1[2 * n..3 * n], &p2[..s + t - n]) {
            1
        } else {
            0
        };
    }

    if vm1_neg {
        cy += if limbs_slice_add_same_length_in_place_left(&mut p[n..3 * n], &scratch[..2 * n]) {
            1
        } else {
            0
        };
    } else {
        cy -= if limbs_sub_same_length_in_place_left(&mut p[n..3 * n], &scratch[..2 * n]) {
            1
        } else {
            0
        };
    }

    assert!(cy < 3);
    assert!(cy2 <= 2);

    limbs_slice_add_limb_in_place(&mut p[2 * n..2 * n + s + t], cy2);
    if cy <= 2 {
        // if s+t==n, cy is zero, but we should not access pp[3*n] at all.
        limbs_slice_add_limb_in_place(&mut p[3 * n..2 * n + s + t], cy);
    } else {
        limbs_sub_limb_in_place(&mut p[3 * n..2 * n + s + t], 1);
    }
}

fn mpn_toom32_mul(prod: &mut [u32], u: &[u32], v: &[u32], scratch: &mut [u32]) {
    println!("{}", prod.len());
    println!("{}", u.len());
    println!("{}", v.len());
    println!("{}", scratch.len());
    unimplemented!();
}

fn mpn_toom33_mul(prod: &mut [u32], u: &[u32], v: &[u32], scratch: &mut [u32]) {
    println!("{}", prod.len());
    println!("{}", u.len());
    println!("{}", v.len());
    println!("{}", scratch.len());
    unimplemented!();
}

// Evaluate in: -1, 0, +1, +2, +inf
//
// <-s-><--n--><--n--><--n-->
//  ___ ______ ______ ______
// |a3_|___a2_|___a1_|___a0_|
//       |_b1_|___b0_|
//       <-t--><--n-->
//
// v0  =  a0             * b0      #   A(0)*B(0)
// v1  = (a0+ a1+ a2+ a3)*(b0+ b1) #   A(1)*B(1)      ah  <= 3  bh <= 1
// vm1 = (a0- a1+ a2- a3)*(b0- b1) #  A(-1)*B(-1)    |ah| <= 1  bh  = 0
// v2  = (a0+2a1+4a2+8a3)*(b0+2b1) #   A(2)*B(2)      ah  <= 14 bh <= 2
// vinf=              a3 *     b1  # A(inf)*B(inf)
fn toom42_mul_n_rec(p: &mut [u32], a: &[u32], b: &[u32]) {
    assert_eq!(a.len(), b.len());
    mpn_mul_n(p, a, b);
}

// Evaluate a degree 3 polynomial in +1 and -1
fn mpn_toom_eval_dgr3_pm1(
    xp1: &mut [u32],
    xm1: &mut [u32],
    x: &[u32],
    n: usize,
    x3n: usize,
    t: &mut [u32],
) -> bool {
    assert!(x3n > 0);
    assert!(x3n <= n);

    xp1[n] = if limbs_add_same_length_to_out(xp1, &x[..n], &x[2 * n..3 * n]) {
        1
    } else {
        0
    };
    t[n] = if limbs_add_to_out(t, &x[n..2 * n], &x[3 * n..x3n + 3 * n]) {
        1
    } else {
        0
    };

    let neg = limbs_cmp_same_length(&xp1[..n + 1], &t[..n + 1]) == Ordering::Less;

    if neg {
        limbs_sub_same_length_to_out(xm1, &t[..n + 1], &xp1[..n + 1]);
    } else {
        limbs_sub_same_length_to_out(xm1, &xp1[..n + 1], &t[..n + 1]);
    }

    limbs_slice_add_same_length_in_place_left(&mut xp1[..n + 1], &t[..n + 1]);

    assert!(xp1[n] <= 3);
    assert!(xm1[n] <= 1);
    neg
}

//TODO move to a better location
fn mpn_divexact_by3_in_place(p: &mut [u32]) -> u32 {
    println!("{}", p.len());
    unimplemented!();
}

fn mpn_toom_interpolate_5pts(
    c: &mut [u32],
    v2: &mut [u32],
    vm1: &mut [u32],
    k: usize,
    twor: usize,
    sa: bool,
    mut vinf0: u32,
) {
    //mp_limb_t cy, saved;
    //mp_size_t twok;
    //mp_size_t kk1;
    //mp_ptr c1, v1, c3, vinf;

    let twok = k + k;
    let kk1 = twok + 1;

    //c1 = c  + k;
    //v1 = c1 + k; c + 2*k
    //c3 = v1 + k; c + 3*k
    //vinf = c3 + k; c + 4*k

    // (1) v2 <- v2-vm1 < v2+|vm1|,       (16 8 4 2 1) - (1 -1 1 -1  1) =
    // thus 0 <= v2 < 50*B^(2k) < 2^6*B^(2k)             (15 9 3  3  0)
    if sa {
        assert!(!limbs_slice_add_same_length_in_place_left(
            &mut v2[..kk1],
            &vm1[..kk1]
        ));
    } else {
        assert!(!limbs_sub_same_length_in_place_left(&mut v2[..kk1], &vm1[..kk1]));
    }

    // {c,2k} {c+2k,2k+1} {c+4k+1,2r-1} {t,2k+1} {t+2k+1,2k+1} {t+4k+2,2r}
    //     v0       v1       hi(vinf)       |vm1|     v2-vm1      EMPTY

    assert_eq!(mpn_divexact_by3_in_place(&mut v2[..kk1]), 0); // v2 <- v2 / 3
                                                              // (5 3 1 1 0)

    // {c,2k} {c+2k,2k+1} {c+4k+1,2r-1} {t,2k+1} {t+2k+1,2k+1} {t+4k+2,2r}
    //    v0       v1      hi(vinf)       |vm1|     (v2-vm1)/3    EMPTY

    // (2) vm1 <- tm1 := (v1 - vm1) / 2  [(1 1 1 1 1) - (1 -1 1 -1 1)] / 2 =
    // tm1 >= 0                                         (0  1 0  1 0)
    // No carry comes out from {v1, kk1} +/- {vm1, kk1},
    // and the division by two is exact.
    // If (sa!=0) the sign of vm1 is negative
    if sa {
        assert!(!limbs_slice_add_same_length_in_place_left(
            &mut vm1[..kk1],
            &c[k..k + kk1]
        ));
        assert_eq!(mpn_rshift_in_place(&mut vm1[..kk1], 1), 0);
    } else {
        assert!(!limbs_sub_same_length_in_place_right(&c[k..k + kk1], &mut vm1[..kk1]));
        assert_eq!(mpn_rshift_in_place(&mut vm1[..kk1], 1), 0);
    }

    // {c,2k} {c+2k,2k+1} {c+4k+1,2r-1} {t,2k+1} {t+2k+1,2k+1} {t+4k+2,2r}
    //   v0       v1        hi(vinf)       tm1     (v2-vm1)/3    EMPTY

    // (3) v1 <- t1 := v1 - v0    (1 1 1 1 1) - (0 0 0 0 1) = (1 1 1 1 0)
    // t1 >= 0
    let carry = {
        let (c1, c2) = c.split_at_mut(twok);
        if limbs_sub_same_length_in_place_left(&mut c2[..twok], c1) {
            1
        } else {
            0
        }
    };
    c[4 * k] -= carry;

    // {c,2k} {c+2k,2k+1} {c+4k+1,2r-1} {t,2k+1} {t+2k+1,2k+1} {t+4k+2,2r}
    //   v0     v1-v0        hi(vinf)       tm1     (v2-vm1)/3    EMPTY
    //
    // (4) v2 <- t2 := ((v2-vm1)/3-t1)/2 = (v2-vm1-3*t1)/6
    // t2 >= 0                  [(5 3 1 1 0) - (1 1 1 1 0)]/2 = (2 1 0 0 0)
    assert!(!limbs_sub_same_length_in_place_left(&mut v2[..kk1], &c[2 * k..2 * k + kk1]));
    assert_eq!(mpn_rshift_in_place(&mut v2[..kk1], 1), 0);

    // {c,2k} {c+2k,2k+1} {c+4k+1,2r-1} {t,2k+1} {t+2k+1,2k+1} {t+4k+2,2r}
    //   v0     v1-v0        hi(vinf)     tm1    (v2-vm1-3t1)/6    EMPTY
    //
    // (5) v1 <- t1-tm1           (1 1 1 1 0) - (0 1 0 1 0) = (1 0 1 0 0)
    // result is v1 >= 0
    assert!(!limbs_sub_same_length_in_place_left(&mut c[2 * k..], &vm1[..kk1]));

    // We do not need to read the value in vm1, so we add it in {c+k, ...}
    let mut cy = if limbs_slice_add_same_length_in_place_left(&mut c[k..], &vm1[..kk1]) {
        1
    } else {
        0
    };
    limbs_slice_add_limb_in_place(&mut c[3 * k + 1..4 * k + twor], cy); // 2n-(3k+1) = 2r+k-1
                                                                        // Memory allocated for vm1 is now free, it can be recycled ...

    // (6) v2 <- v2 - 2*vinf,     (2 1 0 0 0) - 2*(1 0 0 0 0) = (0 1 0 0 0)
    // result is v2 >= 0
    let saved = c[4 * k]; // Remember v1's highest byte (will be overwritten).
    c[4 * k] = vinf0; // Set the right value for vinf0
                      // Overwrite unused vm1
    cy = mpn_lshift(vm1, &c[4 * k..4 * k + twor], 1);
    cy += if limbs_sub_same_length_in_place_left(&mut v2[..twor], &vm1[..twor]) {
        1
    } else {
        0
    };
    limbs_sub_limb_in_place(&mut v2[twor..kk1], cy);

    // Current matrix is
    // [1 0 0 0 0; vinf
    //  0 1 0 0 0; v2
    //  1 0 1 0 0; v1
    //  0 1 0 1 0; vm1
    //  0 0 0 0 1] v0
    // Some values already are in-place (we added vm1 in the correct position)
    // | vinf|  v1 |  v0 |
    //  | vm1 |
    // One still is in a separated area
    // | +v2 |
    // We have to compute v1-=vinf; vm1 -= v2,
    // |-vinf|
    //  | -v2 |
    // Carefully reordering operations we can avoid to compute twice the sum
    // of the high half of v2 plus the low half of vinf.

    // Add the high half of t2 in {vinf}
    if twor > k + 1 {
        // This is the expected flow
        cy = if limbs_slice_add_same_length_in_place_left(
            &mut c[4 * k..5 * k + 1],
            &v2[k..2 * k + 1],
        ) {
            1
        } else {
            0
        };
        limbs_slice_add_limb_in_place(&mut c[3 * k + kk1..2 * k + kk1 + twor - 1], cy); // 2n-(5k+1) = 2r-k-1
    } else {
        // triggered only by very unbalanced cases like
        //   (k+k+(k-2))x(k+k+1) , should be handled by toom32
        assert!(!limbs_slice_add_same_length_in_place_left(
            &mut c[4 * k..4 * k + twor],
            &v2[k..k + twor],
        ));
    }
    // (7) v1 <- v1 - vinf,       (1 0 1 0 0) - (1 0 0 0 0) = (0 0 1 0 0)
    // result is >= 0
    // Side effect: we also subtracted (high half) vm1 -= v2
    assert!(twor <= 2 * k);
    cy = {
        let (c1, c2) = c.split_at_mut(2 * k + twor);
        // vinf is at most twor long.
        if limbs_sub_same_length_in_place_left(&mut c1[2 * k..], &c2[2 * k - twor..2 * k]) {
            1
        } else {
            0
        }
    };
    vinf0 = c[4 * k]; // Save again the right value for vinf0
    c[4 * k] = saved;
    limbs_sub_limb_in_place(&mut c[2 * k + twor..2 * k + kk1], cy); // Treat the last bytes.

    // (8) vm1 <- vm1-v2          (0 1 0 1 0) - (0 1 0 0 0) = (0 0 0 1 0)
    // Operate only on the low half.

    cy = if limbs_sub_same_length_in_place_left(&mut c[k..2 * k], &v2[..k]) {
        1
    } else {
        0
    };
    limbs_sub_limb_in_place(&mut c[2 * k..2 * k + kk1], cy);

    //********************* Beginning the final phase **********************

    // Most of the recomposition was done

    // add t2 in {c+3k, ...}, but only the low half
    cy = if limbs_slice_add_same_length_in_place_left(&mut c[3 * k..4 * k], &v2[..k]) {
        1
    } else {
        0
    };
    c[4 * k] += cy;
    assert!(c[4 * k] >= cy); // No carry
    limbs_slice_add_limb_in_place(&mut c[4 * k..4 * k + twor], vinf0); // Add vinf0, propagate carry.
}

#[allow(unknown_lints, cyclomatic_complexity, many_single_char_names)]
fn mpn_toom42_mul(p: &mut [u32], a: &[u32], b: &[u32], scratch: &mut [u32]) {
    //  mp_size_t n, s, t;
    //int vm1_neg;
    //mp_limb_t cy, vinf0;
    //mp_ptr a0_a2;
    //mp_ptr as1, asm1, as2;
    //mp_ptr bs1, bsm1, bs2;
    //mp_ptr tmp;

    let an = a.len();
    let bn = b.len();
    let n = if an >= 2 * bn {
        (an + 3) >> 2
    } else {
        (bn + 1) >> 1
    };
    let a0 = &a;
    let a1 = &a[n..];
    let a2 = &a[2 * n..];
    let a3 = &a[3 * n..];
    let b0 = &b;
    let b1 = &b[n..];

    let s = an - 3 * n;
    let t = bn - n;

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);

    let mut tmp = vec![0; 6 * n + 5];
    let (as1_asm1_as2, bs1_bsm1_bs2) = tmp.split_at_mut(3 * n + 3);
    let (as1, asm1_as2) = as1_asm1_as2.split_at_mut(n + 1);
    let (asm1, as2) = asm1_as2.split_at_mut(n + 1);
    let (bs1, bsm1_bs2) = bs1_bsm1_bs2.split_at_mut(n + 1);
    let (bsm1, bs2) = bsm1_bs2.split_at_mut(n);

    // Compute as1 and asm1.
    let mut vm1_neg = mpn_toom_eval_dgr3_pm1(as1, asm1, a, n, s, p);

    // Compute as2.
    let mut cy = mpn_lshift(as2, &a3[..s], 1);
    cy += if limbs_slice_add_same_length_in_place_left(&mut as2[..s], &a2[..s]) {
        1
    } else {
        0
    };
    if s != n {
        cy = if limbs_add_limb_to_out(&mut as2[s..], &a2[s..n], cy) {
            1
        } else {
            0
        };
    }
    cy = 2 * cy + mpn_lshift_in_place(&mut as2[..n], 1);
    cy += if limbs_slice_add_same_length_in_place_left(&mut as2[..n], &a1[..n]) {
        1
    } else {
        0
    };
    cy = 2 * cy + mpn_lshift_in_place(&mut as2[..n], 1);
    cy += if limbs_slice_add_same_length_in_place_left(&mut as2[..n], &a0[..n]) {
        1
    } else {
        0
    };
    as2[n] = cy;

    // Compute bs1 and bsm1.
    if t == n {
        bs1[n] = if limbs_add_same_length_to_out(bs1, &b0[..n], &b1[..n]) {
            1
        } else {
            0
        };

        if limbs_cmp_same_length(&b0[..n], &b1[..n]) == Ordering::Less {
            limbs_sub_same_length_to_out(bsm1, &b1[..n], &b0[..n]);
            vm1_neg.not_assign();
        } else {
            limbs_sub_same_length_to_out(bsm1, &b0[..n], &b1[..n]);
        }
    } else {
        bs1[n] = if limbs_add_to_out(bs1, &b0[..n], &b1[..t]) {
            1
        } else {
            0
        };

        if limbs_test_zero(&b0[t..n]) && limbs_cmp_same_length(&b0[..t], &b1[..t]) == Ordering::Less
        {
            limbs_sub_same_length_to_out(bsm1, &b1[..t], &b0[..t]);
            limbs_set_zero(&mut bsm1[t..n]);
            vm1_neg.not_assign();
        } else {
            limbs_sub_to_out(bsm1, &b0[..n], &b1[..t]);
        }
    }

    // Compute bs2, recycling bs1. bs2=bs1+b1
    limbs_add_to_out(bs2, &bs1[..n + 1], &b1[..t]);

    assert!(as1[n] <= 3);
    assert!(bs1[n] <= 1);
    assert!(asm1[n] <= 1);
    assert!(as2[n] <= 14);
    assert!(bs2[n] <= 2);

    let (vm1, v2) = scratch.split_at_mut(2 * n + 1);
    let vinf0 = {
        let (v0, v1_vinf) = p.split_at_mut(2 * n);
        //let (v1, vinf) = v1_vinf.split_at_mut(2 * n);

        // vm1, 2n+1 limbs
        toom42_mul_n_rec(vm1, &asm1[..n], &bsm1[..n]);
        cy = 0;
        if asm1[n] != 0 {
            cy = if limbs_slice_add_same_length_in_place_left(&mut vm1[n..2 * n], &bsm1[..n]) {
                1
            } else {
                0
            };
        }
        vm1[2 * n] = cy;

        toom42_mul_n_rec(v2, &as2[..n + 1], &bs2[..n + 1]); // v2, 2n+1 limbs

        // vinf, s+t limbs
        if s > t {
            mpn_mul(&mut v1_vinf[2 * n..], &a3[..s], &b1[..t]);
        } else {
            mpn_mul(&mut v1_vinf[2 * n..], &b1[..t], &a3[..s]);
        }

        let vinf0 = v1_vinf[2 * n]; // v1 overlaps with this

        // v1, 2n+1 limbs
        toom42_mul_n_rec(v1_vinf, &as1[..n], &bs1[..n]);
        if as1[n] == 1 {
            cy = bs1[n]
                + if limbs_slice_add_same_length_in_place_left(&mut v1_vinf[n..2 * n], &bs1[..n]) {
                    1
                } else {
                    0
                };
        } else if as1[n] == 2 {
            cy = 2 * bs1[n] + mpn_addmul_1(&mut v1_vinf[n..2 * n], &bs1[..n], 2);
        } else if as1[n] == 3 {
            cy = 3 * bs1[n] + mpn_addmul_1(&mut v1_vinf[n..2 * n], &bs1[..n], 3);
        } else {
            cy = 0;
        }

        if bs1[n] != 0 {
            cy += if limbs_slice_add_same_length_in_place_left(&mut v1_vinf[n..2 * n], &as1[..n]) {
                1
            } else {
                0
            };
        }
        v1_vinf[2 * n] = cy;
        toom42_mul_n_rec(v0, &a[..n], &b[..n]); // v0, 2n limbs
        vinf0
    };
    mpn_toom_interpolate_5pts(p, v2, vm1, n, s + t, vm1_neg, vinf0);
}

fn mpn_toom43_mul(prod: &mut [u32], u: &[u32], v: &[u32], scratch: &mut [u32]) {
    println!("{}", prod.len());
    println!("{}", u.len());
    println!("{}", v.len());
    println!("{}", scratch.len());
    unimplemented!();
}

fn mpn_toom44_mul(prod: &mut [u32], u: &[u32], v: &[u32], scratch: &mut [u32]) {
    println!("{}", prod.len());
    println!("{}", u.len());
    println!("{}", v.len());
    println!("{}", scratch.len());
    unimplemented!();
}

fn mpn_toom53_mul(prod: &mut [u32], u: &[u32], v: &[u32], scratch: &mut [u32]) {
    println!("{}", prod.len());
    println!("{}", u.len());
    println!("{}", v.len());
    println!("{}", scratch.len());
    unimplemented!();
}

fn mpn_toom63_mul(prod: &mut [u32], u: &[u32], v: &[u32], scratch: &mut [u32]) {
    println!("{}", prod.len());
    println!("{}", u.len());
    println!("{}", v.len());
    println!("{}", scratch.len());
    unimplemented!();
}

fn mpn_toom6h_mul(prod: &mut [u32], u: &[u32], v: &[u32], scratch: &mut [u32]) {
    println!("{}", prod.len());
    println!("{}", u.len());
    println!("{}", v.len());
    println!("{}", scratch.len());
    unimplemented!();
}

fn mpn_toom8h_mul(prod: &mut [u32], u: &[u32], v: &[u32], scratch: &mut [u32]) {
    println!("{}", prod.len());
    println!("{}", u.len());
    println!("{}", v.len());
    println!("{}", scratch.len());
    unimplemented!();
}

// Multiply u by v and write the result to prod. Must have u.len() >= v.len(). prod must be 0
// initially.
//
// Note that prod gets u.len() + v.len() limbs stored, even if the actual result only needs u.len +
// v.len() - 1.
//
// There's no good reason to call here with vsize >= MUL_TOOM22_THRESHOLD. Currently this is
// allowed, but it might not be in the future.
//
// This is the most critical code for multiplication. All multiplies rely on this, both small and
// huge. Small ones arrive here immediately, huge ones arrive here as this is the base case for
// Karatsuba's recursive algorithm.
fn mpn_mul_basecase(prod: &mut [u32], u: &[u32], v: &[u32]) {
    let u_len = u.len();
    assert!(u_len >= v.len());
    let carry = mpn_mul_1(prod, u, v[0]);
    if carry != 0 {
        prod[u_len] = carry;
    }
    for (i, y) in v.iter().enumerate().skip(1) {
        if *y != 0 {
            let carry = mpn_addmul_1(&mut prod[i..], u, *y);
            if carry != 0 {
                prod[u_len + i] = carry;
            }
        }
    }
}

// 1 < v.len() < MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN < u.len()
//
// This is currently not measurably better than just basecase.
fn mpn_mul_basecase_mem_opt_helper(prod: &mut [u32], u: &[u32], v: &[u32]) {
    let u_len = u.len();
    let v_len = v.len();
    assert!(1 < v_len);
    assert!(v_len < MUL_TOOM22_THRESHOLD);
    assert!(MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN);
    assert!(MUL_BASECASE_MAX_UN < u_len);
    let mut triangle_buffer = [0; MUL_TOOM22_THRESHOLD];
    let mut offset = 0;
    for chunk in u.chunks(MUL_BASECASE_MAX_UN) {
        if chunk.len() >= v_len {
            mpn_mul(&mut prod[offset..], chunk, v);
        } else {
            mpn_mul(&mut prod[offset..], v, chunk);
        }
        if offset != 0 {
            limbs_slice_add_greater_in_place_left(&mut prod[offset..], &triangle_buffer[..v_len]);
        }
        offset += MUL_BASECASE_MAX_UN;
        if offset < u_len {
            triangle_buffer[..v_len].copy_from_slice(&prod[offset..offset + v_len]);
        }
    }
}

fn mpn_mul_basecase_mem_opt(prod: &mut [u32], u: &[u32], v: &[u32]) {
    let u_len = u.len();
    let v_len = v.len();
    assert!(u_len >= v_len);
    if v_len > 1 && v_len < MUL_TOOM22_THRESHOLD && u.len() > MUL_BASECASE_MAX_UN {
        mpn_mul_basecase_mem_opt_helper(prod, u, v)
    } else {
        mpn_mul_basecase(prod, u, v)
    }
}

pub fn mpn_mul(prod: &mut [u32], u: &[u32], v: &[u32]) -> u32 {
    let un = u.len();
    let vn = v.len();
    assert!(un >= vn);
    mpn_mul_basecase(prod, u, v);
    prod[un + vn - 1]
}

// Multiply s1 and s2, and write the (s1.len() + s2.len()) - limb result to r. Return the most
// significant limb of the result. The destination has to have space for s1.len() + s2.len() limbs,
// even if the product’s most significant limb is zero. s1.len() >= s2.len(), vn.len() >= 1.
#[allow(unknown_lints, cyclomatic_complexity)]
pub fn mpn_mul_unfinished(prod: &mut [u32], u: &[u32], v: &[u32]) -> u32 {
    let mut un = u.len();
    let vn = v.len();
    assert!(un >= vn);
    assert!(vn > 1);
    if un == vn {
        mpn_mul_n(prod, u, v);
    } else if vn < MUL_TOOM22_THRESHOLD {
        // plain schoolbook multiplication
        mpn_mul_basecase(prod, u, v);
    } else if vn < MUL_TOOM33_THRESHOLD {
        // Use ToomX2 variants

        let itch_toomx2 = 9 * vn / 2 + u32::WIDTH as usize * 2;
        let mut scratch = vec![0; itch_toomx2];
        assert!(mpn_toom22_mul_itch((5 * vn - 1) / 4) <= itch_toomx2); // 5vn/2+
        assert!(mpn_toom32_mul_itch((7 * vn - 1) / 4, vn) <= itch_toomx2); // 7vn/6+
        assert!(mpn_toom42_mul_itch(3 * vn - 1, vn) <= itch_toomx2); // 9vn/2+

        if un >= 3 * vn {
            //mp_limb_t cy;

            // The maximum ws usage is for the mpn_mul result.
            let mut ws = vec![0; 4 * vn];

            mpn_toom42_mul(prod, &u[..2 * vn], v, &mut scratch);
            un -= 2 * vn;
            let mut u_offset = 0;
            let mut prod_offset = 0;
            u_offset += 2 * vn;
            prod_offset += 2 * vn;

            while un >= 3 * vn {
                mpn_toom42_mul(&mut ws, &u[u_offset..u_offset + 2 * vn], v, &mut scratch);
                un -= 2 * vn;
                u_offset += 2 * vn;
                let cy = limbs_slice_add_same_length_in_place_left(
                    &mut prod[prod_offset..prod_offset + vn],
                    &ws[..vn],
                );
                prod[prod_offset + vn..prod_offset + 3 * vn].copy_from_slice(&ws[vn..3 + vn]);
                if cy {
                    limbs_slice_add_limb_in_place(&mut prod[prod_offset..prod_offset + vn], 1);
                }
                prod_offset += 2 * vn;
            }

            // vn <= un < 3vn

            if 4 * un < 5 * vn {
                mpn_toom22_mul(&mut ws, &u[u_offset..u_offset + un], v, &mut scratch);
            } else if 4 * un < 7 * vn {
                mpn_toom32_mul(&mut ws, &u[u_offset..u_offset + un], v, &mut scratch);
            } else {
                mpn_toom42_mul(&mut ws, &u[u_offset..u_offset + un], v, &mut scratch);
            }

            let cy = limbs_slice_add_same_length_in_place_left(
                &mut prod[prod_offset..prod_offset + vn],
                &ws[..vn],
            );
            prod[prod_offset + vn..prod_offset + un + vn].copy_from_slice(&ws[vn..un + vn]);
            if cy {
                limbs_slice_add_limb_in_place(
                    &mut prod[prod_offset + vn..prod_offset + un + vn],
                    1,
                );
            }
        } else if 4 * un < 5 * vn {
            mpn_toom22_mul(prod, u, v, &mut scratch);
        } else if 4 * un < 7 * vn {
            mpn_toom32_mul(prod, u, v, &mut scratch);
        } else {
            mpn_toom42_mul(prod, u, v, &mut scratch);
        }
    // Handle the largest operands that are not in the FFT range.  The 2nd
    // condition makes very unbalanced operands avoid the FFT code (except
    // perhaps as coefficient products of the Toom code.
    } else if vn < MUL_TOOM44_THRESHOLD || !toom44_ok(un, vn) {
        // Use ToomX3 variants

        let itch_toomx3 = 4 * vn + u32::WIDTH as usize;
        let mut scratch = vec![0; itch_toomx3];
        assert!(mpn_toom33_mul_itch((7 * vn - 1) / 6) <= itch_toomx3); // 7vn/2+
        assert!(mpn_toom43_mul_itch((3 * vn - 1) / 2, vn) <= itch_toomx3); // 9vn/4+
        assert!(mpn_toom32_mul_itch((7 * vn - 1) / 4, vn) <= itch_toomx3); // 7vn/6+
        assert!(mpn_toom53_mul_itch((11 * vn - 1) / 6, vn) <= itch_toomx3); // 11vn/3+
        assert!(mpn_toom42_mul_itch((5 * vn - 1) / 2, vn) <= itch_toomx3); // 15vn/4+
        assert!(mpn_toom63_mul_itch((5 * vn - 1) / 2, vn) <= itch_toomx3); // 15vn/4+

        if 2 * un >= 5 * vn {
            //mp_limb_t cy;
            //mp_ptr ws;

            // The maximum ws usage is for the mpn_mul result.
            let mut ws = vec![0; (7 * vn) >> 1];

            if vn < MUL_TOOM42_TO_TOOM63_THRESHOLD {
                mpn_toom42_mul(prod, &u[..2 * vn], v, &mut scratch);
            } else {
                mpn_toom63_mul(prod, &u[..2 * vn], v, &mut scratch);
            }
            un -= 2 * vn;
            let mut u_offset = 0;
            u_offset += 2 * vn;
            let mut prod_offset = 0;
            prod_offset += 2 * vn;

            while 2 * un >= 5 * vn
            // un >= 2.5vn
            {
                if vn < MUL_TOOM42_TO_TOOM63_THRESHOLD {
                    mpn_toom42_mul(&mut ws, &u[u_offset..u_offset + 2 * vn], v, &mut scratch);
                } else {
                    mpn_toom63_mul(&mut ws, &u[u_offset..u_offset + 2 * vn], v, &mut scratch);
                }
                un -= 2 * vn;
                u_offset += 2 * vn;
                let cy =
                    limbs_slice_add_same_length_in_place_left(&mut prod[prod_offset..], &ws[..vn]);
                prod[prod_offset + vn..prod_offset + 3 * vn].copy_from_slice(&ws[vn..3 * vn]);
                if cy {
                    limbs_slice_add_limb_in_place(&mut prod[prod_offset + vn..], 1);
                }
                prod_offset += 2 * vn;
            }

            // vn / 2 <= un < 2.5vn

            if un < vn {
                mpn_mul(&mut ws, v, &u[u_offset..u_offset + un]);
            } else {
                mpn_mul(&mut ws, &u[u_offset..u_offset + un], v);
            }

            let cy = limbs_slice_add_same_length_in_place_left(
                &mut prod[prod_offset..prod_offset + vn],
                &ws[..vn],
            );
            prod[prod_offset + vn..prod_offset + un + vn].copy_from_slice(&ws[vn..un + vn]);
            if cy {
                limbs_slice_add_limb_in_place(&mut prod[prod_offset + vn..], 1);
            }
        } else if 6 * un < 7 * vn {
            mpn_toom33_mul(prod, u, v, &mut scratch);
        } else if 2 * un < 3 * vn {
            if vn < MUL_TOOM32_TO_TOOM43_THRESHOLD {
                mpn_toom32_mul(prod, u, v, &mut scratch);
            } else {
                mpn_toom43_mul(prod, u, v, &mut scratch);
            }
        } else if 6 * un < 11 * vn {
            if 4 * un < 7 * vn {
                if vn < MUL_TOOM32_TO_TOOM53_THRESHOLD {
                    mpn_toom32_mul(prod, u, v, &mut scratch);
                } else {
                    mpn_toom53_mul(prod, u, v, &mut scratch);
                }
            } else if vn < MUL_TOOM42_TO_TOOM53_THRESHOLD {
                mpn_toom42_mul(prod, u, v, &mut scratch);
            } else {
                mpn_toom53_mul(prod, u, v, &mut scratch);
            }
        } else if vn < MUL_TOOM42_TO_TOOM63_THRESHOLD {
            mpn_toom42_mul(prod, u, v, &mut scratch);
        } else {
            mpn_toom63_mul(prod, u, v, &mut scratch);
        }
    } else if vn < MUL_TOOM6H_THRESHOLD {
        let mut scratch = vec![0; mpn_toom44_mul_itch(un)];
        mpn_toom44_mul(prod, u, v, &mut scratch);
    } else if vn < MUL_TOOM8H_THRESHOLD {
        let mut scratch = vec![0; mpn_toom6h_mul_itch(un, vn)];
        mpn_toom6h_mul(prod, u, v, &mut scratch);
    } else {
        let mut scratch = vec![0; mpn_toom8h_mul_itch(un, vn)];
        mpn_toom8h_mul(prod, u, v, &mut scratch);
    }
    prod[un + vn - 1] // historic
}

fn mul_helper(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let mut product_limbs = vec![0; xs.len() + ys.len()];
    if xs.len() >= ys.len() {
        mpn_mul(&mut product_limbs, xs, ys);
    } else {
        mpn_mul(&mut product_limbs, ys, xs);
    }
    product_limbs
}

fn mul_basecase_mem_opt_helper(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let mut product_limbs = vec![0; xs.len() + ys.len()];
    if xs.len() >= ys.len() {
        mpn_mul_basecase_mem_opt(&mut product_limbs, xs, ys);
    } else {
        mpn_mul_basecase_mem_opt(&mut product_limbs, ys, xs);
    }
    product_limbs
}

/// Multiplies a `Natural` by a `Natural`, taking both `Natural`s by value.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Natural::ONE * Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Natural::from(123u32) * Natural::ZERO).to_string(), "0");
///     assert_eq!((Natural::from(123u32) * Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((Natural::from_str("123456789000").unwrap() * Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl Mul<Natural> for Natural {
    type Output = Natural;

    fn mul(mut self, other: Natural) -> Natural {
        self *= other;
        self
    }
}

/// Multiplies a `Natural` by a `Natural`, taking the left `Natural` by value and the right
/// `Natural` by reference.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Natural::ONE * &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Natural::from(123u32) * &Natural::ZERO).to_string(), "0");
///     assert_eq!((Natural::from(123u32) * &Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((Natural::from_str("123456789000").unwrap() * &Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a> Mul<&'a Natural> for Natural {
    type Output = Natural;

    fn mul(mut self, other: &'a Natural) -> Natural {
        self *= other;
        self
    }
}

/// Multiplies a `Natural` by a `Natural`, taking the left `Natural` by reference and the right
/// `Natural` by value.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Natural::ONE * Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) * Natural::ZERO).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) * Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((&Natural::from_str("123456789000").unwrap() * Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a> Mul<Natural> for &'a Natural {
    type Output = Natural;

    fn mul(self, mut other: Natural) -> Natural {
        other *= self;
        other
    }
}

/// Multiplies a `Natural` by a `Natural`, taking both `Natural`s by reference.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Natural::ONE * &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) * &Natural::ZERO).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) * &Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((&Natural::from_str("123456789000").unwrap() * &Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a, 'b> Mul<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn mul(self, other: &'a Natural) -> Natural {
        if let Small(y) = *other {
            self * y
        } else if let Small(x) = *self {
            other * x
        } else {
            match (self, other) {
                (&Large(ref xs), &Large(ref ys)) => {
                    let mut product = Large(mul_helper(xs, ys));
                    product.trim();
                    product
                }
                _ => unreachable!(),
            }
        }
    }
}

/// Multiplies a `Natural` by a `Natural` in place, taking the `Natural` on the RHS by value.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::One;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Natural::ONE;
///     x *= Natural::from_str("1000").unwrap();
///     x *= Natural::from_str("2000").unwrap();
///     x *= Natural::from_str("3000").unwrap();
///     x *= Natural::from_str("4000").unwrap();
///     assert_eq!(x.to_string(), "24000000000000");
/// }
/// ```
impl MulAssign<Natural> for Natural {
    fn mul_assign(&mut self, mut other: Natural) {
        if let Small(y) = other {
            *self *= y;
        } else if let Small(x) = *self {
            other *= x;
            *self = other;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), Large(ref mut ys)) => {
                    *xs = mul_helper(xs, ys);
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}

/// Multiplies a `Natural` by a `Natural` in place, taking the `Natural` on the RHS by reference.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::One;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Natural::ONE;
///     x *= &Natural::from_str("1000").unwrap();
///     x *= &Natural::from_str("2000").unwrap();
///     x *= &Natural::from_str("3000").unwrap();
///     x *= &Natural::from_str("4000").unwrap();
///     assert_eq!(x.to_string(), "24000000000000");
/// }
/// ```
impl<'a> MulAssign<&'a Natural> for Natural {
    fn mul_assign(&mut self, other: &'a Natural) {
        if let Small(y) = *other {
            *self *= y;
        } else if let Small(x) = *self {
            *self = other * x;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), &Large(ref ys)) => {
                    *xs = mul_helper(xs, ys);
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}

impl Natural {
    pub fn _mul_assign_basecase_mem_opt(&mut self, mut other: Natural) {
        if let Small(y) = other {
            *self *= y;
        } else if let Small(x) = *self {
            other *= x;
            *self = other;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), Large(ref ys)) => {
                    *xs = mul_basecase_mem_opt_helper(xs, ys)
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}
