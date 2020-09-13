import os
import subprocess

def try_building(include_rust_wheels):
    try:
        subprocess.check_call(['cargo', 'check', '--all'], cwd = 'malachite-base-test-util')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--all'], cwd = 'malachite-nz-test-util')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--all', '--features', '32_bit_limbs'], cwd = 'malachite-nz-test-util')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--all'], cwd = 'malachite-base')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--all'], cwd = 'malachite-nz')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--all', '--features', '32_bit_limbs'], cwd = 'malachite-nz')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--tests'], cwd = 'malachite-test')
    except subprocess.CalledProcessError:
        return False
    try:
        subprocess.check_call(['cargo', 'check', '--tests', '--features', '32_bit_limbs'], cwd = 'malachite-test')
    except subprocess.CalledProcessError:
        return False
    if include_rust_wheels:
        try:
            subprocess.check_call(['cargo', 'check', '--tests'], cwd = '../rust-wheels')
        except subprocess.CalledProcessError:
            return False
        try:
            subprocess.check_call(['cargo', 'check', '--tests', '--features', '32_bit_limbs'], cwd = '../rust-wheels')
        except subprocess.CalledProcessError:
            return False
    return True


def replace_and_build(filename, line_number, substring, replacement, include_rust_wheels):
    with open(filename, 'r') as in_f:
        with open('temp.rs', 'w') as out_f:
            n = 1
            for line in in_f.readlines() :
                line = line.rstrip()
                if n == line_number:
                    out_f.write(line.replace(substring, replacement) + '\n')
                else:
                    out_f.write(line + '\n')
                n += 1
    assert subprocess.call('cp ' + filename + ' backup.rs', shell = True) == 0
    assert subprocess.call('mv temp.rs ' + filename, shell = True) == 0
    if try_building(include_rust_wheels):
        assert subprocess.call('rm backup.rs', shell = True) == 0
        return True
    else:
        assert subprocess.call('mv backup.rs ' + filename, shell = True) == 0
        return False


def try_replacing(filename, substring, replacement, exceptions, include_rust_wheels):
    start_line_number = 1
    while True:
        found_instance = False
        with open(filename) as f:
            n = 1
            for line in f.readlines():
                line = line.rstrip()
                if n >= start_line_number and substring in line and all([e not in line for e in exceptions]):
                    found_instance = True
                    break
                n += 1
        if found_instance:
            replace_and_build(filename, n, substring, replacement, include_rust_wheels)
            start_line_number = n + 1
        else:
            return


assert try_building(True)
filename_list = []
for root, directories, filenames in os.walk('.'):
    for filename in filenames: 
        filename = os.path.join(root, filename) 
        if '/target/' not in filename and filename.endswith('.rs'):
            filename_list.append(filename)
for root, directories, filenames in os.walk('../rust-wheels'):
    for filename in filenames: 
        filename = os.path.join(root, filename) 
        if '/target/' not in filename and filename.endswith('.rs'):
            filename_list.append(filename)
filename_list.sort()

for filename in filename_list:
    if '/malachite-test/' in filename:
        try_replacing(filename, 'pub fn', 'pub(crate) fn', ['pub fn main()'], False)
        try_replacing(filename, 'pub(crate) fn', 'fn', [], False)
    else:
        try_replacing(filename, 'pub(crate) fn', 'fn', [], True)
