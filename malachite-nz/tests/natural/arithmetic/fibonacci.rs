// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Fibonacci, Gcd, LucasNumber, Parity, Square};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{
    unsigned_gen_var_5, unsigned_gen_var_11, unsigned_gen_var_32, unsigned_gen_var_33,
    unsigned_pair_gen_var_18,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::fibonacci::limbs_fibonacci_pair;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::natural::arithmetic::fibonacci::{
    fibonacci_naive, fibonacci_pair_naive, lucas_number_naive, lucas_number_pair_naive,
};
use rug::Complete;
use std::panic::catch_unwind;

fn fibonacci_pair_alloc_len(n: u64) -> usize {
    usize::exact_from((((n >> 5) * 23) >> Limb::LOG_WIDTH) + 4)
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_fibonacci_pair() {
    fn test(n: u64, out_fs: &[Limb], out_f1s: &[Limb]) {
        let alloc = fibonacci_pair_alloc_len(n);
        let mut fs = vec![0; alloc];
        let mut f1s = vec![0; alloc];
        let size = limbs_fibonacci_pair(&mut fs, &mut f1s, n);
        assert_eq!(&fs[..size], out_fs);
        assert_eq!(&f1s[..size], out_f1s);
    }
    // - n == 0: fs is F(0) = 0 and f1s is F(-1) = 1
    test(0, &[0], &[1]);
    test(1, &[1], &[0]);
    test(2, &[1], &[1]);
    // - table direct (mask == 1)
    test(10, &[55], &[34]);
    // - the largest n whose F(n) is a table entry
    test(93, &[12200160415121876738], &[7540113804746346429]);
    // - the smallest n requiring a doubling step; F(93) gains a high zero limb
    // - k odd, so the -2 is folded into F(k - 1) ^ 2
    // - high-zero shrink after the squarings
    // - carry limb after F(2k + 1)
    // - new bit 0, so F(2k + 1) is replaced by F(2k), with no shrink
    test(94, &[1293530146158671551, 1], &[12200160415121876738, 0]);
    // - new bit 1, so F(2k - 1) is replaced by F(2k)
    test(95, &[13493690561280548289, 1], &[1293530146158671551, 1]);
    // - k even, so the +2 is folded into 4 * F(k) ^ 2
    // - no carry limb after F(2k + 1)
    // - no shrink after the squarings
    test(96, &[14787220707439219840, 2], &[13493690561280548289, 1]);
    test(100, &[3736710778780434371, 19], &[16008811023750101250, 11]);
    // - high-zero shrink after F(2k)
    test(
        186,
        &[14458561666841997560, 18042485370706291343],
        &[3465294890923511181, 11150869200619234444],
    );
    test(
        200,
        &[17323038258947941269, 9676648027618573582, 824],
        &[4845216997073187469, 10776774982391689558, 509],
    );
}

#[test]
fn limbs_fibonacci_pair_properties() {
    unsigned_gen_var_5().test_properties(|n| {
        let alloc = fibonacci_pair_alloc_len(n);
        let mut fs = vec![0; alloc];
        let mut f1s = vec![0; alloc];
        let size = limbs_fibonacci_pair(&mut fs, &mut f1s, n);
        fs.truncate(size);
        f1s.truncate(size);
        let f = Natural::from_owned_limbs_asc(fs);
        let f1 = Natural::from_owned_limbs_asc(f1s);
        // The naive implementation is quadratic, so only use it as an oracle for small n.
        if n < 2000 {
            assert_eq!(f, fibonacci_naive(n));
        } else {
            assert_eq!(f, Natural::fibonacci(n));
        }
        if n == 0 {
            assert_eq!(f1, 1);
        } else {
            assert_eq!(f1, Natural::fibonacci(n - 1));
        }
    });
}

#[test]
fn test_fibonacci() {
    fn test(n: u64, out: &str) {
        let f = Natural::fibonacci(n);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
    }
    test(0, "0");
    test(1, "1");
    test(2, "1");
    test(3, "2");
    test(4, "3");
    test(5, "5");
    test(10, "55");
    // - the largest table value
    test(93, "12200160415121876738");
    // - the smallest computed value (even final step, no cy carry)
    test(94, "19740274219868223167");
    // - odd final step; k odd, so 2 is subtracted; no cx carry
    test(95, "31940434634990099905");
    test(96, "51680708854858323072");
    // - odd final step; k even, so 2 is added
    test(97, "83621143489848422977");
    test(100, "354224848179261915075");
    // - cx carry in the odd final step
    // - first high-zero shrink of the product
    test(185, "205697230343233228174223751303346572685");
    // - cy carry in the even final step
    test(186, "332825110087067562321196029789634457848");
    // - cy carry in the odd final step
    test(
        373,
        "400778865046997419409593818195095036058794082069603285936485366789883567055193",
    );
    test(
        300,
        "222232244629420445529739893461909967206666939096499764990979600",
    );
    test(
        1000,
        "43466557686937456435688527675040625802564660517371780402481729089536555417949051890403879\
        840079255169295922593080322634775209689623239873322471161642996440906533187938298969649928\
        516003704476137795166849228875",
    );
    // Deterministic deep checks against the naive implementation: both final-step parities and a
    // power of two.
    for n in [1000, 8192, 10000, 10001] {
        assert_eq!(Natural::fibonacci(n), fibonacci_naive(n));
    }
}

#[test]
fn fibonacci_properties() {
    unsigned_gen_var_5().test_properties(|n| {
        let f = Natural::fibonacci(n);
        assert!(f.is_valid());
        // The naive implementation is quadratic, so only use it as an oracle for small n; rug and
        // the identities below cover the rest.
        if n < 2000 {
            assert_eq!(fibonacci_naive(n), f);
        }
        assert_eq!(
            Natural::exact_from(&rug::Integer::fibonacci(u32::exact_from(n)).complete()),
            f
        );
        let (fp, f1p) = Natural::fibonacci_pair(n);
        assert_eq!(fp, f);
        if n == 0 {
            assert_eq!(f1p, 1);
        } else {
            assert_eq!(f1p, Natural::fibonacci(n - 1));
        }
        if n >= 2 {
            assert_eq!(f, Natural::fibonacci(n - 1) + Natural::fibonacci(n - 2));
        }
        // Cassini's identity: F(n - 1) * F(n + 1) - F(n) ^ 2 = (-1) ^ n
        if n != 0 {
            let prod = f1p * Natural::fibonacci(n + 1);
            let square = f.square();
            if n.even() {
                assert_eq!(prod, square + Natural::ONE);
            } else {
                assert_eq!(prod + Natural::ONE, square);
            }
        }
    });

    // Agreement with the primitive-integer tables, over their entire range.
    unsigned_gen_var_32::<u64>().test_properties(|n| {
        assert_eq!(Natural::fibonacci(n), u64::fibonacci(n));
    });

    // gcd(F(m), F(n)) = F(gcd(m, n))
    unsigned_pair_gen_var_18::<u64, u64>().test_properties(|(m, n)| {
        assert_eq!(
            Natural::fibonacci(m).gcd(Natural::fibonacci(n)),
            Natural::fibonacci(m.gcd(n))
        );
    });
}

#[test]
fn test_fibonacci_pair() {
    fn test(n: u64, out: &str, out_1: &str) {
        let (f, f1) = Natural::fibonacci_pair(n);
        assert!(f.is_valid());
        assert!(f1.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(f1.to_string(), out_1);
    }
    // - F(-1) = 1
    test(0, "0", "1");
    test(1, "1", "0");
    test(2, "1", "1");
    test(10, "55", "34");
    test(93, "12200160415121876738", "7540113804746346429");
    test(94, "19740274219868223167", "12200160415121876738");
    test(100, "354224848179261915075", "218922995834555169026");
}

#[test]
fn fibonacci_pair_properties() {
    unsigned_gen_var_5().test_properties(|n| {
        let (f, f1) = Natural::fibonacci_pair(n);
        assert!(f.is_valid());
        assert!(f1.is_valid());
        if n < 2000 {
            assert_eq!(fibonacci_pair_naive(n), (f.clone(), f1.clone()));
        }
        let (rug_f, rug_f1) =
            <(rug::Integer, rug::Integer)>::from(rug::Integer::fibonacci_2(u32::exact_from(n)));
        assert_eq!(Natural::exact_from(&rug_f), f);
        assert_eq!(Natural::exact_from(&rug_f1), f1);
        // The pairs overlap: pair(n + 1) is (F(n) + F(n - 1), F(n)), using F(-1) = 1 at n = 0.
        let (g, g1) = Natural::fibonacci_pair(n + 1);
        assert_eq!(g1, f);
        assert_eq!(g, f + f1);
    });
}

#[test]
fn test_lucas_number() {
    fn test(n: u64, out: &str) {
        let l = Natural::lucas_number(n);
        assert!(l.is_valid());
        assert_eq!(l.to_string(), out);
    }
    test(0, "2");
    test(1, "1");
    test(2, "3");
    test(3, "4");
    test(4, "7");
    test(5, "11");
    test(10, "123");
    // - the largest n on the one-limb path
    test(92, "16860207025497407047");
    // - odd, so the L(2k + 1) formula is used immediately
    test(93, "27280388024614569596");
    // - one trailing zero bit, then the L(2k + 1) formula
    test(94, "44140595050111976643");
    test(95, "71420983074726546239");
    // - two trailing zero bits
    test(96, "115561578124838522882");
    test(100, "792070839848372253127");
    // - trailing zero bit stripped, then the odd formula; square high-zero shrink in the doubling;
    //   2F + F1 carry
    test(186, "744219570773534018669643532396327603218");
    // - F(k - 1) high-zero shrink in the odd formula
    test(189, "3152564691982405848945267213740827495676");
    test(
        1000,
        "97194177735908175207981982079326473737797879155345685082728081084772518818444815269080619\
        149045968297679578305403209347401163036907660573971740862463751801641201490284097309096322\
        681531675707666695323797578127",
    );
    // Deterministic deep checks: 1000 = 8 * 125 strips three zero bits and then uses the odd
    // formula; 4096 doubles up from a table entry; 10001 is immediately odd; 10002 strips one zero
    // bit.
    for n in [1000, 4096, 10001, 10002] {
        assert_eq!(Natural::lucas_number(n), lucas_number_naive(n));
    }
}

#[test]
fn lucas_number_properties() {
    unsigned_gen_var_5().test_properties(|n| {
        let l = Natural::lucas_number(n);
        assert!(l.is_valid());
        if n < 2000 {
            assert_eq!(lucas_number_naive(n), l);
        }
        assert_eq!(
            Natural::exact_from(&rug::Integer::lucas(u32::exact_from(n)).complete()),
            l
        );
        // L(n) = F(n) + 2 * F(n - 1), using F(-1) = 1 at n = 0.
        let (f, f1) = Natural::fibonacci_pair(n);
        assert_eq!(l, &f + (&f1 << 1u32));
        if n >= 2 {
            assert_eq!(
                l,
                Natural::lucas_number(n - 1) + Natural::lucas_number(n - 2)
            );
        }
        // L(2n) = L(n) ^ 2 - 2 * (-1) ^ n
        let l2 = Natural::lucas_number(n << 1);
        let square = (&l).square();
        if n.even() {
            assert_eq!(l2 + Natural::TWO, square);
        } else {
            assert_eq!(l2, square + Natural::TWO);
        }
        // L(n) ^ 2 - 5 * F(n) ^ 2 = 4 * (-1) ^ n
        let l_square = l.square();
        let f_square_5 = f.square() * Natural::from(5u32);
        if n.even() {
            assert_eq!(l_square, f_square_5 + Natural::from(4u32));
        } else {
            assert_eq!(l_square + Natural::from(4u32), f_square_5);
        }
    });

    // Agreement with the primitive-integer tables, over their entire range.
    unsigned_gen_var_33::<u64>().test_properties(|n| {
        assert_eq!(Natural::lucas_number(n), u64::lucas_number(n));
    });
}

#[test]
fn test_lucas_number_pair() {
    fn test(n: u64, out: &str, out_1: &str) {
        let (l, l1) = Natural::lucas_number_pair(n);
        assert!(l.is_valid());
        assert!(l1.is_valid());
        assert_eq!(l.to_string(), out);
        assert_eq!(l1.to_string(), out_1);
    }
    test(1, "1", "2");
    test(2, "3", "1");
    test(3, "4", "3");
    test(10, "123", "76");
    // - the largest table value
    test(92, "16860207025497407047", "10420180999117162549");
    // - the smallest computed value; L(n) carry into a new limb
    test(93, "27280388024614569596", "16860207025497407047");
    // - no L(n) carry
    test(94, "44140595050111976643", "27280388024614569596");
    test(100, "792070839848372253127", "489526700523968661124");
    // - L(n - 1) carry into a new limb
    test(
        186,
        "744219570773534018669643532396327603218",
        "459952989830901896468168308275922343011",
    );
}

#[test]
fn lucas_number_pair_fail() {
    // L(-1) = -1 cannot be represented as a Natural
    assert_panic!(Natural::lucas_number_pair(0));
}

#[test]
fn lucas_number_pair_properties() {
    unsigned_gen_var_11::<u64>().test_properties(|n| {
        let (l, l1) = Natural::lucas_number_pair(n);
        assert!(l.is_valid());
        assert!(l1.is_valid());
        if n < 2000 {
            assert_eq!(lucas_number_pair_naive(n), (l.clone(), l1.clone()));
        }
        assert_eq!(l, Natural::lucas_number(n));
        assert_eq!(l1, Natural::lucas_number(n - 1));
        let (rug_l, rug_l1) =
            <(rug::Integer, rug::Integer)>::from(rug::Integer::lucas_2(u32::exact_from(n)));
        assert_eq!(Natural::exact_from(&rug_l), l);
        assert_eq!(Natural::exact_from(&rug_l1), l1);
    });
}
