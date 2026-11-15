/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs BigExponentFloat's inverse hyperbolic functions against FLINT's arb ball arithmetic. The
    mode `arf_ihyp` accepts lines of the forms `X.f_prec_round(p, rm) = (Z, O)` and
    `BigExponentFloat::f_rational_prec_round(Q, p, rm) = (Z, O)`, and their `_prec` forms, for f
    one of asinh, acosh, atanh, acoth, asech, and acsch. X and Z are BigExponentFloats in
    Malachite's exact hexadecimal format, and Q is a Rational written as `a/b` or `a`.

    The special cases follow Malachite's documentation: NaN gives NaN; asinh(±∞) = ±∞, acosh(∞) =
    ∞, and acoth(±∞) = acsch(±∞) = 0; asinh(0) = atanh(0) = 0 and asech(0) = ∞; acosh(1) =
    asech(1) = 0 and atanh(±1) = acoth(±1) = ±∞; acsch(0) is NaN (a pole, with no signed zero to
    choose a side); and every argument outside the domain gives NaN.

    Every other value is transcendental, and is rounded by bracketing it between consecutive
    numbers with one more bit than the target precision, the value being written as main + small
    (see `eval`), with the working precision doubling while a comparison is undecided.

    Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>

/* mpfr.h comes first, so that arf.h declares its MPFR conversions. */
#include <mpfr.h>

#include <flint/arb.h>
#include <flint/arf.h>
#include <flint/fmpq.h>
#include <flint/fmpz.h>

#include "oracle.h"

#define MAX_ARGS 3
#define MAX_PREC (1L << 26)

static long checked;

static int
report(int line_number, const char * what)
{
    flint_printf("line %d: %s\n", line_number, what);
    return 1;
}

static char *
split(char * s, const char * sep)
{
    char * p = strstr(s, sep);
    if (p == NULL)
        return NULL;
    *p = '\0';
    return p + strlen(sep);
}

static int
parse_rm(const char * s, arf_rnd_t * rnd, int * exact)
{
    *exact = 0;
    if (strcmp(s, "Floor") == 0)
        *rnd = ARF_RND_FLOOR;
    else if (strcmp(s, "Ceiling") == 0)
        *rnd = ARF_RND_CEIL;
    else if (strcmp(s, "Down") == 0)
        *rnd = ARF_RND_DOWN;
    else if (strcmp(s, "Up") == 0)
        *rnd = ARF_RND_UP;
    else if (strcmp(s, "Nearest") == 0)
        *rnd = ARF_RND_NEAR;
    else if (strcmp(s, "Exact") == 0)
    {
        *rnd = ARF_RND_DOWN;
        *exact = 1;
    }
    else
        return -1;
    return 0;
}

static const char *
ordering_name(int c)
{
    return c < 0 ? "Less" : c > 0 ? "Greater" : "Equal";
}

static int
parse_rational(fmpq_t q, const char * s)
{
    char buf[4096];
    size_t n = strlen(s);
    if (n == 0 || n >= sizeof(buf))
        return -1;
    memcpy(buf, s, n + 1);
    char * slash = strchr(buf, '/');
    if (slash != NULL)
        *slash = '\0';
    if (fmpz_set_str(fmpq_numref(q), buf, 10) != 0)
        return -1;
    if (slash == NULL)
        fmpz_one(fmpq_denref(q));
    else if (fmpz_set_str(fmpq_denref(q), slash + 1, 10) != 0 || fmpz_sgn(fmpq_denref(q)) <= 0)
        return -1;
    fmpq_t c;
    fmpq_init(c);
    fmpq_set(c, q);
    fmpq_canonicalise(c);
    int ok = fmpq_equal(c, q);
    fmpq_clear(c);
    return ok ? 0 : -1;
}

typedef enum { F_ASINH, F_ACOSH, F_ATANH, F_ACOTH, F_ASECH, F_ACSCH } fn_t;

static const char * FN_NAMES[] = {"asinh", "acosh", "atanh", "acoth", "asech", "acsch"};

/* The kind of an argument: NaN, an infinity, zero, or a finite nonzero number whose absolute value
   is less than, equal to, or greater than 1. */
typedef enum { K_NAN, K_INF, K_ZERO, K_LESS, K_ONE, K_GREATER } kind_t;

/* The value for an argument of kind k and sign s, per Malachite's documentation; returns 1 and
   sets z, or 0 for a value that is transcendental. */
static int
special(arf_t z, fn_t f, kind_t k, int s)
{
    int nan = 0;
    switch (k)
    {
        case K_NAN:
            nan = 1;
            break;
        case K_INF:
            if (f == F_ASINH || (f == F_ACOSH && s > 0))
            {
                if (s > 0)
                    arf_pos_inf(z);
                else
                    arf_neg_inf(z);
                return 1;
            }
            if (f == F_ACOTH || f == F_ACSCH)
            {
                arf_zero(z);
                return 1;
            }
            nan = 1;
            break;
        case K_ZERO:
            if (f == F_ASINH || f == F_ATANH)
            {
                arf_zero(z);
                return 1;
            }
            if (f == F_ASECH)
            {
                arf_pos_inf(z);
                return 1;
            }
            nan = 1;
            break;
        case K_ONE:
            if (f == F_ATANH || f == F_ACOTH)
            {
                if (s > 0)
                    arf_pos_inf(z);
                else
                    arf_neg_inf(z);
                return 1;
            }
            if (f == F_ACOSH || f == F_ASECH)
            {
                if (s > 0)
                    arf_zero(z);
                else
                    arf_nan(z);
                return 1;
            }
            return 0;
        case K_LESS:
            nan = f == F_ACOSH || f == F_ACOTH || (f == F_ASECH && s < 0);
            break;
        case K_GREATER:
            nan = f == F_ATANH || f == F_ASECH || (f == F_ACOSH && s < 0);
            break;
    }
    if (nan)
        arf_nan(z);
    return nan;
}

static kind_t
arf_kind(const arf_t x)
{
    if (arf_is_nan(x))
        return K_NAN;
    if (arf_is_inf(x))
        return K_INF;
    if (arf_is_zero(x))
        return K_ZERO;
    arf_t one;
    arf_init(one);
    arf_one(one);
    int c = arf_cmpabs(x, one);
    arf_clear(one);
    return c < 0 ? K_LESS : c == 0 ? K_ONE : K_GREATER;
}

static kind_t
fmpq_kind(const fmpq_t q)
{
    if (fmpq_is_zero(q))
        return K_ZERO;
    int c = fmpz_cmpabs(fmpq_numref(q), fmpq_denref(q));
    return c < 0 ? K_LESS : c == 0 ? K_ONE : K_GREATER;
}

typedef struct
{
    fn_t f;
    const arf_struct * xa; /* the argument, or NULL for a Rational */
    const fmpq * xq;
} value_t;

static void
argument(arb_t r, const value_t * v, slong wp)
{
    if (v->xa != NULL)
        arb_set_arf(r, v->xa);
    else
        arb_set_fmpq(r, v->xq, wp);
}

/* main + small for the value. With s = x for asinh and atanh, and s = 1/x for acoth and acsch, an
   exact x with |s| < 2^(-wp/2 - 10) gives main = s and small the second term of the series (-s^3/6
   for asinh and acsch, s^3/3 for atanh and acoth), with twice small * s^2 as the radius, which
   bounds the rest of the series. Otherwise main is arb's value and small is 0: acoth, asech, and
   acsch are atanh, acosh, and asinh of 1/x. */
static void
eval(arb_t main, arb_t small, const value_t * v, slong wp)
{
    arb_t x, t, s2;
    mag_t rad;
    arb_init(x);
    arb_init(t);
    arb_init(s2);
    mag_init(rad);
    arb_zero(small);
    argument(x, v, wp + 64);
    int reciprocal = v->f == F_ACOTH || v->f == F_ASECH || v->f == F_ACSCH;
    if (reciprocal)
        arb_inv(x, x, wp + 64);
    int series = 0;
    if (v->xa != NULL && (v->f == F_ASINH || v->f == F_ATANH))
        series = arf_cmpabs_2exp_si(v->xa, -(wp / 2) - 10) < 0;
    else if (v->xa != NULL && (v->f == F_ACOTH || v->f == F_ACSCH))
        series = arf_cmpabs_2exp_si(v->xa, wp / 2 + 11) > 0;
    if (series)
    {
        arb_set(main, x);
        arb_sqr(s2, x, wp + 64);
        arb_mul(small, s2, x, wp + 64);
        if (v->f == F_ATANH || v->f == F_ACOTH)
            arb_div_ui(small, small, 3, wp + 64);
        else
        {
            arb_div_ui(small, small, 6, wp + 64);
            arb_neg(small, small);
        }
        arb_mul(t, small, s2, wp + 64);
        arb_get_mag(rad, t);
        mag_mul_2exp_si(rad, rad, 1);
        arb_add_error_mag(small, rad);
    }
    else
    {
        switch (v->f)
        {
            case F_ASINH: case F_ACSCH: arb_asinh(main, x, wp); break;
            case F_ACOSH: case F_ASECH: arb_acosh(main, x, wp); break;
            case F_ATANH: case F_ACOTH: arb_atanh(main, x, wp); break;
        }
    }
    arb_clear(x);
    arb_clear(t);
    arb_clear(s2);
    mag_clear(rad);
}

/* The sign of (main - w) + small, or 2 if undecided. */
static int
sign_of_difference(const arb_t main, const arb_t small, const arf_t w, slong wp)
{
    arb_t d;
    arb_init(d);
    if (arb_is_exact(main))
    {
        arf_t e;
        arf_init(e);
        arf_sub(e, arb_midref(main), w, ARF_PREC_EXACT, ARF_RND_DOWN);
        arb_add_arf(d, small, e, wp);
        arf_clear(e);
    }
    else
    {
        arb_sub_arf(d, main, w, wp);
        arb_add(d, d, small, wp);
    }
    int s = arb_is_positive(d) ? 1 : arb_is_negative(d) ? -1 : arb_is_zero(d) ? 0 : 2;
    arb_clear(d);
    return s;
}

static void
neighbor(arf_t r, const arf_t a, slong q, int up)
{
    fmpz_t e;
    fmpz_init(e);
    fmpz_set(e, ARF_EXPREF(a));
    fmpz_sub_ui(e, e, q + 10);
    arf_t tiny;
    arf_init(tiny);
    arf_one(tiny);
    arf_mul_2exp_fmpz(tiny, tiny, e);
    if (up)
    {
        arf_add(r, a, tiny, ARF_PREC_EXACT, ARF_RND_DOWN);
        arf_set_round(r, r, q, ARF_RND_CEIL);
    }
    else
    {
        arf_sub(r, a, tiny, ARF_PREC_EXACT, ARF_RND_DOWN);
        arf_set_round(r, r, q, ARF_RND_FLOOR);
    }
    arf_clear(tiny);
    fmpz_clear(e);
}

/* Checks one value `value` with ordering `ordering` against the expected (z, o). */
static int
compare(int line_number, const char * value, const char * ordering, const arf_t z, int o, int exact)
{
    arf_t got;
    arf_init(got);
    int res = 0;
    if (o == 2)
        res = report(line_number, "working precision limit reached");
    else if (exact && o != 0)
        res = report(line_number, "Exact rounding of an inexact result");
    else if (ordering == NULL || arf_parse_base(value, 4, got) != 0)
        res = report(line_number, "unreadable result");
    else if (!arf_equal(got, z))
        res = report(line_number, "wrong value");
    else if (strcmp(ordering, ordering_name(o)) != 0)
        res = report(line_number, "wrong ordering");
    arf_clear(got);
    return res;
}

static int
round_value(arf_t z, const value_t * v, slong prec, arf_rnd_t rnd)
{
    slong q = prec + 1;
    arb_t main, small, sum;
    arf_t a, b, c;
    arb_init(main);
    arb_init(small);
    arb_init(sum);
    arf_init(a);
    arf_init(b);
    arf_init(c);
    int result = 2;
    for (slong wp = prec + 64; result == 2 && wp <= MAX_PREC; wp *= 2)
    {
        eval(main, small, v, wp);
        arb_add(sum, main, small, wp);
        if (!arb_is_finite(sum) || arf_is_zero(arb_midref(sum)))
            continue;
        arf_set_round(a, arb_midref(sum), q, ARF_RND_FLOOR);
        for (int step = 0; step < 8; step++)
        {
            if (arf_is_zero(a))
                break;
            int sa = sign_of_difference(main, small, a, wp);
            if (sa == 2 || sa == 0)
                break;
            if (sa < 0)
            {
                neighbor(a, a, q, 0);
                continue;
            }
            neighbor(b, a, q, 1);
            int sb = sign_of_difference(main, small, b, wp);
            if (sb == 2 || sb == 0)
                break;
            if (sb > 0)
            {
                arf_set(a, b);
                continue;
            }
            arf_add(c, a, b, ARF_PREC_EXACT, ARF_RND_DOWN);
            arf_mul_2exp_si(c, c, -1);
            arf_set_round(z, c, prec, rnd);
            result = arf_cmp(z, c) < 0 ? -1 : 1;
            break;
        }
    }
    arb_clear(main);
    arb_clear(small);
    arb_clear(sum);
    arf_clear(a);
    arf_clear(b);
    arf_clear(c);
    return result;
}

static int
expected(arf_t z, fn_t f, const arf_t x, const fmpq_t q, int rational, slong prec, arf_rnd_t rnd)
{
    kind_t k = rational ? fmpq_kind(q) : arf_kind(x);
    int sign = rational ? fmpq_sgn(q) : arf_sgn(x);
    if (special(z, f, k, sign))
        return 0;
    value_t v = {f, rational ? NULL : x, rational ? q : NULL};
    return round_value(z, &v, prec, rnd);
}

static int
check_line(char * line, int line_number)
{
    size_t len = strlen(line);
    if (len != 0 && line[len - 1] == '\n')
        line[len - 1] = '\0';
    if (line[0] == '\0')
        return 0;
    char * result = split(line, " = ");
    if (result == NULL)
        return report(line_number, "unrecognized line");
    char * name;
    char * xtext = NULL;
    const char * prefix = "BigExponentFloat::";
    if (strncmp(line, prefix, strlen(prefix)) == 0)
        name = line + strlen(prefix);
    else
    {
        char * dot = NULL;
        for (char * p = strchr(line, '.'); p != NULL && dot == NULL; p = strchr(p + 1, '.'))
        {
            char * r = p + 1;
            while ((*r >= 'a' && *r <= 'z') || *r == '_' || (*r >= '0' && *r <= '9'))
                r++;
            if (r > p + 1 && *r == '(')
                dot = p;
        }
        if (dot == NULL)
            return report(line_number, "unrecognized line");
        *dot = '\0';
        xtext = line;
        name = dot + 1;
    }
    char * args = split(name, "(");
    size_t al = args == NULL ? 0 : strlen(args);
    if (args == NULL || al == 0 || args[al - 1] != ')')
        return report(line_number, "unrecognized line");
    args[al - 1] = '\0';
    char * suffix = strstr(name, "_prec");
    if (suffix == NULL || (strcmp(suffix, "_prec_round") != 0 && strcmp(suffix, "_prec") != 0))
        return report(line_number, "missing precision");
    int with_rm = strcmp(suffix, "_prec_round") == 0;
    *suffix = '\0';
    fn_t f = F_ASINH;
    const char * fam = NULL;
    for (int i = 0; i < 6 && fam == NULL; i++)
        if (strncmp(name, FN_NAMES[i], 5) == 0)
        {
            f = (fn_t) i;
            fam = name + 5;
        }
    if (fam == NULL)
        return report(line_number, "unrecognized function");
    int rational;
    if (strcmp(fam, "") == 0)
        rational = 0;
    else if (strcmp(fam, "_rational") == 0)
        rational = 1;
    else
        return report(line_number, "unrecognized function");
    if ((xtext == NULL) != rational)
        return report(line_number, "unrecognized function");
    char * items[MAX_ARGS];
    int n_items = 0;
    for (char * p = args; *p != '\0' && n_items < MAX_ARGS;)
    {
        items[n_items++] = p;
        char * next = split(p, ", ");
        if (next == NULL)
            break;
        p = next;
    }
    if (n_items != rational + 1 + with_rm)
        return report(line_number, "wrong number of arguments");
    if (rational)
        xtext = items[0];
    char * end = NULL;
    long long prec = strtoll(items[rational], &end, 10);
    arf_rnd_t rnd = ARF_RND_NEAR;
    int exact = 0;
    arf_t x, z1;
    fmpq_t q;
    arf_init(x);
    arf_init(z1);
    fmpq_init(q);
    int res = 0;
    if (rational ? parse_rational(q, xtext) != 0 : arf_parse_base(xtext, 4, x) != 0)
        res = report(line_number, "unreadable argument");
    else if (*end != '\0' || prec <= 0)
        res = report(line_number, "unreadable precision");
    else if (with_rm && parse_rm(items[n_items - 1], &rnd, &exact) != 0)
        res = report(line_number, "unreadable rounding mode");
    if (res == 0)
    {
        int o1 = expected(z1, f, x, q, rational, prec, rnd);
        char * value = result;
        size_t rl = strlen(value);
        if (rl < 2 || value[0] != '(' || value[rl - 1] != ')')
            res = report(line_number, "unrecognized result");
        else
        {
            value[rl - 1] = '\0';
            value++;
            char * ordering = split(value, ", ");
            res = compare(line_number, value, ordering, z1, o1, exact);
        }
    }
    arf_clear(x);
    arf_clear(z1);
    fmpq_clear(q);
    if (res == 0)
        checked++;
    return res;
}

int
run_arf_ihyp(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_ihyp", checked);
}
