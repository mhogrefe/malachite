/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the demos of IntegerPolynomial's slice-level multiplication
    algorithms against FLINT's underscore functions. Each line has the form
    `name(_, [x, ...], [y, ...], a, b) = [z, ...]`: the name of a Malachite function, its inputs
    (the `_` stands for the output slice), and the output. Which inputs follow the vectors depends
    on the function:

    mul (`mul_to_out_*`, `mul_greater_to_out`)           xs, ys          -> _fmpz_poly_mul*
    mullow (`mul_truncated_to_out*`)                     xs, ys          -> _fmpz_poly_mullow*
    mulhigh (`mul_high_to_out_classical`)                xs, ys, start   -> _fmpz_poly_mulhigh_*
    mulmid (`mul_middle_to_out*`)                        xs, ys, lo, hi  -> _fmpz_poly_mulmid*
    sqr (`square_to_out*`)                               xs              -> _fmpz_poly_sqr*
    sqrlow (`square_truncated_to_out*`)                  xs              -> _fmpz_poly_sqrlow*

    For the truncated functions, the length is that of the output. Each mode accepts the lines
    of the Malachite functions that correspond to its FLINT function: the classical modes those of
    the classical algorithms, and the dispatcher modes those of the Malachite dispatchers and of
    the tiny kernels, which FLINT does not export. When `xs` and `ys` are written identically,
    FLINT is given the same vector for both, so that its squaring paths run too. Every coefficient
    FLINT writes is compared: mulhigh sets the coefficients below `start` to zero, as Malachite
    does. Each mode treats any other nonempty line as an error, and an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/fmpz_poly.h>
#include <flint/fmpz_vec.h>

#include "oracle.h"

typedef enum
{
    KIND_MUL,
    KIND_MULLOW,
    KIND_MULHIGH,
    KIND_MULMID,
    KIND_SQR,
    KIND_SQRLOW
} kind_t;

typedef struct
{
    const char * name;
    kind_t kind;
    /* Whether the mode calls the classical algorithm rather than the dispatcher. */
    int classical;
    /* The Malachite functions whose lines the mode accepts, terminated by NULL. */
    const char * functions[4];
} slice_mode_t;

static const slice_mode_t * current_mode;
static long checked;

/* Parses a vector written as `[x, y, ...]` at `*s`, advancing `*s` past the closing bracket.
   Allocates `*v`, which the caller frees with _fmpz_vec_clear. Returns 1 on success. */
static int
parse_vec(char ** s, fmpz ** v, slong * len)
{
    char * p = *s;
    if (*p != '[')
    {
        return 0;
    }
    p++;
    char * close = strchr(p, ']');
    if (close == NULL)
    {
        return 0;
    }
    slong n = 0;
    if (close != p)
    {
        n = 1;
        for (char * c = p; c < close; c++)
        {
            if (*c == ',')
            {
                n++;
            }
        }
    }
    *v = _fmpz_vec_init(n);
    *len = n;
    *close = '\0';
    for (slong i = 0; i < n; i++)
    {
        char * comma = strchr(p, ',');
        if (comma != NULL)
        {
            *comma = '\0';
        }
        if (fmpz_set_str(*v + i, p, 10) != 0)
        {
            return 0;
        }
        if (comma != NULL)
        {
            if (comma[1] != ' ')
            {
                return 0;
            }
            p = comma + 2;
        }
    }
    *s = close + 1;
    return 1;
}

/* Parses `, n` at `*s`, advancing `*s` past it. Returns 1 on success. */
static int
parse_index(char ** s, slong * n)
{
    if (strncmp(*s, ", ", 2) != 0)
    {
        return 0;
    }
    char * end;
    unsigned long long value = strtoull(*s + 2, &end, 10);
    if (end == *s + 2 || value > WORD_MAX)
    {
        return 0;
    }
    *n = (slong) value;
    *s = end;
    return 1;
}

static int
accepts(const char * name, size_t name_len)
{
    for (int i = 0; current_mode->functions[i] != NULL; i++)
    {
        if (strlen(current_mode->functions[i]) == name_len
            && strncmp(current_mode->functions[i], name, name_len) == 0)
        {
            return 1;
        }
    }
    return 0;
}

static int
check_line(char * line, int line_number)
{
    const char * mode = current_mode->name;
    line[strcspn(line, "\r\n")] = '\0';
    if (line[0] == '\0')
    {
        return 0;
    }
    char * open = strstr(line, "(_, ");
    if (open == NULL || !accepts(line, (size_t) (open - line)))
    {
        flint_printf("error in %s test, line %d: unrecognized line\n", mode, line_number);
        return 1;
    }
    checked++;
    char * s = open + 4;
    fmpz * xs = NULL;
    fmpz * ys = NULL;
    fmpz * expected = NULL;
    fmpz * out = NULL;
    slong len1 = 0, len2 = 0, expected_len = 0, a = 0, b = 0, out_len = 0;
    int ok = parse_vec(&s, &xs, &len1) && len1 > 0;
    int two = current_mode->kind != KIND_SQR && current_mode->kind != KIND_SQRLOW;
    int same = 0;
    if (ok && two)
    {
        char * ys_start = s + 2;
        ok = strncmp(s, ", ", 2) == 0;
        s = ys_start;
        ok = ok && parse_vec(&s, &ys, &len2) && len2 > 0;
        if (ok)
        {
            same = len1 == len2 && _fmpz_vec_equal(xs, ys, len1);
        }
    }
    if (ok && current_mode->kind == KIND_MULHIGH)
    {
        ok = parse_index(&s, &a);
    }
    if (ok && current_mode->kind == KIND_MULMID)
    {
        ok = parse_index(&s, &a) && parse_index(&s, &b);
    }
    if (ok)
    {
        ok = strncmp(s, ") = ", 4) == 0;
        s += 4;
    }
    ok = ok && parse_vec(&s, &expected, &expected_len) && *s == '\0';
    if (ok)
    {
        /* The length of the output, and a check of the inputs against FLINT's preconditions. */
        switch (current_mode->kind)
        {
            case KIND_MUL:
                out_len = len1 + len2 - 1;
                break;
            case KIND_MULLOW:
                out_len = expected_len;
                ok = out_len > 0 && out_len <= len1 + len2 - 1;
                break;
            case KIND_MULHIGH:
                out_len = len1 + len2 - 1;
                ok = a <= out_len;
                break;
            case KIND_MULMID:
                out_len = b - a;
                ok = a < b && b <= len1 + len2 - 1;
                break;
            case KIND_SQR:
                out_len = 2 * len1 - 1;
                break;
            case KIND_SQRLOW:
                out_len = expected_len;
                ok = out_len > 0 && out_len <= 2 * len1 - 1;
                break;
        }
    }
    int result = 0;
    if (!ok)
    {
        flint_printf("error in %s test, line %d: unreadable input\n", mode, line_number);
        result = 1;
    }
    else
    {
        const fmpz * y = same ? xs : ys;
        out = _fmpz_vec_init(out_len);
        switch (current_mode->kind)
        {
            case KIND_MUL:
                if (current_mode->classical)
                {
                    _fmpz_poly_mul_classical(out, xs, len1, y, len2);
                }
                else if (len1 >= len2)
                {
                    _fmpz_poly_mul(out, xs, len1, y, len2);
                }
                else
                {
                    _fmpz_poly_mul(out, y, len2, xs, len1);
                }
                break;
            case KIND_MULLOW:
                if (current_mode->classical)
                {
                    _fmpz_poly_mullow_classical(out, xs, len1, y, len2, out_len);
                }
                else
                {
                    _fmpz_poly_mullow(out, xs, len1, y, len2, out_len);
                }
                break;
            case KIND_MULHIGH:
                _fmpz_poly_mulhigh_classical(out, xs, len1, y, len2, a);
                break;
            case KIND_MULMID:
                if (current_mode->classical)
                {
                    _fmpz_poly_mulmid_classical(out, xs, len1, y, len2, a, b);
                }
                else
                {
                    _fmpz_poly_mulmid(out, xs, len1, y, len2, a, b);
                }
                break;
            case KIND_SQR:
                if (current_mode->classical)
                {
                    _fmpz_poly_sqr_classical(out, xs, len1);
                }
                else
                {
                    _fmpz_poly_sqr(out, xs, len1);
                }
                break;
            case KIND_SQRLOW:
                if (current_mode->classical)
                {
                    _fmpz_poly_sqrlow_classical(out, xs, len1, out_len);
                }
                else
                {
                    _fmpz_poly_sqrlow(out, xs, len1, out_len);
                }
                break;
        }
        if (expected_len != out_len || !_fmpz_vec_equal(out, expected, out_len))
        {
            flint_printf("error in %s test, line %d. FLINT: ", mode, line_number);
            _fmpz_vec_print(out, out_len);
            flint_printf("\n");
            result = 1;
        }
    }
    if (xs != NULL)
    {
        _fmpz_vec_clear(xs, len1);
    }
    if (ys != NULL)
    {
        _fmpz_vec_clear(ys, len2);
    }
    if (expected != NULL)
    {
        _fmpz_vec_clear(expected, expected_len);
    }
    if (out != NULL)
    {
        _fmpz_vec_clear(out, out_len);
    }
    return result;
}

static const slice_mode_t SLICE_MODES[] = {
    {"_fmpz_poly_mul_classical", KIND_MUL, 1, {"mul_to_out_classical", NULL}},
    {"_fmpz_poly_mul", KIND_MUL, 0,
     {"mul_greater_to_out", "mul_to_out_tiny_1", "mul_to_out_tiny_2", NULL}},
    {"_fmpz_poly_mullow_classical", KIND_MULLOW, 1, {"mul_truncated_to_out_classical", NULL}},
    {"_fmpz_poly_mullow", KIND_MULLOW, 0,
     {"mul_truncated_to_out", "mul_truncated_to_out_tiny_1", "mul_truncated_to_out_tiny_2",
      NULL}},
    {"_fmpz_poly_mulhigh_classical", KIND_MULHIGH, 1, {"mul_high_to_out_classical", NULL}},
    {"_fmpz_poly_mulmid_classical", KIND_MULMID, 1, {"mul_middle_to_out_classical", NULL}},
    {"_fmpz_poly_mulmid", KIND_MULMID, 0,
     {"mul_middle_to_out", "mul_middle_to_out_tiny_1", "mul_middle_to_out_tiny_2", NULL}},
    {"_fmpz_poly_sqr_classical", KIND_SQR, 1, {"square_to_out_classical", NULL}},
    {"_fmpz_poly_sqr", KIND_SQR, 0,
     {"square_to_out", "square_to_out_tiny_1", "square_to_out_tiny_2", NULL}},
    {"_fmpz_poly_sqrlow_classical", KIND_SQRLOW, 1, {"square_truncated_to_out_classical", NULL}},
    {"_fmpz_poly_sqrlow", KIND_SQRLOW, 0,
     {"square_truncated_to_out", "square_truncated_to_out_tiny_1",
      "square_truncated_to_out_tiny_2", NULL}},
};

static int
run(const char * arg, int index)
{
    current_mode = &SLICE_MODES[index];
    checked = 0;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(current_mode->name, checked);
}

int
run__fmpz_poly_mul_classical(const char * arg)
{
    return run(arg, 0);
}

int
run__fmpz_poly_mul(const char * arg)
{
    return run(arg, 1);
}

int
run__fmpz_poly_mullow_classical(const char * arg)
{
    return run(arg, 2);
}

int
run__fmpz_poly_mullow(const char * arg)
{
    return run(arg, 3);
}

int
run__fmpz_poly_mulhigh_classical(const char * arg)
{
    return run(arg, 4);
}

int
run__fmpz_poly_mulmid_classical(const char * arg)
{
    return run(arg, 5);
}

int
run__fmpz_poly_mulmid(const char * arg)
{
    return run(arg, 6);
}

int
run__fmpz_poly_sqr_classical(const char * arg)
{
    return run(arg, 7);
}

int
run__fmpz_poly_sqr(const char * arg)
{
    return run(arg, 8);
}

int
run__fmpz_poly_sqrlow_classical(const char * arg)
{
    return run(arg, 9);
}

int
run__fmpz_poly_sqrlow(const char * arg)
{
    return run(arg, 10);
}
