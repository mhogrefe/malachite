import subprocess
from shutil import copyfile
import time

platform_32 = False
while True:
	if platform_32:
		assert subprocess.call('cargo run --features 32_bit_limbs --release tune all', shell = True) == 0
	else:
		assert subprocess.call('cargo run --release tune all', shell = True) == 0
	copyfile('benchmarks/platform.txt', 'benchmarks/platform-' + str(int(round(time.time() * 1000))) + '.txt')
	target_file = 'platform_32.rs' if platform_32 else 'platform_64.rs'
	copyfile('benchmarks/platform.txt', '../malachite-nz/src/' + target_file)
