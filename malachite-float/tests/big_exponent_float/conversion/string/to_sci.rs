// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::basic::traits::NegativeOne;
use malachite_base::num::conversion::string::options::{
    FromSciStringOptions, SciSizeOptions, ToSciOptions,
};
use malachite_base::num::conversion::traits::{FromStringBase, ToSci};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::big_exponent_float_to_float;
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_to_sci_options_pair_gen,
    big_exponent_float_to_sci_options_pair_gen_var_1,
};

fn hex(s: &str) -> BigExponentFloat {
    BigExponentFloat::from_string_base(16, s).unwrap()
}

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

// The options for a unit row: `size` is `pN` (precision N), `pNt` (precision N with trailing
// zeros), `sN` (scale N), or `c` (complete).
fn options(base: u8, size: &str, rm: RoundingMode) -> ToSciOptions {
    let mut o = ToSciOptions::default();
    o.set_base(base);
    o.set_rounding_mode(rm);
    if size == "c" {
        o.set_size_complete();
    } else if let Some(n) = size.strip_prefix('s') {
        o.set_scale(n.parse().unwrap());
    } else {
        o.set_precision(size[1..].trim_end_matches('t').parse().unwrap());
        if size.ends_with('t') {
            o.set_include_trailing_zeros(true);
        }
    }
    o
}

// The options for reading a string in `base` with `rm`.
fn from_options(base: u8, rm: RoundingMode) -> FromSciStringOptions {
    let mut o = FromSciStringOptions::default();
    o.set_base(base);
    o.set_rounding_mode(rm);
    o
}

#[test]
fn test_to_sci() {
    // Rows outside `Float`'s exponent range were checked independently with exact decimal
    // arithmetic; the others agree with `Float`, as the properties below check too.
    let test = |s: &str, s_hex: &str, base: u8, size: &str, rm: RoundingMode, out: &str| {
        let x = hex(s_hex);
        assert_eq!(x.to_string(), s);
        let o = options(base, size, rm);
        if out == "INVALID" {
            assert!(!x.fmt_sci_valid(o), "{s_hex} {base} {size} {rm}");
            return;
        }
        assert!(x.fmt_sci_valid(o));
        let t = x.to_sci_with_options(o).to_string();
        assert_eq!(t, out, "{s_hex} {base} {size} {rm}");
        if let Some(f) = big_exponent_float_to_float(&x) {
            assert_eq!(f.to_sci_with_options(o).to_string(), t);
        }
    };
    test("1.5", "0x1.8", 10, "p1", Nearest, "2.0");
    test("1.5", "0x1.8", 10, "p5", Floor, "1.5");
    test("1.5", "0x1.8", 10, "p5", Ceiling, "1.5");
    test("1.5", "0x1.8", 10, "p20", Nearest, "1.5");
    test("1.5", "0x1.8", 10, "p8t", Down, "1.5000000");
    test("1.5", "0x1.8", 3, "p10", Down, "1.111111111");
    test("1.5", "0x1.8", 16, "p8", Up, "1.8");
    test("1.5", "0x1.8", 36, "p6", Nearest, "1.i");
    test("1.5", "0x1.8", 2, "p4", Nearest, "1.1");
    test("1.5", "0x1.8", 10, "s3", Nearest, "1.5");
    test("1.5", "0x1.8", 10, "s0", Floor, "1.0");
    test("1.5", "0x1.8", 16, "c", Nearest, "1.8");
    test("1.5", "0x1.8", 10, "c", Nearest, "1.5");
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        "p1",
        Nearest,
        "3.0",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        "p5",
        Floor,
        "3.1415",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        "p5",
        Ceiling,
        "3.1416",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        "p20",
        Nearest,
        "3.141592653589793116",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        "p8t",
        Down,
        "3.1415926",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        3,
        "p10",
        Down,
        "10.01021101",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        16,
        "p8",
        Up,
        "3.243f6a9",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        36,
        "p6",
        Nearest,
        "3.53i5a",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        2,
        "p4",
        Nearest,
        "11.01",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        "s3",
        Nearest,
        "3.142",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        "s0",
        Floor,
        "3.0",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        16,
        "c",
        Nearest,
        "3.243f6a8885a3",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        "c",
        Nearest,
        "3.141592653589793115997963468544185161590576171875",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        "p1",
        Nearest,
        "0.1",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        "p5",
        Floor,
        "0.1",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        "p5",
        Ceiling,
        "0.10001",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        "p20",
        Nearest,
        "0.10000000000000000555",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        "p8t",
        Down,
        "0.10000000",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        3,
        "p10",
        Down,
        "0.002200220022",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        16,
        "p8",
        Up,
        "0.1999999a",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        36,
        "p6",
        Nearest,
        "0.3llllm",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        2,
        "p4",
        Nearest,
        "0.0001101",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        "s3",
        Nearest,
        "0.1",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        "s0",
        Floor,
        "0.0",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        16,
        "c",
        Nearest,
        "0.1999999999999a",
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a",
        10,
        "c",
        Nearest,
        "0.1000000000000000055511151231257827021181583404541015625",
    );
    test("-1000.0", "-0x3e8.0", 10, "p1", Nearest, "-1.0e3");
    test("-1000.0", "-0x3e8.0", 10, "p5", Floor, "-1000.0");
    test("-1000.0", "-0x3e8.0", 10, "p5", Ceiling, "-1000.0");
    test("-1000.0", "-0x3e8.0", 10, "p20", Nearest, "-1000.0");
    test("-1000.0", "-0x3e8.0", 10, "p8t", Down, "-1000.0000");
    test("-1000.0", "-0x3e8.0", 3, "p10", Down, "-1101001.0");
    test("-1000.0", "-0x3e8.0", 16, "p8", Up, "-3e8.0");
    test("-1000.0", "-0x3e8.0", 36, "p6", Nearest, "-rs.0");
    test("-1000.0", "-0x3e8.0", 2, "p4", Nearest, "-1.0e10");
    test("-1000.0", "-0x3e8.0", 10, "s3", Nearest, "-1000.0");
    test("-1000.0", "-0x3e8.0", 10, "s0", Floor, "-1000.0");
    test("-1000.0", "-0x3e8.0", 16, "c", Nearest, "-3e8.0");
    test("-1000.0", "-0x3e8.0", 10, "c", Nearest, "-1000.0");
    test("9.1e-13", "0x1.0E-10", 10, "p1", Nearest, "9.0e-13");
    test("9.1e-13", "0x1.0E-10", 10, "p5", Floor, "9.0949e-13");
    test("9.1e-13", "0x1.0E-10", 10, "p5", Ceiling, "9.095e-13");
    test(
        "9.1e-13",
        "0x1.0E-10",
        10,
        "p20",
        Nearest,
        "9.0949470177292823792e-13",
    );
    test("9.1e-13", "0x1.0E-10", 10, "p8t", Down, "9.0949470e-13");
    test("9.1e-13", "0x1.0E-10", 3, "p10", Down, "2.022102022e-26");
    test("9.1e-13", "0x1.0E-10", 16, "p8", Up, "1.0e-10");
    test("9.1e-13", "0x1.0E-10", 36, "p6", Nearest, "2.kd98qe-8");
    test("9.1e-13", "0x1.0E-10", 2, "p4", Nearest, "1.0e-40");
    test("9.1e-13", "0x1.0E-10", 10, "s3", Nearest, "0.0");
    test("9.1e-13", "0x1.0E-10", 10, "s0", Floor, "0.0");
    test("9.1e-13", "0x1.0E-10", 16, "c", Nearest, "1.0e-10");
    test(
        "9.1e-13",
        "0x1.0E-10",
        10,
        "c",
        Nearest,
        "9.094947017729282379150390625e-13",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        10,
        "p1",
        Nearest,
        "4.0e121",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        10,
        "p5",
        Floor,
        "4.1315e121",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        10,
        "p5",
        Ceiling,
        "4.1316e121",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        10,
        "p20",
        Nearest,
        "4.131599558676515016e121",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        10,
        "p8t",
        Down,
        "4.1315995e121",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        3,
        "p10",
        Down,
        "2.200020001e254",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        16,
        "p8",
        Up,
        "f.fffffe+100",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        36,
        "p6",
        Nearest,
        "1.odcbue+78",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        2,
        "p4",
        Nearest,
        "1.0e404",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        10,
        "s3",
        Nearest,
        "41315995586765150159839755984608183730299429867783871801200132957882024186911848395498662439090564372667223201149710499840.0",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        10,
        "s0",
        Floor,
        "41315995586765150159839755984608183730299429867783871801200132957882024186911848395498662439090564372667223201149710499840.0",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        16,
        "c",
        Nearest,
        "ffffff00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0",
    );
    test(
        "4.13159956e121",
        "0xf.fffffE+100",
        10,
        "c",
        Nearest,
        "41315995586765150159839755984608183730299429867783871801200132957882024186911848395498662439090564372667223201149710499840.0",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        10,
        "p1",
        Nearest,
        "2.0e30102999566398119521",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        10,
        "p5",
        Floor,
        "2.3653e30102999566398119521",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        10,
        "p5",
        Ceiling,
        "2.3654e30102999566398119521",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        10,
        "p20",
        Nearest,
        "2.3653176496878716463e30102999566398119521",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        10,
        "p8t",
        Down,
        "2.3653176e30102999566398119521",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        3,
        "p10",
        Down,
        "2.211220021e63092975357145743709",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        16,
        "p8",
        Up,
        "1.0e+25000000000000000000",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        36,
        "p6",
        Nearest,
        "6.9qm1pe+19342640361727079343",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        2,
        "p4",
        Nearest,
        "1.0e100000000000000000000",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        10,
        "p1",
        Nearest,
        "4.0e-30102999566398119522",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        10,
        "p5",
        Floor,
        "4.2277e-30102999566398119522",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        10,
        "p5",
        Ceiling,
        "4.2278e-30102999566398119522",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        10,
        "p20",
        Nearest,
        "4.2277619673279842065e-30102999566398119522",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        10,
        "p8t",
        Down,
        "4.2277619e-30102999566398119522",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        3,
        "p10",
        Down,
        "1.001102212e-63092975357145743710",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        16,
        "p8",
        Up,
        "1.0e-25000000000000000000",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        36,
        "p6",
        Nearest,
        "5.qoiike-19342640361727079344",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        2,
        "p4",
        Nearest,
        "1.0e-100000000000000000000",
    );
    test(
        "1.3185758957267685e323228497",
        "0x3.243f6a8885a30E+268435456",
        10,
        "p1",
        Nearest,
        "1.0e323228497",
    );
    test(
        "1.3185758957267685e323228497",
        "0x3.243f6a8885a30E+268435456",
        10,
        "p5",
        Floor,
        "1.3185e323228497",
    );
    test(
        "1.3185758957267685e323228497",
        "0x3.243f6a8885a30E+268435456",
        10,
        "p5",
        Ceiling,
        "1.3186e323228497",
    );
    test(
        "1.3185758957267685e323228497",
        "0x3.243f6a8885a30E+268435456",
        10,
        "p20",
        Nearest,
        "1.3185758957267685138e323228497",
    );
    test(
        "1.3185758957267685e323228497",
        "0x3.243f6a8885a30E+268435456",
        10,
        "p8t",
        Down,
        "1.3185758e323228497",
    );
    test(
        "1.3185758957267685e323228497",
        "0x3.243f6a8885a30E+268435456",
        3,
        "p10",
        Down,
        "1.122122021e677455665",
    );
    test(
        "1.3185758957267685e323228497",
        "0x3.243f6a8885a30E+268435456",
        16,
        "p8",
        Up,
        "3.243f6a9e+268435456",
    );
    test(
        "1.3185758957267685e323228497",
        "0x3.243f6a8885a30E+268435456",
        36,
        "p6",
        Nearest,
        "e.nlhkhe+207690019",
    );
    test(
        "1.3185758957267685e323228497",
        "0x3.243f6a8885a30E+268435456",
        2,
        "p4",
        Nearest,
        "1.101e1073741825",
    );
    test(
        "-5.5e-323228502",
        "-0x1.8E-268435460",
        10,
        "p1",
        Nearest,
        "-5.0e-323228502",
    );
    test(
        "-5.5e-323228502",
        "-0x1.8E-268435460",
        10,
        "p5",
        Floor,
        "-5.4533e-323228502",
    );
    test(
        "-5.5e-323228502",
        "-0x1.8E-268435460",
        10,
        "p5",
        Ceiling,
        "-5.4532e-323228502",
    );
    test(
        "-5.5e-323228502",
        "-0x1.8E-268435460",
        10,
        "p20",
        Nearest,
        "-5.4532582967100930936e-323228502",
    );
    test(
        "-5.5e-323228502",
        "-0x1.8E-268435460",
        10,
        "p8t",
        Down,
        "-5.4532582e-323228502",
    );
    test(
        "-5.5e-323228502",
        "-0x1.8E-268435460",
        3,
        "p10",
        Down,
        "-2.12010001e-677455675",
    );
    test(
        "-5.5e-323228502",
        "-0x1.8E-268435460",
        16,
        "p8",
        Up,
        "-1.8e-268435460",
    );
    test(
        "-5.5e-323228502",
        "-0x1.8E-268435460",
        36,
        "p6",
        Nearest,
        "-8.8o4mme-207690023",
    );
    test(
        "-5.5e-323228502",
        "-0x1.8E-268435460",
        2,
        "p4",
        Nearest,
        "-1.1e-1073741840",
    );
    test(
        "4.2e323228496",
        "0x1.0E+268435456",
        10,
        "p1",
        Nearest,
        "4.0e323228496",
    );
    test(
        "4.2e323228496",
        "0x1.0E+268435456",
        10,
        "p5",
        Floor,
        "4.1971e323228496",
    );
    test(
        "4.2e323228496",
        "0x1.0E+268435456",
        10,
        "p5",
        Ceiling,
        "4.1972e323228496",
    );
    test(
        "4.2e323228496",
        "0x1.0E+268435456",
        10,
        "p20",
        Nearest,
        "4.1971574329347753848e323228496",
    );
    test(
        "4.2e323228496",
        "0x1.0E+268435456",
        10,
        "p8t",
        Down,
        "4.1971574e323228496",
    );
    test(
        "4.2e323228496",
        "0x1.0E+268435456",
        3,
        "p10",
        Down,
        "1.120121221e677455664",
    );
    test(
        "4.2e323228496",
        "0x1.0E+268435456",
        16,
        "p8",
        Up,
        "1.0e+268435456",
    );
    test(
        "4.2e323228496",
        "0x1.0E+268435456",
        36,
        "p6",
        Nearest,
        "4.nxtc9e+207690019",
    );
    test(
        "4.2e323228496",
        "0x1.0E+268435456",
        2,
        "p4",
        Nearest,
        "1.0e1073741824",
    );
    test(
        "-2.10064313e120411998265592478085",
        "-0xa.bcdefE+99999999999999999999",
        10,
        "p1",
        Nearest,
        "-2.0e120411998265592478085",
    );
    test(
        "-2.10064313e120411998265592478085",
        "-0xa.bcdefE+99999999999999999999",
        10,
        "p5",
        Floor,
        "-2.1007e120411998265592478085",
    );
    test(
        "-2.10064313e120411998265592478085",
        "-0xa.bcdefE+99999999999999999999",
        10,
        "p5",
        Ceiling,
        "-2.1006e120411998265592478085",
    );
    test(
        "-2.10064313e120411998265592478085",
        "-0xa.bcdefE+99999999999999999999",
        10,
        "p20",
        Nearest,
        "-2.100643127936978423e120411998265592478085",
    );
    test(
        "-2.10064313e120411998265592478085",
        "-0xa.bcdefE+99999999999999999999",
        10,
        "p8t",
        Down,
        "-2.1006431e120411998265592478085",
    );
    test(
        "-2.10064313e120411998265592478085",
        "-0xa.bcdefE+99999999999999999999",
        3,
        "p10",
        Down,
        "-1.122011022e252371901428582974839",
    );
    test(
        "-2.10064313e120411998265592478085",
        "-0xa.bcdefE+99999999999999999999",
        16,
        "p8",
        Up,
        "-a.bcdefe+99999999999999999999",
    );
    test(
        "-2.10064313e120411998265592478085",
        "-0xa.bcdefE+99999999999999999999",
        36,
        "p6",
        Nearest,
        "-s.tk3r3e+77370561446908317373",
    );
    test(
        "-2.10064313e120411998265592478085",
        "-0xa.bcdefE+99999999999999999999",
        2,
        "p4",
        Nearest,
        "-1.011e399999999999999999999",
    );
    test("0.0", "0x0.0", 10, "p1", Nearest, "0.0");
    test("0.0", "0x0.0", 10, "p5", Floor, "0.0");
    test("0.0", "0x0.0", 10, "p5", Ceiling, "0.0");
    test("0.0", "0x0.0", 10, "p20", Nearest, "0.0");
    test("0.0", "0x0.0", 10, "p8t", Down, "0.0000000");
    test("0.0", "0x0.0", 3, "p10", Down, "0.0");
    test("0.0", "0x0.0", 16, "p8", Up, "0.0");
    test("0.0", "0x0.0", 36, "p6", Nearest, "0.0");
    test("0.0", "0x0.0", 2, "p4", Nearest, "0.0");
    test("0.0", "0x0.0", 10, "s3", Nearest, "0.0");
    test("0.0", "0x0.0", 10, "s0", Floor, "0.0");
    test("0.0", "0x0.0", 16, "c", Nearest, "0.0");
    test("0.0", "0x0.0", 10, "c", Nearest, "0.0");
    test("NaN", "NaN", 10, "p1", Nearest, "NaN");
    test("NaN", "NaN", 10, "p5", Floor, "NaN");
    test("NaN", "NaN", 10, "p5", Ceiling, "NaN");
    test("NaN", "NaN", 10, "p20", Nearest, "NaN");
    test("NaN", "NaN", 10, "p8t", Down, "NaN");
    test("NaN", "NaN", 3, "p10", Down, "NaN");
    test("NaN", "NaN", 16, "p8", Up, "NaN");
    test("NaN", "NaN", 36, "p6", Nearest, "NaN");
    test("NaN", "NaN", 2, "p4", Nearest, "NaN");
    test("NaN", "NaN", 10, "s3", Nearest, "NaN");
    test("NaN", "NaN", 10, "s0", Floor, "NaN");
    test("NaN", "NaN", 16, "c", Nearest, "NaN");
    test("NaN", "NaN", 10, "c", Nearest, "NaN");
    test("Infinity", "Infinity", 10, "p1", Nearest, "Infinity");
    test("Infinity", "Infinity", 10, "p5", Floor, "Infinity");
    test("Infinity", "Infinity", 10, "p5", Ceiling, "Infinity");
    test("Infinity", "Infinity", 10, "p20", Nearest, "Infinity");
    test("Infinity", "Infinity", 10, "p8t", Down, "Infinity");
    test("Infinity", "Infinity", 3, "p10", Down, "Infinity");
    test("Infinity", "Infinity", 16, "p8", Up, "Infinity");
    test("Infinity", "Infinity", 36, "p6", Nearest, "Infinity");
    test("Infinity", "Infinity", 2, "p4", Nearest, "Infinity");
    test("Infinity", "Infinity", 10, "s3", Nearest, "Infinity");
    test("Infinity", "Infinity", 10, "s0", Floor, "Infinity");
    test("Infinity", "Infinity", 16, "c", Nearest, "Infinity");
    test("Infinity", "Infinity", 10, "c", Nearest, "Infinity");
    test("-Infinity", "-Infinity", 10, "p1", Nearest, "-Infinity");
    test("-Infinity", "-Infinity", 10, "p5", Floor, "-Infinity");
    test("-Infinity", "-Infinity", 10, "p5", Ceiling, "-Infinity");
    test("-Infinity", "-Infinity", 10, "p20", Nearest, "-Infinity");
    test("-Infinity", "-Infinity", 10, "p8t", Down, "-Infinity");
    test("-Infinity", "-Infinity", 3, "p10", Down, "-Infinity");
    test("-Infinity", "-Infinity", 16, "p8", Up, "-Infinity");
    test("-Infinity", "-Infinity", 36, "p6", Nearest, "-Infinity");
    test("-Infinity", "-Infinity", 2, "p4", Nearest, "-Infinity");
    test("-Infinity", "-Infinity", 10, "s3", Nearest, "-Infinity");
    test("-Infinity", "-Infinity", 10, "s0", Floor, "-Infinity");
    test("-Infinity", "-Infinity", 16, "c", Nearest, "-Infinity");
    test("-Infinity", "-Infinity", 10, "c", Nearest, "-Infinity");
}

#[test]
#[should_panic]
fn to_sci_fail_1() {
    // `Exact`, but 1.5 needs two digits
    BigExponentFloat::from(1.5)
        .to_sci_with_options(options(10, "p1", Exact))
        .to_string();
}

#[test]
#[should_panic]
fn to_sci_fail_2() {
    // a fractional value has no complete expansion in an odd base
    BigExponentFloat::from(1.5)
        .to_sci_with_options(options(3, "c", Nearest))
        .to_string();
}

// The rounding mode for -x that rounds as `rm` rounds x.
const fn negated_rounding_mode(rm: RoundingMode) -> RoundingMode {
    match rm {
        Floor => Ceiling,
        Ceiling => Floor,
        rm => rm,
    }
}

#[test]
fn to_sci_properties() {
    big_exponent_float_to_sci_options_pair_gen_var_1().test_properties(|(x, options)| {
        let s = x.to_sci_with_options(options).to_string();
        // - agreement with `Float`
        if let Some(f) = big_exponent_float_to_float(&x) {
            assert!(f.fmt_sci_valid(options));
            assert_eq!(f.to_sci_with_options(options).to_string(), s);
        }
        if !x.is_finite() || x == 0u32 {
            return;
        }
        // - negation
        let mut negated_options = options;
        negated_options.set_rounding_mode(negated_rounding_mode(options.get_rounding_mode()));
        let t = (&x * BigExponentFloat::NEGATIVE_ONE)
            .to_sci_with_options(negated_options)
            .to_string();
        if let Some(abs) = s.strip_prefix('-') {
            assert_eq!(t, abs);
        } else {
            assert_eq!(t, format!("-{s}"));
        }
        // - the string's value is on the side of x that the rounding mode says: read it back
        //   rounded the other way, at a precision where that changes nothing that matters
        let digits = match options.get_size_options() {
            SciSizeOptions::Precision(p) => p,
            _ => 64,
        };
        let prec = x.get_prec().unwrap() + digits * 6 + 64;
        let base = options.get_base();
        let back = |rm| {
            BigExponentFloat::from_sci_string_with_options_prec(&s, from_options(base, rm), prec)
                .unwrap()
                .0
        };
        match options.get_rounding_mode() {
            Floor => assert!(back(Floor) <= x),
            Ceiling => assert!(back(Ceiling) >= x),
            Down => assert!(back(Floor).abs() <= (&x).abs() || back(Ceiling).abs() <= (&x).abs()),
            Up => assert!(back(Floor).abs() >= (&x).abs() || back(Ceiling).abs() >= (&x).abs()),
            Exact => assert!(same(&back(Floor), &x)),
            Nearest => {}
        }
    });
}

#[test]
fn to_sci_default_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let s = x.to_sci().to_string();
        if let Some(f) = big_exponent_float_to_float(&x) {
            assert_eq!(f.to_sci().to_string(), s);
        }
    });
}

#[test]
fn fmt_sci_valid_properties() {
    // - this generator, unlike the one above, includes invalid pairs
    big_exponent_float_to_sci_options_pair_gen().test_properties(|(x, options)| {
        let valid = x.fmt_sci_valid(options);
        if let Some(f) = big_exponent_float_to_float(&x) {
            assert_eq!(f.fmt_sci_valid(options), valid);
        }
        if !valid {
            assert!(
                options.get_rounding_mode() == Exact
                    || options.get_size_options() == SciSizeOptions::Complete
            );
        }
    });
}
