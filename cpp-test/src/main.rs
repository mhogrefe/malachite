// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Differential testing of Malachite against FLINT, without FFI: each Malachite demo's output is
// captured to a text file, and a small C oracle (the sources in `oracle/`) recomputes every
// line with FLINT and fails on the first disagreement. The FLINT source is not part of this repository; a
// built FLINT source tree is located through the `MALACHITE_FLINT_DIR` environment variable,
// defaulting to `../../flint-3.6.0`. See README.md.

use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const MODES: [&str; 3] = ["exhaustive", "random", "special_random"];
const LIMIT: usize = 10000;
const TEST_OUT: &str = "test-out.txt";

fn flint_dir() -> PathBuf {
    let dir = env::var("MALACHITE_FLINT_DIR").unwrap_or_else(|_| "../../flint-3.6.0".to_string());
    fs::canonicalize(&dir).unwrap_or_else(|_| {
        panic!(
            "no built FLINT tree at {dir}; build one (bootstrap.sh, configure, make) or set \
            MALACHITE_FLINT_DIR"
        )
    })
}

// Compiles the oracle sources in `oracle/` against the FLINT tree if the binary is missing or
// stale (judged against the newest source or header), returning
// the binary's path.
fn build_oracle() -> PathBuf {
    let flint = flint_dir();
    let binary = Path::new("target").join("flint-oracle");
    let mut sources = Vec::new();
    let mut newest = None;
    for entry in fs::read_dir("oracle").unwrap() {
        let path = entry.unwrap().path();
        match path.extension().and_then(|e| e.to_str()) {
            Some("c") => sources.push(path.clone()),
            Some("h") => {}
            _ => continue,
        }
        let modified = fs::metadata(&path).unwrap().modified().unwrap();
        newest = Some(newest.map_or(modified, |n: std::time::SystemTime| n.max(modified)));
    }
    sources.sort();
    let stale = match (fs::metadata(&binary), newest) {
        (Ok(bin), Some(newest)) => newest >= bin.modified().unwrap(),
        _ => true,
    };
    if stale {
        fs::create_dir_all("target").unwrap();
        let status = Command::new("cc")
            .arg("-O2")
            .arg("-Wall")
            .arg("-Wextra")
            .args(&sources)
            .arg("-o")
            .arg(&binary)
            .arg(format!(
                "-I{}",
                flint.join("build").join("include").display()
            ))
            .arg(format!("-L{}", flint.display()))
            .arg(format!("-Wl,-rpath,{}", flint.display()))
            .arg("-lflint")
            .status()
            .expect("failed to run cc");
        assert!(status.success(), "failed to build flint-oracle");
    }
    binary
}

fn run_oracle(oracle: &Path, mode: &str, input: Option<&str>) {
    let mut command = Command::new(oracle);
    command.arg(mode);
    if let Some(input) = input {
        command.arg(input);
    }
    let output = command.output().expect("failed to run the FLINT oracle");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert!(
        output.status.success(),
        "FLINT oracle failed in mode {mode}: {:?}",
        output.status
    );
}

// Runs a Malachite demo in the given generator mode, capturing its output to `TEST_OUT`.
fn run_demo(crate_dir: &str, demo_name: &str, mode: &str) {
    let output_file = File::create(TEST_OUT).unwrap();
    let mut command = Command::new("cargo");
    command
        .arg("run")
        .arg("--release")
        .arg("-j")
        .arg("4")
        .arg("--features")
        .arg("bin_build")
        .arg("--")
        .arg("-l")
        .arg(format!("{LIMIT}"))
        .arg("-m")
        .arg(mode)
        .arg("-d")
        .arg(demo_name);
    command.current_dir(crate_dir);
    command.stdout(Stdio::from(output_file));
    let output = command.output().expect("failed to run Malachite demo");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success(), "demo {demo_name} failed");
}

// Runs a Malachite demo in every generator mode, diffing each run's output against FLINT.
fn check_demo_against_flint(oracle: &Path, crate_dir: &str, demo_name: &str, flint_mode: &str) {
    for mode in MODES {
        println!("testing {demo_name} in mode {mode}");
        run_demo(crate_dir, demo_name, mode);
        run_oracle(oracle, flint_mode, Some(TEST_OUT));
    }
}

// For demos whose generators do not support the special_random mode, such as those built on
// primitive-integer generators declared with new_no_special.
fn check_demo_against_flint_no_special(
    oracle: &Path,
    crate_dir: &str,
    demo_name: &str,
    flint_mode: &str,
) {
    for mode in &MODES[..2] {
        println!("testing {demo_name} in mode {mode}");
        run_demo(crate_dir, demo_name, mode);
        run_oracle(oracle, flint_mode, Some(TEST_OUT));
    }
}

fn write_primitive_root_prime_unit_test(output_file: &mut File, n: u64, out: u64) {
    writeln!(output_file, "primitive_root_prime({n}) = {out}").unwrap();
}

// The rows of test_evaluate in malachite-nz's IntegerPolynomial tests: polynomial, point, value.
const EVALUATE_UNIT_ROWS: [(&str, &str, &str); 22] = [
    ("0", "0", "0"),
    ("0", "5", "0"),
    ("5", "0", "5"),
    ("5", "-3", "5"),
    ("x", "7", "7"),
    ("x", "-7", "-7"),
    ("x^2+3*x+2", "0", "2"),
    ("x^2+3*x+2", "1", "6"),
    ("x^2+3*x+2", "-1", "0"),
    ("x^2+3*x+2", "-2", "0"),
    ("x^2+3*x+2", "10", "132"),
    ("-x^3+x", "2", "-6"),
    ("2*x^5-x", "-3", "-483"),
    ("1000000000000000000000*x+1", "1000000000000000000000", "1000000000000000000000000000000000000000001"),
    ("x^100", "2", "1267650600228229401496703205376"),
    ("x^100-1", "-1", "0"),
    ("x^100-1", "1", "0"),
    ("-x^51+x^50", "3", "-1435795975383705177540498"),
    ("x^64+x^63+x^62+x^61+x^60+x^59+x^58+x^57+x^56+x^55+x^54+x^53+x^52+x^51+x^50+x^49+x^48+x^47+x^46+x^45+x^44+x^43+x^42+x^41+x^40+x^39+x^38+x^37+x^36+x^35+x^34+x^33+x^32+x^31+x^30+x^29+x^28+x^27+x^26+x^25+x^24+x^23+x^22+x^21+x^20+x^19+x^18+x^17+x^16+x^15+x^14+x^13+x^12+x^11+x^10+x^9+x^8+x^7+x^6+x^5+x^4+x^3+x^2+x+1", "2", "36893488147419103231"),
    ("-60*x^59+59*x^58-58*x^57+57*x^56-56*x^55+55*x^54-54*x^53+53*x^52-52*x^51+51*x^50-50*x^49+49*x^48-48*x^47+47*x^46-46*x^45+45*x^44-44*x^43+43*x^42-42*x^41+41*x^40-40*x^39+39*x^38-38*x^37+37*x^36-36*x^35+35*x^34-34*x^33+33*x^32-32*x^31+31*x^30-30*x^29+29*x^28-28*x^27+27*x^26-26*x^25+25*x^24-24*x^23+23*x^22-22*x^21+21*x^20-20*x^19+19*x^18-18*x^17+17*x^16-16*x^15+15*x^14-14*x^13+13*x^12-12*x^11+11*x^10-10*x^9+9*x^8-8*x^7+7*x^6-6*x^5+5*x^4-4*x^3+3*x^2-2*x+1", "-7", "5066106889042355226407376748809603448230666511559010"),
    ("9784*x^99+9587*x^98+9392*x^97+9199*x^96+9008*x^95+8819*x^94+8632*x^93+8447*x^92+8264*x^91+8083*x^90+7904*x^89+7727*x^88+7552*x^87+7379*x^86+7208*x^85+7039*x^84+6872*x^83+6707*x^82+6544*x^81+6383*x^80+6224*x^79+6067*x^78+5912*x^77+5759*x^76+5608*x^75+5459*x^74+5312*x^73+5167*x^72+5024*x^71+4883*x^70+4744*x^69+4607*x^68+4472*x^67+4339*x^66+4208*x^65+4079*x^64+3952*x^63+3827*x^62+3704*x^61+3583*x^60+3464*x^59+3347*x^58+3232*x^57+3119*x^56+3008*x^55+2899*x^54+2792*x^53+2687*x^52+2584*x^51+2483*x^50+2384*x^49+2287*x^48+2192*x^47+2099*x^46+2008*x^45+1919*x^44+1832*x^43+1747*x^42+1664*x^41+1583*x^40+1504*x^39+1427*x^38+1352*x^37+1279*x^36+1208*x^35+1139*x^34+1072*x^33+1007*x^32+944*x^31+883*x^30+824*x^29+767*x^28+712*x^27+659*x^26+608*x^25+559*x^24+512*x^23+467*x^22+424*x^21+383*x^20+344*x^19+307*x^18+272*x^17+239*x^16+208*x^15+179*x^14+152*x^13+127*x^12+104*x^11+83*x^10+64*x^9+47*x^8+32*x^7+19*x^6+8*x^5-x^4-8*x^3-13*x^2-16*x-17", "123456789", "11233088674388196659638106735131591839166548083223584752609337477840565263920759467205201792569605062323689106967242747447142719735787704935988060074488242556418098342821416865689089919782161504815853514764869873044444515035189993474293983892651357072996407659905992643370741697506205191413279313299517054957672257373531426095292237506291594818469678780098989926572878993093480071655931849110899831120462935877745542469464218352312688993920936452050494412396196484060948698541230081486603196996630328122818305356600433091601255228498382535348704462211149100175714658354611477366969883180664273267520510824913404473291092274169854120387534200186218496993372669172389310944542158471295824345459646532687811319382642589462110839631441644713386357192784175490905703838543133970901163980134681850567601726986050"),
    ("100000000000000000050*x^50+100000000000000000049*x^49+100000000000000000048*x^48+100000000000000000047*x^47+100000000000000000046*x^46+100000000000000000045*x^45+100000000000000000044*x^44+100000000000000000043*x^43+100000000000000000042*x^42+100000000000000000041*x^41+100000000000000000040*x^40+100000000000000000039*x^39+100000000000000000038*x^38+100000000000000000037*x^37+100000000000000000036*x^36+100000000000000000035*x^35+100000000000000000034*x^34+100000000000000000033*x^33+100000000000000000032*x^32+100000000000000000031*x^31+100000000000000000030*x^30+100000000000000000029*x^29+100000000000000000028*x^28+100000000000000000027*x^27+100000000000000000026*x^26+100000000000000000025*x^25+100000000000000000024*x^24+100000000000000000023*x^23+100000000000000000022*x^22+100000000000000000021*x^21+100000000000000000020*x^20+100000000000000000019*x^19+100000000000000000018*x^18+100000000000000000017*x^17+100000000000000000016*x^16+100000000000000000015*x^15+100000000000000000014*x^14+100000000000000000013*x^13+100000000000000000012*x^12+100000000000000000011*x^11+100000000000000000010*x^10+100000000000000000009*x^9+100000000000000000008*x^8+100000000000000000007*x^7+100000000000000000006*x^6+100000000000000000005*x^5+100000000000000000004*x^4+100000000000000000003*x^3+100000000000000000002*x^2+100000000000000000001*x+100000000000000000000", "-10000000000", "9999999999000000005099999999500000000048999999995200000000469999999954000000004499999999560000000042999999995800000000409999999960000000003899999999620000000036999999996400000000349999999966000000003299999999680000000030999999997000000000289999999972000000002699999999740000000024999999997600000000229999999978000000002099999999800000000018999999998200000000169999999984000000001499999999860000000012999999998800000000109999999990000000000899999999920000000006999999999400000000049999999996000000000299999999990000000000"),
];

// The rows of test_evaluate in malachite-nz's NaturalPolynomial tests: polynomial, point, value.
const NATURAL_EVALUATE_UNIT_ROWS: [(&str, &str, &str); 17] = [
    ("0", "0", "0"),
    ("0", "5", "0"),
    ("5", "0", "5"),
    ("5", "3", "5"),
    ("x", "7", "7"),
    ("x^2+3*x+2", "0", "2"),
    ("x^2+3*x+2", "1", "6"),
    ("x^2+3*x+2", "10", "132"),
    ("2*x^5+x", "3", "489"),
    ("1000000000000000000000*x+1", "1000000000000000000000", "1000000000000000000000000000000000000000001"),
    ("x^100", "2", "1267650600228229401496703205376"),
    ("x^100+1", "1", "2"),
    ("x^51+x^50", "3", "2871591950767410355080996"),
    ("x^64+x^63+x^62+x^61+x^60+x^59+x^58+x^57+x^56+x^55+x^54+x^53+x^52+x^51+x^50+x^49+x^48+x^47+x^46+x^45+x^44+x^43+x^42+x^41+x^40+x^39+x^38+x^37+x^36+x^35+x^34+x^33+x^32+x^31+x^30+x^29+x^28+x^27+x^26+x^25+x^24+x^23+x^22+x^21+x^20+x^19+x^18+x^17+x^16+x^15+x^14+x^13+x^12+x^11+x^10+x^9+x^8+x^7+x^6+x^5+x^4+x^3+x^2+x+1", "2", "36893488147419103231"),
    ("60*x^59+59*x^58+58*x^57+57*x^56+56*x^55+55*x^54+54*x^53+53*x^52+52*x^51+51*x^50+50*x^49+49*x^48+48*x^47+47*x^46+46*x^45+45*x^44+44*x^43+43*x^42+42*x^41+41*x^40+40*x^39+39*x^38+38*x^37+37*x^36+36*x^35+35*x^34+34*x^33+33*x^32+32*x^31+31*x^30+30*x^29+29*x^28+28*x^27+27*x^26+26*x^25+25*x^24+24*x^23+23*x^22+22*x^21+21*x^20+20*x^19+19*x^18+18*x^17+17*x^16+16*x^15+15*x^14+14*x^13+13*x^12+12*x^11+11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1", "7", "5066106889042355226407376748809603448230666511559010"),
    ("9818*x^99+9621*x^98+9426*x^97+9233*x^96+9042*x^95+8853*x^94+8666*x^93+8481*x^92+8298*x^91+8117*x^90+7938*x^89+7761*x^88+7586*x^87+7413*x^86+7242*x^85+7073*x^84+6906*x^83+6741*x^82+6578*x^81+6417*x^80+6258*x^79+6101*x^78+5946*x^77+5793*x^76+5642*x^75+5493*x^74+5346*x^73+5201*x^72+5058*x^71+4917*x^70+4778*x^69+4641*x^68+4506*x^67+4373*x^66+4242*x^65+4113*x^64+3986*x^63+3861*x^62+3738*x^61+3617*x^60+3498*x^59+3381*x^58+3266*x^57+3153*x^56+3042*x^55+2933*x^54+2826*x^53+2721*x^52+2618*x^51+2517*x^50+2418*x^49+2321*x^48+2226*x^47+2133*x^46+2042*x^45+1953*x^44+1866*x^43+1781*x^42+1698*x^41+1617*x^40+1538*x^39+1461*x^38+1386*x^37+1313*x^36+1242*x^35+1173*x^34+1106*x^33+1041*x^32+978*x^31+917*x^30+858*x^29+801*x^28+746*x^27+693*x^26+642*x^25+593*x^24+546*x^23+501*x^22+458*x^21+417*x^20+378*x^19+341*x^18+306*x^17+273*x^16+242*x^15+213*x^14+186*x^13+161*x^12+138*x^11+117*x^10+98*x^9+81*x^8+66*x^7+53*x^6+42*x^5+33*x^4+26*x^3+21*x^2+18*x+17", "123456789", "11272124346402862226822055002451805006164768168677155253992094690087348941850340407059751269334375094532325065293972983593992817388853167630623870017660297504613703464134701626182477789599315874234410693190467936702209984255685704902683309727393249187628781634029018502588280542713690192124271702206035686109683450156552174931718758061550884762448158819596795912435752893331494397385244133022188554384190613767967182738927800292634953823073228272091229761095587680255404764510387184538170247932370334455025149090296543199231032445306671278334942970561737259906692916121327986141714983850522586611720378997095429222436463227755242634934326731353366490301063485041893597295346660443175924059646556798986928468823905760993444391430839005571311018023001184969111670815598927836492704496065582699776382728749050"),
    ("100000000000000000050*x^50+100000000000000000049*x^49+100000000000000000048*x^48+100000000000000000047*x^47+100000000000000000046*x^46+100000000000000000045*x^45+100000000000000000044*x^44+100000000000000000043*x^43+100000000000000000042*x^42+100000000000000000041*x^41+100000000000000000040*x^40+100000000000000000039*x^39+100000000000000000038*x^38+100000000000000000037*x^37+100000000000000000036*x^36+100000000000000000035*x^35+100000000000000000034*x^34+100000000000000000033*x^33+100000000000000000032*x^32+100000000000000000031*x^31+100000000000000000030*x^30+100000000000000000029*x^29+100000000000000000028*x^28+100000000000000000027*x^27+100000000000000000026*x^26+100000000000000000025*x^25+100000000000000000024*x^24+100000000000000000023*x^23+100000000000000000022*x^22+100000000000000000021*x^21+100000000000000000020*x^20+100000000000000000019*x^19+100000000000000000018*x^18+100000000000000000017*x^17+100000000000000000016*x^16+100000000000000000015*x^15+100000000000000000014*x^14+100000000000000000013*x^13+100000000000000000012*x^12+100000000000000000011*x^11+100000000000000000010*x^10+100000000000000000009*x^9+100000000000000000008*x^8+100000000000000000007*x^7+100000000000000000006*x^6+100000000000000000005*x^5+100000000000000000004*x^4+100000000000000000003*x^3+100000000000000000002*x^2+100000000000000000001*x+100000000000000000000", "10000000000", "10000000001000000005100000000500000000049000000004800000000470000000046000000004500000000440000000043000000004200000000410000000040000000003900000000380000000037000000003600000000350000000034000000003300000000320000000031000000003000000000290000000028000000002700000000260000000025000000002400000000230000000022000000002100000000200000000019000000001800000000170000000016000000001500000000140000000013000000001200000000110000000010000000000900000000080000000007000000000600000000050000000004000000000300000000010000000000"),
];

// The rows of test_evaluate_integer_polynomial_rational in malachite-q's RationalPolynomial tests:
// polynomial, point, value.
const RATIONAL_EVALUATE_UNIT_ROWS: [(&str, &str, &str); 24] = [
    ("0", "0", "0"),
    ("0", "1/2", "0"),
    ("5", "0", "5"),
    ("5", "-3/7", "5"),
    ("x", "1/2", "1/2"),
    ("x", "-7/3", "-7/3"),
    ("x^2+3*x+2", "0", "2"),
    ("x^2+3*x+2", "1", "6"),
    ("x^2+3*x+2", "-1", "0"),
    ("x^2+3*x+2", "1/2", "15/4"),
    ("x^2+3*x+2", "-2/3", "4/9"),
    ("4*x^2-1", "1/2", "0"),
    ("4*x^2-1", "-1/2", "0"),
    ("3*x^2-2*x+5", "2/3", "5"),
    ("2*x^5-x", "-3/2", "-219/16"),
    ("1000000000000000000000*x+1", "1/1000000000000000000000", "2"),
    ("x^100", "1/2", "1/1267650600228229401496703205376"),
    ("x^100-1", "-1", "0"),
    ("-x^51+x^50", "3/5", "1435795975383705177540498/444089209850062616169452667236328125"),
    ("x^64+x^63+x^62+x^61+x^60+x^59+x^58+x^57+x^56+x^55+x^54+x^53+x^52+x^51+x^50+x^49+x^48+x^47+x^46+x^45+x^44+x^43+x^42+x^41+x^40+x^39+x^38+x^37+x^36+x^35+x^34+x^33+x^32+x^31+x^30+x^29+x^28+x^27+x^26+x^25+x^24+x^23+x^22+x^21+x^20+x^19+x^18+x^17+x^16+x^15+x^14+x^13+x^12+x^11+x^10+x^9+x^8+x^7+x^6+x^5+x^4+x^3+x^2+x+1", "1/2", "36893488147419103231/18446744073709551616"),
    ("-60*x^59+59*x^58-58*x^57+57*x^56-56*x^55+55*x^54-54*x^53+53*x^52-52*x^51+51*x^50-50*x^49+49*x^48-48*x^47+47*x^46-46*x^45+45*x^44-44*x^43+43*x^42-42*x^41+41*x^40-40*x^39+39*x^38-38*x^37+37*x^36-36*x^35+35*x^34-34*x^33+33*x^32-32*x^31+31*x^30-30*x^29+29*x^28-28*x^27+27*x^26-26*x^25+25*x^24-24*x^23+23*x^22-22*x^21+21*x^20-20*x^19+19*x^18-18*x^17+17*x^16-16*x^15+15*x^14-14*x^13+13*x^12-12*x^11+11*x^10-10*x^9+9*x^8-8*x^7+7*x^6-6*x^5+5*x^4-4*x^3+3*x^2-2*x+1", "-7/3", "2508357937401890366278306875355211178514115109779830/4710128697246244834921603689"),
    ("9784*x^99+9587*x^98+9392*x^97+9199*x^96+9008*x^95+8819*x^94+8632*x^93+8447*x^92+8264*x^91+8083*x^90+7904*x^89+7727*x^88+7552*x^87+7379*x^86+7208*x^85+7039*x^84+6872*x^83+6707*x^82+6544*x^81+6383*x^80+6224*x^79+6067*x^78+5912*x^77+5759*x^76+5608*x^75+5459*x^74+5312*x^73+5167*x^72+5024*x^71+4883*x^70+4744*x^69+4607*x^68+4472*x^67+4339*x^66+4208*x^65+4079*x^64+3952*x^63+3827*x^62+3704*x^61+3583*x^60+3464*x^59+3347*x^58+3232*x^57+3119*x^56+3008*x^55+2899*x^54+2792*x^53+2687*x^52+2584*x^51+2483*x^50+2384*x^49+2287*x^48+2192*x^47+2099*x^46+2008*x^45+1919*x^44+1832*x^43+1747*x^42+1664*x^41+1583*x^40+1504*x^39+1427*x^38+1352*x^37+1279*x^36+1208*x^35+1139*x^34+1072*x^33+1007*x^32+944*x^31+883*x^30+824*x^29+767*x^28+712*x^27+659*x^26+608*x^25+559*x^24+512*x^23+467*x^22+424*x^21+383*x^20+344*x^19+307*x^18+272*x^17+239*x^16+208*x^15+179*x^14+152*x^13+127*x^12+104*x^11+83*x^10+64*x^9+47*x^8+32*x^7+19*x^6+8*x^5-x^4-8*x^3-13*x^2-16*x-17", "2/3", "-3607642645124092491947916080347518664151166126775/171792506910670443678820376588540424234035840667"),
    ("100000000000000000050*x^50+100000000000000000049*x^49+100000000000000000048*x^48+100000000000000000047*x^47+100000000000000000046*x^46+100000000000000000045*x^45+100000000000000000044*x^44+100000000000000000043*x^43+100000000000000000042*x^42+100000000000000000041*x^41+100000000000000000040*x^40+100000000000000000039*x^39+100000000000000000038*x^38+100000000000000000037*x^37+100000000000000000036*x^36+100000000000000000035*x^35+100000000000000000034*x^34+100000000000000000033*x^33+100000000000000000032*x^32+100000000000000000031*x^31+100000000000000000030*x^30+100000000000000000029*x^29+100000000000000000028*x^28+100000000000000000027*x^27+100000000000000000026*x^26+100000000000000000025*x^25+100000000000000000024*x^24+100000000000000000023*x^23+100000000000000000022*x^22+100000000000000000021*x^21+100000000000000000020*x^20+100000000000000000019*x^19+100000000000000000018*x^18+100000000000000000017*x^17+100000000000000000016*x^16+100000000000000000015*x^15+100000000000000000014*x^14+100000000000000000013*x^13+100000000000000000012*x^12+100000000000000000011*x^11+100000000000000000010*x^10+100000000000000000009*x^9+100000000000000000008*x^8+100000000000000000007*x^7+100000000000000000006*x^6+100000000000000000005*x^5+100000000000000000004*x^4+100000000000000000003*x^3+100000000000000000002*x^2+100000000000000000001*x+100000000000000000000", "-1/10", "113636363636363636363533057851239669421487603305785237603305785123967/1250000000000000000000000000000000000000000000000"),
    ("984770902183611232881*x^44+328256967394537077627*x^43+109418989131512359209*x^42+36472996377170786403*x^41+12157665459056928801*x^40+4052555153018976267*x^39+1350851717672992089*x^38+450283905890997363*x^37+150094635296999121*x^36+50031545098999707*x^35+16677181699666569*x^34+5559060566555523*x^33+1853020188851841*x^32+617673396283947*x^31+205891132094649*x^30+68630377364883*x^29+22876792454961*x^28+7625597484987*x^27+2541865828329*x^26+847288609443*x^25+282429536481*x^24+94143178827*x^23+31381059609*x^22+10460353203*x^21+3486784401*x^20+1162261467*x^19+387420489*x^18+129140163*x^17+43046721*x^16+14348907*x^15+4782969*x^14+1594323*x^13+531441*x^12+177147*x^11+59049*x^10+19683*x^9+6561*x^8+2187*x^7+729*x^6+243*x^5+81*x^4+27*x^3+9*x^2+3*x+1", "1/3", "45"),
];

// The rows of test_evaluate_rational_polynomial in malachite-q's RationalPolynomial tests:
// polynomial, point, value.
const RATIONAL_POLYNOMIAL_EVALUATE_UNIT_ROWS: [(&str, &str, &str); 19] = [
    ("0", "0", "0"),
    ("0", "1/2", "0"),
    ("1/2", "0", "1/2"),
    ("1/2", "-3/7", "1/2"),
    ("x", "1/2", "1/2"),
    ("1/2*x", "1/2", "1/4"),
    ("1/2*x^2-1/3*x+2", "0", "2"),
    ("1/2*x^2-1/3*x+2", "3/2", "21/8"),
    ("1/2*x^2-1/3*x+2", "-3", "15/2"),
    ("1/4*x^2-1/4", "1", "0"),
    ("1/4*x^2-1/4", "-1", "0"),
    ("1/4*x^2-1/4", "1/2", "-3/16"),
    ("x^2+3*x+2", "-2/3", "4/9"),
    ("1/6*x^3+1/2*x^2+1/3*x", "1", "1"),
    ("1/6*x^3+1/2*x^2+1/3*x", "2", "4"),
    ("-1/7*x^5+x", "7", "-2394"),
    (
        "1/1000000000000000000000*x+1",
        "1000000000000000000000",
        "2",
    ),
    (
        "1/3*x^100",
        "3",
        "171792506910670443678820376588540424234035840667",
    ),
    ("2/3*x+4/3", "1", "2"),
];

// The rows of test_evaluate_rational_polynomial_integer in malachite-q's RationalPolynomial tests:
// polynomial, point, value.
const RATIONAL_POLYNOMIAL_EVALUATE_INTEGER_UNIT_ROWS: [(&str, &str, &str); 20] = [
    ("0", "0", "0"),
    ("0", "5", "0"),
    ("1/2", "0", "1/2"),
    ("1/2", "-3", "1/2"),
    ("x", "7", "7"),
    ("1/2*x", "3", "3/2"),
    ("1/2*x", "4", "2"),
    ("1/2*x^2-1/3*x+2", "0", "2"),
    ("1/2*x^2-1/3*x+2", "3", "11/2"),
    ("1/2*x^2-1/3*x+2", "-3", "15/2"),
    ("1/2*x^2+1/2*x", "4", "10"),
    ("1/2*x^2+1/2*x", "-5", "10"),
    ("1/6*x^3+1/2*x^2+1/3*x", "7", "84"),
    ("1/4*x^2-1/4", "1", "0"),
    ("1/4*x^2-1/4", "3", "2"),
    ("-1/7*x^5+x", "7", "-2394"),
    ("1/1000000000000000000000*x+1", "1000000000000000000000", "2"),
    ("1/3*x^100", "3", "171792506910670443678820376588540424234035840667"),
    ("2/3*x+4/3", "1", "2"),
    ("1/2*x^60+1/3", "-1000000000000", "1500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001/3"),
];

fn main() {
    let oracle = build_oracle();

    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_mod_sqrt",
        "fmpz_sqrtmod",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-base",
        "demo_mod_sqrt_u64",
        "n_sqrtmod",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_mod_div",
        "fmpz_mod_divides",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-base",
        "demo_mod_div_u64",
        "fmpz_mod_divides",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_mod_div_list",
        "fmpz_divides_mod_list",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-base",
        "demo_mod_div_list_u64",
        "fmpz_divides_mod_list",
    );
    check_demo_against_flint(&oracle, "../malachite-nz", "demo_natural_crt", "fmpz_CRT");
    check_demo_against_flint(&oracle, "../malachite-base", "demo_crt_u64", "fmpz_CRT");
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_integer_balanced_crt",
        "fmpz_CRT_balanced",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_multi_crt",
        "fmpz_multi_CRT",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_integer_multi_balanced_crt",
        "fmpz_multi_CRT_balanced",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_crt_comb_reduce",
        "fmpz_multi_mod_ui",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_crt_comb_combine",
        "fmpz_multi_CRT_ui",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_crt_comb_combine_balanced",
        "fmpz_multi_CRT_ui_balanced",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_rising_factorial",
        "fmpz_rfac",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_integer_rising_factorial",
        "fmpz_rfac",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-nz",
        "demo_natural_extended_gcd_partial",
        "fmpz_xgcd_partial",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_to_height",
        "fmpq_height",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_into_height",
        "fmpq_height",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_height_significant_bits",
        "fmpq_height_bits",
    );
    check_demo_against_flint(&oracle, "../malachite-q", "demo_rational_gcd", "fmpq_gcd");
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_extended_gcd",
        "fmpq_gcd_cofactors",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_farey_neighbors",
        "fmpq_farey_neighbors",
    );
    check_demo_against_flint_no_special(
        &oracle,
        "../malachite-q",
        "demo_rational_harmonic_number",
        "fmpq_harmonic",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_dedekind_sum",
        "fmpq_dedekind_sum",
    );
    check_demo_against_flint_no_special(
        &oracle,
        "../malachite-nz",
        "demo_natural_bell_number",
        "arith_bell_number",
    );
    check_demo_against_flint_no_special(
        &oracle,
        "../malachite-nz",
        "demo_natural_bell_numbers_prefix",
        "arith_bell_number_vec",
    );
    check_demo_against_flint_no_special(
        &oracle,
        "../malachite-nz",
        "demo_natural_landau_function_prefix",
        "arith_landau_function_vec",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_exhaustive_by_height",
        "fmpq_next_minimal",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_exhaustive_signed_by_height",
        "fmpq_next_signed_minimal",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_reconstruct",
        "fmpq_reconstruct",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_reconstruct_ref",
        "fmpq_reconstruct",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_reconstruct_with_bounds",
        "fmpq_reconstruct_2",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_reconstruct_with_bounds_ref",
        "fmpq_reconstruct_2",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_reconstruct_tier_rows",
        "fmpq_reconstruct",
    );
    check_demo_against_flint(
        &oracle,
        "../malachite-q",
        "demo_rational_reconstruct_tier_rows",
        "fmpq_reconstruct_2",
    );

    println!("testing primitive_root_prime unit tests");
    {
        let mut output_file = File::create(TEST_OUT).unwrap();
        write_primitive_root_prime_unit_test(&mut output_file, 2, 1);
        write_primitive_root_prime_unit_test(&mut output_file, 3, 2);
        write_primitive_root_prime_unit_test(&mut output_file, 5, 2);
        write_primitive_root_prime_unit_test(&mut output_file, 7, 3);
        write_primitive_root_prime_unit_test(&mut output_file, 11, 2);
        write_primitive_root_prime_unit_test(&mut output_file, 191, 19);
        write_primitive_root_prime_unit_test(&mut output_file, 9223372036854775807, 2);
        write_primitive_root_prime_unit_test(&mut output_file, 8760810010780182161, 3);
    }
    run_oracle(&oracle, "n_primitive_root_prime", Some(TEST_OUT));

    // Edge rows for the multi-modulus CRT quirks: a single modulus of 1 is usable, but a 1 or a
    // 0 among two or more moduli is not, and unusable lists must report None.
    println!("testing multi_crt unit tests");
    {
        let mut output_file = File::create(TEST_OUT).unwrap();
        for line in [
            "multi_crt([5], [3]) = Some(3)",
            "multi_crt([1], [0]) = Some(0)",
            "multi_crt([0], [0]) = None",
            "multi_crt([3, 5, 7], [2, 3, 2]) = Some(23)",
            "multi_crt([4, 6], [1, 3]) = None",
            "multi_crt([5, 5], [1, 1]) = None",
            "multi_crt([3, 1], [2, 0]) = None",
            "multi_crt([0, 3], [0, 2]) = None",
            "multi_crt_balanced([5], [3]) = Some(-2)",
            "multi_crt_balanced([3, 5], [2, 3]) = Some(-7)",
            "multi_crt_balanced([2, 7], [1, 0]) = Some(7)",
            "multi_crt_balanced([4, 6], [1, 3]) = None",
        ] {
            writeln!(output_file, "{line}").unwrap();
        }
    }
    run_oracle(&oracle, "fmpz_multi_CRT", Some(TEST_OUT));
    run_oracle(&oracle, "fmpz_multi_CRT_balanced", Some(TEST_OUT));

    // Every case from test_balanced_mod in malachite-nz's IntegerPolynomial tests, including the
    // negative moduli, which the oracle passes to FLINT as their absolute values.
    println!("testing IntegerPolynomial balanced_mod unit tests");
    {
        let mut output_file = File::create(TEST_OUT).unwrap();
        for line in [
            "(0).balanced_mod(1) = 0",
            "(0).balanced_mod(-7) = 0",
            "(x^2+27*x-23).balanced_mod(1) = 0",
            "(x^2+27*x-23).balanced_mod(-1) = 0",
            "(x^2+27*x-23).balanced_mod(10) = x^2-3*x-3",
            "(x^2+27*x-23).balanced_mod(-10) = x^2-3*x-3",
            "(5*x-5).balanced_mod(10) = 5*x+5",
            "(x^2+x+1).balanced_mod(2) = x^2+x+1",
            "(-x^2-x-1).balanced_mod(2) = x^2+x+1",
            "(2*x-2).balanced_mod(5) = 2*x-2",
            "(3*x-3).balanced_mod(5) = -2*x+2",
            "(10*x^2+7*x+5).balanced_mod(-10) = -3*x+5",
            "(-6*x^2-3*x-9).balanced_mod(3) = 0",
            "(x^3-6*x^2+2).balanced_mod(3) = x^3-1",
            "(1000000000000000000000000*x+1).balanced_mod(1234567890987) = 530068894399*x+1",
        ] {
            writeln!(output_file, "{line}").unwrap();
        }
    }
    run_oracle(&oracle, "fmpz_poly_scalar_smod_fmpz", Some(TEST_OUT));
    // The generated cases from balanced_mod_properties: the (polynomial, nonzero modulus) pairs,
    // by value, by reference, and in place, and every polynomial against the moduli 1, -1, and 2.
    for demo_name in [
        "demo_integer_polynomial_balanced_mod",
        "demo_integer_polynomial_balanced_mod_ref",
        "demo_integer_polynomial_balanced_mod_assign",
        "demo_integer_polynomial_balanced_mod_small_moduli",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "fmpz_poly_scalar_smod_fmpz",
        );
    }

    // Every case from test_mod_op in malachite-nz's IntegerPolynomial tests, and the
    // u128 cases from test_mod_op_unsigned, whose modulus is too wide for an nmod_poly.
    println!("testing IntegerPolynomial mod_op unit tests");
    {
        let mut output_file = File::create(TEST_OUT).unwrap();
        for line in [
            "(0).mod_op(1) = 0",
            "(0).mod_op(7) = 0",
            "(x^2-4*x-5).mod_op(1) = 0",
            "(x^2-4*x-5).mod_op(3) = x^2+2*x+1",
            "(-1).mod_op(10) = 9",
            "(-x).mod_op(18446744073709551616) = 18446744073709551615*x",
            "(x^2+4*x+5).mod_op(3) = x^2+x+2",
            "(-6*x+1).mod_op(3) = 1",
            "(-6*x^2+3*x-1).mod_op(3) = 2",
            "(-6*x^2-3*x-9).mod_op(3) = 0",
            "(x^3-6*x^2+2).mod_op(3) = x^3+2",
            "(-1000000000000000000000000*x+1).mod_op(1234567890987) = 704498996588*x+1",
            "(x^2-4*x+5).mod_op(1000000000000000000000000) = x^2+999999999999999999999996*x+5",
            "(-x).mod_op(340282366920938463463374607431768211455) = 340282366920938463463374607431768211454*x",
        ] {
            writeln!(output_file, "{line}").unwrap();
        }
    }
    run_oracle(&oracle, "fmpz_poly_scalar_mod_fmpz", Some(TEST_OUT));
    // The generated cases from mod_op_properties, and the u128 ones from
    // mod_op_unsigned_properties.
    for demo_name in [
        "demo_integer_polynomial_mod_op",
        "demo_integer_polynomial_mod_op_ref",
        "demo_integer_polynomial_mod_op_power_of_2_moduli",
        "demo_integer_polynomial_mod_op_unsigned_u128",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "fmpz_poly_scalar_mod_fmpz",
        );
    }

    // The cases from test_mod_op_unsigned whose modulus fits in a word.
    println!("testing IntegerPolynomial mod_op by a word unit tests");
    {
        let mut output_file = File::create(TEST_OUT).unwrap();
        for line in [
            "(0).mod_op(1) = 0",
            "(0).mod_op(7) = 0",
            "(x^2-4*x-5).mod_op(3) = x^2+2*x+1",
            "(x^2-4*x-5).mod_op(1) = 0",
            "(-1).mod_op(255) = 254",
            "(-1).mod_op(18446744073709551615) = 18446744073709551614",
            "(-1000000000001*x^2+2000000000003*x-5).mod_op(7) = 5*x^2+5*x+2",
            "(-100000000000000000000*x+1).mod_op(18446744073709551615) = 10680464442257309690*x+1",
            "(-1024*x^2-3).mod_op(4) = 1",
            "(-4294967296*x^2+4294967296).mod_op(65536) = 0",
        ] {
            writeln!(output_file, "{line}").unwrap();
        }
    }
    run_oracle(&oracle, "fmpz_poly_get_nmod_poly", Some(TEST_OUT));
    // The generated cases from mod_op_unsigned_properties, for every word-sized type.
    for demo_name in [
        "demo_integer_polynomial_mod_op_unsigned_u8",
        "demo_integer_polynomial_mod_op_unsigned_u16",
        "demo_integer_polynomial_mod_op_unsigned_u32",
        "demo_integer_polynomial_mod_op_unsigned_u64",
        "demo_integer_polynomial_mod_op_unsigned_usize",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "fmpz_poly_get_nmod_poly",
        );
    }

    // Every case from test_rem, test_rem_lowers_the_degree, test_rem_unsigned, and test_mod_op
    // in malachite-nz's NaturalPolynomial tests.
    println!("testing NaturalPolynomial rem unit tests");
    {
        let mut output_file = File::create(TEST_OUT).unwrap();
        for line in [
            "(0) % 1 = 0",
            "(0) % 7 = 0",
            "(x^2+4*x+5) % 3 = x^2+x+2",
            "(x^2+4*x+5) % 7 = x^2+4*x+5",
            "(x^2+4*x+5) % 1 = 0",
            "(1000000000001*x^2+2000000000003*x+5) % 1000000000000 = x^2+3*x+5",
            "(1000000000000000000000000*x+1) % 1234567890987 = 530068894399*x+1",
            "(x^2+4*x+5) % 1000000000000000000000000 = x^2+4*x+5",
            "(4*x^2+3) % 4 = 3",
            "(6*x^2+3*x+2) % 3 = 2",
            "(6*x^2+3*x+9) % 3 = 0",
            "(x^3+6*x^2+2) % 3 = x^3+2",
            "(5*x^5+10*x^4+15*x^3+1) % 5 = 1",
            "(2000000000000*x^2+1000000000000*x+7) % 1000000000000 = 7",
            "(0) % 1 = 0",
            "(0) % 7 = 0",
            "(x^2+4*x+5) % 3 = x^2+x+2",
            "(x^2+4*x+5) % 1 = 0",
            "(1000000000001*x^2+2000000000003*x+5) % 1000 = x^2+3*x+5",
            "(1000000000001*x^2+2000000000003*x+5) % 7 = 2*x^2+5*x+5",
            "(256*x^2+257*x+3) % 255 = x^2+2*x+3",
            "(100000000000000000000*x+1) % 18446744073709551615 = 7766279631452241925*x+1",
            "(1000000000000000000000000000000000000000*x) % 340282366920938463463374607431768211455 = 319435266158123073073250785136463577090*x",
            "(1024*x^2+3) % 4 = 3",
            "(4294967296*x^2+4294967296) % 65536 = 0",
            "(0).mod_op(1) = 0",
            "(x^2+4*x+5).mod_op(3) = x^2+x+2",
            "(x^2+4*x+5).mod_op(1) = 0",
            "(4*x^2+3).mod_op(4) = 3",
            "(1000000000000000000000000*x+1).mod_op(1234567890987) = 530068894399*x+1",
        ] {
            writeln!(output_file, "{line}").unwrap();
        }
    }
    run_oracle(&oracle, "fmpz_mod_poly_set_fmpz_poly", Some(TEST_OUT));
    // The generated cases from rem_properties and rem_unsigned_properties, in every form.
    for demo_name in [
        "demo_natural_polynomial_rem",
        "demo_natural_polynomial_rem_ref",
        "demo_natural_polynomial_rem_assign",
        "demo_natural_polynomial_mod_op",
        "demo_natural_polynomial_mod_assign",
        "demo_natural_polynomial_rem_special_moduli",
        "demo_natural_polynomial_rem_unsigned_u8",
        "demo_natural_polynomial_rem_unsigned_u16",
        "demo_natural_polynomial_rem_unsigned_u32",
        "demo_natural_polynomial_rem_unsigned_u64",
        "demo_natural_polynomial_rem_unsigned_u128",
        "demo_natural_polynomial_rem_unsigned_usize",
        "demo_natural_polynomial_rem_unsigned_ref_u8",
        "demo_natural_polynomial_rem_unsigned_ref_u16",
        "demo_natural_polynomial_rem_unsigned_ref_u32",
        "demo_natural_polynomial_rem_unsigned_ref_u64",
        "demo_natural_polynomial_rem_unsigned_ref_u128",
        "demo_natural_polynomial_rem_unsigned_ref_usize",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-nz",
            demo_name,
            "fmpz_mod_poly_set_fmpz_poly",
        );
    }

    for demo_name in [
        "demo_u8_primitive_root_prime",
        "demo_u16_primitive_root_prime",
        "demo_u32_primitive_root_prime",
        "demo_u64_primitive_root_prime",
        "demo_usize_primitive_root_prime",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-base",
            demo_name,
            "n_primitive_root_prime",
        );
    }

    // Every case from test_evaluate in malachite-nz's IntegerPolynomial and NaturalPolynomial tests,
    // checked against
    // FLINT's evaluation and against each of the two algorithms it chooses between.
    println!("testing IntegerPolynomial evaluate unit tests");
    for (method, flint_mode) in [
        ("evaluate", "fmpz_poly_evaluate_fmpz"),
        ("evaluate_horner", "fmpz_poly_evaluate_horner_fmpz"),
        (
            "evaluate_divide_and_conquer",
            "fmpz_poly_evaluate_divconquer_fmpz",
        ),
    ] {
        {
            let mut output_file = File::create(TEST_OUT).unwrap();
            for (p, x, r) in EVALUATE_UNIT_ROWS.iter().chain(&NATURAL_EVALUATE_UNIT_ROWS) {
                writeln!(output_file, "(&({p})).{method}({x}) = {r}").unwrap();
            }
        }
        run_oracle(&oracle, flint_mode, Some(TEST_OUT));
    }
    // The generated cases from evaluate_properties: by value and by reference, polynomials of
    // degree at least 50 (which take the divide-and-conquer path), and each algorithm on its own.
    for (demo_name, flint_mode) in [
        (
            "demo_integer_polynomial_evaluate",
            "fmpz_poly_evaluate_fmpz",
        ),
        (
            "demo_integer_polynomial_evaluate_ref",
            "fmpz_poly_evaluate_fmpz",
        ),
        (
            "demo_integer_polynomial_evaluate_long",
            "fmpz_poly_evaluate_fmpz",
        ),
        (
            "demo_integer_polynomial_evaluate_horner",
            "fmpz_poly_evaluate_horner_fmpz",
        ),
        (
            "demo_integer_polynomial_evaluate_divide_and_conquer",
            "fmpz_poly_evaluate_divconquer_fmpz",
        ),
        (
            "demo_integer_polynomial_evaluate_divide_and_conquer_long",
            "fmpz_poly_evaluate_divconquer_fmpz",
        ),
        (
            "demo_natural_polynomial_evaluate",
            "fmpz_poly_evaluate_fmpz",
        ),
        (
            "demo_natural_polynomial_evaluate_ref",
            "fmpz_poly_evaluate_fmpz",
        ),
        (
            "demo_natural_polynomial_evaluate_long",
            "fmpz_poly_evaluate_fmpz",
        ),
        (
            "demo_natural_polynomial_evaluate_horner",
            "fmpz_poly_evaluate_horner_fmpz",
        ),
        (
            "demo_natural_polynomial_evaluate_divide_and_conquer",
            "fmpz_poly_evaluate_divconquer_fmpz",
        ),
        (
            "demo_natural_polynomial_evaluate_divide_and_conquer_long",
            "fmpz_poly_evaluate_divconquer_fmpz",
        ),
    ] {
        check_demo_against_flint(&oracle, "../malachite-nz", demo_name, flint_mode);
    }

    // Every case from test_evaluate_integer_polynomial_rational in malachite-q's tests, checked
    // against FLINT's evaluation at a rational and against each of its two algorithms.
    println!("testing IntegerPolynomial evaluate at Rational unit tests");
    for (method, flint_mode) in [
        ("evaluate", "fmpz_poly_evaluate_fmpq"),
        ("evaluate_horner", "fmpz_poly_evaluate_horner_fmpq"),
        (
            "evaluate_divide_and_conquer",
            "fmpz_poly_evaluate_divconquer_fmpq",
        ),
    ] {
        {
            let mut output_file = File::create(TEST_OUT).unwrap();
            for (p, x, r) in RATIONAL_EVALUATE_UNIT_ROWS {
                writeln!(output_file, "(&({p})).{method}({x}) = {r}").unwrap();
            }
        }
        run_oracle(&oracle, flint_mode, Some(TEST_OUT));
    }
    // The generated cases from evaluate_integer_polynomial_rational_properties.
    for (demo_name, flint_mode) in [
        (
            "demo_integer_polynomial_evaluate_rational",
            "fmpz_poly_evaluate_fmpq",
        ),
        (
            "demo_integer_polynomial_evaluate_rational_ref",
            "fmpz_poly_evaluate_fmpq",
        ),
        (
            "demo_integer_polynomial_evaluate_rational_long",
            "fmpz_poly_evaluate_fmpq",
        ),
        (
            "demo_integer_polynomial_evaluate_rational_horner",
            "fmpz_poly_evaluate_horner_fmpq",
        ),
        (
            "demo_integer_polynomial_evaluate_rational_divide_and_conquer",
            "fmpz_poly_evaluate_divconquer_fmpq",
        ),
        (
            "demo_integer_polynomial_evaluate_rational_divide_and_conquer_long",
            "fmpz_poly_evaluate_divconquer_fmpq",
        ),
    ] {
        check_demo_against_flint(&oracle, "../malachite-q", demo_name, flint_mode);
    }

    // Every case from test_evaluate_rational_polynomial in malachite-q's tests, and the generated
    // cases from evaluate_rational_polynomial_properties.
    println!("testing RationalPolynomial evaluate unit tests");
    {
        let mut output_file = File::create(TEST_OUT).unwrap();
        for (p, x, r) in RATIONAL_POLYNOMIAL_EVALUATE_UNIT_ROWS {
            writeln!(output_file, "(&({p})).evaluate({x}) = {r}").unwrap();
        }
    }
    run_oracle(&oracle, "fmpq_poly_evaluate_fmpq", Some(TEST_OUT));
    for demo_name in [
        "demo_rational_polynomial_evaluate",
        "demo_rational_polynomial_evaluate_ref",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-q",
            demo_name,
            "fmpq_poly_evaluate_fmpq",
        );
    }

    // Every case from test_evaluate_rational_polynomial_integer in malachite-q's tests, and the
    // generated cases from evaluate_rational_polynomial_integer_properties.
    println!("testing RationalPolynomial evaluate at Integer unit tests");
    {
        let mut output_file = File::create(TEST_OUT).unwrap();
        for (p, x, r) in RATIONAL_POLYNOMIAL_EVALUATE_INTEGER_UNIT_ROWS {
            writeln!(output_file, "(&({p})).evaluate({x}) = {r}").unwrap();
        }
    }
    run_oracle(&oracle, "fmpq_poly_evaluate_fmpz", Some(TEST_OUT));
    for demo_name in [
        "demo_rational_polynomial_evaluate_integer",
        "demo_rational_polynomial_evaluate_integer_ref",
    ] {
        check_demo_against_flint(
            &oracle,
            "../malachite-q",
            demo_name,
            "fmpq_poly_evaluate_fmpz",
        );
    }
}
