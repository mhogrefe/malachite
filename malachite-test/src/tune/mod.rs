use malachite_base::named::Named;
use malachite_nz::platform::{
    DoubleLimb, FloatWithLimbWidth, HalfLimb, Limb, SignedDoubleLimb, SignedHalfLimb, SignedLimb,
};
use std::fs::File;
use std::io::Write;

pub mod aorsmul;
pub mod compare_two;
pub mod fft;
pub mod toom_22;
pub mod toom_32_to_43;
pub mod toom_32_to_53;
pub mod toom_33;
pub mod toom_42_to_53;
pub mod toom_42_to_63;
pub mod toom_44;
pub mod toom_6h;
pub mod toom_8h;

fn display_lines(lines: &[String]) {
    for line in lines {
        println!("{}", line);
    }
}

pub fn tune(param_group: &str) {
    match param_group {
        "AORSMUL" => display_lines(&aorsmul::tune()),
        "Toom22" => display_lines(&toom_22::tune()),
        "Toom33" => display_lines(&toom_33::tune()),
        "Toom44" => display_lines(&toom_44::tune()),
        "Toom6h" => display_lines(&toom_6h::tune()),
        "Toom8h" => display_lines(&toom_8h::tune()),
        "FFT" => display_lines(&fft::tune()),
        "Toom32to43" => display_lines(&toom_32_to_43::tune()),
        "Toom32to53" => display_lines(&toom_32_to_53::tune()),
        "Toom42to53" => display_lines(&toom_42_to_53::tune()),
        "Toom42to63" => display_lines(&toom_42_to_63::tune()),
        "all" => {
            let mut lines = Vec::new();
            lines.push(format!("pub type Limb = {};", Limb::NAME));
            lines.push(format!("pub type HalfLimb = {};", HalfLimb::NAME));
            lines.push(format!("pub type DoubleLimb = {};", DoubleLimb::NAME));
            lines.push(format!("pub type SignedLimb = {};", SignedLimb::NAME));
            lines.push(format!(
                "pub type SignedHalfLimb = {};",
                SignedHalfLimb::NAME
            ));
            lines.push(format!(
                "pub type SignedDoubleLimb = {};",
                SignedDoubleLimb::NAME
            ));
            lines.push(format!(
                "pub type FloatWithLimbWidth = {};",
                FloatWithLimbWidth::NAME
            ));
            lines.push(String::new());
            lines.extend(aorsmul::tune());
            lines.push(String::new());
            lines.extend(toom_22::tune());
            lines.extend(toom_33::tune());
            lines.extend(toom_44::tune());
            lines.extend(toom_6h::tune());
            lines.extend(toom_8h::tune());
            lines.push(String::new());
            lines.extend(toom_32_to_43::tune());
            lines.extend(toom_32_to_53::tune());
            lines.extend(toom_42_to_53::tune());
            lines.extend(toom_42_to_63::tune());
            lines.push(String::new());
            lines.extend(fft::tune());
            let filename = "benchmarks/platform.txt";
            let mut output = File::create(filename).unwrap();
            for line in lines {
                writeln!(output, "{}", line);
            }
        }
        _ => panic!("Invalid tuning param group"),
    }
}
