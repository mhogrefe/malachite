/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `multi_crt([m, ...], [V, ...]) = Some(R)` and `multi_balanced_crt(...)` lines, where the
    `V` are the residue vectors, one per modulus, against _fmpz_vec_multi_CRT_ui at signs 0 and 1.
    Any other nonempty line is an error, as is an input with no lines at all.

    FLINT aborts on moduli it cannot use rather than reporting them, so a `= None` line is checked
    here instead: it must appear exactly when the moduli are unusable under Malachite's documented
    conditions (with one modulus, if it is 0; with more, if any is less than 2 or two are not
    coprime), and every other line must be `Some` and agree with FLINT.
*/

#include <string.h>

#include <flint/fmpz_vec.h>

#include "oracle.h"

static long checked;

/* Parses `[a, b, c]` of ulongs at `*pp` strictly, advancing past the closing bracket. The caller
   frees the array, which is allocated even when empty. */
static int
parse_ulong_list(const char ** pp, ulong ** out, slong * len)
{
    const char * p = *pp;
    *len = 0;
    *out = flint_malloc(sizeof(ulong) * (strlen(p) / 2 + 1));
    if (*p != '[')
    {
        return 0;
    }
    p++;
    if (*p == ']')
    {
        *pp = p + 1;
        return 1;
    }
    while (1)
    {
        size_t digits = strspn(p, "0123456789");
        if (digits == 0 || digits > 20 || (digits > 1 && *p == '0'))
        {
            return 0;
        }
        char buf[21];
        memcpy(buf, p, digits);
        buf[digits] = '\0';
        fmpz_t x;
        fmpz_init(x);
        fmpz_set_str(x, buf, 10);
        int fits = fmpz_abs_fits_ui(x);
        (*out)[(*len)++] = fits ? fmpz_get_ui(x) : 0;
        fmpz_clear(x);
        if (!fits)
        {
            return 0;
        }
        p += digits;
        if (*p == ']')
        {
            *pp = p + 1;
            return 1;
        }
        if (strncmp(p, ", ", 2) != 0)
        {
            return 0;
        }
        p += 2;
    }
}

/* Copies the parenthesized vector at `*pp` into a new string and parses it, advancing past the
   closing parenthesis. */
static fmpz *
parse_vector(const char ** pp, slong * len)
{
    const char * close = strchr(*pp, ')');
    *len = 0;
    if (**pp != '(' || close == NULL)
    {
        return NULL;
    }
    size_t n = close - *pp + 1;
    char * buf = flint_malloc(n + 1);
    memcpy(buf, *pp, n);
    buf[n] = '\0';
    fmpz * v = fmpz_vec_set_str_malachite(len, buf);
    flint_free(buf);
    *pp = close + 1;
    return v;
}

/* Euclid's algorithm. FLINT's n_gcd is an inline wrapper around a GMP routine, and the oracle
   does not link GMP directly. */
static ulong
word_gcd(ulong a, ulong b)
{
    while (b != 0)
    {
        ulong t = a % b;
        a = b;
        b = t;
    }
    return a;
}

static int
moduli_usable(const ulong * moduli, slong n)
{
    if (n == 1)
    {
        return moduli[0] != 0;
    }
    for (slong i = 0; i < n; i++)
    {
        if (moduli[i] < 2)
        {
            return 0;
        }
        for (slong j = 0; j < i; j++)
        {
            if (word_gcd(moduli[i], moduli[j]) != 1)
            {
                return 0;
            }
        }
    }
    return 1;
}

static int
check_multi_crt_line(char * line, int line_number)
{
    line[strcspn(line, "\r\n")] = '\0';
    if (line[0] == '\0')
    {
        return 0;
    }
    int sign;
    const char * p;
    if (strncmp(line, "multi_crt(", 10) == 0)
    {
        sign = 0;
        p = line + 10;
    }
    else if (strncmp(line, "multi_balanced_crt(", 19) == 0)
    {
        sign = 1;
        p = line + 19;
    }
    else
    {
        flint_printf("error in _fmpz_vec_multi_CRT_ui test, line %d: unrecognized line\n",
                     line_number);
        return 1;
    }
    checked++;
    int ok = 1;
    int result = 0;
    ulong * moduli;
    slong n;
    ok = parse_ulong_list(&p, &moduli, &n) && n > 0 && strncmp(p, ", [", 3) == 0;
    p += ok ? 3 : 0;
    fmpz ** residues = flint_calloc(n > 0 ? n : 1, sizeof(fmpz *));
    slong * lens = flint_calloc(n > 0 ? n : 1, sizeof(slong));
    for (slong j = 0; ok && j < n; j++)
    {
        residues[j] = parse_vector(&p, lens + j);
        ok = residues[j] != NULL && lens[j] == lens[0];
        if (ok)
        {
            const char * sep = j + 1 < n ? ", " : "]) = ";
            ok = strncmp(p, sep, strlen(sep)) == 0;
            p += strlen(sep);
        }
    }
    slong len = n > 0 ? lens[0] : 0;
    int expect_some = 0;
    fmpz * expected = NULL;
    slong expected_len = 0;
    if (ok)
    {
        if (strcmp(p, "None") == 0)
        {
            expect_some = 0;
        }
        else if (strncmp(p, "Some(", 5) == 0 && p[strlen(p) - 1] == ')')
        {
            expect_some = 1;
            p += 5;
            expected = parse_vector(&p, &expected_len);
            ok = expected != NULL && strcmp(p, ")") == 0;
        }
        else
        {
            ok = 0;
        }
    }
    if (!ok)
    {
        flint_printf("error in _fmpz_vec_multi_CRT_ui test, line %d: unreadable input\n",
                     line_number);
        result = 1;
    }
    else if (!moduli_usable(moduli, n))
    {
        if (expect_some)
        {
            flint_printf("error in _fmpz_vec_multi_CRT_ui test, line %d: the moduli are "
                         "unusable, but the result is Some\n", line_number);
            result = 1;
        }
    }
    else if (!expect_some)
    {
        flint_printf("error in _fmpz_vec_multi_CRT_ui test, line %d: the moduli are usable, but "
                     "the result is None\n", line_number);
        result = 1;
    }
    else
    {
        /* FLINT reads word residues; every one must fit, and Malachite requires them reduced. */
        ulong ** words = flint_malloc(n * sizeof(ulong *));
        for (slong j = 0; j < n; j++)
        {
            words[j] = flint_malloc((len > 0 ? len : 1) * sizeof(ulong));
            for (slong i = 0; i < len; i++)
            {
                if (fmpz_sgn(residues[j] + i) < 0 || !fmpz_abs_fits_ui(residues[j] + i)
                    || fmpz_get_ui(residues[j] + i) >= moduli[j])
                {
                    ok = 0;
                }
                words[j][i] = ok ? fmpz_get_ui(residues[j] + i) : 0;
            }
        }
        if (!ok)
        {
            flint_printf("error in _fmpz_vec_multi_CRT_ui test, line %d: a residue is not "
                         "reduced\n", line_number);
            result = 1;
        }
        else
        {
            fmpz * r = _fmpz_vec_init(len);
            _fmpz_vec_multi_CRT_ui(r, (nn_srcptr *) words, len, moduli, n, sign);
            if (expected_len != len || !_fmpz_vec_equal(r, expected, len))
            {
                flint_printf("error in _fmpz_vec_multi_CRT_ui test, line %d. FLINT: ",
                             line_number);
                fmpz_vec_print_malachite(r, len);
                flint_printf("\n");
                result = 1;
            }
            _fmpz_vec_clear(r, len);
        }
        for (slong j = 0; j < n; j++)
        {
            flint_free(words[j]);
        }
        flint_free(words);
    }
    for (slong j = 0; j < n; j++)
    {
        if (residues[j] != NULL)
        {
            _fmpz_vec_clear(residues[j], lens[j]);
        }
    }
    if (expected != NULL)
    {
        _fmpz_vec_clear(expected, expected_len);
    }
    flint_free(residues);
    flint_free(lens);
    flint_free(moduli);
    return result;
}

int
run__fmpz_vec_multi_CRT_ui(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_multi_crt_line);
    return result != 0 ? result : require_some_lines("_fmpz_vec_multi_CRT_ui", checked);
}
