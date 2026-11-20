/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs BigExponentFloat's gamma functions against FLINT's arb ball arithmetic. The mode
    `arf_gamma` accepts lines of the forms

    - `X.op_prec_round(p, rm) = (Z, O)` and `X.op_prec(p) = (Z, O)`, for op one of gamma and
      reciprocal_gamma;
    - `BigExponentFloat::op_prec_round(Q, p, rm) = (Z, O)` and
      `BigExponentFloat::op_prec(Q, p) = (Z, O)`, for op one of gamma_rational and
      reciprocal_gamma_rational;

    where X and Z are BigExponentFloats in Malachite's exact hexadecimal format and Q is a Rational
    written as `a/b` or `a`.

    Gamma(x) and 1/Gamma(x) are evaluated with arb_gamma and arb_rgamma (arb_gamma_fmpq and its
    reciprocal for a rational x); arb_t exponents are unbounded, like a BigExponentFloat's, so no
    input needs to be reduced. The result is rounded by bracketing f(x) between consecutive numbers
    with one more bit than the target precision (see `round_function`), with the working precision
    doubling while a comparison is undecided.

    The comparisons never decide if the exact result is representable with one more bit than the
    target precision. At a positive integer n, Gamma(n) = (n - 1)! and 1/Gamma(n) = 1/(n - 1)! are
    therefore rounded exactly, with fmpz and fmpq arithmetic. Nothing suggests that Gamma is ever
    dyadic at a non-integer, but this is not known in general; such a line would fail with
    "working precision limit reached".

    The special cases follow Malachite's documentation: Gamma is NaN at 0, at the negative integers,
    and at -infinity, and infinity at infinity; 1/Gamma is 0 at 0, at the negative integers, and at
    infinity, and NaN at -infinity. NaN maps to NaN.

    Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>

/* mpfr.h comes first, so that arf.h declares its MPFR conversions. */
#include <mpfr.h>

#include <flint/arb.h>
#include <flint/arb_hypgeom.h>
#include <flint/arf.h>
#include <flint/fmpq.h>
#include <flint/fmpz.h>

#include "oracle.h"

#define MAX_ARGS 4
#define MAX_PREC (1L << 26)
/* The largest positive integer at which (n - 1)! is computed exactly. */
#define MAX_FACTORIAL_ARG (1L << 20)

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

typedef struct
{
    const char * name;
    int reciprocal;
    int rational;
} op_t;

static const op_t OPS[] = {
    {"gamma", 0, 0},
    {"reciprocal_gamma", 1, 0},
    {"gamma_rational", 0, 1},
    {"reciprocal_gamma_rational", 1, 1},
};

/* Rounds the exact rational v, which is an integer or 1/k for an integer k > 0, to `prec` bits,
   returning the ordering of the result relative to v, or 2 if v is not dyadic (then
   `round_function` decides it, since it is not a number with prec + 1 bits either). */
static int
round_fmpq(arf_t z, const fmpq_t v, slong prec, arf_rnd_t rnd)
{
    if (fmpz_is_one(fmpq_denref(v)))
    {
        arf_set_round_fmpz(z, fmpq_numref(v), prec, rnd);
        fmpq_t c;
        fmpq_init(c);
        arf_get_fmpq(c, z);
        int o = fmpq_cmp(c, v);
        fmpq_clear(c);
        return o;
    }
    /* v = 1/k for an integer k > 1. If k is a power of 2, v is dyadic. */
    if (fmpz_is_one(fmpq_numref(v)))
    {
        slong k = fmpz_val2(fmpq_denref(v));
        fmpz_t t;
        fmpz_init(t);
        fmpz_one(t);
        fmpz_mul_2exp(t, t, k);
        int dyadic = fmpz_equal(t, fmpq_denref(v));
        fmpz_clear(t);
        if (dyadic)
        {
            arf_one(z);
            arf_mul_2exp_si(z, z, -k);
            return 0;
        }
    }
    /* Not dyadic: round a ball; the bracketing of `round_function` decides it. */
    return 2;
}

/* The exact value of f at the positive integer n, if it is computed exactly: Gamma(n) = (n - 1)!
   or 1/Gamma(n) = 1/(n - 1)!, as a rational. Returns 1 if v was set. */
static int
exact_at_positive_integer(fmpq_t v, const op_t * op, const fmpz_t n)
{
    if (fmpz_cmp_si(n, MAX_FACTORIAL_ARG) > 0)
        return 0;
    fmpz_fac_ui(fmpq_numref(v), fmpz_get_ui(n) - 1);
    fmpz_one(fmpq_denref(v));
    if (op->reciprocal)
        fmpq_inv(v, v);
    return 1;
}

/* Whether x is so close to 0 that f(x) is compared with the forms of `sign_near_zero_ball`. */
static int
near_zero(const arf_t xa, slong wp)
{
    return xa != NULL && !arf_is_zero(xa) && arf_cmpabs_2exp_si(xa, -wp - 64) < 0;
}

/* Adds the error bound c * |x|^k to y. */
static void
add_error_power(arb_t y, const arf_t x, int k, int c)
{
    mag_t e, m;
    mag_init(e);
    mag_init(m);
    arf_get_mag(m, x);
    mag_one(e);
    for (int i = 0; i < k; i++)
        mag_mul(e, e, m);
    mag_mul_ui(e, e, c);
    arb_add_error_mag(y, e);
    mag_clear(e);
    mag_clear(m);
}

/* For 0 < |x| <= 1/4, sets d to a ball containing f(x) - w, in forms that avoid cancellation
   between f(x) and a nearby w. Gamma(x) - 1/x is analytic in |x| < 1 with constant term -gamma
   (Euler's constant) and later Taylor coefficients below 1 in absolute value, so Gamma(x) - w =
   (1 - wx)/x - gamma + e with |e| <= |x| / (1 - |x|) <= 2|x|; 1 - wx is exact. Likewise 1/Gamma(x)
   = x + gamma x^2 + ... is entire with later coefficients below 1, so 1/Gamma(x) - w = (x - w) +
   gamma x^2 + e with |e| <= 2|x|^3; x - w is exact. When w is null, f(x) itself is enclosed.
   arb_gamma would need a working precision of about the size of x's exponent to decide these
   comparisons. */
static void
sign_near_zero_ball(arb_t d, const op_t * op, const arf_t x, const arf_t w, slong wp)
{
    arb_t t, g;
    arf_t u;
    arb_init(t);
    arb_init(g);
    arf_init(u);
    arb_const_euler(g, wp);
    if (!op->reciprocal)
    {
        /* (1 - wx) / x - gamma */
        if (w != NULL)
            arf_mul(u, w, x, ARF_PREC_EXACT, ARF_RND_DOWN);
        else
            arf_zero(u);
        arf_sub_ui(u, u, 1, ARF_PREC_EXACT, ARF_RND_DOWN);
        arf_neg(u, u);
        arb_set_arf(d, u);
        arb_set_arf(t, x);
        arb_div(d, d, t, wp);
        arb_sub(d, d, g, wp);
        add_error_power(d, x, 1, 2);
    }
    else
    {
        /* (x - w) + gamma x^2 */
        if (w != NULL)
            arf_sub(u, x, w, ARF_PREC_EXACT, ARF_RND_DOWN);
        else
            arf_set(u, x);
        arb_set_arf(d, u);
        arb_set_arf(t, x);
        arb_sqr(t, t, wp);
        arb_mul(t, t, g, wp);
        arb_add(d, d, t, wp);
        add_error_power(d, x, 3, 2);
    }
    arb_clear(t);
    arb_clear(g);
    arf_clear(u);
}

/* Sets y to a ball containing f(x), with working precision wp. */
static void
eval(arb_t y, const op_t * op, const arf_t xa, const fmpq_t xq, slong wp)
{
    if (near_zero(xa, wp))
        sign_near_zero_ball(y, op, xa, NULL, wp);
    else if (xa != NULL)
    {
        arb_t x;
        arb_init(x);
        arb_set_arf(x, xa);
        if (op->reciprocal)
            arb_rgamma(y, x, wp);
        else
            arb_gamma(y, x, wp);
        arb_clear(x);
    }
    else
    {
        arb_gamma_fmpq(y, xq, wp);
        if (op->reciprocal)
            arb_inv(y, y, wp);
    }
}

/* The sign of f(x) - w, given a ball y containing f(x), or 2 if it is not decided. Near 0, the
   difference is computed by `sign_near_zero_ball` instead. */
static int
sign_of_difference(const arb_t y, const op_t * op, const arf_t xa, const arf_t w, slong wp)
{
    arb_t d;
    arb_init(d);
    if (near_zero(xa, wp))
        sign_near_zero_ball(d, op, xa, w, wp);
    else
    {
        arb_set_arf(d, w);
        arb_sub(d, y, d, wp);
    }
    int s = arb_is_positive(d) ? 1 : arb_is_negative(d) ? -1 : 2;
    arb_clear(d);
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
   twice the size of x's integer part plus 64 bits, since an absolute error in log Gamma(x), which
   is about x log x, becomes a relative error in Gamma(x); it doubles while the comparisons are
   undecided. Returns the ordering, or 2 if the working
   precision grew too large. */
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
        fmpz_abs(t, t);
        e = fmpz_bits(t) + 1;
        fmpz_clear(t);
    }
    if (e < 0)
        e = 0;
    if (e > MAX_PREC)
        return 2;
    slong q = prec + 1;
    arb_t y;
    arf_t a, b, c;
    arb_init(y);
    arf_init(a);
    arf_init(b);
    arf_init(c);
    int result = 2;
    for (slong wp = prec + 2 * e + 64; result == 2 && wp <= MAX_PREC + 2 * e; wp *= 2)
    {
        eval(y, op, xa, xq, wp + 64);
        if (!arb_is_finite(y))
            continue;
        arf_set_round(a, arb_midref(y), q, ARF_RND_FLOOR);
        for (int step = 0; step < 8; step++)
        {
            if (arf_is_zero(a))
                break;
            int sa = sign_of_difference(y, op, xa, a, wp);
            if (sa == 2 || sa == 0)
                break;
            if (sa < 0)
            {
                neighbor(a, a, q, 0);
                continue;
            }
            neighbor(b, a, q, 1);
            int sb = sign_of_difference(y, op, xa, b, wp);
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
    arb_clear(y);
    arf_clear(a);
    arf_clear(b);
    arf_clear(c);
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
            if (op->reciprocal)
                arf_zero(z);
            else
                arf_pos_inf(z);
            return 0;
        }
        if (arf_is_neg_inf(xa))
        {
            arf_nan(z);
            return 0;
        }
    }
    fmpz_t n;
    fmpz_init(n);
    int integer;
    if (xa != NULL)
    {
        integer = arf_is_int(xa);
        if (integer)
            arf_get_fmpz(n, xa, ARF_RND_DOWN);
    }
    else
    {
        integer = fmpz_is_one(fmpq_denref(xq));
        if (integer)
            fmpz_set(n, fmpq_numref(xq));
    }
    int result = 2;
    if (integer && fmpz_sgn(n) <= 0)
    {
        /* a pole of Gamma */
        if (op->reciprocal)
            arf_zero(z);
        else
            arf_nan(z);
        result = 0;
    }
    else if (integer)
    {
        fmpq_t v;
        fmpq_init(v);
        if (exact_at_positive_integer(v, op, n))
            result = round_fmpq(z, v, prec, rnd);
        fmpq_clear(v);
    }
    fmpz_clear(n);
    if (result != 2)
        return result;
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
    if (op == NULL || (xtext != NULL) == op->rational)
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
    if (n_items != (op->rational ? 1 : 0) + 1 + with_rm)
        return report(line_number, "wrong number of arguments");
    if (op->rational)
        xtext = items[0];
    char * end = NULL;
    long long prec = strtoll(items[op->rational ? 1 : 0], &end, 10);
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
run_arf_gamma(const char * arg)
{
    checked = 0;
    int result = arf_for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_gamma", checked);
}
