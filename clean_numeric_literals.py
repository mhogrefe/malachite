import collections
import sys

def process_number(number):
    cleaned_number = ''
    m3 = len(number) % 3
    if m3 == 0:
        i = 0
    elif m3 == 1:
        i = 2
    elif m3 == 2:
        i = 1
    for c in number:
        if i != 0 and i % 3 == 0:
            cleaned_number += '_'
        cleaned_number += c
        i += 1
    return cleaned_number


with open(sys.argv[1]) as f:
    in_quotes = False
    reading_number = False
    current_number = ''
    for line in f.readlines():
        replacements = collections.OrderedDict()
        line = line.rstrip()
        for (i, c) in enumerate(line):
            if c == '"':
                in_quotes = not in_quotes
            elif not in_quotes:
                if reading_number:
                    if c.isdigit():
                        current_number += c
                    else:
                        reading_number = False
                        number_end_index = i
                        if len(current_number) > 3:
                            replacements[(number_start_index, number_end_index)] = process_number(current_number)
                        current_number = ''
                else:
                    if c.isdigit():
                        current_number += c
                        reading_number = True
                        number_start_index = i
        if reading_number:
            reading_number = False
            number_end_index = len(line)
            if len(current_number) > 3:
                replacements[(number_start_index, number_end_index)] = process_number(current_number)
            current_number = ''
        if replacements:
            cleaned_line = ''
            previous_index = 0
            for (number_start_index, number_end_index), number in replacements.items():
                cleaned_line += line[previous_index:number_start_index]
                cleaned_line += number
                previous_index = number_end_index
            if previous_index != len(line):
                cleaned_line += line[previous_index:]
            print cleaned_line
        else:
            print line

