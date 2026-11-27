// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Point arithmetic on y^2 = x^3 + a x + b modulo n, in Jacobian coordinates (X : Y : Z), with Z = 0
// for the point at infinity. All operations are done as if every nonzero residue were a unit (n
// prime); the quantities whose invertibility is assumed are multiplied into acc, and a computation
// fails as soon as acc is zero.
//
// FLINT does the arithmetic through its generic rings, with mpn_mod for moduli of 2 to 16 limbs and
// fmpz_mod beyond; here it is Montgomery arithmetic on limb vectors for every odd n.

use crate::natural::Natural;
use crate::natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use crate::natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::natural::modular::mod_pow::LimbsRedc;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::num::modular::traits::{Mod, ModInverse};
use malachite_base::slices::slice_test_zero;

// This is equivalent to `ecpp_point_struct` from `ecpp.h`, FLINT 3.7.0-dev.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EcppPoint {
    pub x: Natural,
    pub y: Natural,
    pub z: Natural,
}

impl EcppPoint {
    // This is equivalent to `ecpp_point_set_affine` from `ecpp/point.c`, FLINT 3.7.0-dev.
    pub const fn from_affine(x: Natural, y: Natural) -> Self {
        Self {
            x,
            y,
            z: Natural::ONE,
        }
    }

    // This is equivalent to `ecpp_point_is_zero` from `ecpp/point.c`, FLINT 3.7.0-dev.
    pub fn is_zero(&self) -> bool {
        self.z == 0u32
    }
}

// A residue modulo n in Montgomery form: a vector of as many limbs as n, reduced mod n.
type Elem = Vec<Limb>;

// Arithmetic modulo an odd n in Montgomery form. Every value is reduced mod n, so that a value is
// zero exactly when its residue is: the REDC of a product of two reduced values is less than 2n, so
// one subtraction reduces it.
struct Field<'a> {
    ms: &'a [Limb],
    redc: LimbsRedc<'a>,
    product: Elem,
    one: Elem,
}

// Reduces x < 2n mod n.
fn reduce_once(xs: &mut [Limb], ms: &[Limb]) {
    if limbs_cmp_same_length(xs, ms) != Less {
        limbs_sub_same_length_in_place_left(xs, ms);
    }
}

impl<'a> Field<'a> {
    fn new(ms: &'a [Limb]) -> Self {
        let redc = LimbsRedc::new(ms);
        let mut one = redc.to_redc(&[1]);
        reduce_once(&mut one, ms);
        Self {
            ms,
            redc,
            product: vec![0; ms.len()],
            one,
        }
    }

    fn zero(&self) -> Elem {
        vec![0; self.ms.len()]
    }

    // Converts x, which must be reduced mod n, to Montgomery form.
    fn encode(&self, x: &Natural) -> Elem {
        if *x == 0u32 {
            self.zero()
        } else {
            self.redc.to_redc(x.as_limbs_asc())
        }
    }

    // Converts x out of Montgomery form.
    fn decode(&mut self, x: &[Limb]) -> Natural {
        Natural::from_owned_limbs_asc(self.redc.redc_to_reduced(x))
    }

    fn mul(&mut self, x: &[Limb], y: &[Limb]) -> Elem {
        let mut out = self.zero();
        self.redc.mul(&mut out, x, y);
        reduce_once(&mut out, self.ms);
        out
    }

    fn mul_assign(&mut self, x: &mut Elem, y: &[Limb]) {
        self.redc.mul(&mut self.product, x, y);
        reduce_once(&mut self.product, self.ms);
        swap(x, &mut self.product);
    }

    fn square(&mut self, x: &[Limb]) -> Elem {
        let mut out = self.zero();
        self.redc.square(&mut out, x);
        reduce_once(&mut out, self.ms);
        out
    }

    fn square_assign(&mut self, x: &mut Elem) {
        self.redc.square(&mut self.product, x);
        reduce_once(&mut self.product, self.ms);
        swap(x, &mut self.product);
    }

    fn add_assign(&self, x: &mut [Limb], y: &[Limb]) {
        if limbs_slice_add_same_length_in_place_left(x, y)
            || limbs_cmp_same_length(x, self.ms) != Less
        {
            limbs_sub_same_length_in_place_left(x, self.ms);
        }
    }

    fn double_assign(&self, x: &mut Elem) {
        let y = x.clone();
        self.add_assign(x, &y);
    }

    fn sub_assign(&self, x: &mut [Limb], y: &[Limb]) {
        if limbs_sub_same_length_in_place_left(x, y) {
            limbs_slice_add_same_length_in_place_left(x, self.ms);
        }
    }

    fn neg(&self, x: &[Limb]) -> Elem {
        let mut out = self.zero();
        if !slice_test_zero(x) {
            out.copy_from_slice(self.ms);
            limbs_sub_same_length_in_place_left(&mut out, x);
        }
        out
    }

    fn inverse(&mut self, x: &[Limb]) -> Option<Elem> {
        let x = self.decode(x);
        let inverse = x.mod_inverse(Natural::from_limbs_asc(self.ms))?;
        Some(self.encode(&inverse))
    }
}

// A point in Jacobian coordinates, in Montgomery form.
#[derive(Clone)]
struct Point {
    x: Elem,
    y: Elem,
    z: Elem,
}

impl Point {
    fn with_zero_z(&self, f: &Field) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            z: f.zero(),
        }
    }
}

// This is equivalent to `_gpoint_double` from `ecpp/point_gr.c`, FLINT 3.7.0-dev.
fn point_double(f: &mut Field, p: &Point, a: &[Limb], acc: &mut Elem) -> Point {
    if slice_test_zero(&p.z) || slice_test_zero(&p.y) {
        return p.with_zero_z(f);
    }
    f.mul_assign(acc, &p.y);
    // t1 = M = 3 X^2 + a Z^4
    let mut t1 = f.square(&p.x);
    let mut t2 = t1.clone();
    f.double_assign(&mut t2);
    f.add_assign(&mut t1, &t2);
    let mut t2 = f.square(&p.z);
    f.square_assign(&mut t2);
    f.mul_assign(&mut t2, a);
    f.add_assign(&mut t1, &t2);
    // t2 = S = 4 X Y^2, t3 = 8 Y^4
    let mut t3 = f.square(&p.y);
    let mut t2 = f.mul(&p.x, &t3);
    f.double_assign(&mut t2);
    f.double_assign(&mut t2);
    f.square_assign(&mut t3);
    f.double_assign(&mut t3);
    f.double_assign(&mut t3);
    f.double_assign(&mut t3);
    // Z3 = 2 Y Z
    let mut z = f.mul(&p.y, &p.z);
    f.double_assign(&mut z);
    // X3 = M^2 - 2S
    let mut x = f.square(&t1);
    f.sub_assign(&mut x, &t2);
    f.sub_assign(&mut x, &t2);
    // Y3 = M (S - X3) - 8 Y^4
    f.sub_assign(&mut t2, &x);
    f.mul_assign(&mut t2, &t1);
    let mut y = t2;
    f.sub_assign(&mut y, &t3);
    Point { x, y, z }
}

// R = P + Q with Q = (qx : qy : 1) affine.
//
// This is equivalent to `_gpoint_add_affine` from `ecpp/point_gr.c`, FLINT 3.7.0-dev.
fn point_add_affine(
    f: &mut Field,
    p: &Point,
    qx: &[Limb],
    qy: &[Limb],
    a: &[Limb],
    acc: &mut Elem,
) -> Point {
    if slice_test_zero(&p.z) {
        return Point {
            x: qx.to_vec(),
            y: qy.to_vec(),
            z: f.one.clone(),
        };
    }
    // t1 = Z1^2, t2 = U2 = X2 Z1^2, t3 = S2 = Y2 Z1^3
    let mut t1 = f.square(&p.z);
    let mut t2 = f.mul(qx, &t1);
    f.mul_assign(&mut t1, &p.z);
    let mut t3 = f.mul(qy, &t1);
    // t2 = H = U2 - X1, t3 = r = S2 - Y1
    f.sub_assign(&mut t2, &p.x);
    f.sub_assign(&mut t3, &p.y);
    if slice_test_zero(&t2) {
        if slice_test_zero(&t3) {
            return point_double(f, p, a, acc);
        }
        // P = -Q
        return p.with_zero_z(f);
    }
    f.mul_assign(acc, &t2);
    // Z3 = Z1 H
    let z = f.mul(&p.z, &t2);
    // t1 = H^2, t4 = H^3, t1 = V = X1 H^2
    let mut t1 = f.square(&t2);
    let mut t4 = f.mul(&t1, &t2);
    f.mul_assign(&mut t1, &p.x);
    // X3 = r^2 - H^3 - 2V
    let mut x = f.square(&t3);
    f.sub_assign(&mut x, &t4);
    f.sub_assign(&mut x, &t1);
    f.sub_assign(&mut x, &t1);
    // Y3 = r (V - X3) - Y1 H^3
    f.sub_assign(&mut t1, &x);
    f.mul_assign(&mut t1, &t3);
    f.mul_assign(&mut t4, &p.y);
    let mut y = t1;
    f.sub_assign(&mut y, &t4);
    Point { x, y, z }
}

// Plain double-and-add; P affine, acc nonzero.
//
// This is equivalent to `_gpoint_mul_binary` from `ecpp/point_gr.c`, FLINT 3.7.0-dev.
fn point_mul_binary(
    f: &mut Field,
    p: &Point,
    k: &Natural,
    a: &[Limb],
    acc: &mut Elem,
) -> Option<Point> {
    let mut t = p.clone();
    for i in (0..k.significant_bits() - 1).rev() {
        t = point_double(f, &t, a, acc);
        if k.get_bit(i) {
            t = point_add_affine(f, &t, &p.x, &p.y, a, acc);
        }
        if slice_test_zero(acc) {
            return None;
        }
    }
    Some(t)
}

// Normalises Jacobian points to affine coordinates with one inversion, multiplying acc by the Z's.
// Returns false if some Z is zero or the product is not invertible (the points are then left
// alone).
//
// This is equivalent to `_gpoints_to_affine` from `ecpp/point_gr.c`, FLINT 3.7.0-dev.
fn points_to_affine(f: &mut Field, ps: &mut [Point], acc: &mut Elem) -> bool {
    if ps.iter().any(|p| slice_test_zero(&p.z)) {
        return false;
    }
    let mut prod = Vec::with_capacity(ps.len());
    prod.push(ps[0].z.clone());
    for i in 1..ps.len() {
        let pr = f.mul(&prod[i - 1], &ps[i].z);
        prod.push(pr);
    }
    f.mul_assign(acc, &prod[ps.len() - 1]);
    let Some(mut inv) = f.inverse(&prod[ps.len() - 1]) else {
        return false;
    };
    for i in (0..ps.len()).rev() {
        let t = if i > 0 {
            let t = f.mul(&inv, &prod[i - 1]);
            f.mul_assign(&mut inv, &ps[i].z);
            t
        } else {
            inv.clone()
        };
        let mut g = f.square(&t);
        f.mul_assign(&mut ps[i].x, &g);
        f.mul_assign(&mut g, &t);
        f.mul_assign(&mut ps[i].y, &g);
        ps[i].z = f.one.clone();
    }
    true
}

const NAF_W: u64 = 4;
// the number of odd multiples of P in the table
const NTAB: usize = 1 << (NAF_W - 2);
// the range of the NAF digits is (-NAF_HALF, NAF_HALF)
const NAF_HALF: i8 = 1 << (NAF_W - 1);
const NAF_FULL: i8 = 1 << NAF_W;

// The width-4 non-adjacent form of k, least significant digit first: odd digits in (-8, 8), each
// followed by at least three zeros, with no trailing zeros. FLINT computes it by repeatedly
// subtracting the digit from k and halving it; this reads it off the bits of k with a carry, which
// gives the same digits without any multiprecision arithmetic.
//
// This is equivalent to the recoding in `ecpp_point_mul_gr` from `ecpp/point_gr.c`, FLINT
// 3.7.0-dev.
crate_test_fn! {
naf_digits(k: &Natural) -> Vec<i8> {
    let bits = k.significant_bits();
    let mut naf = Vec::with_capacity(usize::exact_from(bits) + 1);
    // The value still to be recoded is (k >> i) + carry.
    let mut i = 0;
    let mut carry = 0u8;
    while i < bits || carry != 0 {
        let low = u8::from(k.get_bit(i)) + carry;
        if low.even() {
            naf.push(0);
            carry = low >> 1;
            i += 1;
            continue;
        }
        // The remaining value mod 2^NAF_W, which is odd and so at most 2^NAF_W - 1.
        let mut window = carry;
        for j in 0..NAF_W {
            window += u8::from(k.get_bit(i + j)) << j;
        }
        let mut d = i8::exact_from(window);
        if d >= NAF_HALF {
            d -= NAF_FULL;
        }
        naf.push(d);
        // Subtracting d clears the low NAF_W bits of the remaining value; a negative d carries 1.
        carry = u8::from(d < 0);
        i += NAF_W;
        if i >= bits && carry == 0 {
            break;
        }
        naf.extend_from_slice(&[0; 3]);
    }
    naf
}}

// R = k P for P in Montgomery form, with acc nonzero.
fn point_mul_field(
    f: &mut Field,
    p: &Point,
    k: &Natural,
    a: &[Limb],
    acc: &mut Elem,
) -> Option<Point> {
    if k.significant_bits() < 32 {
        return point_mul_binary(f, p, k, a, acc);
    }
    // odd multiples: 2P affine, T[i] = (2i + 1) P, with the y coordinates of -T[i] alongside
    let mut p2 = [point_double(f, p, a, acc)];
    let mut fallback = !points_to_affine(f, &mut p2, acc);
    let mut t = Vec::with_capacity(NTAB);
    if !fallback {
        t.push(p.clone());
        for i in 1..NTAB {
            let ti = point_add_affine(f, &t[i - 1], &p2[0].x, &p2[0].y, a, acc);
            t.push(ti);
        }
        fallback = !points_to_affine(f, &mut t[1..], acc);
    }
    if fallback {
        // some small multiple of P is O or not normalisable: plain method
        return if slice_test_zero(acc) {
            None
        } else {
            point_mul_binary(f, p, k, a, acc)
        };
    }
    let neg_ys: Vec<Elem> = t.iter().map(|ti| f.neg(&ti.y)).collect();
    let mut q = Point {
        x: f.zero(),
        y: f.zero(),
        z: f.zero(),
    };
    for &d in naf_digits(k).iter().rev() {
        q = point_double(f, &q, a, acc);
        if d != 0 {
            let i = usize::from((d.unsigned_abs() - 1) >> 1);
            let y = if d > 0 { &t[i].y } else { &neg_ys[i] };
            q = point_add_affine(f, &q, &t[i].x, y, a, acc);
        }
        if slice_test_zero(acc) {
            return None;
        }
    }
    Some(q)
}

// R = k P for an affine point P, multiplying acc by all quantities whose invertibility was assumed;
// returns None if acc became zero or an operation failed (n composite). Width-4 signed sliding
// window with the odd multiples in affine coordinates. P, a, and acc must be reduced mod n, and n
// must be odd.
//
// This is equivalent to `ecpp_point_mul_gr` from `ecpp/point_gr.c`, FLINT 3.7.0-dev.
pub fn point_mul_reduced(
    p: &EcppPoint,
    k: &Natural,
    a: &Natural,
    acc: &mut Natural,
    n: &Natural,
) -> Option<EcppPoint> {
    if *k == 0u32 || p.z == 0u32 {
        return Some(EcppPoint::default());
    }
    // every path below fails at once if acc is zero
    if *acc == 0u32 {
        return None;
    }
    let mut f = Field::new(n.as_limbs_asc());
    let p = Point {
        x: f.encode(&p.x),
        y: f.encode(&p.y),
        z: f.encode(&p.z),
    };
    let a = f.encode(a);
    let mut acc_field = f.encode(acc);
    let r = point_mul_field(&mut f, &p, k, &a, &mut acc_field);
    *acc = f.decode(&acc_field);
    let r = r?;
    Some(EcppPoint {
        x: f.decode(&r.x),
        y: f.decode(&r.y),
        z: f.decode(&r.z),
    })
}

// R = k P, reducing P, a, and acc mod n first. On failure R is the point at infinity; acc is
// written back either way (it may be zero). n must be odd.
//
// This is equivalent to `ecpp_point_mul` from `ecpp/point_gr.c`, FLINT 3.7.0-dev.
pub fn point_mul(
    p: &EcppPoint,
    k: &Natural,
    a: &Natural,
    acc: &mut Natural,
    n: &Natural,
) -> (EcppPoint, bool) {
    let p = EcppPoint {
        x: (&p.x).mod_op(n),
        y: (&p.y).mod_op(n),
        z: (&p.z).mod_op(n),
    };
    let a = a.mod_op(n);
    *acc = (&*acc).mod_op(n);
    match point_mul_reduced(&p, k, &a, acc, n) {
        Some(r) => (r, true),
        None => (EcppPoint::default(), false),
    }
}
