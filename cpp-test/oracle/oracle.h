/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Shared declarations for the differential-testing oracle. Each mode lives in its own file and
    exposes one `run_*` entry point; `main.c` maps mode names to entry points. To add a mode, add
    a file with a `run_*` function, declare it here, and add a row to the table in `main.c`.
*/

#ifndef MALACHITE_ORACLE_H
#define MALACHITE_ORACLE_H

#include <flint/flint.h>
#include <flint/fmpz.h>

/* util.c */

/* Calls `handler` on each line of the file at `path`, passing 1-based line numbers. The handler
   may mutate the line in place. A nonzero handler result stops the iteration and is returned;
   an unopenable file returns 2. */
int for_each_line(const char * path, int (* handler)(char * line, int line_number));

/* Splits a line of the form `recv.method(arg1, arg2) = rest` in place: `pieces[0]` receives the
   receiver, `pieces[1..n_pieces]` the parenthesized arguments, and `*rest` the text after the
   closing parenthesis. Returns 0, without reporting an error, if the line does not contain
   `method` (which should include the leading dot and opening parenthesis, like ".mod_div(") or
   does not have exactly `n_pieces - 1` arguments. */
int split_method_call(char * line, const char * method, char ** pieces, int n_pieces,
                      char ** rest);

/* Parses a trailing ` = Some(x)` or ` = None`, returning 1 and filling `expected` in the `Some`
   case and returning 0 in the `None` case. The `Some` case mutates `rest` in place. */
int parse_option_fmpz(char * rest, fmpz_t expected);
int parse_option_ulong(const char * rest, ulong * expected);

/* Mode entry points, one per file. `arg` is the input-file path, except for `sqrtmod_stress`,
   where it is the iteration count. */

/* primitive_root.c */
int run_n_primitive_root_prime(const char * arg);

/* sqrtmod.c */
int run_fmpz_sqrtmod(const char * arg);
int run_n_sqrtmod(const char * arg);
int run_sqrtmod_stress(const char * arg);

/* mod_divides.c */
int run_fmpz_mod_divides(const char * arg);
int run_fmpz_divides_mod_list(const char * arg);

/* crt.c */
int run_fmpz_CRT(const char * arg);
int run_fmpz_CRT_balanced(const char * arg);

/* multi_crt.c */
int run_fmpz_multi_CRT(const char * arg);
int run_fmpz_multi_CRT_balanced(const char * arg);

/* crt_comb.c */
int run_fmpz_multi_mod_ui(const char * arg);
int run_fmpz_multi_CRT_ui(const char * arg);
int run_fmpz_multi_CRT_ui_balanced(const char * arg);

/* rfac.c */
int run_fmpz_rfac(const char * arg);

/* xgcd_partial.c */
int run_fmpz_xgcd_partial(const char * arg);

#endif
