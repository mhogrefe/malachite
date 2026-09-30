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
    mulhigh (`mul_high_to_out*`)                         xs, ys, start   -> _fmpz_poly_mulhigh*
    mulhigh_n (`mul_high_to_out_karatsuba_n`)            xs, ys          -> _fmpz_poly_mulhigh_karatsuba_n
    mulmid (`mul_middle_to_out*`)                        xs, ys, lo, hi  -> _fmpz_poly_mulmid*
    fft (`mul_middle_to_out_fft`)                        xs, ys, lo, hi  -> ..._mul_mid_default_mpn_ctx
    sqr (`square_to_out*`)                               xs              -> _fmpz_poly_sqr*
    sqrlow (`square_truncated_to_out*`)                  xs              -> _fmpz_poly_sqrlow*

    For the truncated functions, the length is that of the output. Each mode accepts the lines
    of the Malachite functions that correspond to its FLINT function: the classical, Karatsuba, and
    Kronecker-substitution modes those of the corresponding algorithms, and the dispatcher modes those of the Malachite dispatchers and of
    the tiny kernels, which FLINT does not export. When `xs` and `ys` are written identically,
    FLINT is given the same vector for both, so that its squaring paths run too. Every coefficient
    FLINT writes is compared, except that the mulhigh dispatcher defines only the coefficients from
    `start` on: its classical and Karatsuba algorithms set the ones below to zero, as Malachite's
    do, but Kronecker substitution leaves them holding coefficients of the whole product. Each mode treats any other nonempty line as an error, and an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/fmpz_poly.h>
#include <flint/fft_small.h>
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

/* The FLINT function a mode calls for its kind of product. */
typedef enum
{
    VARIANT_DISPATCHER,
    VARIANT_CLASSICAL,
    VARIANT_KARATSUBA,
    /* The Karatsuba functions whose inputs have exactly the length of the output (mullow and
       sqrlow) or the same length as each other (mulhigh). */
    VARIANT_KARATSUBA_N,
    VARIANT_KS,
    /* The small-prime FFT, which may decline: its lines end in `Some([z, ...])` or `None`. */
    VARIANT_FFT
} variant_t;

typedef struct
{
    const char * name;
    kind_t kind;
    variant_t variant;
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
    if (ok && current_mode->kind == KIND_MULHIGH && current_mode->variant != VARIANT_KARATSUBA_N)
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
    int expect_some = 1;
    if (ok && current_mode->variant == VARIANT_FFT)
    {
        if (strcmp(s, "None") == 0)
        {
            expect_some = 0;
            expected_len = b - a;
        }
        else
        {
            ok = strncmp(s, "Some(", 5) == 0;
            s += 5;
            ok = ok && parse_vec(&s, &expected, &expected_len) && strcmp(s, ")") == 0;
        }
    }
    else
    {
        ok = ok && parse_vec(&s, &expected, &expected_len) && *s == '\0';
    }
    if (ok)
    {
        /* The length of the output, and a check of the inputs against FLINT's preconditions. */
        switch (current_mode->kind)
        {
            case KIND_MUL:
                out_len = len1 + len2 - 1;
                if (current_mode->variant == VARIANT_KARATSUBA)
                {
                    ok = len1 >= len2;
                }
                break;
            case KIND_MULLOW:
                out_len = expected_len;
                ok = out_len > 0 && out_len <= len1 + len2 - 1;
                if (current_mode->variant == VARIANT_KARATSUBA_N)
                {
                    ok = ok && len1 >= out_len && len2 >= out_len;
                }
                break;
            case KIND_MULHIGH:
                out_len = len1 + len2 - 1;
                if (current_mode->variant == VARIANT_KARATSUBA_N)
                {
                    ok = len1 == len2;
                }
                else
                {
                    ok = a <= out_len;
                }
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
                if (current_mode->variant == VARIANT_KARATSUBA_N)
                {
                    ok = ok && len1 >= out_len;
                }
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
        variant_t v = current_mode->variant;
        switch (current_mode->kind)
        {
            case KIND_MUL:
                if (v == VARIANT_CLASSICAL)
                {
                    _fmpz_poly_mul_classical(out, xs, len1, y, len2);
                }
                else if (v == VARIANT_KS)
                {
                    _fmpz_poly_mul_KS(out, xs, len1, y, len2);
                }
                else if (v == VARIANT_KARATSUBA)
                {
                    _fmpz_poly_mul_karatsuba(out, xs, len1, y, len2);
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
                if (v == VARIANT_CLASSICAL)
                {
                    _fmpz_poly_mullow_classical(out, xs, len1, y, len2, out_len);
                }
                else if (v == VARIANT_KS)
                {
                    _fmpz_poly_mullow_KS(out, xs, len1, y, len2, out_len);
                }
                else if (v == VARIANT_KARATSUBA)
                {
                    _fmpz_poly_mullow_karatsuba(out, xs, len1, y, len2, out_len);
                }
                else if (v == VARIANT_KARATSUBA_N)
                {
                    _fmpz_poly_mullow_karatsuba_n(out, xs, y, out_len);
                }
                else
                {
                    _fmpz_poly_mullow(out, xs, len1, y, len2, out_len);
                }
                break;
            case KIND_MULHIGH:
                if (v == VARIANT_CLASSICAL)
                {
                    _fmpz_poly_mulhigh_classical(out, xs, len1, y, len2, a);
                }
                else if (v == VARIANT_KARATSUBA_N)
                {
                    _fmpz_poly_mulhigh_karatsuba_n(out, xs, y, len1);
                }
                else
                {
                    _fmpz_poly_mulhigh(out, xs, len1, y, len2, a);
                }
                break;
            case KIND_MULMID:
                if (v == VARIANT_CLASSICAL)
                {
                    _fmpz_poly_mulmid_classical(out, xs, len1, y, len2, a, b);
                }
                else if (v == VARIANT_FFT)
                {
                    int computed = _fmpz_poly_mul_mid_default_mpn_ctx(out, a, b, xs, len1, y,
                                                                      len2);
                    if (computed != expect_some)
                    {
                        flint_printf("error in %s test, line %d: FLINT %s\n", mode, line_number,
                                     computed ? "computed the product" : "declined");
                        result = 1;
                    }
                }
                else if (v == VARIANT_KS)
                {
                    _fmpz_poly_mulmid_KS(out, xs, len1, y, len2, a, b);
                }
                else
                {
                    _fmpz_poly_mulmid(out, xs, len1, y, len2, a, b);
                }
                break;
            case KIND_SQR:
                if (v == VARIANT_CLASSICAL)
                {
                    _fmpz_poly_sqr_classical(out, xs, len1);
                }
                else if (v == VARIANT_KS)
                {
                    _fmpz_poly_sqr_KS(out, xs, len1);
                }
                else if (v == VARIANT_KARATSUBA)
                {
                    _fmpz_poly_sqr_karatsuba(out, xs, len1);
                }
                else
                {
                    _fmpz_poly_sqr(out, xs, len1);
                }
                break;
            case KIND_SQRLOW:
                if (v == VARIANT_CLASSICAL)
                {
                    _fmpz_poly_sqrlow_classical(out, xs, len1, out_len);
                }
                else if (v == VARIANT_KS)
                {
                    _fmpz_poly_sqrlow_KS(out, xs, len1, out_len);
                }
                else if (v == VARIANT_KARATSUBA)
                {
                    _fmpz_poly_sqrlow_karatsuba(out, xs, len1, out_len);
                }
                else if (v == VARIANT_KARATSUBA_N)
                {
                    _fmpz_poly_sqrlow_karatsuba_n(out, xs, out_len);
                }
                else
                {
                    _fmpz_poly_sqrlow(out, xs, len1, out_len);
                }
                break;
        }
        /* The mulhigh dispatcher may leave anything in the coefficients below `start`, since
           Kronecker substitution computes the whole product; only the rest are compared. */
        slong skip = current_mode->kind == KIND_MULHIGH && v == VARIANT_DISPATCHER ? a : 0;
        if (result == 0 && expect_some
            && (expected_len != out_len
                || !_fmpz_vec_equal(out + skip, expected + skip, out_len - skip)))
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
    {"_fmpz_poly_mul_classical", KIND_MUL, VARIANT_CLASSICAL, {"mul_to_out_classical", NULL}},
    {"_fmpz_poly_mul", KIND_MUL, VARIANT_DISPATCHER,
     {"mul_greater_to_out", "mul_to_out_tiny_1", "mul_to_out_tiny_2", NULL}},
    {"_fmpz_poly_mullow_classical", KIND_MULLOW, VARIANT_CLASSICAL,
     {"mul_truncated_to_out_classical", NULL}},
    {"_fmpz_poly_mullow", KIND_MULLOW, VARIANT_DISPATCHER,
     {"mul_truncated_to_out", "mul_truncated_to_out_tiny_1", "mul_truncated_to_out_tiny_2",
      NULL}},
    {"_fmpz_poly_mulhigh_classical", KIND_MULHIGH, VARIANT_CLASSICAL,
     {"mul_high_to_out_classical", NULL}},
    {"_fmpz_poly_mulmid_classical", KIND_MULMID, VARIANT_CLASSICAL,
     {"mul_middle_to_out_classical", NULL}},
    {"_fmpz_poly_mulmid", KIND_MULMID, VARIANT_DISPATCHER,
     {"mul_middle_to_out", "mul_middle_to_out_tiny_1", "mul_middle_to_out_tiny_2", NULL}},
    {"_fmpz_poly_sqr_classical", KIND_SQR, VARIANT_CLASSICAL, {"square_to_out_classical", NULL}},
    {"_fmpz_poly_sqr", KIND_SQR, VARIANT_DISPATCHER,
     {"square_to_out", "square_to_out_tiny_1", "square_to_out_tiny_2", NULL}},
    {"_fmpz_poly_sqrlow_classical", KIND_SQRLOW, VARIANT_CLASSICAL,
     {"square_truncated_to_out_classical", NULL}},
    {"_fmpz_poly_sqrlow", KIND_SQRLOW, VARIANT_DISPATCHER,
     {"square_truncated_to_out", "square_truncated_to_out_tiny_1",
      "square_truncated_to_out_tiny_2", NULL}},
    {"_fmpz_poly_mul_karatsuba", KIND_MUL, VARIANT_KARATSUBA, {"mul_to_out_karatsuba", NULL}},
    {"_fmpz_poly_mullow_karatsuba", KIND_MULLOW, VARIANT_KARATSUBA,
     {"mul_truncated_to_out_karatsuba", NULL}},
    {"_fmpz_poly_mullow_karatsuba_n", KIND_MULLOW, VARIANT_KARATSUBA_N,
     {"mul_truncated_to_out_karatsuba_n", NULL}},
    {"_fmpz_poly_mulhigh_karatsuba_n", KIND_MULHIGH, VARIANT_KARATSUBA_N,
     {"mul_high_to_out_karatsuba_n", NULL}},
    {"_fmpz_poly_mulhigh", KIND_MULHIGH, VARIANT_DISPATCHER, {"mul_high_to_out", NULL}},
    {"_fmpz_poly_sqr_karatsuba", KIND_SQR, VARIANT_KARATSUBA, {"square_to_out_karatsuba", NULL}},
    {"_fmpz_poly_sqrlow_karatsuba", KIND_SQRLOW, VARIANT_KARATSUBA,
     {"square_truncated_to_out_karatsuba", NULL}},
    {"_fmpz_poly_sqrlow_karatsuba_n", KIND_SQRLOW, VARIANT_KARATSUBA_N,
     {"square_truncated_to_out_karatsuba_n", NULL}},
    {"_fmpz_poly_mul_KS", KIND_MUL, VARIANT_KS, {"mul_to_out_kronecker", NULL}},
    {"_fmpz_poly_mullow_KS", KIND_MULLOW, VARIANT_KS, {"mul_truncated_to_out_kronecker", NULL}},
    {"_fmpz_poly_mulmid_KS", KIND_MULMID, VARIANT_KS, {"mul_middle_to_out_kronecker", NULL}},
    {"_fmpz_poly_sqr_KS", KIND_SQR, VARIANT_KS, {"square_to_out_kronecker", NULL}},
    {"_fmpz_poly_sqrlow_KS", KIND_SQRLOW, VARIANT_KS, {"square_truncated_to_out_kronecker", NULL}},
    {"_fmpz_poly_mul_mid_default_mpn_ctx", KIND_MULMID, VARIANT_FFT,
     {"mul_middle_to_out_fft", NULL}},
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

int
run__fmpz_poly_mul_karatsuba(const char * arg)
{
    return run(arg, 11);
}

int
run__fmpz_poly_mullow_karatsuba(const char * arg)
{
    return run(arg, 12);
}

int
run__fmpz_poly_mullow_karatsuba_n(const char * arg)
{
    return run(arg, 13);
}

int
run__fmpz_poly_mulhigh_karatsuba_n(const char * arg)
{
    return run(arg, 14);
}

int
run__fmpz_poly_mulhigh(const char * arg)
{
    return run(arg, 15);
}

int
run__fmpz_poly_sqr_karatsuba(const char * arg)
{
    return run(arg, 16);
}

int
run__fmpz_poly_sqrlow_karatsuba(const char * arg)
{
    return run(arg, 17);
}

int
run__fmpz_poly_sqrlow_karatsuba_n(const char * arg)
{
    return run(arg, 18);
}

int
run__fmpz_poly_mul_KS(const char * arg)
{
    return run(arg, 19);
}

int
run__fmpz_poly_mullow_KS(const char * arg)
{
    return run(arg, 20);
}

int
run__fmpz_poly_mulmid_KS(const char * arg)
{
    return run(arg, 21);
}

int
run__fmpz_poly_sqr_KS(const char * arg)
{
    return run(arg, 22);
}

int
run__fmpz_poly_sqrlow_KS(const char * arg)
{
    return run(arg, 23);
}

int
run__fmpz_poly_mul_mid_default_mpn_ctx(const char * arg)
{
    return run(arg, 24);
}
