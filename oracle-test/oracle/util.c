/*
    Copyright © 2026 Mikhail Hogrefe

    Uses code adapted from the FLINT Library examples.

        Copyright © 2022 Fredrik Johansson

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Line iteration and parsing helpers shared by the oracle modes. The demos print one
    `input = output` line per case; these helpers split such lines into their pieces, leaving the
    numeral conversions (fmpz_set_str, strtoul) to each mode.
*/

#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <flint/fmpz_poly.h>
#include <flint/fmpz_vec.h>

#include "oracle.h"

int
for_each_line(const char * path, int (* handler)(char * line, int line_number))
{
    FILE * fp = fopen(path, "r");
    if (fp == NULL)
    {
        flint_printf("cannot open %s\n", path);
        return 2;
    }
    char * line = NULL;
    size_t len = 0;
    int line_number = 1;
    int result = 0;
    while (getline(&line, &len, fp) != -1)
    {
        result = handler(line, line_number);
        if (result != 0)
        {
            break;
        }
        line_number += 1;
    }
    free(line);
    fclose(fp);
    return result;
}

int
split_method_call(char * line, const char * method, char ** pieces, int n_pieces, char ** rest)
{
    char * dot = strstr(line, method);
    if (dot == NULL)
    {
        return 0;
    }
    *dot = '\0';
    pieces[0] = line;
    char * p = dot + strlen(method);
    for (int i = 1; i < n_pieces; i++)
    {
        pieces[i] = p;
        int last = i == n_pieces - 1;
        char * sep = last ? strchr(p, ')') : strstr(p, ", ");
        if (sep == NULL)
        {
            return 0;
        }
        *sep = '\0';
        p = sep + (last ? 1 : 2);
    }
    *rest = p;
    return 1;
}

int
parse_option_fmpz(char * rest, fmpz_t expected)
{
    char * some = strstr(rest, "Some(");
    if (some == NULL)
    {
        return 0;
    }
    char * start = some + strlen("Some(");
    char * end = strchr(start, ')');
    *end = '\0';
    fmpz_set_str(expected, start, 10);
    return 1;
}

int
parse_option_ulong(const char * rest, ulong * expected)
{
    const char * some = strstr(rest, "Some(");
    if (some == NULL)
    {
        return 0;
    }
    char * end;
    *expected = strtoul(some + strlen("Some("), &end, 10);
    return 1;
}

int
fmpz_poly_set_str_malachite(fmpz_poly_t poly, const char * s)
{
    fmpz_poly_zero(poly);
    if (strcmp(s, "0") == 0)
    {
        return 1;
    }
    size_t n = strlen(s);
    char * buf = flint_malloc(n + 1);
    fmpz_t c, old;
    fmpz_init(c);
    fmpz_init(old);
    int ok = 1;
    int first = 1;
    const char * p = s;
    while (ok && *p != '\0')
    {
        int negative = 0;
        if (*p == '-')
        {
            negative = 1;
            p++;
        }
        else if (*p == '+')
        {
            /* A leading term carries no plus sign. */
            ok = !first;
            p++;
        }
        else if (!first)
        {
            ok = 0;
        }
        int has_x;
        if (isdigit((unsigned char) *p))
        {
            size_t digits = strspn(p, "0123456789");
            memcpy(buf, p, digits);
            buf[digits] = '\0';
            fmpz_set_str(c, buf, 10);
            p += digits;
            /* A printed coefficient is never zero. */
            ok = ok && !fmpz_is_zero(c);
            has_x = *p == '*';
            if (has_x)
            {
                p++;
                ok = ok && *p == 'x';
                p++;
            }
        }
        else if (*p == 'x')
        {
            fmpz_one(c);
            has_x = 1;
            p++;
        }
        else
        {
            ok = 0;
            break;
        }
        ulong exponent = 0;
        if (has_x)
        {
            exponent = 1;
            if (*p == '^')
            {
                p++;
                ok = ok && isdigit((unsigned char) *p);
                char * end;
                exponent = strtoul(p, &end, 10);
                p = end;
            }
        }
        if (negative)
        {
            fmpz_neg(c, c);
        }
        fmpz_poly_get_coeff_fmpz(old, poly, exponent);
        fmpz_add(old, old, c);
        fmpz_poly_set_coeff_fmpz(poly, exponent, old);
        first = 0;
    }
    fmpz_clear(c);
    fmpz_clear(old);
    flint_free(buf);
    return ok;
}

int
fmpq_poly_set_str_malachite(fmpq_poly_t poly, const char * s)
{
    fmpq_poly_zero(poly);
    if (strcmp(s, "0") == 0)
    {
        return 1;
    }
    size_t n = strlen(s);
    char * buf = flint_malloc(n + 1);
    fmpq_t c, old;
    fmpq_init(c);
    fmpq_init(old);
    int ok = 1;
    int first = 1;
    const char * p = s;
    while (ok && *p != '\0')
    {
        int negative = 0;
        if (*p == '-')
        {
            negative = 1;
            p++;
        }
        else if (*p == '+')
        {
            /* A leading term carries no plus sign. */
            ok = !first;
            p++;
        }
        else if (!first)
        {
            ok = 0;
        }
        int has_x;
        if (isdigit((unsigned char) *p))
        {
            /* A coefficient, `a` or `a/b`. */
            size_t digits = strspn(p, "0123456789");
            if (p[digits] == '/')
            {
                digits += 1 + strspn(p + digits + 1, "0123456789");
            }
            memcpy(buf, p, digits);
            buf[digits] = '\0';
            ok = ok && fmpq_set_str(c, buf, 10) == 0;
            p += digits;
            /* A printed coefficient is never zero, and is in lowest terms. */
            if (ok)
            {
                fmpq_t canonical;
                fmpq_init(canonical);
                fmpq_set(canonical, c);
                fmpq_canonicalise(canonical);
                ok = !fmpq_is_zero(c) && fmpq_equal(canonical, c);
                fmpq_clear(canonical);
            }
            has_x = *p == '*';
            if (has_x)
            {
                p++;
                ok = ok && *p == 'x';
                p++;
            }
        }
        else if (*p == 'x')
        {
            fmpq_one(c);
            has_x = 1;
            p++;
        }
        else
        {
            ok = 0;
            break;
        }
        ulong exponent = 0;
        if (has_x)
        {
            exponent = 1;
            if (*p == '^')
            {
                p++;
                ok = ok && isdigit((unsigned char) *p);
                char * end;
                exponent = strtoul(p, &end, 10);
                p = end;
            }
        }
        if (negative)
        {
            fmpq_neg(c, c);
        }
        fmpq_poly_get_coeff_fmpq(old, poly, exponent);
        fmpq_add(old, old, c);
        fmpq_poly_set_coeff_fmpq(poly, exponent, old);
        first = 0;
    }
    fmpq_clear(c);
    fmpq_clear(old);
    flint_free(buf);
    return ok;
}

/* Cuts `line` at `needle`, returning the text after it, or NULL if the needle is absent. */
static char *
cut_after(char * line, const char * needle)
{
    char * at = strstr(line, needle);
    if (at == NULL)
    {
        return NULL;
    }
    *at = '\0';
    return at + strlen(needle);
}

int
split_polynomial_scalar_line(char * line, const char * method, const char * assign_method,
                             const char * op, char ** receiver, char ** arg, char ** result)
{
    line[strcspn(line, "\r\n")] = '\0';
    char needle[64];
    if (strncmp(line, "p := ", 5) == 0)
    {
        /* `p := P; p.assign_method(M); p = R` or `p := P; p op= M; p = R` */
        if (assign_method != NULL)
        {
            snprintf(needle, sizeof(needle), "; p.%s(", assign_method);
            if (strstr(line, needle) != NULL)
            {
                *receiver = line + 5;
                *arg = cut_after(line, needle);
                *result = cut_after(*arg, "); p = ");
                return *result != NULL;
            }
        }
        if (op != NULL)
        {
            snprintf(needle, sizeof(needle), "; p %s= ", op);
            if (strstr(line, needle) != NULL)
            {
                *receiver = line + 5;
                *arg = cut_after(line, needle);
                *result = cut_after(*arg, "; p = ");
                return *result != NULL;
            }
        }
        return 0;
    }
    if (method != NULL)
    {
        /* `(P).method(M) = R` or `(&(P)).method(M) = R` */
        snprintf(needle, sizeof(needle), ").%s(", method);
        char * close = strstr(line, needle);
        if (close != NULL)
        {
            if (strncmp(line, "(&(", 3) == 0)
            {
                *receiver = line + 3;
                close[-1] = '\0';
            }
            else if (line[0] == '(')
            {
                *receiver = line + 1;
            }
            else
            {
                return 0;
            }
            *arg = cut_after(close, needle);
            *result = cut_after(*arg, ") = ");
            return *result != NULL;
        }
    }
    if (op != NULL)
    {
        /* `(P) op M = R` or `&(P) op M = R` */
        snprintf(needle, sizeof(needle), ") %s ", op);
        char * close = strstr(line, needle);
        if (close != NULL)
        {
            if (strncmp(line, "&(", 2) == 0)
            {
                *receiver = line + 2;
            }
            else if (line[0] == '(')
            {
                *receiver = line + 1;
            }
            else
            {
                return 0;
            }
            *arg = cut_after(close, needle);
            *result = cut_after(*arg, " = ");
            return *result != NULL;
        }
    }
    return 0;
}

fmpz *
fmpz_vec_set_str_malachite(slong * len, const char * s)
{
    *len = 0;
    if (*s != '(')
    {
        return NULL;
    }
    s++;
    slong cap = 4;
    fmpz * vec = _fmpz_vec_init(cap);
    size_t n = strlen(s);
    char * buf = flint_malloc(n + 1);
    int ok = 1;
    if (*s == ')')
    {
        s++;
    }
    else
    {
        while (1)
        {
            /* An optional minus sign, then digits with no leading zeros other than 0 itself. */
            size_t sign = *s == '-';
            size_t digits = strspn(s + sign, "0123456789");
            if (digits == 0 || (digits > 1 && s[sign] == '0') || (sign && digits == 1
                                                                   && s[1] == '0'))
            {
                ok = 0;
                break;
            }
            memcpy(buf, s, sign + digits);
            buf[sign + digits] = '\0';
            if (*len == cap)
            {
                fmpz * bigger = _fmpz_vec_init(2 * cap);
                _fmpz_vec_swap(bigger, vec, cap);
                _fmpz_vec_clear(vec, cap);
                vec = bigger;
                cap *= 2;
            }
            fmpz_set_str(vec + *len, buf, 10);
            (*len)++;
            s += sign + digits;
            if (*s == ')')
            {
                s++;
                break;
            }
            if (strncmp(s, ", ", 2) != 0)
            {
                ok = 0;
                break;
            }
            s += 2;
        }
    }
    flint_free(buf);
    if (!ok || *s != '\0')
    {
        _fmpz_vec_clear(vec, cap);
        *len = 0;
        return NULL;
    }
    /* The caller frees exactly `*len` entries, so shed the spare capacity. */
    fmpz * result = _fmpz_vec_init(*len);
    _fmpz_vec_swap(result, vec, *len);
    _fmpz_vec_clear(vec, cap);
    return result;
}

void
fmpz_vec_print_malachite(const fmpz * vec, slong len)
{
    flint_printf("(");
    for (slong i = 0; i < len; i++)
    {
        if (i != 0)
        {
            flint_printf(", ");
        }
        fmpz_print(vec + i);
    }
    flint_printf(")");
}

int
split_vector_scalar_line(char * line, const char * method, const char * assign_method,
                         char ** receiver, char ** arg, char ** result)
{
    line[strcspn(line, "\r\n")] = '\0';
    char needle[64];
    if (strncmp(line, "v := ", 5) == 0)
    {
        /* `v := V; v.assign_method(M); v = R` */
        if (assign_method == NULL)
        {
            return 0;
        }
        snprintf(needle, sizeof(needle), "; v.%s(", assign_method);
        if (strstr(line, needle) == NULL)
        {
            return 0;
        }
        *receiver = line + 5;
        *arg = cut_after(line, needle);
        *result = cut_after(*arg, "); v = ");
        return *result != NULL;
    }
    if (method == NULL)
    {
        return 0;
    }
    /* `V.method(M) = R` or `(&V).method(M) = R`, where V is parenthesized and holds no other
       parentheses. */
    if (strncmp(line, "(&(", 3) == 0)
    {
        snprintf(needle, sizeof(needle), ")).%s(", method);
        char * close = strstr(line, needle);
        if (close == NULL)
        {
            return 0;
        }
        *receiver = line + 2;
        close[1] = '\0';
        *arg = close + strlen(needle);
    }
    else if (line[0] == '(')
    {
        snprintf(needle, sizeof(needle), ").%s(", method);
        char * close = strstr(line, needle);
        if (close == NULL)
        {
            return 0;
        }
        *receiver = line;
        close[1] = '\0';
        *arg = close + strlen(needle);
    }
    else
    {
        return 0;
    }
    *result = cut_after(*arg, ") = ");
    return *result != NULL;
}

int
require_some_lines(const char * name, long checked)
{
    if (checked == 0)
    {
        flint_printf("error in %s test: no line of the input had the expected shape\n", name);
        return 1;
    }
    return 0;
}
