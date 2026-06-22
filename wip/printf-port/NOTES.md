# Stashed: Float printf-formatting port (paused pending exponentiation)

This is in-progress work on MPFR-compatible `Float` string formatting, set aside on
2026-06-21 so that exponentiation (`mpfr_exp` → … → `pow_z`) can be implemented first.
The blocker: `printf` → `floor_log10` → `pow_ui`, and `pow_ui`'s rare internal
over/underflow path needs the `mpfr_pow_z` fallback, which depends on a full `exp`.
Without it we can't faithfully replace `Display`.

These files are plain `.rs` but live outside `src/`, so they are NOT compiled. Restore
by moving each back to its original path and re-registering its module.

## `printf.rs`  (faithful port of the `%R…` path of `mpfr_vasprintf`, `vasprintf.c`, MPFR 4.2.2)
- Original path: `malachite-float/src/conversion/string/printf.rs`
- Module line removed from `malachite-float/src/conversion/string/mod.rs`:
  `pub(crate) mod printf;`
- Done: `ArgType` (`arg_t`), `PrintfSpec` (`printf_spec`), `specinfo_init`,
  `specinfo_is_valid`, `parse_flags`, `parse_arg_type`, `StringBuffer` +
  `buffer_init`/`buffer_cat`/`buffer_pad`/`buffer_sandwich`, `PadType` (`pad_t`),
  `NumberParts` (`number_parts`), `DecimalInfo` (`decimal_info`), `mpfr_get_str_wrapper`.
- Remaining: `next_base_power_p`, `floor_log10`, `regular_ab`/`regular_eg`/`regular_fg`,
  `partition_number`, `sprnt_fp`, the width/precision spec parsing from the main loop,
  and the Rust-facing entry point.
- Conventions established: C `char *format` → `&[u8]` (parsers return the unconsumed
  tail); digit-bearing `NumberParts` fields are owned `Vec<u8>` (the `ip_ptr`/`fp_ptr`
  shared-allocation aliasing is dropped); `string_list` (manual frees) omitted under RAII;
  the size-0 count-only snprintf mode is dropped (always full output).
- `floor_log10` will need `ceil_mul` from `get_str.rs`, currently private — re-expose it as
  `pub(crate)` on restore.

## `pow.rs`  (faithful port of `mpfr_pow_ui`, `pow_ui.c`, MPFR 4.2.2)
- Original path: `malachite-float/src/arithmetic/pow.rs`
- Module line removed from `malachite-float/src/arithmetic/mod.rs`:
  `pub(crate) mod pow;`
- `pow_ui_prec_round_ref(x, n, prec, rm)` is complete EXCEPT the internal
  over/underflow fallback (a `todo!` standing in for `mpfr_pow_z`). All squarings and
  mults round away-from-zero (`Up`); Ziv uses Malachite's increment-halving schedule;
  error bound `err = working_prec - 1 - nlen`.
- Fill in the `todo!` once `pow_z`/`exp` exist, then the public API surface (`Pow`-style
  method family) + demos/tests, then wire `floor_log10` onto it.

Both files carry a `#![allow(dead_code)]` WIP marker to remove once they're wired in.
