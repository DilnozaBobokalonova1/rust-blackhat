#[path = "../fuzz"]
mod fuzz {
    pub mod fuzz_targets;  // Import the fuzz_targets module
}

use fuzz::fuzz_targets::fuzz_target_1::MemcopyInput;

pub fn vulnerable_memcopy(dest: &mut [u8], src: &[u8], n: usize) {

    let mut i = 0;

    while i < n {
        dest[i] = src[i];
        i += 1;
    }
}