use std::fs::File;
use std::io::Write;

use malachite_base::named::Named;
use malachite_nz::platform::{
    DoubleLimb, FloatWithLimbWidth, HalfLimb, Limb, SignedDoubleLimb, SignedHalfLimb, SignedLimb,
};

pub mod aorsmul;
pub mod barrett_helper;
pub mod barrett_product;
pub mod basecase_to_newton;
pub mod compare_two;
pub mod div_balance;
pub mod divide_and_conquer_approx_to_barrett_approx;
pub mod divide_and_conquer_to_barrett;
pub mod fft;
pub mod schoolbook_approx_to_divide_and_conquer_approx;
pub mod schoolbook_to_divide_and_conquer;
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
        "SchoolbookToDivideAndConquer" => display_lines(&schoolbook_to_divide_and_conquer::tune()),
        "SchoolbookApproxToDivideAndConquerApprox" => {
            display_lines(&schoolbook_approx_to_divide_and_conquer_approx::tune())
        }
        "BasecaseToNewton" => display_lines(&basecase_to_newton::tune()),
        "DivideAndConquerToBarrett" => display_lines(&divide_and_conquer_to_barrett::tune()),
        "BarrettProduct" => display_lines(&barrett_product::tune()),
        "BarrettHelper" => display_lines(&barrett_helper::tune()),
        "DivideAndConquerApproxToBarrettApprox" => {
            display_lines(&divide_and_conquer_approx_to_barrett_approx::tune())
        }
        "DivBalance" => display_lines(&div_balance::tune()),
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
            lines.push(String::new());
            lines.extend(schoolbook_to_divide_and_conquer::tune());
            lines.extend(schoolbook_approx_to_divide_and_conquer_approx::tune());
            lines.extend(basecase_to_newton::tune());
            lines.extend(divide_and_conquer_to_barrett::tune());
            lines.extend(barrett_product::tune());
            lines.extend(barrett_helper::tune());
            lines.push(String::new());
            lines.extend(divide_and_conquer_approx_to_barrett_approx::tune());
            lines.extend(div_balance::tune());
            let filename = "benchmarks/platform.txt";
            let mut output = File::create(filename).unwrap();
            for line in lines {
                writeln!(output, "{}", line);
            }
        }
        _ => panic!("Invalid tuning param group"),
    }
}
