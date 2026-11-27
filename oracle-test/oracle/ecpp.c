/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs Malachite's elliptic curve primality certificates against FLINT's ecpp module:

    - ecpp_verify: `ecpp_verify(n, <Pari certificate>) = true|false` against ecpp_verify, on the
      certificate that ecpp_cert_set_str reads from the Pari text.
    - ecpp_verify_step: `ecpp_verify_step(EcppStep { n: .., d: .., a: .., b: .., m: .., q: .., x: ..,
      y: .. }) = true|false` against ecpp_verify_step. FLINT assumes that a, b, x, and y are reduced
      mod n and Malachite reduces them, so they are reduced here first (when n > 3; FLINT rejects
      smaller n before using them).
    - ecpp_cert_get_str: `ecpp_cert_get_str(<Pari certificate>, Flint) = "<text>"`, with the text
      escaped as by Rust's Debug, against ecpp_cert_get_str in the FLINT format.
    - ecpp_cert_set_str: `ecpp_cert_set_str("<text>") = None` or `= Some(EcppCertificate { steps:
      [EcppStep { .. }, ..] })` against ecpp_cert_set_str. Malachite rejects some texts that FLINT
      accepts, which are allowed to differ: those with a negative n, a, b, m, q, x, or y (FLINT keeps
      them as signed integers) or a D that does not fit in 64 bits.

    All four modes are strict: a nonempty line of any other shape is an error, and so is an input in
    which no line was checked.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/fmpz.h>
#include <flint/ecpp.h>

#include "oracle.h"

static long ecpp_checked = 0;

static void
chomp(char * line)
{
    size_t len = strlen(line);
    while (len > 0 && (line[len - 1] == '\n' || line[len - 1] == '\r'))
    {
        line[--len] = '\0';
    }
}

static int
cut_prefix(char ** line, const char * prefix)
{
    size_t len = strlen(prefix);
    if (strncmp(*line, prefix, len) != 0)
    {
        return 0;
    }
    *line += len;
    return 1;
}

/* Reads an optionally negative decimal integer at *p, advancing *p. */
static int
read_int_at(fmpz_t x, const char ** p)
{
    const char * q = *p;
    if (*q == '-')
    {
        q++;
    }
    if (*q < '0' || *q > '9')
    {
        return 0;
    }
    while (*q >= '0' && *q <= '9')
    {
        q++;
    }
    size_t l = q - *p;
    char * buf = malloc(l + 1);
    memcpy(buf, *p, l);
    buf[l] = '\0';
    int ok = fmpz_set_str(x, buf, 10) == 0;
    free(buf);
    *p = q;
    return ok;
}

static int
read_natural_whole(fmpz_t x, const char * s)
{
    if (s[0] == '-')
    {
        return 0;
    }
    const char * p = s;
    return read_int_at(x, &p) && *p == '\0';
}

/* Undoes Rust's Debug escaping of a string literal at s (which starts at the opening quote);
   returns the string, or NULL; *end is set past the closing quote. */
static char *
unescape(const char * s, const char ** end)
{
    if (*s != '"')
    {
        return NULL;
    }
    s++;
    char * out = malloc(strlen(s) + 1);
    size_t k = 0;
    while (*s && *s != '"')
    {
        if (*s == '\\')
        {
            s++;
            switch (*s)
            {
                case 'n': out[k++] = '\n'; break;
                case 't': out[k++] = '\t'; break;
                case 'r': out[k++] = '\r'; break;
                case '0': out[k++] = '\0'; break;
                case '\\': out[k++] = '\\'; break;
                case '"': out[k++] = '"'; break;
                case '\'': out[k++] = '\''; break;
                default: free(out); return NULL;
            }
            s++;
        }
        else
        {
            out[k++] = *s++;
        }
    }
    if (*s != '"')
    {
        free(out);
        return NULL;
    }
    out[k] = '\0';
    *end = s + 1;
    return out;
}

static const char * STEP_LABELS[8] = {"n: ", "d: ", "a: ", "b: ", "m: ", "q: ", "x: ", "y: "};

/* Reads `EcppStep { n: .., d: .., a: .., b: .., m: .., q: .., x: .., y: .. }` at *p into s. */
static int
read_step(ecpp_step_struct * st, const char ** p)
{
    const char * q = *p;
    if (strncmp(q, "EcppStep { ", 11) != 0)
    {
        return 0;
    }
    q += 11;
    fmpz_t d;
    fmpz_init(d);
    fmpz * fields[8] = {st->n, d, st->a, st->b, st->m, st->q, st->x, st->y};
    int ok = 1;
    for (int f = 0; f < 8 && ok; f++)
    {
        if (f > 0)
        {
            ok = strncmp(q, ", ", 2) == 0;
            q += 2;
        }
        ok = ok && strncmp(q, STEP_LABELS[f], 3) == 0;
        if (ok)
        {
            q += 3;
            ok = read_int_at(fields[f], &q);
        }
    }
    ok = ok && strncmp(q, " }", 2) == 0 && fmpz_fits_si(d);
    if (ok)
    {
        st->D = fmpz_get_si(d);
        *p = q + 2;
    }
    fmpz_clear(d);
    return ok;
}

static int
read_bool(int * b, const char * s)
{
    if (strcmp(s, "true") == 0)
    {
        *b = 1;
        return 1;
    }
    if (strcmp(s, "false") == 0)
    {
        *b = 0;
        return 1;
    }
    return 0;
}

static int
fail_line(const char * mode, int line_number, const char * what, const char * line)
{
    flint_printf("error in %s test, line %d: %s\n  %s\n", mode, line_number, what, line);
    return 1;
}

static int
check_ecpp_verify_line(char * line, int line_number)
{
    chomp(line);
    if (line[0] == '\0')
    {
        return 0;
    }
    char * p = line;
    char * eq = strstr(line, ") = ");
    char * comma = strstr(line, ", [");
    int expected;
    if (!cut_prefix(&p, "ecpp_verify(") || eq == NULL || comma == NULL || comma > eq
        || !read_bool(&expected, eq + 4))
    {
        return fail_line("ecpp_verify", line_number, "unrecognized line", line);
    }
    *comma = '\0';
    *eq = '\0';
    fmpz_t n;
    fmpz_init(n);
    ecpp_cert_t cert;
    ecpp_cert_init(cert);
    int result = 0;
    if (!read_natural_whole(n, p) || !ecpp_cert_set_str(cert, comma + 2))
    {
        *comma = ',';
        result = fail_line("ecpp_verify", line_number, "unreadable n or certificate", line);
    }
    else
    {
        int flint_output = ecpp_verify(cert, n);
        ecpp_checked += 1;
        if (flint_output != expected)
        {
            flint_printf("error in ecpp_verify test, line %d. FLINT: %d, Malachite: %d\n",
                line_number, flint_output, expected);
            result = 1;
        }
    }
    ecpp_cert_clear(cert);
    fmpz_clear(n);
    return result;
}

static void
reduce_step(ecpp_step_struct * st)
{
    if (fmpz_cmp_ui(st->n, 3) > 0)
    {
        fmpz_mod(st->a, st->a, st->n);
        fmpz_mod(st->b, st->b, st->n);
        fmpz_mod(st->x, st->x, st->n);
        fmpz_mod(st->y, st->y, st->n);
    }
}

static int
check_ecpp_verify_step_line(char * line, int line_number)
{
    chomp(line);
    if (line[0] == '\0')
    {
        return 0;
    }
    char * p = line;
    char * eq = strstr(line, ") = ");
    int expected;
    if (!cut_prefix(&p, "ecpp_verify_step(") || eq == NULL || !read_bool(&expected, eq + 4))
    {
        return fail_line("ecpp_verify_step", line_number, "unrecognized line", line);
    }
    ecpp_cert_t cert;
    ecpp_cert_init(cert);
    ecpp_step_struct * st = ecpp_cert_push(cert);
    const char * q = p;
    int result = 0;
    if (!read_step(st, &q) || q != eq)
    {
        result = fail_line("ecpp_verify_step", line_number, "unreadable step", line);
    }
    else
    {
        reduce_step(st);
        int flint_output = ecpp_verify_step(st);
        ecpp_checked += 1;
        if (flint_output != expected)
        {
            flint_printf("error in ecpp_verify_step test, line %d. FLINT: %d, Malachite: %d\n  %s\n",
                line_number, flint_output, expected, line);
            result = 1;
        }
    }
    ecpp_cert_clear(cert);
    return result;
}

static int
check_ecpp_cert_get_str_line(char * line, int line_number)
{
    chomp(line);
    if (line[0] == '\0')
    {
        return 0;
    }
    char * p = line;
    char * mid = strstr(line, ", Flint) = \"");
    if (!cut_prefix(&p, "ecpp_cert_get_str(") || mid == NULL)
    {
        return fail_line("ecpp_cert_get_str", line_number, "unrecognized line", line);
    }
    const char * end;
    char * expected = unescape(mid + 11, &end);
    *mid = '\0';
    ecpp_cert_t cert;
    ecpp_cert_init(cert);
    int result = 0;
    if (expected == NULL || *end != '\0' || !ecpp_cert_set_str(cert, p))
    {
        result = fail_line("ecpp_cert_get_str", line_number, "unreadable line", p);
    }
    else
    {
        char * flint_output = ecpp_cert_get_str(cert, ECPP_CERT_FORMAT_FLINT);
        ecpp_checked += 1;
        if (strcmp(flint_output, expected) != 0)
        {
            flint_printf("error in ecpp_cert_get_str test, line %d. FLINT:\n%s\nMalachite:\n%s\n",
                line_number, flint_output, expected);
            result = 1;
        }
        flint_free(flint_output);
    }
    free(expected);
    ecpp_cert_clear(cert);
    return result;
}

/* Whether FLINT read a certificate that Malachite is allowed to reject: one with a negative field
   or a D that does not fit in 64 bits (slong is 64 bits on the platforms tested). */
static int
malachite_may_reject(const ecpp_cert_t cert)
{
    for (slong i = 0; i < cert->num; i++)
    {
        const ecpp_step_struct * st = cert->steps + i;
        if (fmpz_sgn(st->n) < 0 || fmpz_sgn(st->a) < 0 || fmpz_sgn(st->b) < 0
            || fmpz_sgn(st->m) < 0 || fmpz_sgn(st->q) < 0 || fmpz_sgn(st->x) < 0
            || fmpz_sgn(st->y) < 0)
        {
            return 1;
        }
    }
    return 0;
}

static int
check_ecpp_cert_set_str_line(char * line, int line_number)
{
    chomp(line);
    if (line[0] == '\0')
    {
        return 0;
    }
    char * p = line;
    if (!cut_prefix(&p, "ecpp_cert_set_str("))
    {
        return fail_line("ecpp_cert_set_str", line_number, "unrecognized line", line);
    }
    const char * end;
    char * input = unescape(p, &end);
    if (input == NULL || strncmp(end, ") = ", 4) != 0)
    {
        free(input);
        return fail_line("ecpp_cert_set_str", line_number, "unrecognized line", line);
    }
    const char * out = end + 4;
    ecpp_cert_t flint_cert, mal_cert;
    ecpp_cert_init(flint_cert);
    ecpp_cert_init(mal_cert);
    int result = 0;
    /* a NUL inside the input ends FLINT's string there; Malachite's escaping never produces one */
    int flint_ok = ecpp_cert_set_str(flint_cert, input);
    if (strcmp(out, "None") == 0)
    {
        ecpp_checked += 1;
        if (flint_ok && !malachite_may_reject(flint_cert))
        {
            /* a D too large for 64 bits is truncated by FLINT, and rejected by Malachite; FLINT's
               D can't show that, so look for an overlong D in the input */
            const char * d = strstr(input, "D =");
            int long_d = 0;
            while (d != NULL)
            {
                const char * q = d + 3;
                while (*q == ' ' || *q == '\n' || *q == '\t' || *q == '\r') q++;
                if (*q == '-' || *q == '+') q++;
                const char * start = q;
                while (*q >= '0' && *q <= '9') q++;
                if (q - start >= 19) long_d = 1;
                d = strstr(q, "D =");
            }
            if (!long_d)
            {
                result = fail_line("ecpp_cert_set_str", line_number,
                    "FLINT read a certificate but Malachite returned None", line);
            }
        }
    }
    else
    {
        const char * q = out;
        int ok = cut_prefix((char **) &q, "Some(EcppCertificate { steps: [");
        while (ok && strncmp(q, "] })", 4) != 0)
        {
            if (mal_cert->num > 0)
            {
                ok = strncmp(q, ", ", 2) == 0;
                q += 2;
            }
            ok = ok && read_step(ecpp_cert_push(mal_cert), &q);
        }
        if (!ok || strcmp(q, "] })") != 0)
        {
            result = fail_line("ecpp_cert_set_str", line_number, "unreadable result", line);
        }
        else
        {
            ecpp_checked += 1;
            int same = flint_ok && flint_cert->num == mal_cert->num;
            for (slong i = 0; same && i < mal_cert->num; i++)
            {
                const ecpp_step_struct * s = flint_cert->steps + i;
                const ecpp_step_struct * t = mal_cert->steps + i;
                same = fmpz_equal(s->n, t->n) && s->D == t->D && fmpz_equal(s->a, t->a)
                    && fmpz_equal(s->b, t->b) && fmpz_equal(s->m, t->m)
                    && fmpz_equal(s->q, t->q) && fmpz_equal(s->x, t->x)
                    && fmpz_equal(s->y, t->y);
            }
            if (!same)
            {
                result = fail_line("ecpp_cert_set_str", line_number,
                    flint_ok ? "the certificates differ" : "FLINT rejected the text", line);
            }
        }
    }
    free(input);
    ecpp_cert_clear(flint_cert);
    ecpp_cert_clear(mal_cert);
    return result;
}

#define ECPP_RUNNER(name, checker) \
    int \
    run_##name(const char * arg) \
    { \
        ecpp_checked = 0; \
        int result = for_each_line(arg, checker); \
        if (result != 0) \
        { \
            return result; \
        } \
        if (require_some_lines(#name, ecpp_checked)) \
        { \
            return 1; \
        } \
        flint_printf(#name ": %ld lines\n", ecpp_checked); \
        return 0; \
    }

ECPP_RUNNER(ecpp_verify, check_ecpp_verify_line)
ECPP_RUNNER(ecpp_verify_step, check_ecpp_verify_step_line)
ECPP_RUNNER(ecpp_cert_get_str, check_ecpp_cert_get_str_line)
ECPP_RUNNER(ecpp_cert_set_str, check_ecpp_cert_set_str_line)
