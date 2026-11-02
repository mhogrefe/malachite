/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the series reversion demos against FLINT.

    Mode mod_revert_series reads `(P).mod_compositional_inverse_truncated(N, M) = R` and mode
    mod_power_of_2_revert_series reads `(P).mod_power_of_2_compositional_inverse_truncated(N, K) =
    R`, from the UnsignedPolynomial and NaturalPolynomial demos, with the modulus m or 2^K. When P
    has constant coefficient 0 and a coefficient of x invertible modulo m, R is checked against
    gr_poly_revert_series and gr_poly_revert_series_newton over fmpz_mod, against
    gr_poly_revert_series_lagrange and _lagrange_fast whenever they succeed, and against
    nmod_poly_revert_series when m fits in a word; otherwise R must be None. Modulo 1, every result
    must be 0.

    Mode fmpz_poly_revert_series reads `(P).compositional_inverse_truncated(N) = R` from the
    IntegerPolynomial demos: when P has constant coefficient 0 and coefficient of x 1 or -1, R is
    checked against fmpz_poly_revert_series and the fmpq_poly functions below; otherwise R must be
    None. Mode fmpq_poly_revert_series reads the same shape from the RationalPolynomial demos: when
    P has constant coefficient 0 and a nonzero coefficient of x, R is checked against
    fmpq_poly_revert_series, _lagrange, _lagrange_fast, and _newton; otherwise R must be None.

    In every mode, a reversible P with N less than 2 must give 0, where FLINT's functions are not
    defined. Polynomials may be written `&(...)` and moduli `&M`. Any other nonempty line is an
    error, as is an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_poly.h>
#include <flint/fmpq_poly.h>
#include <flint/gr.h>
#include <flint/gr_poly.h>
#include <flint/nmod_poly.h>

#include "oracle.h"

static long checked;
static long reversible_lines;
static long nmod_lines;
static int power_of_2;
/* Whether the lines come from IntegerPolynomial demos rather than RationalPolynomial ones. */
static int integer_mode;
static const char * mode_name;

/* Strips a leading `&` from an argument, in place. */
static char *
strip_amp(char * argument)
{
    return argument[0] == '&' ? argument + 1 : argument;
}

/* Compares a FLINT result with the expected polynomial, printing an error that names `function`
   if they differ. */
static int
compare(const fmpz_poly_t actual, const fmpz_poly_t expected, const char * function,
        int line_number)
{
    if (fmpz_poly_equal(actual, expected))
    {
        return 0;
    }
    flint_printf("error in %s test, line %d, by %s. FLINT: ", mode_name, line_number, function);
    fmpz_poly_print(actual);
    flint_printf("\n");
    return 1;
}

static int
compare_q(const fmpq_poly_t actual, const fmpq_poly_t expected, const char * function,
          int line_number)
{
    if (fmpq_poly_equal(actual, expected))
    {
        return 0;
    }
    flint_printf("error in %s test, line %d, by %s. FLINT: ", mode_name, line_number, function);
    fmpq_poly_print(actual);
    flint_printf("\n");
    return 1;
}

/* Checks one `gr_poly` reversion function over fmpz_mod. A function marked `may_fail` may report
   that it cannot compute the result, as the Lagrange algorithms do when a division by an index is
   not possible; the others must succeed. */
static int
check_gr(int (* revert)(gr_poly_t, const gr_poly_t, slong, gr_ctx_t), int may_fail,
         const gr_poly_t f, slong n, gr_ctx_t ctx, const fmpz_poly_t expected,
         const char * function, int line_number)
{
    int result = 0;
    gr_poly_t r;
    gr_poly_init(r, ctx);
    int status = revert(r, f, n, ctx);
    if (status != GR_SUCCESS)
    {
        if (!may_fail)
        {
            flint_printf("error in %s test, line %d: %s failed\n", mode_name, line_number,
                         function);
            result = 1;
        }
    }
    else
    {
        fmpz_poly_t actual;
        fmpz_poly_init(actual);
        for (slong i = 0; i < r->length; i++)
        {
            fmpz_poly_set_coeff_fmpz(actual, i, (const fmpz *) gr_poly_entry_srcptr(r, i, ctx));
        }
        result = compare(actual, expected, function, line_number);
        fmpz_poly_clear(actual);
    }
    gr_poly_clear(r, ctx);
    return result;
}

static int
check_modular(const fmpz_poly_t p, slong n, const fmpz_t m, const fmpz_poly_t expected,
              int line_number)
{
    int result = 0;
    gr_ctx_t ctx;
    gr_ctx_init_fmpz_mod(ctx, m);
    gr_poly_t f;
    gr_poly_init(f, ctx);
    GR_MUST_SUCCEED(gr_poly_set_fmpz_poly(f, p, ctx));
    result |= check_gr(gr_poly_revert_series, 0, f, n, ctx, expected, "gr_poly_revert_series",
                       line_number);
    result |= check_gr(gr_poly_revert_series_newton, 0, f, n, ctx, expected,
                       "gr_poly_revert_series_newton", line_number);
    result |= check_gr(gr_poly_revert_series_lagrange, 1, f, n, ctx, expected,
                       "gr_poly_revert_series_lagrange", line_number);
    result |= check_gr(gr_poly_revert_series_lagrange_fast, 1, f, n, ctx, expected,
                       "gr_poly_revert_series_lagrange_fast", line_number);
    gr_poly_clear(f, ctx);
    gr_ctx_clear(ctx);
    if (fmpz_abs_fits_ui(m))
    {
        nmod_lines++;
        nmod_poly_t g, r;
        nmod_poly_init(g, fmpz_get_ui(m));
        nmod_poly_init(r, fmpz_get_ui(m));
        fmpz_poly_get_nmod_poly(g, p);
        nmod_poly_revert_series(r, g, n);
        fmpz_poly_t actual;
        fmpz_poly_init(actual);
        fmpz_poly_set_nmod_poly_unsigned(actual, r);
        result |= compare(actual, expected, "nmod_poly_revert_series", line_number);
        fmpz_poly_clear(actual);
        nmod_poly_clear(g);
        nmod_poly_clear(r);
    }
    return result;
}

/* Splits a line `(P).METHOD(ARGS) = R` in place, with `METHOD` given including the dot and the
   opening parenthesis. Returns 0 if the line does not have that shape. */
static int
split(char * line, const char * method, char ** p, char ** args, char ** r)
{
    line[strcspn(line, "\r\n")] = '\0';
    if (line[0] != '(')
        return 0;
    char * at = strstr(line, method);
    if (at == NULL || at == line || at[-1] != ')')
        return 0;
    at[-1] = '\0';
    *p = line + 1;
    size_t len = strlen(*p);
    if (len >= 3 && (*p)[0] == '&' && (*p)[1] == '(' && (*p)[len - 1] == ')')
    {
        (*p)[len - 1] = '\0';
        *p += 2;
    }
    *args = at + strlen(method);
    char * eq = strstr(*args, ") = ");
    if (eq == NULL)
        return 0;
    *eq = '\0';
    *r = eq + 4;
    return 1;
}

static int
unrecognized(char * line, int line_number)
{
    if (line[0] == '\0')
    {
        return 0;
    }
    flint_printf("error in %s test, line %d: unrecognized line\n", mode_name, line_number);
    return 1;
}

static int
unreadable(int line_number)
{
    flint_printf("error in %s test, line %d: unreadable input\n", mode_name, line_number);
    return 1;
}

static int
expect_none(const char * r, int line_number)
{
    if (strcmp(r, "None") == 0)
    {
        return 0;
    }
    flint_printf("error in %s test, line %d: the polynomial has no inverse, but the result is %s\n",
                 mode_name, line_number, r);
    return 1;
}

static int
check_modular_line(char * line, int line_number)
{
    char * p_str;
    char * args;
    char * r_str;
    const char * method = power_of_2 ? ".mod_power_of_2_compositional_inverse_truncated("
                                     : ".mod_compositional_inverse_truncated(";
    if (!split(line, method, &p_str, &args, &r_str))
    {
        return unrecognized(line, line_number);
    }
    char * m_str = strstr(args, ", ");
    if (m_str == NULL)
    {
        return unrecognized(line, line_number);
    }
    *m_str = '\0';
    m_str = strip_amp(m_str + 2);
    checked++;
    int result = 0;
    slong n = atol(args);
    fmpz_poly_t p, expected;
    fmpz_t m, g;
    fmpz_poly_init(p);
    fmpz_poly_init(expected);
    fmpz_init(m);
    fmpz_init(g);
    if (!fmpz_poly_set_str_malachite(p, p_str))
    {
        result = unreadable(line_number);
        goto cleanup;
    }
    if (power_of_2)
    {
        fmpz_one(m);
        fmpz_mul_2exp(m, m, strtoul(m_str, NULL, 10));
    }
    else if (fmpz_set_str(m, m_str, 10) != 0 || fmpz_sgn(m) <= 0)
    {
        result = unreadable(line_number);
        goto cleanup;
    }
    if (fmpz_is_one(m))
    {
        if (strcmp(r_str, "0") != 0)
        {
            flint_printf("error in %s test, line %d: every result modulo 1 must be 0\n", mode_name,
                         line_number);
            result = 1;
        }
        goto cleanup;
    }
    int reversible = 0;
    if (fmpz_poly_length(p) >= 2 && fmpz_is_zero(p->coeffs))
    {
        fmpz_gcd(g, p->coeffs + 1, m);
        reversible = fmpz_is_one(g);
    }
    if (!reversible)
    {
        result = expect_none(r_str, line_number);
        goto cleanup;
    }
    if (!fmpz_poly_set_str_malachite(expected, r_str))
    {
        result = unreadable(line_number);
        goto cleanup;
    }
    reversible_lines++;
    if (n < 2)
    {
        if (!fmpz_poly_is_zero(expected))
        {
            flint_printf("error in %s test, line %d: the inverse to fewer than 2 terms must be 0\n",
                         mode_name, line_number);
            result = 1;
        }
        goto cleanup;
    }
    result = check_modular(p, n, m, expected, line_number);
cleanup:
    fmpz_poly_clear(p);
    fmpz_poly_clear(expected);
    fmpz_clear(m);
    fmpz_clear(g);
    return result;
}

static void (* const fmpq_reverts[])(fmpq_poly_t, const fmpq_poly_t, slong) = {
    fmpq_poly_revert_series, fmpq_poly_revert_series_lagrange,
    fmpq_poly_revert_series_lagrange_fast, fmpq_poly_revert_series_newton};
static const char * const fmpq_revert_names[] = {
    "fmpq_poly_revert_series", "fmpq_poly_revert_series_lagrange",
    "fmpq_poly_revert_series_lagrange_fast", "fmpq_poly_revert_series_newton"};

static int
check_unmodular_line(char * line, int line_number)
{
    char * p_str;
    char * n_str;
    char * r_str;
    if (!split(line, ".compositional_inverse_truncated(", &p_str, &n_str, &r_str))
    {
        return unrecognized(line, line_number);
    }
    checked++;
    int result = 0;
    slong n = atol(n_str);
    fmpq_poly_t p, expected, r;
    fmpq_poly_init(p);
    fmpq_poly_init(expected);
    fmpq_poly_init(r);
    if (!fmpq_poly_set_str_malachite(p, p_str))
    {
        result = unreadable(line_number);
        goto cleanup;
    }
    int reversible = fmpq_poly_length(p) >= 2 && fmpz_is_zero(p->coeffs)
                     && !fmpz_is_zero(p->coeffs + 1);
    int integer = fmpz_is_one(p->den);
    if (integer_mode && reversible)
    {
        /* Over the integers, the coefficient of x must be 1 or -1. */
        reversible = integer && fmpz_is_pm1(p->coeffs + 1);
    }
    if (!reversible)
    {
        result = expect_none(r_str, line_number);
        goto cleanup;
    }
    if (!fmpq_poly_set_str_malachite(expected, r_str))
    {
        result = unreadable(line_number);
        goto cleanup;
    }
    reversible_lines++;
    if (n < 2)
    {
        if (!fmpq_poly_is_zero(expected))
        {
            flint_printf("error in %s test, line %d: the inverse to fewer than 2 terms must be 0\n",
                         mode_name, line_number);
            result = 1;
        }
        goto cleanup;
    }
    for (int i = 0; i < 4; i++)
    {
        fmpq_reverts[i](r, p, n);
        result |= compare_q(r, expected, fmpq_revert_names[i], line_number);
    }
    if (integer && fmpz_is_pm1(p->coeffs + 1))
    {
        fmpz_poly_t pz, rz, ez;
        fmpz_poly_init(pz);
        fmpz_poly_init(rz);
        fmpz_poly_init(ez);
        fmpq_poly_get_numerator(pz, p);
        fmpq_poly_get_numerator(ez, expected);
        fmpz_poly_revert_series(rz, pz, n);
        if (!fmpz_is_one(expected->den))
        {
            flint_printf("error in %s test, line %d: the inverse must have integer coefficients\n",
                         mode_name, line_number);
            result = 1;
        }
        else
        {
            result |= compare(rz, ez, "fmpz_poly_revert_series", line_number);
        }
        fmpz_poly_clear(pz);
        fmpz_poly_clear(rz);
        fmpz_poly_clear(ez);
    }
cleanup:
    fmpq_poly_clear(p);
    fmpq_poly_clear(expected);
    fmpq_poly_clear(r);
    return result;
}

static int
run(const char * arg, int pow2, int integers, const char * name, int (* handler)(char *, int))
{
    integer_mode = integers;
    checked = 0;
    reversible_lines = 0;
    nmod_lines = 0;
    power_of_2 = pow2;
    mode_name = name;
    int result = for_each_line(arg, handler);
    if (result == 0)
    {
        flint_printf("%s: %ld lines, %ld of them with an inverse, %ld of those also by nmod_poly\n",
                     name, checked, reversible_lines, nmod_lines);
    }
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_mod_revert_series(const char * arg)
{
    return run(arg, 0, 0, "mod_revert_series", check_modular_line);
}

int
run_mod_power_of_2_revert_series(const char * arg)
{
    return run(arg, 1, 0, "mod_power_of_2_revert_series", check_modular_line);
}

int
run_fmpz_poly_revert_series(const char * arg)
{
    return run(arg, 0, 1, "fmpz_poly_revert_series", check_unmodular_line);
}

int
run_fmpq_poly_revert_series(const char * arg)
{
    return run(arg, 0, 0, "fmpq_poly_revert_series", check_unmodular_line);
}
