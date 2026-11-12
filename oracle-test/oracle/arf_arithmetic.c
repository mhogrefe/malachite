/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the arithmetic of Malachite's BigExponentFloat against FLINT's arf_t. A BigExponentFloat
    is written in Malachite's exact hexadecimal format. The mode `arf_arithmetic` accepts these
    lines:

    - `-X = Z`, `X.abs() = Z`, and `X.sign() = O`, against arf_neg, arf_abs, and arf_sgn.
    - `X << K = Z` and `X >> K = Z`, for any integer K, against arf_mul_2exp_fmpz.
    - `X + Y = Z`, `X - Y = Z`, `X * Y = Z`, and `X.square() = Z`, against arf_add, arf_sub, and
      arf_mul with precision ARF_PREC_EXACT.
    - `X.op_prec_round(Y, p, rm) = (Z, O)` and `X.op_prec(Y, p) = (Z, O)` for op among add, sub,
      mul, and div, and `X.op_prec_round(p, rm) = (Z, O)` and `X.op_prec(p) = (Z, O)` for op among
      square, reciprocal, and sqrt, against arf_add, arf_sub, arf_mul, arf_div, arf_ui_div with
      numerator 1, and arf_sqrt.

    FLINT reports only whether a rounded result is inexact, so the ordering O is derived here: an
    exact result gives `Equal`; `Floor` and `Ceiling` round in a known direction; `Down` and `Up`
    round toward or away from zero, so the direction follows from the result's sign; and a
    `Nearest` result is compared with the same operation rounded toward zero, whose direction is
    known. This needs no exact result, which, for a sum of values whose exponents are far apart,
    could be enormous. `Exact` requires the operation to be exact.

    Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/arf.h>
#include <flint/fmpz.h>

#include "oracle.h"

static long checked;

static int
report(int line_number, const char * what)
{
    flint_printf("error in arf_arithmetic test, line %d: %s\n", line_number, what);
    return 1;
}

/* Splits `line` at the first occurrence of `sep`, in place, returning the text after it, or NULL. */
static char *
split(char * line, const char * sep)
{
    char * p = strstr(line, sep);
    if (p == NULL)
        return NULL;
    *p = '\0';
    return p + strlen(sep);
}

/* Parses a rounding mode; `*exact` is set for `Exact`, which is checked with rounding down. */
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

typedef enum
{
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_DIV,
    OP_SQUARE,
    OP_RECIPROCAL,
    OP_SQRT
} op_kind;

/* Applies `op` to `x` and, for the binary operations, `y`, returning FLINT's inexact flag. */
static int
apply(op_kind op, arf_t z, const arf_t x, const arf_t y, slong prec, arf_rnd_t rnd)
{
    switch (op)
    {
        case OP_ADD: return arf_add(z, x, y, prec, rnd);
        case OP_SUB: return arf_sub(z, x, y, prec, rnd);
        case OP_MUL: return arf_mul(z, x, y, prec, rnd);
        case OP_DIV: return arf_div(z, x, y, prec, rnd);
        case OP_SQUARE: return arf_mul(z, x, x, prec, rnd);
        case OP_RECIPROCAL: return arf_ui_div(z, 1, x, prec, rnd);
        default: return arf_sqrt(z, x, prec, rnd);
    }
}

/* The ordering of the rounded result `z` relative to the exact result, as described above. */
static int
rounding_direction(op_kind op, const arf_t z, int inexact, const arf_t x, const arf_t y,
                   slong prec, arf_rnd_t rnd)
{
    if (!inexact)
        return 0;
    switch (rnd)
    {
        case ARF_RND_FLOOR: return -1;
        case ARF_RND_CEIL: return 1;
        case ARF_RND_DOWN: return -arf_sgn(z);
        case ARF_RND_UP: return arf_sgn(z);
        default:
        {
            arf_t down;
            arf_init(down);
            apply(op, down, x, y, prec, ARF_RND_DOWN);
            int toward_zero = arf_equal(down, z);
            arf_clear(down);
            return toward_zero ? -arf_sgn(z) : arf_sgn(z);
        }
    }
}

static const struct
{
    const char * name;
    op_kind op;
    int binary;
} methods[] = {
    {"add", OP_ADD, 1},
    {"sub", OP_SUB, 1},
    {"mul", OP_MUL, 1},
    {"div", OP_DIV, 1},
    {"square", OP_SQUARE, 0},
    {"reciprocal", OP_RECIPROCAL, 0},
    {"sqrt", OP_SQRT, 0},
};

/* `X.op_prec_round(...) = (Z, O)` or `X.op_prec(...) = (Z, O)`; `method` starts after the `.`. */
static int
check_rounded(const char * x_str, char * method, int line_number)
{
    char * args = split(method, "(");
    char * result = args == NULL ? NULL : split(args, ") = (");
    if (result == NULL)
        return report(line_number, "unrecognized line");
    size_t n = strlen(result);
    if (n == 0 || result[n - 1] != ')')
        return report(line_number, "unrecognized result");
    result[n - 1] = '\0';
    char * o_str = strrchr(result, ',');
    if (o_str == NULL || o_str[1] != ' ')
        return report(line_number, "unrecognized result");
    *o_str = '\0';
    o_str += 2;
    char * z_str = result;
    int with_rm;
    char * op_name = method;
    char * suffix = strstr(method, "_prec");
    if (suffix == NULL)
        return report(line_number, "unrecognized method");
    *suffix = '\0';
    suffix += 5;
    if (strcmp(suffix, "_round") == 0)
        with_rm = 1;
    else if (*suffix == '\0')
        with_rm = 0;
    else
        return report(line_number, "unrecognized method");
    int m = -1;
    for (size_t i = 0; i < sizeof(methods) / sizeof(methods[0]); i++)
        if (strcmp(op_name, methods[i].name) == 0)
            m = (int) i;
    if (m < 0)
        return report(line_number, "unrecognized method");
    /* The arguments: `Y, ` for a binary operation, then the precision and the rounding mode. */
    char * y_str = NULL;
    if (methods[m].binary)
    {
        y_str = args;
        args = split(args, ", ");
        if (args == NULL)
            return report(line_number, "unreadable arguments");
    }
    char * rm_str = with_rm ? split(args, ", ") : NULL;
    arf_rnd_t rnd = ARF_RND_NEAR;
    int exact = 0;
    char * end;
    unsigned long long prec = strtoull(args, &end, 10);
    if (*end != '\0' || prec == 0 || prec > (unsigned long long) WORD_MAX
        || (with_rm && (rm_str == NULL || parse_rm(rm_str, &rnd, &exact))))
        return report(line_number, "unreadable arguments");
    arf_t x, y, z, expected;
    arf_init(x);
    arf_init(y);
    arf_init(z);
    arf_init(expected);
    int res;
    if (arf_parse_base(x_str, 4, x) || (y_str != NULL && arf_parse_base(y_str, 4, y))
        || arf_parse_base(z_str, 4, expected))
    {
        res = report(line_number, "unreadable value");
    }
    else
    {
        op_kind op = methods[m].op;
        int inexact = apply(op, z, x, y, (slong) prec, rnd);
        int c = rounding_direction(op, z, inexact, x, y, (slong) prec, rnd);
        if (exact && inexact)
            res = report(line_number, "Exact rounding of an inexact result");
        else if (!arf_equal(z, expected))
            res = report(line_number, "wrong value");
        else if (strcmp(ordering_name(c), o_str) != 0)
            res = report(line_number, "wrong ordering");
        else
            res = 0;
    }
    arf_clear(x);
    arf_clear(y);
    arf_clear(z);
    arf_clear(expected);
    return res;
}

/* The `.` that begins a method call in `line`: one followed by a lowercase identifier and `(`.
   A hexadecimal value's point is followed by digits, which never reach a `(`. */
static char *
find_method(char * line)
{
    for (char * p = strchr(line, '.'); p != NULL; p = strchr(p + 1, '.'))
    {
        char * q = p + 1;
        while ((*q >= 'a' && *q <= 'z') || *q == '_')
            q++;
        if (q > p + 1 && *q == '(')
            return p;
    }
    return NULL;
}

static int
check_line(char * line, int line_number)
{
    size_t n = strlen(line);
    if (n != 0 && line[n - 1] == '\n')
        line[n - 1] = '\0';
    if (line[0] == '\0')
        return 0;
    int result;
    char * dot = find_method(line);
    if (dot != NULL && strstr(dot, "_prec") != NULL)
    {
        *dot = '\0';
        result = check_rounded(line, dot + 1, line_number);
    }
    else
    {
        /* The exact operations: `LHS = Z`, where LHS is `-X`, `X op Y`, or `X.method()`. */
        char * z_str = split(line, " = ");
        if (z_str == NULL)
            return report(line_number, "unrecognized line");
        arf_t x, y, z;
        arf_init(x);
        arf_init(y);
        arf_init(z);
        result = 0;
        const char * operators[] = {" + ", " - ", " * ", " << ", " >> "};
        int operator = -1;
        char * rhs = NULL;
        for (int i = 0; i < 5 && dot == NULL; i++)
        {
            rhs = strstr(line, operators[i]);
            if (rhs != NULL)
            {
                *rhs = '\0';
                rhs += strlen(operators[i]);
                operator = i;
                break;
            }
        }
        if (dot != NULL)
        {
            *dot = '\0';
            const char * method = dot + 1;
            if (arf_parse_base(line, 4, x))
                result = report(line_number, "unreadable value");
            else if (strcmp(method, "sign()") == 0)
            {
                if (arf_is_nan(x))
                    result = report(line_number, "sign of NaN");
                else if (strcmp(ordering_name(arf_sgn(x)), z_str) != 0)
                    result = report(line_number, "wrong sign");
            }
            else if (arf_parse_base(z_str, 4, y))
                result = report(line_number, "unreadable value");
            else
            {
                if (strcmp(method, "abs()") == 0)
                    arf_abs(z, x);
                else if (strcmp(method, "square()") == 0)
                    arf_mul(z, x, x, ARF_PREC_EXACT, ARF_RND_DOWN);
                else
                    result = report(line_number, "unrecognized method");
                if (result == 0 && !arf_equal(z, y))
                    result = report(line_number, "wrong value");
            }
        }
        else if (operator < 0)
        {
            /* `-X = Z` */
            if (line[0] != '-' || arf_parse_base(line + 1, 4, x) || arf_parse_base(z_str, 4, y))
                result = report(line_number, "unrecognized line");
            else
            {
                arf_neg(z, x);
                if (!arf_equal(z, y))
                    result = report(line_number, "wrong value");
            }
        }
        else if (arf_parse_base(line, 4, x) || arf_parse_base(z_str, 4, y))
            result = report(line_number, "unreadable value");
        else if (operator >= 3)
        {
            fmpz_t k;
            fmpz_init(k);
            if (fmpz_set_str(k, rhs, 10))
                result = report(line_number, "unreadable shift");
            else
            {
                if (operator == 4)
                    fmpz_neg(k, k);
                arf_mul_2exp_fmpz(z, x, k);
                if (!arf_equal(z, y))
                    result = report(line_number, "wrong value");
            }
            fmpz_clear(k);
        }
        else
        {
            arf_t w;
            arf_init(w);
            if (arf_parse_base(rhs, 4, w))
                result = report(line_number, "unreadable value");
            else
            {
                op_kind op = operator == 0 ? OP_ADD : operator == 1 ? OP_SUB : OP_MUL;
                apply(op, z, x, w, ARF_PREC_EXACT, ARF_RND_DOWN);
                if (!arf_equal(z, y))
                    result = report(line_number, "wrong value");
            }
            arf_clear(w);
        }
        arf_clear(x);
        arf_clear(y);
        arf_clear(z);
    }
    if (result == 0)
        checked++;
    return result;
}

int
run_arf_arithmetic(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_arithmetic", checked);
}
