// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::string::options::SciSizeOptions;
use malachite_base::test_util::generators::{
    to_sci_options_bool_pair_gen, to_sci_options_gen, to_sci_options_rounding_mode_pair_gen,
    to_sci_options_signed_pair_gen_var_1, to_sci_options_unsigned_pair_gen_var_1,
    to_sci_options_unsigned_pair_gen_var_2, to_sci_options_unsigned_pair_gen_var_3,
};

#[test]
fn to_sci_options_properties() {
    to_sci_options_gen().test_properties(|options| {
        let mut options_alt = options;

        let base = options.get_base();
        assert!(base >= 2);
        assert!(base <= 36);
        options_alt.set_base(base);
        assert_eq!(options_alt, options);

        let rounding_mode = options.get_rounding_mode();
        options_alt.set_rounding_mode(rounding_mode);
        assert_eq!(options_alt, options);

        let size_options = options.get_size_options();
        match size_options {
            SciSizeOptions::Complete => options_alt.set_size_complete(),
            SciSizeOptions::Precision(p) => options_alt.set_precision(p),
            SciSizeOptions::Scale(s) => options_alt.set_scale(s),
        }
        assert_eq!(options_alt, options);

        let neg_exp_threshold = options.get_neg_exp_threshold();
        assert!(neg_exp_threshold < 0);
        options_alt.set_neg_exp_threshold(neg_exp_threshold);
        assert_eq!(options_alt, options);

        let lowercase = options.get_lowercase();
        if lowercase {
            options_alt.set_lowercase();
        } else {
            options_alt.set_uppercase();
        }
        assert_eq!(options_alt, options);

        let e_lowercase = options.get_e_lowercase();
        if e_lowercase {
            options_alt.set_e_lowercase();
        } else {
            options_alt.set_e_uppercase();
        }
        assert_eq!(options_alt, options);

        let force_exponent_plus_sign = options.get_force_exponent_plus_sign();
        options_alt.set_force_exponent_plus_sign(force_exponent_plus_sign);
        assert_eq!(options_alt, options);

        let include_trailing_zeros = options.get_include_trailing_zeros();
        options_alt.set_include_trailing_zeros(include_trailing_zeros);
        assert_eq!(options_alt, options);

        let mut options = options;
        let old_options = options;
        let old_lowercase = options.get_lowercase();
        options.set_lowercase();
        assert!(options.get_lowercase());
        if old_lowercase {
            options.set_lowercase();
        } else {
            options.set_uppercase();
        }
        assert_eq!(options, old_options);

        let old_lowercase = options.get_lowercase();
        options.set_uppercase();
        assert!(!options.get_lowercase());
        if old_lowercase {
            options.set_lowercase();
        } else {
            options.set_uppercase();
        }
        assert_eq!(options, old_options);

        let old_e_lowercase = options.get_e_lowercase();
        options.set_e_lowercase();
        assert!(options.get_e_lowercase());
        if old_e_lowercase {
            options.set_e_lowercase();
        } else {
            options.set_e_uppercase();
        }
        assert_eq!(options, old_options);

        let old_e_lowercase = options.get_e_lowercase();
        options.set_e_uppercase();
        assert!(!options.get_e_lowercase());
        if old_e_lowercase {
            options.set_e_lowercase();
        } else {
            options.set_e_uppercase();
        }
        assert_eq!(options, old_options);
    });

    to_sci_options_unsigned_pair_gen_var_1().test_properties(|(mut options, base)| {
        let old_options = options;
        let old_base = options.get_base();
        options.set_base(base);
        assert_eq!(options.get_base(), base);
        options.set_base(old_base);
        assert_eq!(options, old_options);
    });

    to_sci_options_rounding_mode_pair_gen().test_properties(|(mut options, rm)| {
        let old_options = options;
        let old_rm = options.get_rounding_mode();
        options.set_rounding_mode(rm);
        assert_eq!(options.get_rounding_mode(), rm);
        options.set_rounding_mode(old_rm);
        assert_eq!(options, old_options);
    });

    to_sci_options_gen().test_properties(|mut options| {
        let old_options = options;
        let old_size_options = options.get_size_options();
        options.set_size_complete();
        assert_eq!(options.get_size_options(), SciSizeOptions::Complete);
        match old_size_options {
            SciSizeOptions::Complete => options.set_size_complete(),
            SciSizeOptions::Precision(p) => options.set_precision(p),
            SciSizeOptions::Scale(s) => options.set_scale(s),
        }
        assert_eq!(options, old_options);
    });

    to_sci_options_unsigned_pair_gen_var_3().test_properties(|(mut options, precision)| {
        let old_options = options;
        let old_size_options = options.get_size_options();
        options.set_precision(precision);
        assert_eq!(
            options.get_size_options(),
            SciSizeOptions::Precision(precision)
        );
        match old_size_options {
            SciSizeOptions::Complete => options.set_size_complete(),
            SciSizeOptions::Precision(p) => options.set_precision(p),
            SciSizeOptions::Scale(s) => options.set_scale(s),
        }
        assert_eq!(options, old_options);
    });

    to_sci_options_unsigned_pair_gen_var_2().test_properties(|(mut options, scale)| {
        let old_options = options;
        let old_size_options = options.get_size_options();
        options.set_scale(scale);
        assert_eq!(options.get_size_options(), SciSizeOptions::Scale(scale));
        match old_size_options {
            SciSizeOptions::Complete => options.set_size_complete(),
            SciSizeOptions::Precision(p) => options.set_precision(p),
            SciSizeOptions::Scale(s) => options.set_scale(s),
        }
        assert_eq!(options, old_options);
    });

    to_sci_options_signed_pair_gen_var_1().test_properties(|(mut options, neg_exp_threshold)| {
        let old_options = options;
        let old_neg_exp_threshold = options.get_neg_exp_threshold();
        options.set_neg_exp_threshold(neg_exp_threshold);
        assert_eq!(options.get_neg_exp_threshold(), neg_exp_threshold);
        options.set_neg_exp_threshold(old_neg_exp_threshold);
        assert_eq!(options, old_options);
    });

    to_sci_options_bool_pair_gen().test_properties(|(mut options, b)| {
        let old_options = options;
        let old_force_exponent_plus_sign = options.get_force_exponent_plus_sign();
        options.set_force_exponent_plus_sign(b);
        assert_eq!(options.get_force_exponent_plus_sign(), b);
        options.set_force_exponent_plus_sign(old_force_exponent_plus_sign);
        assert_eq!(options, old_options);

        let old_include_trailing_zeros = options.get_include_trailing_zeros();
        options.set_include_trailing_zeros(b);
        assert_eq!(options.get_include_trailing_zeros(), b);
        options.set_include_trailing_zeros(old_include_trailing_zeros);
        assert_eq!(options, old_options);
    });
}
