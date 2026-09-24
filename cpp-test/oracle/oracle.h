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
#include <flint/fmpz_poly.h>

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

/* Parses a polynomial as Malachite displays it, such as `x^2-3*x-3`, `-x+1`, or `0`: terms of
   the form `c*x^e`, with a coefficient of 1 and an exponent of 1 left out, and a sign between
   terms. Returns 1 on success and 0 if the text is not in that form. */
int fmpz_poly_set_str_malachite(fmpz_poly_t poly, const char * s);

/* Splits, in place, a line in which a polynomial `P` is combined with a scalar `M` to give `R`.
   The shapes recognized are `(P).method(M) = R`, `(&(P)).method(M) = R`,
   `p := P; p.assign_method(M); p = R`, `(P) op M = R`, `&(P) op M = R`, and
   `p := P; p op= M; p = R`; pass NULL for `method`, `assign_method`, or `op` to skip those
   shapes. The in-place name is passed separately because it is not always derivable from the
   other (`mod_op` goes with `mod_assign`). Strips a trailing newline. Returns 1 and sets the
   three pointers on success, and 0 if the line has none of the shapes. */
int split_polynomial_scalar_line(char * line, const char * method, const char * assign_method,
                                 const char * op, char ** receiver, char ** arg, char ** result);

/* Returns 0 if `checked` is positive, and otherwise reports that no line of the input had the
   shape the mode `name` looks for and returns 1. A mode skips lines it cannot parse, so without
   this check a change to a demo's output format would make every run pass vacuously. */
int require_some_lines(const char * name, long checked);

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

/* height.c */
int run_fmpq_height(const char * arg);
int run_fmpq_height_bits(const char * arg);

/* fmpq_gcd.c */
int run_fmpq_gcd(const char * arg);
int run_fmpq_gcd_cofactors(const char * arg);

/* fmpq_enumeration.c */
int run_fmpq_farey_neighbors(const char * arg);
int run_arith_bell_number(const char * arg);
int run_arith_landau_function_vec(const char * arg);
int run_arith_bell_number_vec(const char * arg);
int run_fmpq_dedekind_sum(const char * arg);
int run_fmpq_harmonic(const char * arg);
int run_fmpq_next_minimal(const char * arg);
int run_fmpq_next_signed_minimal(const char * arg);

/* fmpq_reconstruct.c */
int run_fmpq_reconstruct(const char * arg);
int run_fmpq_reconstruct_2(const char * arg);

/* fmpz_poly_smod.c */
int run_fmpz_poly_scalar_smod_fmpz(const char * arg);

/* fmpz_poly_mod.c */
int run_fmpz_poly_scalar_mod_fmpz(const char * arg);
int run_fmpz_poly_get_nmod_poly(const char * arg);
int run_fmpz_mod_poly_set_fmpz_poly(const char * arg);

/* fmpz_poly_evaluate.c */
int run_fmpz_poly_evaluate_fmpz(const char * arg);
int run_fmpz_poly_evaluate_horner_fmpz(const char * arg);
int run_fmpz_poly_evaluate_divconquer_fmpz(const char * arg);

#endif
