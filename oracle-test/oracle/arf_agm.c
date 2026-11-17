/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs BigExponentFloat's arithmetic-geometric mean (AGM) against FLINT's arb ball arithmetic.
    The mode `arf_agm` accepts lines of the forms

    - `X.agm_prec_round(Y, p, rm) = (Z, O)` and `X.agm_prec(Y, p) = (Z, O)`;
    - `BigExponentFloat::agm_rational_prec_round(Q, R, p, rm) = (Z, O)` and
      `BigExponentFloat::agm_rational_prec(Q, R, p) = (Z, O)`;
    - `X.agm_big_exponent_float_rational_prec_round(Q, p, rm) = (Z, O)` and
      `X.agm_big_exponent_float_rational_prec(Q, p) = (Z, O)`;

    where X, Y, and Z are BigExponentFloats in Malachite's exact hexadecimal format and Q and R are
    Rationals written as `a/b` or `a`.

    FLINT has no AGM of arf_t values (arb_agm returns an indeterminate ball for any infinite
    argument), so the special cases follow Malachite's documentation, which takes them from MPFR:
    NaN first; then an infinity gives infinity if the other argument is positive, and NaN
    otherwise; then a zero gives 0; then a negative argument gives NaN. There is no negative zero.
    For equal arguments the AGM is the argument itself, rounded exactly.

    Otherwise the AGM of distinct positive numbers is evaluated with arb_agm (arb_t exponents are
    unbounded, like a BigExponentFloat's, so no input needs to be reduced). The result is rounded by
    bracketing it between consecutive numbers with one more bit than the target precision, each
    comparison being the sign of a ball difference, with the working precision doubling while a
    comparison is undecided. This never decides if the AGM is a number with prec + 1 bits; like
    Malachite and MPFR, the oracle assumes that the AGM of distinct positive rationals is
    irrational.

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

/* An argument: an arf_t, or a Rational if `rational`. */
typedef struct
{
    int rational;
    arf_t a;
    fmpq_t q;
} value_t;

static void
value_init(value_t * v)
{
    v->rational = 0;
    arf_init(v->a);
    fmpq_init(v->q);
}

static void
value_clear(value_t * v)
{
    arf_clear(v->a);
    fmpq_clear(v->q);
}

static int
value_is_nan(const value_t * v)
{
    return !v->rational && arf_is_nan(v->a);
}

static int
value_is_inf(const value_t * v)
{
    return !v->rational && arf_is_inf(v->a);
}

static int
value_sgn(const value_t * v)
{
    return v->rational ? fmpq_sgn(v->q) : arf_sgn(v->a);
}

/* Whether the finite values u and v are equal. An arf_t is compared with a Rational exactly, after
   ruling out exponents too far apart for equality (so that a huge exponent is never expanded). */
static int
value_equal(const value_t * u, const value_t * v)
{
    if (!u->rational && !v->rational)
        return arf_equal(u->a, v->a);
    if (u->rational && v->rational)
        return fmpq_equal(u->q, v->q);
    const value_t * x = u->rational ? v : u;
    const value_t * q = u->rational ? u : v;
    if (arf_is_zero(x->a) || fmpq_is_zero(q->q))
        return arf_is_zero(x->a) && fmpq_is_zero(q->q);
    /* 2^(bits(n) - bits(d) - 1) < |n/d| < 2^(bits(n) - bits(d) + 1) */
    slong e = fmpz_bits(fmpq_numref(q->q)) - fmpz_bits(fmpq_denref(q->q));
    fmpz_t d;
    fmpz_init(d);
    fmpz_sub_si(d, ARF_EXPREF(x->a), e);
    fmpz_abs(d, d);
    int far = fmpz_cmp_ui(d, 4) > 0;
    fmpz_clear(d);
    if (far)
        return 0;
    fmpq_t t;
    fmpq_init(t);
    arf_get_fmpq(t, x->a);
    int eq = fmpq_equal(t, q->q);
    fmpq_clear(t);
    return eq;
}

/* The ball enclosing v, with a relative radius of about 2^(-wp) for a Rational. */
static void
value_ball(arb_t r, const value_t * v, slong wp)
{
    if (v->rational)
        arb_set_fmpq(r, v->q, wp);
    else
        arb_set_arf(r, v->a);
}

/* Rounds the finite value v to `prec` bits, returning the ordering of the result relative to v. */
static int
round_value(arf_t z, const value_t * v, slong prec, arf_rnd_t rnd)
{
    if (!v->rational)
    {
        arf_set_round(z, v->a, prec, rnd);
        return arf_cmp(z, v->a);
    }
    arf_set_fmpq(z, v->q, prec, rnd);
    fmpq_t t;
    fmpq_init(t);
    arf_get_fmpq(t, z);
    int c = fmpq_cmp(t, v->q);
    fmpq_clear(t);
    return c < 0 ? -1 : c > 0;
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

/* The sign of m - w if the ball m excludes w, and 2 otherwise. */
static int
sign_of_difference(const arb_t m, const arf_t w, slong wp)
{
    arb_t t;
    arb_init(t);
    arb_sub_arf(t, m, w, wp);
    int s = arb_is_positive(t) ? 1 : arb_is_negative(t) ? -1 : 2;
    arb_clear(t);
    return s;
}

/* Rounds AGM(x, y) to `prec` bits for distinct positive finite x and y. Consecutive numbers a < b
   with prec + 1 bits are found such that a < AGM(x, y) < b; every number strictly between a and b
   rounds like their midpoint c, and the rounded value is outside (a, b), so its ordering is its
   comparison with c. Returns the ordering, or 2 if the working precision grew too large. */
static int
round_agm(arf_t z, const value_t * x, const value_t * y, slong prec, arf_rnd_t rnd)
{
    slong q = prec + 1;
    arb_t xb, yb, m;
    arf_t a, b, c;
    arb_init(xb);
    arb_init(yb);
    arb_init(m);
    arf_init(a);
    arf_init(b);
    arf_init(c);
    int result = 2;
    for (slong wp = prec + 64; result == 2 && wp <= MAX_PREC; wp *= 2)
    {
        value_ball(xb, x, wp + 64);
        value_ball(yb, y, wp + 64);
        arb_agm(m, xb, yb, wp + 64);
        if (!arb_is_finite(m) || !arb_is_positive(m))
            continue;
        arf_set_round(a, arb_midref(m), q, ARF_RND_FLOOR);
        for (int step = 0; step < 8; step++)
        {
            int sa = sign_of_difference(m, a, wp + 64);
            if (sa == 2 || sa == 0)
                break;
            if (sa < 0)
            {
                neighbor(a, a, q, 0);
                continue;
            }
            neighbor(b, a, q, 1);
            int sb = sign_of_difference(m, b, wp + 64);
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
    arb_clear(xb);
    arb_clear(yb);
    arb_clear(m);
    arf_clear(a);
    arf_clear(b);
    arf_clear(c);
    return result;
}

/* Whether v is positive infinity or a positive finite value. */
static int
value_positive(const value_t * v)
{
    return value_sgn(v) > 0;
}

/* The expected AGM, with the special cases described at the top. Returns the ordering, or 2 if it
   could not be determined. */
static int
expected(arf_t z, const value_t * x, const value_t * y, slong prec, arf_rnd_t rnd)
{
    if (value_is_nan(x) || value_is_nan(y))
    {
        arf_nan(z);
        return 0;
    }
    if ((value_is_inf(x) && value_sgn(x) > 0 && value_positive(y))
        || (value_is_inf(y) && value_sgn(y) > 0 && value_positive(x)))
    {
        arf_pos_inf(z);
        return 0;
    }
    if (value_is_inf(x) || value_is_inf(y))
    {
        arf_nan(z);
        return 0;
    }
    if (value_sgn(x) == 0 || value_sgn(y) == 0)
    {
        arf_zero(z);
        return 0;
    }
    if (value_sgn(x) < 0 || value_sgn(y) < 0)
    {
        arf_nan(z);
        return 0;
    }
    if (value_equal(x, y))
        return round_value(z, x, prec, rnd);
    return round_agm(z, x, y, prec, rnd);
}

typedef enum
{
    OP_AGM,
    OP_RATIONAL,
    OP_MIXED
} op_t;

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
    op_t op;
    if (strcmp(name, "agm") == 0 && xtext != NULL)
        op = OP_AGM;
    else if (strcmp(name, "agm_rational") == 0 && xtext == NULL)
        op = OP_RATIONAL;
    else if (strcmp(name, "agm_big_exponent_float_rational") == 0 && xtext != NULL)
        op = OP_MIXED;
    else
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
    int n_values = op == OP_RATIONAL ? 2 : 1;
    if (n_items != n_values + 1 + with_rm)
        return report(line_number, "wrong number of arguments");
    const char * xs = op == OP_RATIONAL ? items[0] : xtext;
    const char * ys = op == OP_RATIONAL ? items[1] : items[0];
    char * end = NULL;
    long long prec = strtoll(items[n_values], &end, 10);
    arf_rnd_t rnd = ARF_RND_NEAR;
    int exact = 0;
    value_t x, y;
    value_init(&x);
    value_init(&y);
    x.rational = op == OP_RATIONAL;
    y.rational = op != OP_AGM;
    arf_t z, got;
    arf_init(z);
    arf_init(got);
    int res = 0;
    if ((x.rational ? parse_rational(x.q, xs) : arf_parse_base(xs, 4, x.a)) != 0
        || (y.rational ? parse_rational(y.q, ys) : arf_parse_base(ys, 4, y.a)) != 0)
        res = report(line_number, "unreadable argument");
    else if (*end != '\0' || prec <= 0)
        res = report(line_number, "unreadable precision");
    else if (with_rm && parse_rm(items[n_items - 1], &rnd, &exact) != 0)
        res = report(line_number, "unreadable rounding mode");
    if (res == 0)
    {
        int o = expected(z, &x, &y, prec, rnd);
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
    value_clear(&x);
    value_clear(&y);
    arf_clear(z);
    arf_clear(got);
    if (res == 0)
        checked++;
    return res;
}

int
run_arf_agm(const char * arg)
{
    checked = 0;
    int result = arf_for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_agm", checked);
}
