/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the comparisons of Malachite's BigExponentFloat against FLINT's arf_t. A BigExponentFloat
    or a Float is written in Malachite's exact hexadecimal format, an integer in decimal, a rational
    as `a/b`, a double in its shortest round-tripping decimal form, and a Gaussian integer or
    rational as Malachite prints it (`3+4i`, `-3/2+i/2`). The modes:

    - `arf_cmp`: the IEEE comparisons `A < B`, `A = B`, `A > B`, `A ≠ B`, and
      `A and B are incomparable`, with each side optionally inside `|...|` for an absolute-value
      comparison, between a BigExponentFloat and any of the types above (in either order),
      against arf_cmp and arf_cmpabs, with an exact comparison of an arf_t and an fmpq_t (through
      arf_get_fmpq when the exponent is small and the exponents alone otherwise) and of the square
      of an arf_t and the squared modulus of a Gaussian number. NaN is incomparable to and unequal
      to everything.
    - `arf_cmp_total`: the same lines for two BigExponentFloats in the total order of
      ComparableBigExponentFloat: NaN equals NaN and lies between the negative values and zero,
      or, for absolute values, below zero.
    - `arf_min_max`: `A.min(B) = Z`, `A.min_prec_round(B, p, rm) = (Z, o)`, `A.min_prec(B, p) =
      (Z, o)`, `A.min_rational_prec_round(Q, p, rm) = (Z, o)`, `A.min_rational_prec(Q, p) =
      (Z, o)`, and the same for `max`, against a selection made here (a NaN loses; the exact
      comparison picks the winner, which a tie leaves with the BigExponentFloat) followed by
      arf_set_round or arf_fmpz_div_fmpz.

    Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/arf.h>
#include <flint/fmpq.h>
#include <flint/fmpz.h>

#include "oracle.h"

typedef enum
{
    KIND_CMP,
    KIND_CMP_TOTAL,
    KIND_MIN_MAX
} comparison_kind;

static long checked;
static comparison_kind kind;
static const char * mode_name;

static int
fail(int line_number, const char * what)
{
    flint_printf("error in %s test, line %d: %s\n", mode_name, line_number, what);
    return 1;
}

/* Splits `line` at the first occurrence of `sep`, in place, returning the text after it, or
   NULL. */
static char *
split(char * line, const char * sep)
{
    char * p = strstr(line, sep);
    if (p == NULL)
        return NULL;
    *p = '\0';
    return p + strlen(sep);
}

/* Strips `open` from the start of `s` and `close` from its end, in place, returning the inside or
   NULL. */
static char *
strip(char * s, const char * open, char close)
{
    size_t n = strlen(s);
    size_t k = strlen(open);
    if (strncmp(s, open, k) != 0 || n <= k || s[n - 1] != close)
        return NULL;
    s[n - 1] = '\0';
    return s + k;
}

/* The comparison result meaning "incomparable": one side is NaN. */
#define INCOMPARABLE 2

/* -------------------------------------------------------------------------------------------- */
/* Values                                                                                       */

typedef enum
{
    VALUE_ARF,
    VALUE_FMPQ,
    VALUE_GAUSSIAN
} value_kind;

typedef struct
{
    value_kind kind;
    arf_t x;
    fmpq_t re;
    fmpq_t im;
} value_struct;

typedef value_struct value_t[1];

static void
value_init(value_t v)
{
    arf_init(v->x);
    fmpq_init(v->re);
    fmpq_init(v->im);
}

static void
value_clear(value_t v)
{
    arf_clear(v->x);
    fmpq_clear(v->re);
    fmpq_clear(v->im);
}

/* Parses a decimal integer or a rational `a/b`, which must be the whole of `s`, into `q`. */
static int
parse_fmpq(const char * s, fmpq_t q)
{
    if (s[0] == '\0' || s[0] == '+')
        return 1;
    const char * slash = strchr(s, '/');
    if (slash == NULL)
    {
        fmpz_one(fmpq_denref(q));
        return fmpz_set_str(fmpq_numref(q), s, 10);
    }
    size_t n = slash - s;
    char * num = flint_malloc(n + 1);
    memcpy(num, s, n);
    num[n] = '\0';
    int result = n == 0 || fmpz_set_str(fmpq_numref(q), num, 10)
        || fmpz_set_str(fmpq_denref(q), slash + 1, 10) || fmpz_sgn(fmpq_denref(q)) <= 0;
    flint_free(num);
    if (result == 0)
        fmpq_canonicalise(q);
    return result;
}

/* Parses one part of a Gaussian number, the text of `s` from `start` to `end`, which is a rational
   (the real part) or, if `imaginary`, a rational with an `i` after its numerator (`i`, `-i`,
   `3i`, `i/2`, `-5i/6`). */
static int
parse_gaussian_part(const char * start, const char * end, int imaginary, fmpq_t q)
{
    size_t n = end - start;
    char * buffer = flint_malloc(n + 2);
    memcpy(buffer, start, n);
    buffer[n] = '\0';
    int result = 0;
    if (imaginary)
    {
        char * i = strchr(buffer, 'i');
        if (i == NULL)
            result = 1;
        else
        {
            /* Remove the `i`, writing a numerator of 1 if it was left empty. */
            int empty = i == buffer || (i == buffer + 1 && buffer[0] == '-');
            if (empty)
            {
                memmove(i + 1, i + 1, strlen(i + 1) + 1);
                *i = '1';
            }
            else
                memmove(i, i + 1, strlen(i + 1) + 1);
        }
    }
    if (result == 0)
        result = parse_fmpq(buffer, q);
    flint_free(buffer);
    return result;
}

/* Parses a Gaussian integer or rational as Malachite prints it. */
static int
parse_gaussian(const char * s, fmpq_t re, fmpq_t im)
{
    fmpq_zero(re);
    fmpq_zero(im);
    if (strchr(s, 'i') == NULL)
        return parse_fmpq(s, re);
    /* The imaginary part starts at the last sign that is not at the start. */
    const char * split_at = NULL;
    for (const char * p = s + 1; *p != '\0'; p++)
        if (*p == '+' || *p == '-')
            split_at = p;
    if (split_at == NULL)
        return parse_gaussian_part(s, s + strlen(s), 1, im);
    const char * im_start = *split_at == '+' ? split_at + 1 : split_at;
    return parse_gaussian_part(s, split_at, 0, re)
        || parse_gaussian_part(im_start, s + strlen(s), 1, im);
}

/* Parses any operand: a hexadecimal BigExponentFloat or Float, a special value, a double, a
   Gaussian number, or a rational. */
static int
parse_value(const char * s, value_t v)
{
    if (strcmp(s, "NaN") == 0 || strcmp(s, "Infinity") == 0 || strcmp(s, "-Infinity") == 0
        || strncmp(s, "0x", 2) == 0 || strncmp(s, "-0x", 3) == 0)
    {
        v->kind = VALUE_ARF;
        return arf_parse_base(s, 4, v->x);
    }
    if (strchr(s, 'i') != NULL)
    {
        v->kind = VALUE_GAUSSIAN;
        return parse_gaussian(s, v->re, v->im);
    }
    if (strchr(s, '.') != NULL || strchr(s, 'e') != NULL)
    {
        /* A double in its shortest round-tripping form; strtod rounds correctly, so this recovers
           it exactly. */
        char * end;
        double d = strtod(s, &end);
        if (*end != '\0')
            return 1;
        v->kind = VALUE_ARF;
        arf_set_d(v->x, d);
        return 0;
    }
    v->kind = VALUE_FMPQ;
    return parse_fmpq(s, v->re);
}

/* -------------------------------------------------------------------------------------------- */
/* Exact comparisons                                                                            */

/* Whether the exponent of a finite nonzero arf_t is small enough to make it an fmpq_t. */
static int
exponent_is_small(const arf_t x)
{
    return fmpz_bits(ARF_EXPREF(x)) <= 24;
}

/* Compares a finite arf_t with an fmpq_t exactly. A huge exponent decides the comparison alone:
   the rationals in the tests have far fewer bits than such an exponent. */
static int
cmp_arf_fmpq(const arf_t x, const fmpq_t q)
{
    if (arf_is_zero(x))
        return -fmpq_sgn(q);
    if (exponent_is_small(x))
    {
        fmpq_t t;
        fmpq_init(t);
        arf_get_fmpq(t, x);
        int c = fmpq_cmp(t, q);
        fmpq_clear(t);
        return c < 0 ? -1 : c > 0 ? 1 : 0;
    }
    if (fmpz_sgn(ARF_EXPREF(x)) > 0 || fmpq_is_zero(q))
        return arf_sgn(x);
    return -fmpq_sgn(q);
}

/* Compares the square of a finite arf_t with a nonnegative fmpq_t exactly. */
static int
cmp_arf_square_fmpq(const arf_t x, const fmpq_t q)
{
    if (arf_is_zero(x))
        return fmpq_is_zero(q) ? 0 : -1;
    if (exponent_is_small(x))
    {
        fmpq_t t;
        fmpq_init(t);
        arf_get_fmpq(t, x);
        fmpq_mul(t, t, t);
        int c = fmpq_cmp(t, q);
        fmpq_clear(t);
        return c < 0 ? -1 : c > 0 ? 1 : 0;
    }
    if (fmpz_sgn(ARF_EXPREF(x)) > 0 || fmpq_is_zero(q))
        return 1;
    return -1;
}

static int
sign_of(int c)
{
    return c < 0 ? -1 : c > 0 ? 1 : 0;
}

/* Compares `a` with `b`, or their absolute values, returning -1, 0, 1, or INCOMPARABLE. At least
   one side is an arf_t. */
static int
compare_values(const value_t a, const value_t b, int absolute)
{
    if (a->kind != VALUE_ARF)
    {
        int c = compare_values(b, a, absolute);
        return c == INCOMPARABLE ? c : -c;
    }
    if (arf_is_nan(a->x))
        return INCOMPARABLE;
    if (b->kind == VALUE_ARF)
    {
        if (arf_is_nan(b->x))
            return INCOMPARABLE;
        return sign_of(absolute ? arf_cmpabs(a->x, b->x) : arf_cmp(a->x, b->x));
    }
    if (b->kind == VALUE_FMPQ || fmpq_is_zero(b->im))
    {
        if (!absolute)
        {
            if (arf_is_inf(a->x))
                return arf_sgn(a->x);
            return cmp_arf_fmpq(a->x, b->re);
        }
        if (arf_is_inf(a->x))
            return 1;
        arf_t t;
        fmpq_t q;
        arf_init(t);
        fmpq_init(q);
        arf_abs(t, a->x);
        fmpq_abs(q, b->re);
        int c = cmp_arf_fmpq(t, q);
        arf_clear(t);
        fmpq_clear(q);
        return c;
    }
    /* A Gaussian number with a nonzero imaginary part: only absolute values are compared. */
    if (!absolute)
        return 99;
    if (arf_is_inf(a->x))
        return 1;
    fmpq_t n, t;
    fmpq_init(n);
    fmpq_init(t);
    fmpq_mul(n, b->re, b->re);
    fmpq_mul(t, b->im, b->im);
    fmpq_add(n, n, t);
    int c = cmp_arf_square_fmpq(a->x, n);
    fmpq_clear(n);
    fmpq_clear(t);
    return c;
}

/* The total order of ComparableBigExponentFloat, for two arf_t values. */
static int
compare_total(const arf_t a, const arf_t b, int absolute)
{
    int a_nan = arf_is_nan(a);
    int b_nan = arf_is_nan(b);
    if (a_nan && b_nan)
        return 0;
    if (a_nan)
        return absolute || arf_sgn(b) >= 0 ? -1 : 1;
    if (b_nan)
        return absolute || arf_sgn(a) >= 0 ? 1 : -1;
    return sign_of(absolute ? arf_cmpabs(a, b) : arf_cmp(a, b));
}

/* -------------------------------------------------------------------------------------------- */
/* Comparison lines                                                                             */

static int
check_cmp(char * line, int line_number, int total)
{
    /* The relation, and the two operands. */
    const char * relations[] = {" are incomparable", " and ", " < ", " > ", " = ", " ≠ "};
    char * lhs = line;
    char * rhs = NULL;
    int relation = -1;
    char * incomparable = strstr(line, relations[0]);
    if (incomparable != NULL)
    {
        if (incomparable[strlen(relations[0])] != '\0')
            return fail(line_number, "text after `are incomparable`");
        *incomparable = '\0';
        rhs = split(line, relations[1]);
        relation = 1;
    }
    else
    {
        for (int r = 2; r < 6; r++)
        {
            if (strstr(line, relations[r]) != NULL)
            {
                rhs = split(line, relations[r]);
                relation = r;
                break;
            }
        }
    }
    if (rhs == NULL)
        return fail(line_number, "unrecognized line");
    int absolute = lhs[0] == '|';
    if (absolute)
    {
        lhs = strip(lhs, "|", '|');
        rhs = strip(rhs, "|", '|');
        if (lhs == NULL || rhs == NULL)
            return fail(line_number, "unbalanced absolute-value bars");
    }
    value_t a, b;
    value_init(a);
    value_init(b);
    int result = 0;
    if (parse_value(lhs, a) || parse_value(rhs, b))
        result = fail(line_number, "bad operand");
    else if (a->kind != VALUE_ARF && b->kind != VALUE_ARF)
        result = fail(line_number, "neither operand is a BigExponentFloat");
    else
    {
        int c;
        if (total)
        {
            if (a->kind != VALUE_ARF || b->kind != VALUE_ARF)
                result = fail(line_number, "total order needs two BigExponentFloats");
            c = compare_total(a->x, b->x, absolute);
        }
        else
            c = compare_values(a, b, absolute);
        if (result == 0)
        {
            /* A non-real Gaussian number is unequal to every BigExponentFloat and unordered. */
            if (c == 99 && relation != 5)
                result = fail(line_number, "a non-real Gaussian number ordered or equal");
            else if (c != 99)
            {
                int ok;
                switch (relation)
                {
                    case 1: ok = c == INCOMPARABLE; break;
                    case 2: ok = c == -1; break;
                    case 3: ok = c == 1; break;
                    case 4: ok = c == 0; break;
                    default: ok = c != 0; break;
                }
                if (!ok)
                    result = fail(line_number, "wrong comparison");
            }
        }
    }
    value_clear(a);
    value_clear(b);
    return result;
}

/* -------------------------------------------------------------------------------------------- */
/* min and max                                                                                  */

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
        return 1;
    return 0;
}

static const char *
ordering_name(int c)
{
    return c < 0 ? "Less" : c > 0 ? "Greater" : "Equal";
}

/* Parses a positive decimal precision. */
static int
parse_prec(const char * s, slong * prec)
{
    char * end;
    long p = strtol(s, &end, 10);
    if (*end != '\0' || p <= 0)
        return 1;
    *prec = p;
    return 0;
}

/* Rounds `x` to `prec` bits with `rnd`, writing the result and the direction of the rounding.
   With `exact`, the value must be representable. */
static int
round_arf(arf_t z, int * o, const arf_t x, slong prec, arf_rnd_t rnd, int exact)
{
    if (arf_is_special(x))
    {
        arf_set(z, x);
        *o = 0;
        return 0;
    }
    int inexact = arf_set_round(z, x, prec, rnd);
    if (exact && inexact)
        return 1;
    *o = sign_of(arf_cmp(z, x));
    return 0;
}

/* Rounds `q` to `prec` bits with `rnd`, writing the result and the direction of the rounding. */
static int
round_fmpq(arf_t z, int * o, const fmpq_t q, slong prec, arf_rnd_t rnd, int exact)
{
    int inexact = arf_fmpz_div_fmpz(z, fmpq_numref(q), fmpq_denref(q), prec, rnd);
    if (exact && inexact)
        return 1;
    *o = cmp_arf_fmpq(z, q);
    return 0;
}

static int
check_min_max(char * line, int line_number)
{
    char * rhs = split(line, " = ");
    if (rhs == NULL)
        return fail(line_number, "no ` = `");
    /* Find the method: `.min` or `.max` cannot occur inside a hexadecimal number. */
    char * dot_min = strstr(line, ".min");
    char * dot_max = strstr(line, ".max");
    char * dot = dot_min != NULL ? dot_min : dot_max;
    if (dot == NULL || (dot_min != NULL && dot_max != NULL))
        return fail(line_number, "no `.min` or `.max`");
    int is_max = dot == dot_max;
    *dot = '\0';
    char * method = dot + 1;
    char * args = strchr(method, '(');
    if (args == NULL)
        return fail(line_number, "no arguments");
    *args = '\0';
    args = strip(args + 1, "", ')');
    if (args == NULL)
        return fail(line_number, "unclosed arguments");
    const char * suffix = method + 3;
    int rational = 0, has_prec = 0, has_rm = 0;
    if (strcmp(suffix, "") == 0)
        ;
    else if (strcmp(suffix, "_prec_round") == 0)
        has_prec = has_rm = 1;
    else if (strcmp(suffix, "_prec") == 0)
        has_prec = 1;
    else if (strcmp(suffix, "_rational_prec_round") == 0)
        rational = has_prec = has_rm = 1;
    else if (strcmp(suffix, "_rational_prec") == 0)
        rational = has_prec = 1;
    else
        return fail(line_number, "unknown method");
    /* The arguments: the other operand, then the precision and rounding mode. */
    char * pieces[3] = {args, NULL, NULL};
    int n_pieces = 1 + has_prec + has_rm;
    for (int k = 1; k < n_pieces; k++)
    {
        pieces[k] = split(pieces[k - 1], ", ");
        if (pieces[k] == NULL)
            return fail(line_number, "too few arguments");
    }
    slong prec = 0;
    arf_rnd_t rnd = ARF_RND_NEAR;
    int exact = 0;
    if (has_prec && parse_prec(pieces[1], &prec))
        return fail(line_number, "bad precision");
    if (has_rm && parse_rm(pieces[2], &rnd, &exact))
        return fail(line_number, "bad rounding mode");
    /* The output. */
    char * out_value = rhs;
    char * out_ordering = NULL;
    if (has_prec)
    {
        char * inner = strip(rhs, "(", ')');
        if (inner == NULL)
            return fail(line_number, "bad output pair");
        out_value = inner;
        out_ordering = split(inner, ", ");
        if (out_ordering == NULL)
            return fail(line_number, "bad output pair");
    }
    arf_t x, y, z, expected;
    fmpq_t q;
    arf_init(x);
    arf_init(y);
    arf_init(z);
    arf_init(expected);
    fmpq_init(q);
    int result = 0;
    if (arf_parse_base(line, 4, x) || arf_parse_base(out_value, 4, z)
        || (rational ? parse_fmpq(pieces[0], q) : arf_parse_base(pieces[0], 4, y)))
        result = fail(line_number, "bad operand");
    else
    {
        /* Select the winner. */
        int second;
        if (rational)
        {
            int c = arf_is_nan(x) ? INCOMPARABLE
                : arf_is_inf(x) ? arf_sgn(x) : cmp_arf_fmpq(x, q);
            second = c == INCOMPARABLE || c == (is_max ? -1 : 1);
        }
        else if (arf_is_nan(y))
            second = 0;
        else if (arf_is_nan(x))
            second = 1;
        else
            second = sign_of(arf_cmp(x, y)) == (is_max ? -1 : 1);
        int o = 0;
        if (!has_prec)
            arf_set(expected, second ? y : x);
        else if (second && rational)
            result = round_fmpq(expected, &o, q, prec, rnd, exact);
        else
            result = round_arf(expected, &o, second ? y : x, prec, rnd, exact);
        if (result != 0)
            result = fail(line_number, "an Exact rounding is inexact");
        else if (!arf_equal(expected, z))
            result = fail(line_number, "wrong value");
        else if (has_prec && strcmp(out_ordering, ordering_name(o)) != 0)
            result = fail(line_number, "wrong ordering");
    }
    arf_clear(x);
    arf_clear(y);
    arf_clear(z);
    arf_clear(expected);
    fmpq_clear(q);
    return result;
}

/* -------------------------------------------------------------------------------------------- */

static int
check_line(char * line, int line_number)
{
    size_t n = strlen(line);
    if (n != 0 && line[n - 1] == '\n')
        line[n - 1] = '\0';
    if (line[0] == '\0')
        return 0;
    int result;
    switch (kind)
    {
        case KIND_CMP: result = check_cmp(line, line_number, 0); break;
        case KIND_CMP_TOTAL: result = check_cmp(line, line_number, 1); break;
        default: result = check_min_max(line, line_number); break;
    }
    if (result == 0)
        checked++;
    return result;
}

static int
run(const char * arg, const char * name, comparison_kind k)
{
    checked = 0;
    kind = k;
    mode_name = name;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_arf_cmp(const char * arg)
{
    return run(arg, "arf_cmp", KIND_CMP);
}

int
run_arf_cmp_total(const char * arg)
{
    return run(arg, "arf_cmp_total", KIND_CMP_TOTAL);
}

int
run_arf_min_max(const char * arg)
{
    return run(arg, "arf_min_max", KIND_MIN_MAX);
}
