import subprocess

def substitute_mul_fft_modf_threshold(input_filename, output_filename, value):
    prefix = 'pub(crate) const MUL_FFT_MODF_THRESHOLD: usize = '
    with open(input_filename, 'r') as in_f:
    	with open(output_filename, 'w') as out_f:
        	for line in in_f.readlines():
        	    line = line.rstrip()
        	    if line.startswith(prefix):
        	    	original_value = line[len(prefix):-1]
        	    	out_f.write(prefix + value + ';\n')
        	    else:
        	    	out_f.write(line + '\n')


path = 'src/natural/arithmetic/mul/mul_mod.rs'
assert subprocess.call('cp ' + path + ' backup.rs', shell = True) == 0
substitute_mul_fft_modf_threshold('backup.rs', 'temp.rs', '40')
assert subprocess.call('mv temp.rs ' + path, shell = True) == 0
assert subprocess.call('cargo test --test lib --features 32_bit_limbs --features serde --features fail_on_untested_path -- test_limbs_mul_greater_to_out_fft', shell = True) == 0
substitute_mul_fft_modf_threshold('backup.rs', 'temp.rs', '4')
assert subprocess.call('mv temp.rs ' + path, shell = True) == 0
assert subprocess.call('cargo test --test lib --features 32_bit_limbs --features serde --features fail_on_untested_path -- test_limbs_mul_greater_to_out_fft', shell = True) == 0
substitute_mul_fft_modf_threshold('backup.rs', 'temp.rs', '396')
assert subprocess.call('mv temp.rs ' + path, shell = True) == 0
assert subprocess.call('cargo test --test lib --features 32_bit_limbs --features serde --features fail_on_untested_path -- test_limbs_mul_greater_to_out_fft', shell = True) == 0
assert subprocess.call('mv backup.rs ' + path, shell = True) == 0
