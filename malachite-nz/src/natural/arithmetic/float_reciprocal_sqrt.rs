// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2008-2024 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_slice_add_greater_in_place_left, limbs_slice_add_limb_in_place,
};
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len,
};
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::arithmetic::sub::{limbs_sub_greater_in_place_left, limbs_sub_limb_in_place};
use crate::natural::logic::not::{limbs_not_in_place, limbs_not_to_out};
use crate::natural::{
    LIMB_HIGH_BIT, bit_to_limb_count_ceiling, bit_to_limb_count_floor, limb_to_bit_count,
};
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{PowerOf2, Square, XMulYToZZ};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;

// the following T1 and T2 are bipartite tables giving initial approximation for the inverse square
// root, with 13-bit input split in 5 + 4 + 4, and 11-bit output. More precisely, if 2048 <= i <
// 8192, with i = a * 2 ^ 8 + b * 2 ^ 4 + c, we use for approximation of 2048 / sqrt(i / 2048) the
// value x = T1[16 * (a - 8) + b] + T2[16 * (a - 8) + c]. The largest error is obtained for i =
// 2054, where x = 2044, and 2048 / sqrt(i / 2048) = 2045.006576...
//
// This is T1 from rec_sqrt.c, MPFR 4.3.0.
const T1: [u16; 384] = [
    2040, 2033, 2025, 2017, 2009, 2002, 1994, 1987, 1980, 1972, 1965, 1958, 1951, 1944, 1938, 1931,
    1925, 1918, 1912, 1905, 1899, 1892, 1886, 1880, 1874, 1867, 1861, 1855, 1849, 1844, 1838, 1832,
    1827, 1821, 1815, 1810, 1804, 1799, 1793, 1788, 1783, 1777, 1772, 1767, 1762, 1757, 1752, 1747,
    1742, 1737, 1733, 1728, 1723, 1718, 1713, 1709, 1704, 1699, 1695, 1690, 1686, 1681, 1677, 1673,
    1669, 1664, 1660, 1656, 1652, 1647, 1643, 1639, 1635, 1631, 1627, 1623, 1619, 1615, 1611, 1607,
    1603, 1600, 1596, 1592, 1588, 1585, 1581, 1577, 1574, 1570, 1566, 1563, 1559, 1556, 1552, 1549,
    1545, 1542, 1538, 1535, 1532, 1528, 1525, 1522, 1518, 1515, 1512, 1509, 1505, 1502, 1499, 1496,
    1493, 1490, 1487, 1484, 1481, 1478, 1475, 1472, 1469, 1466, 1463, 1460, 1457, 1454, 1451, 1449,
    1446, 1443, 1440, 1438, 1435, 1432, 1429, 1427, 1424, 1421, 1419, 1416, 1413, 1411, 1408, 1405,
    1403, 1400, 1398, 1395, 1393, 1390, 1388, 1385, 1383, 1380, 1378, 1375, 1373, 1371, 1368, 1366,
    1363, 1360, 1358, 1356, 1353, 1351, 1349, 1346, 1344, 1342, 1340, 1337, 1335, 1333, 1331, 1329,
    1327, 1325, 1323, 1321, 1319, 1316, 1314, 1312, 1310, 1308, 1306, 1304, 1302, 1300, 1298, 1296,
    1294, 1292, 1290, 1288, 1286, 1284, 1282, 1280, 1278, 1276, 1274, 1272, 1270, 1268, 1266, 1265,
    1263, 1261, 1259, 1257, 1255, 1253, 1251, 1250, 1248, 1246, 1244, 1242, 1241, 1239, 1237, 1235,
    1234, 1232, 1230, 1229, 1227, 1225, 1223, 1222, 1220, 1218, 1217, 1215, 1213, 1212, 1210, 1208,
    1206, 1204, 1203, 1201, 1199, 1198, 1196, 1195, 1193, 1191, 1190, 1188, 1187, 1185, 1184, 1182,
    1181, 1180, 1178, 1177, 1175, 1174, 1172, 1171, 1169, 1168, 1166, 1165, 1163, 1162, 1160, 1159,
    1157, 1156, 1154, 1153, 1151, 1150, 1149, 1147, 1146, 1144, 1143, 1142, 1140, 1139, 1137, 1136,
    1135, 1133, 1132, 1131, 1129, 1128, 1127, 1125, 1124, 1123, 1121, 1120, 1119, 1117, 1116, 1115,
    1114, 1113, 1111, 1110, 1109, 1108, 1106, 1105, 1104, 1103, 1101, 1100, 1099, 1098, 1096, 1095,
    1093, 1092, 1091, 1090, 1089, 1087, 1086, 1085, 1084, 1083, 1081, 1080, 1079, 1078, 1077, 1076,
    1075, 1073, 1072, 1071, 1070, 1069, 1068, 1067, 1065, 1064, 1063, 1062, 1061, 1060, 1059, 1058,
    1057, 1056, 1055, 1054, 1052, 1051, 1050, 1049, 1048, 1047, 1046, 1045, 1044, 1043, 1042, 1041,
    1040, 1039, 1038, 1037, 1036, 1035, 1034, 1033, 1032, 1031, 1030, 1029, 1028, 1027, 1026, 1025,
];

// This is T2 from rec_sqrt.c, MPFR 4.3.0.
const T2: [u8; 384] = [
    7, 7, 6, 6, 5, 5, 4, 4, 4, 3, 3, 2, 2, 1, 1, 0, 6, 5, 5, 5, 4, 4, 3, 3, 3, 2, 2, 2, 1, 1, 0, 0,
    5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 1, 1, 1, 0, 0, 4, 4, 3, 3, 3, 3, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0,
    3, 3, 3, 3, 2, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 3, 3, 3, 2, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0,
    3, 3, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
    2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
    3, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 0, 2, 2, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

// Put in X a p-bit approximation of 1/sqrt(A), where X = {x, n} / B ^ n, n = ceil(p / Limb::WIDTH),
// A = 2 ^ (1 + a_s) * {a, an} / B ^ an, a_s is 0 or 1, an = ceil(ap / Limb::WIDTH), where B = 2 ^
// Limb::WIDTH.
//
// We have 1 <= A < 4 and 1/2 <= X < 1.
//
// The error in the approximate result with respect to the true value 1/sqrt(A) is bounded by 1
// ulp(X), i.e., 2^{-p} since 1/2 <= X < 1.
//
// Note: x and a are left-aligned, i.e., the most significant bit of a[an-1] is set, and so is the
// most significant bit of the output x[n-1].
//
// If p is not a multiple of Limb::WIDTH, the extra low bits of the input A are taken into account
// to compute the approximation of 1/sqrt(A), but whether or not they are zero, the error between X
// and 1/sqrt(A) is bounded by 1 ulp(X) [in precision p]. The extra low bits of the output X (if p
// is not a multiple of Limb::WIDTH) are set to 0.
//
// Assumptions:
// - A should be normalized, i.e., the most significant bit of a[an-1] should be 1. If a_s = 0, we
//   have 1 <= A < 2; if a_s = 1, we have 2 <= A < 4.
// - p >= 12
// - {a, an} and {x, n} should not overlap
// - Limb::WIDTH >= 12 and is even
//
// Note: this routine is much more efficient when ap is small compared to p, including the case
// where ap <= Limb::WIDTH, thus it can be used to implement an efficient mpfr_rec_sqrt_ui function.
//
// References: [1] Modern Computer Algebra, Richard Brent and Paul Zimmermann,
// https://members.loria.fr/PZimmermann/mca/pub226.html
//
// This is mpfr_mpn_rec_sqrt from rec_sqrt.c, MPFR 4.3.0.
pub fn limbs_reciprocal_sqrt(
    out: &mut [Limb],
    out_prec: u64,
    mut xs: &[Limb],
    x_prec: u64,
    parity: bool,
) {
    let out_len = bit_to_limb_count_ceiling(out_prec);
    let out = &mut out[..out_len];
    let mut xs_len = bit_to_limb_count_ceiling(x_prec);
    // A should be normalized
    assert!(xs[xs_len - 1].get_highest_bit());
    assert!(out_prec >= 11);
    if xs_len > out_len {
        // we can cut the input to n limbs
        xs = &xs[xs_len - out_len..];
        xs_len = out_len;
    }
    if out_prec == 11 {
        // should happen only from recursive calls
        //
        // take the 12 + a_s most significant bits of A
        let i =
            usize::exact_from(xs[xs_len - 1] >> (const { Limb::WIDTH - 12 } - u64::from(parity)));
        let ab = i >> 4;
        let ac = (ab & 0x3f0) | (i & 0xf);
        let t = T1[ab - 0x80] + u16::from(T2[ac - 0x80]); // fits on 16 bits
        // x has only one limb
        out[0] = Limb::from(t) << (Limb::WIDTH - out_prec);
    } else {
        // p >= 12
        //
        // compared to Algorithm 3.9 of [1], we have {a, an} = A / 2 if a_s = 0, and A / 4 if a_s =
        // 1.
        //
        // h = max(11, ceil((p + 3) / 2)) is the bitsize of the recursive call
        let h = if out_prec < 18 {
            11
        } else {
            (out_prec >> 1) + 2
        };
        // limb size of the recursive Xh
        let xs_rec_len = bit_to_limb_count_ceiling(h);
        // a priori limb size of Xh^2
        let rn = bit_to_limb_count_ceiling(h << 1);
        let ln = out_len - xs_rec_len; // remaining limbs to be computed
        // Since |Xh - A ^ {-1 / 2}| <= 2 ^ {-h}, then by multiplying by Xh + A ^ {-1 / 2} we get
        // |Xh ^ 2 - 1 / A| <= 2 ^ {-h + 1}, thus |A * Xh ^ 2 - 1| <= 2 ^ {-h + 3}, thus the h-3
        // most significant bits of t should be zero, which is in fact h + 1 + a_s - 3 because of
        // the normalization of A. This corresponds to th=floor((h + 1 + a_s - 3) / Limb::WIDTH)
        // limbs.
        //
        // More precisely we have |Xh ^ 2 - 1 / A| <= 2^{-h} * (Xh + A^{-1 / 2}) <= 2^{-h} * (2
        // A^{-1 / 2} + 2^{-h}) <= 2.001 * 2^{-h} * A^{-1 / 2} since A < 4 and h >= 11, thus |A * Xh
        // ^ 2 - 1| <= 2.001 * 2^{-h} * A^{1 / 2} <= 1.001 * 2 ^ {2 - h}. This is sufficient to
        // prove that the upper limb of {t,tn} below is less that 0.501 * 2 ^ Limb::WIDTH, thus cu =
        // 0 below.
        let hp = h + 1 + u64::from(parity);
        let th = bit_to_limb_count_floor(hp - 3);
        let mut ts_len = bit_to_limb_count_ceiling(hp + h);
        // we need h + 1 + a_s bits of a
        //
        // number of high limbs of A needed for the recursive call
        let mut ahn = bit_to_limb_count_ceiling(hp);
        if ahn > xs_len {
            ahn = xs_len;
        }
        let (out_lo, out_hi) = out.split_at_mut(ln);
        limbs_reciprocal_sqrt(
            out_hi,
            h,
            &xs[xs_len - ahn..],
            limb_to_bit_count(ahn),
            parity,
        );
        // the most h significant bits of X are set, X has ceil(h / Limb::WIDTH) limbs, the low (-h)
        // % Limb::WIDTH bits are zero
        //
        // compared to Algorithm 3.9 of [1], we have {x+ln,xn} = X_h
        //
        // first step: square X in r, result is exact
        let mut us_len = xs_rec_len + (ts_len - th);
        // We use the same temporary buffer to store r and u: r needs 2*xn limbs where u needs
        // xn+(tn-th) limbs. Since tn can store at least 2h bits, and th at most h bits, then tn-th
        // can store at least h bits, thus tn - th >= xn, and reserving the space for u is enough.
        assert!(xs_rec_len << 1 <= us_len);
        let sn = xs_len + rn;
        let mut scratch = vec![0; (us_len << 1) + sn];
        split_into_chunks_mut!(scratch, us_len, [us, rs], ss);
        let mut rs = rs;
        if h << 1 <= Limb::WIDTH {
            // xn=rn=1, and since p <= 2h-3, n=1, thus ln = 0
            assert_eq!(ln, 0);
            let cy = out_hi[0] >> const { Limb::WIDTH >> 1 };
            rs = &mut rs[1..];
            rs[0] = cy.square();
        } else if xs_rec_len == 1 {
            // xn = 1, rn = 2
            (rs[1], rs[0]) = Limb::x_mul_y_to_zz(out_hi[0], out_hi[0]);
        } else {
            let mut scratch = vec![0; limbs_square_to_out_scratch_len(xs_rec_len)];
            limbs_square_to_out(rs, out_hi, &mut scratch);
            // we have {r, 2 * xn} = X_h ^ 2
            if rn < xs_rec_len << 1 {
                rs = &mut rs[1..];
            }
        }
        // now the 2h most significant bits of {r, rn} contains X ^ 2, r has rn limbs, and the low
        // (-2h) % Limb::WIDTH bits are zero
        //
        // Second step: s <- A * (r ^ 2), and truncate the low ap bits, i.e., at weight 2 ^ {-2h} (s
        // is aligned to the low significant bits)
        if rn == 1 {
            // rn=1 implies n=1, since rn*Limb::WIDTH >= 2h, and 2h >= p + 3
            //
            // necessarily p <= Limb::WIDTH-3: we can ignore the two low bits from A
            //
            // since n=1, and we ensured an <= n, we also have an=1
            assert_eq!(xs_len, 1);
            (ss[1], ss[0]) = Limb::x_mul_y_to_zz(rs[0], xs[0]);
        } else {
            // we have p <= n * Limb::WIDTH 2h <= rn * Limb::WIDTH with p+3 <= 2h <= p+4 thus n <=
            // rn <= n + 1
            assert!(rn <= out_len + 1);
            // since we ensured an <= n, we have an <= rn
            assert!(xs_len <= rn);
            let mut scratch = vec![0; limbs_mul_greater_to_out_scratch_len(rn, xs_len)];
            limbs_mul_greater_to_out(ss, &rs[..rn], xs, &mut scratch);
            // s should be near B ^ sn / 2 ^ (1 + a_s), thus s[sn-1] is either
            // - 100000... or 011111... if a_s = 0, or
            // - 010000... or 001111... if a_s = 1.
            // We ignore the bits of s after the first 2h + 1 + a_s ones. We have {s, rn+an} = A *
            // X_h ^ 2 / 2 if a_s = 0, A * X_h ^ 2 / 4 if a_s = 1.
        }
        // We ignore the bits of s after the first 2h + 1 + a_s ones: s has rn + an limbs, where rn
        // = LIMBS(2h), an = LIMBS(a), and tn = LIMBS(2h + 1 + a_s).
        let ts = &mut ss[sn - ts_len..]; // pointer to low limb of the high part of t
        // the upper h-3 bits of 1 - t should be zero, where 1 corresponds to the most significant
        // bit of t[tn - 1] if a_s = 0, and to the 2nd most significant bit of t[tn - 1] if a_s = 1
        //
        // compute t <- 1 - t, which is B^tn - {t, tn + 1}, with rounding toward -Inf, i.e.,
        // rounding the input t toward +Inf. We could only modify the low tn - th limbs from t, but
        // it gives only a small speedup, and would make the code more complex.
        let bit = if parity {
            const { LIMB_HIGH_BIT >> 1 }
        } else {
            LIMB_HIGH_BIT
        };
        let ts_last = ts.last_mut().unwrap();
        let neg = *ts_last & bit;
        if neg == 0 {
            // Ax ^ 2 < 1: we have t = th + eps, where 0 <= eps < ulp(th) is the part truncated
            // above, thus 1 - t rounded to -Inf is 1 - th - ulp(th)
            //
            // since the 1 + a_s most significant bits of t are zero, set them to 1 before the
            // one-complement
            *ts_last |= LIMB_HIGH_BIT | bit;
            limbs_not_in_place(ts);
            // we should add 1 here to get 1-th complement, and subtract 1 for -ulp(th), thus we do
            // nothing
        } else {
            // negative case: we want 1 - t rounded toward -Inf, i.e., th + eps rounded toward +Inf,
            // which is th + ulp(th): we discard the bit corresponding to 1, and we add 1 to the
            // least significant bit of t
            *ts_last ^= neg;
            limbs_slice_add_limb_in_place(ts, 1);
        }
        // we know at least th = floor((h + 1 + a_s - 3) / Limb::WIDTH) of the high limbs of {t, tn}
        // are zero
        ts_len -= th;
        // tn = rn - th, where rn * Limb::WIDTH >= 2 * h and th * Limb::WIDTH <= h + 1 + a_s - 3,
        // thus tn > 0
        assert_ne!(ts_len, 0);
        // u <- x * t, where {t, tn} contains at least h + 3 bits, and {x, xn} contains h bits, thus
        // tn >= xn
        assert!(ts_len >= xs_rec_len);
        if ts_len == 1 {
            // necessarily xn = 1
            (us[1], us[0]) = Limb::x_mul_y_to_zz(ts[0], out_hi[0]);
        } else {
            let mut scratch = vec![0; limbs_mul_greater_to_out_scratch_len(ts_len, xs_rec_len)];
            limbs_mul_greater_to_out(us, &ts[..ts_len], out_hi, &mut scratch);
        }
        // we have {u, tn+xn} = T_l X_h / 2 if a_s = 0, T_l X_h / 4 if a_s = 1
        //
        // we have already discarded the upper th high limbs of t, thus we only have to consider the
        // upper n - th limbs of u
        us_len = out_len - th;
        // un cannot be zero, since p <= n * Limb::WIDTH, h = ceil((p + 3) / 2) <= (p + 4) / 2, th *
        // Limb::WIDTH <= h - 1 <= p / 2 + 1, thus (n - th) * Limb::WIDTH >= p / 2 - 1.
        assert_ne!(us_len, 0);
        let u_offset = ts_len + xs_rec_len - us_len;
        // xn + tn - un = xn + (original_tn - th) - (n - th) = xn + original_tn - n = LIMBS(h) +
        // LIMBS(2h + 1 + a_s) - LIMBS(p) > 0 since 2h >= p + 3
        assert_ne!(u_offset, 0); // will allow to access u[-1] below
        // In case a_s = 0, u contains |x * (1 - Ax ^ 2) / 2|, which is exactly what we need to add
        // or subtract. In case a_s = 1, u contains |x * (1 - Ax ^ 2) / 4|, thus we need to multiply
        // u by 2.
        if parity {
            // shift on un+1 limbs to get most significant bit of u[-1] into least significant bit
            // of u[0]
            limbs_slice_shl_in_place(&mut us[u_offset - 1..u_offset + us_len], 1);
        }
        // now {u,un} represents U / 2 from Algorithm 3.9
        let shift_bit = Limb::power_of_2(limb_to_bit_count(out_len) - out_prec);
        // We want that the low pl bits are zero after rounding to nearest, thus we round u to
        // nearest at bit pl - 1 of u[0]
        let (us_head, us_tail) = us[u_offset - 1..].split_first_mut().unwrap();
        let us_tail = &mut us_tail[..us_len];
        let cu = if shift_bit == 1 {
            // round bit is in u[-1]
            limbs_slice_add_limb_in_place(us_tail, *us_head >> const { Limb::WIDTH - 1 })
        } else {
            let uu = us_tail[0];
            let cu = limbs_slice_add_limb_in_place(us_tail, uu & (shift_bit >> 1));
            // mask bits 0..pl - 1 of u[0]
            us_tail[0] &= !(shift_bit - 1);
            cu
        };
        assert!(!cu);
        // We already have filled {x + ln, xn = n - ln}, and we want to add or subtract {u, un} at
        // position x.
        // - un = n - th, where th contains <= h + 1 + a_s - 3 <= h - 1 bits
        // - ln = n - xn, where xn contains >= h bits
        // - thus un > ln.
        // Warning: ln might be zero.
        assert!(us_len > ln);
        // we can have un = ln + 2, for example with Limb::WIDTH = 32 and p = 62, a_s = 0, then h =
        // 33, n = 2, th = 0, xn = 2, thus un = 2 and ln = 0.
        assert!(us_len == ln + 1 || us_len == ln + 2);
        // the high un-ln limbs of u will overlap the low part of {x+ln,xn}, we need to add or
        // subtract the overlapping part {u + ln, un - ln}
        //
        // Warning! th may be 0, in which case the mpn_add_1 and mpn_sub_1 below (with size = th)
        // mustn't be used.
        let mut carry;
        let (us_lo, us_hi) = us_tail.split_at_mut(ln);
        if neg == 0 {
            if ln != 0 {
                out_lo.copy_from_slice(us_lo);
            }
            carry = limbs_slice_add_greater_in_place_left(out_hi, us_hi);
            // cy is the carry at x + (ln + xn) = x + n
        } else {
            // negative case
            //
            // subtract {u+ln, un-ln} from {x+ln,un}
            carry = limbs_sub_greater_in_place_left(out_hi, us_hi);
            // cy is the borrow at x + (ln + xn) = x + n
            //
            // cy cannot be non-zero, since the most significant bit of Xh is 1, and the correction
            // is bounded by 2^{-h+3}
            assert!(!carry);
            if ln != 0 {
                limbs_not_to_out(out, us_lo);
                // we must add one for the 2-complement
                carry = limbs_slice_add_limb_in_place(out, 1);
                // ... and subtract 1 at x[ln], where n = ln + xn
                if limbs_sub_limb_in_place(&mut out[ln..], 1) {
                    assert!(carry);
                    carry = false;
                }
            }
        }
        // cy can be 1 when A=1, i.e., {a, n} = B ^ n. In that case we should have X = B ^ n, and
        // setting X to 1-2^{-p} satisfies the error bound of 1 ulp.
        if carry {
            assert!(limbs_sub_limb_in_place(out, shift_bit));
        }
    }
}
