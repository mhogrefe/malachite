/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the multipoint evaluation and product-of-roots demos against FLINT.

    `mod_evaluate_many` reads `(P).mod_evaluate_many(&[X, ...], M) = [Y, ...]` and
    `(P).mod_evaluate_many_precomputed(M, &data([X, ...])) = [Y, ...]`, with the polynomial
    possibly written `&(P)` and the modulus possibly written `&M`, and checks them against
    fmpz_mod_poly_evaluate_fmpz_vec and, when M is at least 2 and fits in a word, against
    nmod_poly_evaluate_nmod_vec_fast as well.

    `mod_power_of_2_evaluate_many` reads the same shapes with `mod_power_of_2_evaluate_many` and
    `mod_power_of_2_evaluate_many_precomputed`, an exponent k in place of M, and the modulus 2^k.

    `mod_from_roots` reads `T::mod_from_roots(&[R, ...], M) = P`, with the modulus possibly written
    `&M`, and `T::mod_power_of_2_from_roots(&[R, ...], k) = P`, and checks them against
    fmpz_mod_poly_product_roots_fmpz_vec and, when the modulus is at least 2 and fits in a word,
    against nmod_poly_product_roots_nmod_vec as well.

    `from_roots` reads `IntegerPolynomial::from_roots(&[R, ...]) = P`,
    `IntegerPolynomial::from_rational_roots(&[R, ...]) = P`, and
    `RationalPolynomial::from_roots(&[R, ...]) = P`, and checks them against
    fmpz_poly_product_roots_fmpz_vec and fmpz_poly_product_roots_fmpq_vec, the last made monic.

    Modulo 1 every polynomial and every value is 0, and that is checked directly. Any other
    nonempty line is an error, as is an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_poly.h>
#include <flint/fmpz_vec.h>
#include <flint/fmpq_vec.h>
#include <flint/nmod_poly.h>

#include "oracle.h"

static long checked;
static long nmod_lines;

/* Cuts `line` at `needle`, returning the text after it, or NULL if the needle is absent. */
static char *
cut(char * line, const char * needle)
{
    char * at = strstr(line, needle);
    if (at == NULL)
    {
        return NULL;
    }
    *at = '\0';
    return at + strlen(needle);
}

/* Strips one leading `&` from an argument, as in `&M`. */
static char *
strip_amp(char * s)
{
    return s[0] == '&' ? s + 1 : s;
}

/* The polynomial in a receiver written `(P)` or `(&(P))`, in place, or NULL. Cutting at the
   method name, which begins with the receiver's closing parenthesis, leaves `(P` or `(&(P)`. */
static char *
receiver_polynomial(char * receiver)
{
    if (receiver[0] != '(')
    {
        return NULL;
    }
    receiver++;
    if (receiver[0] == '&')
    {
        size_t len = strlen(receiver);
        if (len < 3 || receiver[1] != '(' || receiver[len - 1] != ')')
        {
            return NULL;
        }
        receiver[len - 1] = '\0';
        receiver += 2;
    }
    return receiver;
}

/* Splits a bracketed list `[a, b, c]` in place into its entries, returning their number, or -1 if
   the text is not in that form. At most `max` entries are stored. */
static slong
split_list(char * s, char ** entries, slong max)
{
    size_t len = strlen(s);
    if (len < 2 || s[0] != '[' || s[len - 1] != ']')
    {
        return -1;
    }
    s[len - 1] = '\0';
    s++;
    if (s[0] == '\0')
    {
        return 0;
    }
    slong n = 0;
    while (1)
    {
        if (n == max)
        {
            return -1;
        }
        entries[n++] = s;
        char * comma = strstr(s, ", ");
        if (comma == NULL)
        {
            break;
        }
        *comma = '\0';
        s = comma + 2;
    }
    return n;
}

/* Parses a bracketed list of integers, returning a new vector of `*len` entries, or NULL. */
static fmpz *
parse_fmpz_list(char * s, slong * len)
{
    slong cap = (slong) strlen(s) / 2 + 2;
    char ** entries = flint_malloc(cap * sizeof(char *));
    slong n = split_list(s, entries, cap);
    if (n < 0)
    {
        flint_free(entries);
        return NULL;
    }
    fmpz * v = _fmpz_vec_init(n + 1);
    for (slong i = 0; i < n; i++)
    {
        if (fmpz_set_str(v + i, entries[i], 10) != 0)
        {
            flint_free(entries);
            _fmpz_vec_clear(v, n + 1);
            return NULL;
        }
    }
    flint_free(entries);
    *len = n;
    return v;
}

/* Parses a bracketed list of rationals, returning a new vector of `*len` entries, or NULL. */
static fmpq *
parse_fmpq_list(char * s, slong * len)
{
    slong cap = (slong) strlen(s) / 2 + 2;
    char ** entries = flint_malloc(cap * sizeof(char *));
    slong n = split_list(s, entries, cap);
    if (n < 0)
    {
        flint_free(entries);
        return NULL;
    }
    fmpq * v = _fmpq_vec_init(n + 1);
    for (slong i = 0; i < n; i++)
    {
        if (fmpq_set_str(v + i, entries[i], 10) != 0)
        {
            flint_free(entries);
            _fmpq_vec_clear(v, n + 1);
            return NULL;
        }
        fmpq_canonicalise(v + i);
    }
    flint_free(entries);
    *len = n;
    return v;
}

static int
report(const char * mode, int line_number, const char * what)
{
    flint_printf("error in %s test, line %d: %s\n", mode, line_number, what);
    return 1;
}

/* Checks values `ys` of the polynomial `p` at the points `xs` modulo `m`. */
static int
check_values(const char * mode, int line_number, const fmpz_poly_t p, const fmpz * xs,
             const fmpz * ys, slong n, const fmpz_t m)
{
    if (fmpz_is_one(m))
    {
        if (!fmpz_poly_is_zero(p) || !_fmpz_vec_is_zero(xs, n) || !_fmpz_vec_is_zero(ys, n))
        {
            return report(mode, line_number, "modulo 1, every value must be 0");
        }
        return 0;
    }
    fmpz_mod_ctx_t ctx;
    fmpz_mod_ctx_init(ctx, m);
    fmpz_mod_poly_t q;
    fmpz_mod_poly_init(q, ctx);
    fmpz_mod_poly_set_fmpz_poly(q, p, ctx);
    fmpz * expected = _fmpz_vec_init(n + 1);
    fmpz_mod_poly_evaluate_fmpz_vec(expected, q, xs, n, ctx);
    int failed = !_fmpz_vec_equal(expected, ys, n);
    if (!failed && fmpz_abs_fits_ui(m))
    {
        nmod_poly_t r;
        nmod_poly_init(r, fmpz_get_ui(m));
        fmpz_poly_get_nmod_poly(r, p);
        ulong * xs_n = flint_malloc((n + 1) * sizeof(ulong));
        ulong * ys_n = flint_malloc((n + 1) * sizeof(ulong));
        for (slong i = 0; i < n; i++)
        {
            xs_n[i] = fmpz_get_ui(xs + i);
        }
        nmod_poly_evaluate_nmod_vec_fast(ys_n, r, xs_n, n);
        for (slong i = 0; i < n; i++)
        {
            if (!fmpz_equal_ui(ys + i, ys_n[i]))
            {
                failed = 1;
            }
        }
        flint_free(xs_n);
        flint_free(ys_n);
        nmod_poly_clear(r);
        nmod_lines++;
    }
    _fmpz_vec_clear(expected, n + 1);
    fmpz_mod_poly_clear(q, ctx);
    fmpz_mod_ctx_clear(ctx);
    return failed ? report(mode, line_number, "values disagree") : 0;
}

/* Handles one evaluation line, where `method` and `precomputed_method` are the method names with
   their leading dot and opening parenthesis, and `power_of_2` says whether the modulus argument
   is an exponent. */
static int
check_evaluate_line(char * line, int line_number, const char * mode, const char * method,
                    const char * precomputed_method, int power_of_2)
{
    line[strcspn(line, "\r\n")] = '\0';
    if (line[0] == '\0')
    {
        return 0;
    }
    char * rest;
    char * xs_str;
    char * m_str;
    int precomputed = strstr(line, precomputed_method) != NULL;
    if (precomputed)
    {
        rest = cut(line, precomputed_method);
        m_str = rest;
        xs_str = cut(rest, ", &data(");
        if (xs_str == NULL)
        {
            return report(mode, line_number, "unrecognized line");
        }
    }
    else
    {
        rest = cut(line, method);
        if (rest == NULL)
        {
            return report(mode, line_number, "unrecognized line");
        }
        xs_str = rest;
        m_str = NULL;
    }
    char * ys_str = precomputed ? cut(xs_str, ")) = ") : cut(xs_str, ") = ");
    if (ys_str == NULL)
    {
        return report(mode, line_number, "unrecognized line");
    }
    if (!precomputed)
    {
        if (xs_str[0] != '&')
        {
            return report(mode, line_number, "unrecognized line");
        }
        xs_str++;
        char * close = strstr(xs_str, "], ");
        if (close == NULL)
        {
            return report(mode, line_number, "unrecognized line");
        }
        close[1] = '\0';
        m_str = close + 3;
    }
    m_str = strip_amp(m_str);
    /* the method names begin with the receiver's closing parenthesis, which `cut` removed */
    char * p_str = receiver_polynomial(line);
    if (p_str == NULL)
    {
        return report(mode, line_number, "unrecognized receiver");
    }
    fmpz_poly_t p;
    fmpz_poly_init(p);
    fmpz_t m;
    fmpz_init(m);
    slong n = 0;
    slong n_ys = 0;
    fmpz * xs = parse_fmpz_list(xs_str, &n);
    fmpz * ys = parse_fmpz_list(ys_str, &n_ys);
    int ok = xs != NULL && ys != NULL && fmpz_poly_set_str_malachite(p, p_str);
    if (ok)
    {
        if (power_of_2)
        {
            ulong k = strtoul(m_str, NULL, 10);
            fmpz_one(m);
            fmpz_mul_2exp(m, m, k);
        }
        else
        {
            ok = fmpz_set_str(m, m_str, 10) == 0;
        }
    }
    int result;
    if (!ok || n != n_ys)
    {
        result = report(mode, line_number, "malformed operands");
    }
    else
    {
        result = check_values(mode, line_number, p, xs, ys, n, m);
    }
    if (xs != NULL)
    {
        _fmpz_vec_clear(xs, n + 1);
    }
    if (ys != NULL)
    {
        _fmpz_vec_clear(ys, n_ys + 1);
    }
    fmpz_clear(m);
    fmpz_poly_clear(p);
    checked++;
    return result;
}

static int
mod_evaluate_many_line(char * line, int line_number)
{
    return check_evaluate_line(line, line_number, "mod_evaluate_many", ").mod_evaluate_many(",
                               ").mod_evaluate_many_precomputed(", 0);
}

static int
mod_power_of_2_evaluate_many_line(char * line, int line_number)
{
    return check_evaluate_line(line, line_number, "mod_power_of_2_evaluate_many",
                               ").mod_power_of_2_evaluate_many(",
                               ").mod_power_of_2_evaluate_many_precomputed(", 1);
}

int
run_mod_evaluate_many(const char * arg)
{
    checked = 0;
    nmod_lines = 0;
    int result = for_each_line(arg, mod_evaluate_many_line);
    if (result == 0)
    {
        result = require_some_lines("mod_evaluate_many", checked);
    }
    if (result == 0)
    {
        flint_printf("mod_evaluate_many: %wd lines agree (%wd also with nmod)\n", checked,
                     nmod_lines);
    }
    return result;
}

int
run_mod_power_of_2_evaluate_many(const char * arg)
{
    checked = 0;
    nmod_lines = 0;
    int result = for_each_line(arg, mod_power_of_2_evaluate_many_line);
    if (result == 0)
    {
        result = require_some_lines("mod_power_of_2_evaluate_many", checked);
    }
    if (result == 0)
    {
        flint_printf("mod_power_of_2_evaluate_many: %wd lines agree (%wd also with nmod)\n",
                     checked, nmod_lines);
    }
    return result;
}

static int
mod_from_roots_line(char * line, int line_number)
{
    const char * mode = "mod_from_roots";
    line[strcspn(line, "\r\n")] = '\0';
    if (line[0] == '\0')
    {
        return 0;
    }
    int power_of_2 = strstr(line, "::mod_power_of_2_from_roots(&") != NULL;
    char * args = cut(line, power_of_2 ? "::mod_power_of_2_from_roots(&" : "::mod_from_roots(&");
    if (args == NULL)
    {
        return report(mode, line_number, "unrecognized line");
    }
    char * p_str = cut(args, ") = ");
    char * m_str = p_str == NULL ? NULL : strstr(args, "], ");
    if (m_str == NULL)
    {
        return report(mode, line_number, "unrecognized line");
    }
    m_str[1] = '\0';
    m_str = strip_amp(m_str + 3);
    fmpz_poly_t p;
    fmpz_poly_init(p);
    fmpz_t m;
    fmpz_init(m);
    slong n = 0;
    fmpz * roots = parse_fmpz_list(args, &n);
    int ok = roots != NULL && fmpz_poly_set_str_malachite(p, p_str);
    if (ok)
    {
        if (power_of_2)
        {
            fmpz_one(m);
            fmpz_mul_2exp(m, m, strtoul(m_str, NULL, 10));
        }
        else
        {
            ok = fmpz_set_str(m, m_str, 10) == 0;
        }
    }
    int result = 0;
    if (!ok)
    {
        result = report(mode, line_number, "malformed operands");
    }
    else if (fmpz_is_one(m))
    {
        if (!fmpz_poly_is_zero(p))
        {
            result = report(mode, line_number, "modulo 1, the product must be 0");
        }
    }
    else
    {
        fmpz_mod_ctx_t ctx;
        fmpz_mod_ctx_init(ctx, m);
        fmpz_mod_poly_t expected;
        fmpz_mod_poly_init(expected, ctx);
        fmpz_mod_poly_product_roots_fmpz_vec(expected, roots, n, ctx);
        fmpz_poly_t e;
        fmpz_poly_init(e);
        fmpz_mod_poly_get_fmpz_poly(e, expected, ctx);
        if (!fmpz_poly_equal(e, p))
        {
            result = report(mode, line_number, "products disagree");
        }
        else if (fmpz_abs_fits_ui(m))
        {
            nmod_poly_t r;
            nmod_poly_init(r, fmpz_get_ui(m));
            ulong * rs = flint_malloc((n + 1) * sizeof(ulong));
            for (slong i = 0; i < n; i++)
            {
                rs[i] = fmpz_get_ui(roots + i);
            }
            nmod_poly_product_roots_nmod_vec(r, rs, n);
            fmpz_poly_t f;
            fmpz_poly_init(f);
            fmpz_poly_set_nmod_poly_unsigned(f, r);
            if (!fmpz_poly_equal(f, p))
            {
                result = report(mode, line_number, "products disagree with nmod");
            }
            fmpz_poly_clear(f);
            flint_free(rs);
            nmod_poly_clear(r);
            nmod_lines++;
        }
        fmpz_poly_clear(e);
        fmpz_mod_poly_clear(expected, ctx);
        fmpz_mod_ctx_clear(ctx);
    }
    if (roots != NULL)
    {
        _fmpz_vec_clear(roots, n + 1);
    }
    fmpz_clear(m);
    fmpz_poly_clear(p);
    checked++;
    return result;
}

int
run_mod_from_roots(const char * arg)
{
    checked = 0;
    nmod_lines = 0;
    int result = for_each_line(arg, mod_from_roots_line);
    if (result == 0)
    {
        result = require_some_lines("mod_from_roots", checked);
    }
    if (result == 0)
    {
        flint_printf("mod_from_roots: %wd lines agree (%wd also with nmod)\n", checked,
                     nmod_lines);
    }
    return result;
}

static int
from_roots_line(char * line, int line_number)
{
    const char * mode = "from_roots";
    line[strcspn(line, "\r\n")] = '\0';
    if (line[0] == '\0')
    {
        return 0;
    }
    int rational_roots = strstr(line, "IntegerPolynomial::from_rational_roots(&") != NULL;
    int rational_polynomial = strstr(line, "RationalPolynomial::from_roots(&") != NULL;
    char * args;
    if (rational_roots)
    {
        args = cut(line, "IntegerPolynomial::from_rational_roots(&");
    }
    else if (rational_polynomial)
    {
        args = cut(line, "RationalPolynomial::from_roots(&");
    }
    else
    {
        args = cut(line, "IntegerPolynomial::from_roots(&");
    }
    char * p_str = args == NULL ? NULL : cut(args, ") = ");
    if (p_str == NULL)
    {
        return report(mode, line_number, "unrecognized line");
    }
    int result = 0;
    slong n = 0;
    if (rational_roots || rational_polynomial)
    {
        fmpq * roots = parse_fmpq_list(args, &n);
        if (roots == NULL)
        {
            return report(mode, line_number, "malformed roots");
        }
        fmpz_poly_t e;
        fmpz_poly_init(e);
        fmpz_poly_product_roots_fmpq_vec(e, roots, n);
        if (rational_roots)
        {
            fmpz_poly_t p;
            fmpz_poly_init(p);
            if (!fmpz_poly_set_str_malachite(p, p_str))
            {
                result = report(mode, line_number, "malformed polynomial");
            }
            else if (!fmpz_poly_equal(e, p))
            {
                result = report(mode, line_number, "products disagree");
            }
            fmpz_poly_clear(p);
        }
        else
        {
            fmpq_poly_t p;
            fmpq_poly_t q;
            fmpq_poly_init(p);
            fmpq_poly_init(q);
            fmpq_poly_set_fmpz_poly(q, e);
            fmpq_poly_make_monic(q, q);
            if (!fmpq_poly_set_str_malachite(p, p_str))
            {
                result = report(mode, line_number, "malformed polynomial");
            }
            else if (!fmpq_poly_equal(q, p))
            {
                result = report(mode, line_number, "products disagree");
            }
            fmpq_poly_clear(p);
            fmpq_poly_clear(q);
        }
        fmpz_poly_clear(e);
        _fmpq_vec_clear(roots, n + 1);
    }
    else
    {
        fmpz * roots = parse_fmpz_list(args, &n);
        fmpz_poly_t p;
        fmpz_poly_init(p);
        if (roots == NULL || !fmpz_poly_set_str_malachite(p, p_str))
        {
            result = report(mode, line_number, "malformed operands");
        }
        else
        {
            fmpz_poly_t e;
            fmpz_poly_init(e);
            fmpz_poly_product_roots_fmpz_vec(e, roots, n);
            if (!fmpz_poly_equal(e, p))
            {
                result = report(mode, line_number, "products disagree");
            }
            fmpz_poly_clear(e);
        }
        if (roots != NULL)
        {
            _fmpz_vec_clear(roots, n + 1);
        }
        fmpz_poly_clear(p);
    }
    checked++;
    return result;
}

int
run_from_roots(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, from_roots_line);
    if (result == 0)
    {
        result = require_some_lines("from_roots", checked);
    }
    if (result == 0)
    {
        flint_printf("from_roots: %wd lines agree\n", checked);
    }
    return result;
}
