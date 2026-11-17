/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs BigExponentFloat's exponential functions against FLINT's arb ball arithmetic. The mode
    `arf_exponential` accepts lines of the forms

    - `X.op_prec_round(p, rm) = (Z, O)` and `X.op_prec(p) = (Z, O)`, for op one of exp,
      exp_x_minus_1, power_of_2_x_minus_1, and power_of_10_x_minus_1;
    - `BigExponentFloat::op_prec_round(X, p, rm) = (Z, O)` and
      `BigExponentFloat::op_prec(X, p) = (Z, O)`, for op one of power_of_2_of_big_exponent_float
      and power_of_10_of_big_exponent_float;
    - `BigExponentFloat::op_prec_round(Q, p, rm) = (Z, O)` and
      `BigExponentFloat::op_prec(Q, p) = (Z, O)`, for op one of exp_rational,
      exp_x_minus_1_rational, power_of_2_rational, power_of_2_x_minus_1_rational,
      power_of_10_rational, and power_of_10_x_minus_1_rational;

    where X and Z are BigExponentFloats in Malachite's exact hexadecimal format and Q is a Rational
    written as `a/b` or `a`.

    The functions are e^x, e^x - 1, 2^x = e^(x log 2), 2^x - 1, 10^x = e^(x log 10), and 10^x - 1,
    evaluated with arb_exp and arb_expm1 (which keeps a small relative radius even when t is tiny);
    arb_t exponents are unbounded, like a BigExponentFloat's, so no input needs to be reduced. The
    result is rounded by bracketing f(x) between consecutive numbers with one more bit than the
    target precision (see `round_function`), comparing f(x) with each of them in a form that avoids
    cancellation (see `sign_of_difference`), with the working precision doubling while a comparison
    is undecided.

    The comparisons never decide if the exact result is representable with the target precision,
    or is a midpoint between two such values. This can only happen at integers x: e^x and e^x - 1 are
    irrational at every nonzero x, and so are 2^x and 10^x and their counterparts minus 1 at every
    non-integer x. At an integer n, 2^n and 2^n - 1 are rounded exactly by arf_t arithmetic, and so
    are 10^n and 10^n - 1 when 0 <= n <= 2^17 (10^n - 1 with n > 0 is an odd number of more than 3n
    bits, and 10^n with n < 0 is not dyadic).

    The special cases follow Malachite's documentation: there is no negative zero, the functions
    are 1 (or 0, for those minus 1) at 0, infinity at infinity, and 0 (or -1) at -infinity.

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

/* Splits `s` at the first occurrence of `sep`, returning the text after it, or NULL. */
static char *
split(char * s, const char * sep)
{
    char * p = strstr(s, sep);
    if (p == NULL)
        return NULL;
    *p = '\0';
    return p + strlen(sep);
}

/* The rounding mode named `s`. `Exact` maps to rounding down and sets `*exact`. Returns 0 on
   success. */
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

/* Parses a Rational written as `a/b` or `a`, in lowest terms with b > 0. Returns 0 on success. */
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

/* The base: 0 for e, otherwise 2 or 10. */
typedef struct
{
    const char * name;
    int base;
    int minus_one;
    int rational;
    int method;
} op_t;

static const op_t OPS[] = {
    {"exp", 0, 0, 0, 1},
    {"exp_x_minus_1", 0, 1, 0, 1},
    {"power_of_2_of_big_exponent_float", 2, 0, 0, 0},
    {"power_of_2_x_minus_1", 2, 1, 0, 1},
    {"power_of_10_of_big_exponent_float", 10, 0, 0, 0},
    {"power_of_10_x_minus_1", 10, 1, 0, 1},
    {"exp_rational", 0, 0, 1, 0},
    {"exp_x_minus_1_rational", 0, 1, 1, 0},
    {"power_of_2_rational", 2, 0, 1, 0},
    {"power_of_2_x_minus_1_rational", 2, 1, 1, 0},
    {"power_of_10_rational", 10, 0, 1, 0},
    {"power_of_10_x_minus_1_rational", 10, 1, 1, 0},
};

/* Rounds the exact value v to `prec` bits, returning the ordering of the result relative to v. */
static int
round_exact(arf_t z, const arf_t v, slong prec, arf_rnd_t rnd)
{
    arf_set_round(z, v, prec, rnd);
    return arf_cmp(z, v);
}

/* The function at the integer n, rounded to `prec` bits, if it is one of the values computed
   exactly (see the comment at the top); returns the ordering, or 2 if it is not. 2^n - 1 with
   |n| > prec + 1 is rounded by arf_sub_ui, which handles the gap between 2^n and 1 without
   materializing it; since 2^n - 1 then has more than prec bits, the result is not equal to it. For
   n > 0 it is greater than 2^n - 1 exactly when it is at least 2^n, and for n < 0 exactly when it
   is greater than -1. */
static int
exact_at_integer(arf_t z, const op_t * op, const fmpz_t n, slong prec, arf_rnd_t rnd)
{
    arf_t v;
    arf_init(v);
    int result = 2;
    if (op->base == 2)
    {
        arf_one(v);
        arf_mul_2exp_fmpz(v, v, n);
        if (!op->minus_one)
            result = round_exact(z, v, prec, rnd);
        else
        {
            fmpz_t a;
            fmpz_init(a);
            fmpz_abs(a, n);
            if (fmpz_cmp_ui(a, prec + 1) > 0)
            {
                arf_sub_ui(z, v, 1, prec, rnd);
                if (fmpz_sgn(n) > 0)
                    result = arf_cmp(z, v) >= 0 ? 1 : -1;
                else
                    result = arf_cmp_si(z, -1) > 0 ? 1 : -1;
            }
            else
            {
                arf_sub_ui(v, v, 1, ARF_PREC_EXACT, ARF_RND_DOWN);
                result = round_exact(z, v, prec, rnd);
            }
            fmpz_clear(a);
        }
    }
    else if (op->base == 10 && fmpz_sgn(n) >= 0 && fmpz_cmp_ui(n, 1UL << 17) <= 0)
    {
        fmpz_t p;
        fmpz_init(p);
        fmpz_ui_pow_ui(p, 10, fmpz_get_ui(n));
        if (op->minus_one)
            fmpz_sub_ui(p, p, 1);
        arf_set_fmpz(v, p);
        fmpz_clear(p);
        result = round_exact(z, v, prec, rnd);
    }
    arf_clear(v);
    return result;
}

/* Whether the ball t is nonzero with |t| < 2^-8, for which e^t - 1 - t is computed by
   `tail_ball`. */
static int
small(const arb_t t)
{
    return arb_is_finite(t) && arb_contains_zero(t) == 0
        && arf_cmpabs_2exp_si(arb_midref(t), -9) < 0;
}

/* For a ball t with |t| < 2^-8, sets r to a ball containing e^t - 1 - t = t^2/2! + t^3/3! + ...,
   summed until a term is below 2^-wp times the first. Each later term is at most |t| / (k + 1) <=
   1/4 times the one before, so the omitted terms add up to less than the last term computed, which
   is added as an error bound. */
static void
tail_ball(arb_t r, const arb_t t, slong wp)
{
    arb_t term;
    arb_init(term);
    arb_sqr(term, t, wp);
    arb_mul_2exp_si(term, term, -1);
    arb_set(r, term);
    mag_t first, m;
    mag_init(first);
    mag_init(m);
    arb_get_mag(first, term);
    mag_mul_2exp_si(first, first, -wp);
    for (ulong k = 3;; k++)
    {
        arb_mul(term, term, t, wp);
        arb_div_ui(term, term, k, wp);
        arb_add(r, r, term, wp);
        arb_get_mag(m, term);
        if (mag_cmp(m, first) < 0)
            break;
    }
    arb_add_error_mag(r, m);
    mag_clear(first);
    mag_clear(m);
    arb_clear(term);
}

/* Sets t to x, x log 2, or x log 10, computed with working precision wp. */
static void
eval_t(arb_t t, const op_t * op, const arb_t x, slong wp)
{
    if (op->base == 0)
        arb_set(t, x);
    else
    {
        if (op->base == 2)
            arb_const_log2(t, wp);
        else
            arb_const_log10(t, wp);
        arb_mul(t, t, x, wp);
    }
}

/* Sets ex and e1 to balls containing e^t and e^t - 1, for t from `eval_t`, computed with working
   precision wp, and, when t is small, tl to e^t - 1 - t from `tail_ball` (returning 1). For small
   t, e^t - 1 is computed as t plus the tail rather than with arb_expm1, whose enclosure loses
   accuracy at extreme exponents. */
static int
eval_balls(arb_t ex, arb_t e1, arb_t tl, const arb_t t, slong wp)
{
    if (small(t))
    {
        tail_ball(tl, t, wp);
        arb_add(e1, t, tl, wp);
        arb_add_ui(ex, e1, 1, wp);
        return 1;
    }
    arb_exp(ex, t, wp);
    arb_expm1(e1, t, wp);
    return 0;
}

/* Whether 1/2 <= |w| <= 2. */
static int
near_one(const arf_t w)
{
    return arf_cmpabs_2exp_si(w, -1) >= 0 && arf_cmpabs_2exp_si(w, 1) <= 0;
}

/* The sign of f(x) - w, given the balls ex and e1 from `eval_balls`, or 2 if they do not decide
   it. To avoid cancellation, a w near f(x) = e^t near 1 is compared as (e^t - 1) - (w - 1), and a
   w near f(x) = e^t - 1 near -1 as e^t - (w + 1); otherwise f(x) is compared with w directly. The
   adjusted w is exact when it fits in wp + 64 bits, and otherwise a ball. For a small t, e^t - 1 -
   w' (where w' is w or w - 1) is computed as (t - w') + (e^t - 1 - t), since t - w' may cancel to
   far below the radius of e^t - 1: the w nearest e^x - 1 can be x itself. */
static int
sign_of_difference(const arb_t ex, const arb_t e1, const arb_t t, const arb_t tl, int has_tail,
                   const op_t * op, const arf_t w, slong wp)
{
    arb_t d, v;
    arb_init(d);
    arb_init(v);
    arb_set_arf(v, w);
    const arb_struct * f;
    if (!op->minus_one && near_one(w))
    {
        arb_sub_ui(v, v, 1, wp + 64);
        f = e1;
    }
    else if (op->minus_one && arf_sgn(w) < 0 && near_one(w))
    {
        arb_add_ui(v, v, 1, wp + 64);
        f = ex;
    }
    else
        f = op->minus_one ? e1 : ex;
    if (f == e1 && has_tail)
    {
        arb_sub(d, t, v, wp);
        arb_add(d, d, tl, wp);
    }
    else
        arb_sub(d, f, v, wp);
    int s = arb_is_positive(d) ? 1 : arb_is_negative(d) ? -1 : 2;
    arb_clear(d);
    arb_clear(v);
    return s;
}

/* The neighbor of the nonzero a, which has at most q bits, among the numbers with q bits: above it
   if `up`, and below it otherwise. */
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

/* Rounds f(x) to `prec` bits, where x is the arf_t `xa` or, if `xa` is NULL, the fmpq_t `xq`, and
   f(x) is not a number with prec + 1 bits. Consecutive numbers a < b with prec + 1 bits are found
   such that a < f(x) < b, each comparison decided by `sign_of_difference`; every number strictly
   between a and b rounds like their midpoint c, and the rounded value is outside (a, b), so its
   ordering is its comparison with c. The working precision starts at the target precision plus
   the size of x's integer part plus 64 bits and doubles while the comparisons are undecided.
   Returns the ordering, or 2 if the working precision grew too large. */
static int
round_function(arf_t z, const op_t * op, const arf_t xa, const fmpq_t xq, slong prec,
               arf_rnd_t rnd)
{
    /* |x| < 2^e */
    slong e;
    if (xa != NULL)
        e = arf_abs_bound_lt_2exp_si(xa);
    else
    {
        fmpz_t t;
        fmpz_init(t);
        fmpz_cdiv_q(t, fmpq_numref(xq), fmpq_denref(xq));
        e = fmpz_bits(t) + 1;
        fmpz_clear(t);
    }
    if (e < 0)
        e = 0;
    if (e > MAX_PREC)
        return 2;
    slong q = prec + 1;
    arb_t x, t, ex, e1, tl;
    arf_t a, b, c, y;
    arb_init(x);
    arb_init(t);
    arb_init(ex);
    arb_init(e1);
    arb_init(tl);
    arf_init(a);
    arf_init(b);
    arf_init(c);
    arf_init(y);
    int result = 2;
    for (slong wp = prec + e + 64; result == 2 && wp <= MAX_PREC + e; wp *= 2)
    {
        if (xa != NULL)
            arb_set_arf(x, xa);
        else
            arb_set_fmpq(x, xq, wp + e + 64);
        eval_t(t, op, x, wp + 64);
        int has_tail = eval_balls(ex, e1, tl, t, wp + 64);
        arf_set(y, arb_midref(op->minus_one ? e1 : ex));
        if (arf_is_zero(y))
            continue;
        arf_set_round(a, y, q, ARF_RND_FLOOR);
        for (int step = 0; step < 8; step++)
        {
            if (arf_is_zero(a))
                break;
            int sa = sign_of_difference(ex, e1, t, tl, has_tail, op, a, wp);
            if (sa == 2 || sa == 0)
                break;
            if (sa < 0)
            {
                neighbor(a, a, q, 0);
                continue;
            }
            neighbor(b, a, q, 1);
            int sb = sign_of_difference(ex, e1, t, tl, has_tail, op, b, wp);
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
    arb_clear(x);
    arb_clear(t);
    arb_clear(ex);
    arb_clear(e1);
    arb_clear(tl);
    arf_clear(a);
    arf_clear(b);
    arf_clear(c);
    arf_clear(y);
    return result;
}

/* Computes the expected result of `op` at x, which is the arf_t `xa` or, if `xa` is NULL, the
   fmpq_t `xq`. Returns the ordering, or 2 if it could not be determined. */
static int
expected(arf_t z, const op_t * op, const arf_t xa, const fmpq_t xq, slong prec, arf_rnd_t rnd)
{
    if (xa != NULL)
    {
        if (arf_is_nan(xa))
        {
            arf_nan(z);
            return 0;
        }
        if (arf_is_pos_inf(xa))
        {
            arf_pos_inf(z);
            return 0;
        }
        if (arf_is_neg_inf(xa))
        {
            if (op->minus_one)
                arf_set_si(z, -1);
            else
                arf_zero(z);
            return 0;
        }
    }
    int zero = xa != NULL ? arf_is_zero(xa) : fmpq_is_zero(xq);
    if (zero)
    {
        if (op->minus_one)
            arf_zero(z);
        else
            arf_one(z);
        return 0;
    }
    int integer = xa != NULL ? arf_is_int(xa) : fmpz_is_one(fmpq_denref(xq));
    if (integer && op->base != 0)
    {
        fmpz_t n;
        fmpz_init(n);
        if (xa != NULL)
            arf_get_fmpz(n, xa, ARF_RND_DOWN);
        else
            fmpz_set(n, fmpq_numref(xq));
        int o = exact_at_integer(z, op, n, prec, rnd);
        fmpz_clear(n);
        if (o != 2)
            return o;
    }
    return round_function(z, op, xa, xq, prec, rnd);
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
    /* `X.name(args)` or `BigExponentFloat::name(args)` */
    char * name;
    char * xtext = NULL;
    const char * prefix = "BigExponentFloat::";
    if (strncmp(line, prefix, strlen(prefix)) == 0)
        name = line + strlen(prefix);
    else
    {
        /* The `.` that begins the method call: one followed by an identifier and a `(`. The `.`
           inside a hexadecimal value is followed by hex digits that do not reach a `(`. */
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
    const op_t * op = NULL;
    for (size_t i = 0; i < sizeof(OPS) / sizeof(OPS[0]); i++)
        if (strcmp(OPS[i].name, name) == 0)
            op = &OPS[i];
    if (op == NULL || (xtext != NULL) != op->method)
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
    if (n_items != (op->method ? 0 : 1) + 1 + with_rm)
        return report(line_number, "wrong number of arguments");
    if (!op->method)
        xtext = items[0];
    char * end = NULL;
    long long prec = strtoll(items[op->method ? 0 : 1], &end, 10);
    arf_rnd_t rnd = ARF_RND_NEAR;
    int exact = 0;
    arf_t x, z, got;
    fmpq_t q;
    arf_init(x);
    arf_init(z);
    arf_init(got);
    fmpq_init(q);
    int res = 0;
    if (op->rational ? parse_rational(q, xtext) != 0 : arf_parse_base(xtext, 4, x) != 0)
        res = report(line_number, "unreadable argument");
    else if (*end != '\0' || prec <= 0)
        res = report(line_number, "unreadable precision");
    else if (with_rm && parse_rm(items[n_items - 1], &rnd, &exact) != 0)
        res = report(line_number, "unreadable rounding mode");
    if (res == 0)
    {
        int o = expected(z, op, op->rational ? NULL : x, q, prec, rnd);
        /* `(Z, O)` */
        char * value = result;
        char * ordering = NULL;
        size_t rl = strlen(value);
        if (rl < 2 || value[0] != '(' || value[rl - 1] != ')')
            res = report(line_number, "unrecognized result");
        else
        {
            value[rl - 1] = '\0';
            value++;
            ordering = split(value, ", ");
        }
        if (res != 0)
            ;
        else if (o == 2)
            res = report(line_number, "working precision limit reached");
        else if (exact && o != 0)
            res = report(line_number, "Exact rounding of an inexact result");
        else if (ordering == NULL || arf_parse_base(value, 4, got) != 0)
            res = report(line_number, "unreadable result");
        else if (!arf_equal(got, z))
            res = report(line_number, "wrong value");
        else if (strcmp(ordering, ordering_name(o)) != 0)
            res = report(line_number, "wrong ordering");
    }
    arf_clear(x);
    arf_clear(z);
    arf_clear(got);
    fmpq_clear(q);
    if (res == 0)
        checked++;
    return res;
}

int
run_arf_exponential(const char * arg)
{
    checked = 0;
    int result = arf_for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_exponential", checked);
}
