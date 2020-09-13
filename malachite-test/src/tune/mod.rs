use std::fs::File;
use std::io::Write;

use malachite_base::named::Named;
use malachite_nz::platform::{
    DoubleLimb, FloatWithLimbWidth, HalfLimb, Limb, SignedDoubleLimb, SignedHalfLimb, SignedLimb,
};

pub mod aorsmul;
pub mod barrett_helper;
pub mod barrett_product;
pub mod div_approx_divide_and_conquer_to_barrett;
pub mod div_approx_schoolbook_to_divide_and_conquer;
pub mod div_balance;
pub mod div_mod_divide_and_conquer_to_barrett;
pub mod div_mod_schoolbook_to_divide_and_conquer;
pub mod divisible_by_limb;
pub mod fft;
pub mod invert_basecase_to_newton;
pub mod mod_limb_any_leading_zeros;
pub mod mod_limb_any_leading_zeros_from_normalized;
pub mod mod_limb_any_leading_zeros_from_unnormalized;
pub mod mod_limb_at_least_1_leading_zero;
pub mod mod_limb_at_least_2_leading_zeros;
pub mod mod_limb_small_normalized;
pub mod mod_limb_small_unnormalized;
pub mod modular_div_divide_and_conquer_to_barrett;
pub mod modular_div_mod_divide_and_conquer_to_barrett;
pub mod modular_div_mod_schoolbook_to_divide_and_conquer;
pub mod modular_div_schoolbook_to_divide_and_conquer;
pub mod modular_invert_small_to_large;
pub mod mul_basecase_to_mul_low_basecase;
pub mod mul_low_basecase_to_divide_and_conquer;
pub mod mul_low_divide_and_conquer_to_large;
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

pub(crate) fn tune(param_group: &str) {
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
        "DivModSchoolbookToDivideAndConquer" => {
            display_lines(&div_mod_schoolbook_to_divide_and_conquer::tune())
        }
        "DivApproxSchoolbookToDivideAndConquer" => {
            display_lines(&div_approx_schoolbook_to_divide_and_conquer::tune())
        }
        "InvertBasecaseToNewton" => display_lines(&invert_basecase_to_newton::tune()),
        "DivModDivideAndConquerToBarrett" => {
            display_lines(&div_mod_divide_and_conquer_to_barrett::tune())
        }
        "BarrettProduct" => display_lines(&barrett_product::tune()),
        "BarrettHelper" => display_lines(&barrett_helper::tune()),
        "DivApproxDivideAndConquerToBarrett" => {
            display_lines(&div_approx_divide_and_conquer_to_barrett::tune())
        }
        "DivBalance" => display_lines(&div_balance::tune()),
        "MulBasecaseToMulLowBasecase" => display_lines(&mul_basecase_to_mul_low_basecase::tune()),
        "MulLowBasecaseToDivideAndConquer" => {
            display_lines(&mul_low_basecase_to_divide_and_conquer::tune())
        }
        "MulLowDivideAndConquerToLarge" => {
            display_lines(&mul_low_divide_and_conquer_to_large::tune())
        }
        "ModularInvertSmallToLarge" => display_lines(&modular_invert_small_to_large::tune()),
        "ModularDivModSchoolbookToDivideAndConquer" => {
            display_lines(&modular_div_mod_schoolbook_to_divide_and_conquer::tune())
        }
        "ModularDivModDivideAndConquerToBarett" => {
            display_lines(&modular_div_mod_divide_and_conquer_to_barrett::tune())
        }
        "ModularDivSchoolbookToDivideAndConquer" => {
            display_lines(&modular_div_schoolbook_to_divide_and_conquer::tune())
        }
        "ModularDivDivideAndConquerToBarett" => {
            display_lines(&modular_div_divide_and_conquer_to_barrett::tune())
        }
        "ModLimbAnyLeadingZeros" => display_lines(&mod_limb_any_leading_zeros::tune()),
        "ModLimbSmallNormalized" => display_lines(&mod_limb_small_normalized::tune()),
        "ModLimbSmallUnnormalized" => display_lines(&mod_limb_small_unnormalized::tune()),
        "ModLimbAnyLeadingZerosFromNormalized" => {
            display_lines(&mod_limb_any_leading_zeros_from_normalized::tune())
        }
        "ModLimbAnyLeadingZerosFromUnnormalized" => {
            display_lines(&mod_limb_any_leading_zeros_from_unnormalized::tune())
        }
        "ModLimbAtLeast1LeadingZero" => display_lines(&mod_limb_at_least_1_leading_zero::tune()),
        "ModLimbAtLeast2LeadingZeros" => display_lines(&mod_limb_at_least_2_leading_zeros::tune()),
        "DivisibleByLimb" => display_lines(&divisible_by_limb::tune()),
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
            lines.extend(div_mod_schoolbook_to_divide_and_conquer::tune());
            lines.extend(div_approx_schoolbook_to_divide_and_conquer::tune());
            lines.extend(invert_basecase_to_newton::tune());
            lines.extend(div_mod_divide_and_conquer_to_barrett::tune());
            lines.extend(barrett_product::tune());
            lines.extend(barrett_helper::tune());
            lines.push(String::new());
            lines.extend(div_approx_divide_and_conquer_to_barrett::tune());
            lines.extend(div_balance::tune());
            lines.push(String::new());
            lines.extend(mul_basecase_to_mul_low_basecase::tune());
            lines.extend(mul_low_basecase_to_divide_and_conquer::tune());
            lines.extend(mul_low_divide_and_conquer_to_large::tune());
            lines.push(String::new());
            lines.extend(modular_invert_small_to_large::tune());
            lines.extend(modular_div_mod_schoolbook_to_divide_and_conquer::tune());
            lines.extend(modular_div_mod_divide_and_conquer_to_barrett::tune());
            lines.extend(modular_div_schoolbook_to_divide_and_conquer::tune());
            lines.extend(modular_div_divide_and_conquer_to_barrett::tune());
            lines.push(String::new());
            lines.extend(mod_limb_any_leading_zeros::tune());
            lines.extend(mod_limb_small_normalized::tune());
            lines.extend(mod_limb_small_unnormalized::tune());
            lines.extend(mod_limb_any_leading_zeros_from_normalized::tune());
            lines.extend(mod_limb_any_leading_zeros_from_unnormalized::tune());
            lines.extend(mod_limb_at_least_1_leading_zero::tune());
            lines.extend(mod_limb_at_least_2_leading_zeros::tune());
            lines.push(String::new());
            lines.extend(divisible_by_limb::tune());
            let filename = "benchmarks/platform.txt";
            let mut output = File::create(filename).unwrap();
            for line in lines {
                writeln!(output, "{}", line);
            }
        }
        _ => panic!("Invalid tuning param group"),
    }
}
