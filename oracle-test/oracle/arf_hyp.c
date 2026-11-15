/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs BigExponentFloat's forward hyperbolic functions against FLINT's arb ball arithmetic. The
    mode `arf_hyp` accepts lines of the forms `X.f_prec_round(p, rm) = R` and
    `BigExponentFloat::f_rational_prec_round(Q, p, rm) = R`, and their `_prec` forms, for f one of
    sinh, cosh, tanh, coth, sech, csch, and sinh_cosh; R is `(Z, O)`, or `(S, C, O_S, O_C)` for
    sinh_cosh. X and Z are BigExponentFloats in Malachite's exact hexadecimal format, and Q is a
    Rational written as `a/b` or `a`.

    The special cases follow Malachite's documentation: NaN gives NaN; sinh(±∞) = ±∞, cosh(±∞) =
    ∞, tanh(±∞) = coth(±∞) = ±1, and sech(±∞) = csch(±∞) = 0; and sinh(0) = tanh(0) = 0, cosh(0) =
    sech(0) = 1, and coth(0) = csch(0) = NaN (a pole, with no signed zero to choose a side).

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

#define MAX_ARGS 4
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

typedef enum { F_SINH, F_COSH, F_TANH, F_COTH, F_SECH, F_CSCH } fn_t;

static const char * FN_NAMES[] = {"sinh", "cosh", "tanh", "coth", "sech", "csch"};

/* The value at NaN, ±∞, and 0, per Malachite's documentation (coth and csch have a pole at 0,
   where the result is NaN); returns 1 and sets z, or 0. */
static int
special(arf_t z, fn_t f, const arf_t x)
{
    if (arf_is_nan(x))
    {
        arf_nan(z);
        return 1;
    }
    if (arf_is_inf(x))
    {
        int s = arf_sgn(x);
        switch (f)
        {
            case F_SINH: if (s > 0) arf_pos_inf(z); else arf_neg_inf(z); break;
            case F_COSH: arf_pos_inf(z); break;
            case F_TANH: case F_COTH: arf_set_si(z, s); break;
            default: arf_zero(z); break;
        }
        return 1;
    }
    if (arf_is_zero(x))
    {
        switch (f)
        {
            case F_SINH: case F_TANH: arf_zero(z); break;
            case F_COSH: case F_SECH: arf_one(z); break;
            default: arf_nan(z); break;
        }
        return 1;
    }
    return 0;
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

/* main + small for the value. For a tiny exact argument (|x| < 2^(-wp/2 - 10)), main is the
   leading term (x, 1, or 1/x) and small the second term of the series (x^3/6 for sinh, -x^3/3 for
   tanh, x^2/2 and -x^2/2 for cosh and sech, x/3 and -x/6 for coth and csch) with twice the next
   term as the radius (the ratios of consecutive terms are below 1 here). For tanh and coth at |x| >
   wp, main = ±1 and small = tanh x ∓ 1 = -2/(e^(2|x|) + 1) or coth x ∓ 1 = 2/(e^(2|x|) - 1), with
   the sign of x. Otherwise main is arb's value and small is 0. */
static void
eval(arb_t main, arb_t small, const value_t * v, slong wp)
{
    arb_t x, t, x2;
    mag_t rad;
    arb_init(x);
    arb_init(t);
    arb_init(x2);
    mag_init(rad);
    arb_zero(small);
    argument(x, v, wp + 64);
    int exact = v->xa != NULL;
    int sx = v->xa != NULL ? arf_sgn(v->xa) : fmpq_sgn(v->xq);
    if (exact && arf_cmpabs_2exp_si(v->xa, -(wp / 2) - 10) < 0)
    {
        arb_sqr(x2, x, wp + 64);
        switch (v->f)
        {
            case F_SINH: case F_TANH:
                arb_set(main, x);
                arb_mul(small, x2, x, wp + 64);
                arb_div_ui(small, small, v->f == F_SINH ? 6 : 3, wp + 64);
                if (v->f == F_TANH)
                    arb_neg(small, small);
                break;
            case F_COSH: case F_SECH:
                arb_one(main);
                arb_mul_2exp_si(small, x2, -1);
                if (v->f == F_SECH)
                    arb_neg(small, small);
                break;
            default:
                arb_inv(main, x, wp + 64);
                arb_div_ui(small, x, v->f == F_COTH ? 3 : 6, wp + 64);
                if (v->f == F_CSCH)
                    arb_neg(small, small);
                break;
        }
        arb_mul(t, small, x2, wp + 64);
        arb_get_mag(rad, t);
        mag_mul_2exp_si(rad, rad, 1);
        arb_add_error_mag(small, rad);
    }
    else if ((v->f == F_TANH || v->f == F_COTH) && arb_is_finite(x)
             && arf_cmpabs_ui(arb_midref(x), wp) > 0)
    {
        arb_set_si(main, sx);
        arb_abs(t, x);
        arb_mul_2exp_si(t, t, 1);
        arb_exp(t, t, wp + 64);
        if (v->f == F_TANH)
            arb_add_ui(t, t, 1, wp + 64);
        else
            arb_sub_ui(t, t, 1, wp + 64);
        arb_ui_div(small, 2, t, wp + 64);
        if ((v->f == F_TANH) == (sx > 0))
            arb_neg(small, small);
    }
    else
    {
        switch (v->f)
        {
            case F_SINH: arb_sinh(main, x, wp); break;
            case F_COSH: arb_cosh(main, x, wp); break;
            case F_TANH: arb_tanh(main, x, wp); break;
            case F_COTH: arb_coth(main, x, wp); break;
            case F_SECH: arb_sech(main, x, wp); break;
            case F_CSCH: arb_csch(main, x, wp); break;
        }
    }
    arb_clear(x);
    arb_clear(t);
    arb_clear(x2);
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
    if (!rational && special(z, f, x))
        return 0;
    if (rational && fmpq_is_zero(q))
    {
        arf_t zero;
        arf_init(zero);
        special(z, f, zero);
        arf_clear(zero);
        return 0;
    }
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
    int sincos = 0;
    fn_t f = F_SINH;
    const char * fam = NULL;
    if (strncmp(name, "sinh_cosh", 9) == 0)
    {
        sincos = 1;
        fam = name + 9;
    }
    else
        for (int i = 0; i < 6 && fam == NULL; i++)
            if (strncmp(name, FN_NAMES[i], 4) == 0)
            {
                f = (fn_t) i;
                fam = name + 4;
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
    arf_t x, z1, z2;
    fmpq_t q;
    arf_init(x);
    arf_init(z1);
    arf_init(z2);
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
        int o1 = expected(z1, sincos ? F_SINH : f, x, q, rational, prec, rnd);
        int o2 = sincos ? expected(z2, F_COSH, x, q, rational, prec, rnd) : 0;
        char * value = result;
        size_t rl = strlen(value);
        if (rl < 2 || value[0] != '(' || value[rl - 1] != ')')
            res = report(line_number, "unrecognized result");
        else
        {
            value[rl - 1] = '\0';
            value++;
            if (sincos)
            {
                char * c = split(value, ", ");
                char * os = c == NULL ? NULL : split(c, ", ");
                char * oc = os == NULL ? NULL : split(os, ", ");
                if (oc == NULL)
                    res = report(line_number, "unrecognized result");
                else
                {
                    res = compare(line_number, value, os, z1, o1, exact);
                    if (res == 0)
                        res = compare(line_number, c, oc, z2, o2, exact);
                }
            }
            else
            {
                char * ordering = split(value, ", ");
                res = compare(line_number, value, ordering, z1, o1, exact);
            }
        }
    }
    arf_clear(x);
    arf_clear(z1);
    arf_clear(z2);
    fmpq_clear(q);
    if (res == 0)
        checked++;
    return res;
}

int
run_arf_hyp(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_hyp", checked);
}
