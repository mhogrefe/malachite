// This comment is deliberately longer than one hundred characters, in order to exercise the basic case of the lint.
fn main() {
    // This is another deliberately overlong line, but it is listed as an exception in dylint.toml, so it is fine.
    let _short_line_listed_as_an_exception_which_makes_the_exception_stale = 0;
}

#[cfg_attr(dylint_lib = "malachite_lints", allow(long_lines))]
/// This overlong doc line is attributed to the item it documents, whose `allow` attribute covers it just fine.
fn allowed() {}

#[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
/// This overlong doc line is covered by an `expect` attribute on its item, which the line also fulfills nicely.
fn expected() {}

#[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
fn unfulfilled() {}
