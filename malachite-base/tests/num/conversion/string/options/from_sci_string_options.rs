// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::{
    from_sci_string_options_gen, from_sci_string_options_rounding_mode_pair_gen,
    from_sci_string_options_unsigned_pair_gen_var_1,
};

#[test]
fn from_sci_string_options_properties() {
    from_sci_string_options_gen().test_properties(|options| {
        let mut options_alt = options;

        let base = options.get_base();
        assert!(base >= 2);
        assert!(base <= 36);
        options_alt.set_base(base);
        assert_eq!(options_alt, options);

        let rounding_mode = options.get_rounding_mode();
        options_alt.set_rounding_mode(rounding_mode);
        assert_eq!(options_alt, options);
    });

    from_sci_string_options_unsigned_pair_gen_var_1().test_properties(|(mut options, base)| {
        let old_options = options;
        let old_base = options.get_base();
        options.set_base(base);
        assert_eq!(options.get_base(), base);
        options.set_base(old_base);
        assert_eq!(options, old_options);
    });

    from_sci_string_options_rounding_mode_pair_gen().test_properties(|(mut options, rm)| {
        let old_options = options;
        let old_rm = options.get_rounding_mode();
        options.set_rounding_mode(rm);
        assert_eq!(options.get_rounding_mode(), rm);
        options.set_rounding_mode(old_rm);
        assert_eq!(options, old_options);
    });
}
