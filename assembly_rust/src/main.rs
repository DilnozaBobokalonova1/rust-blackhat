use core::arch::asm;

const SYS_WRITE: usize = 64; // The correct system call number for write on ARM64
const STDOUT: usize = 1;
static MESSAGE: &str = "hello world\n";

unsafe fn syscall3(scnum: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let ret: usize;

    //for ARM-specific registers, we use different registers
    asm!(
        "svc 0",
        inout("x3") scnum => _,
        inout("x0") arg1 => _,
        inout("x1") arg2 => _,
        inout("x2") arg3 => _,
        //Indirect result location register
        lateout("x8") ret,
        options(nostack),
    );
    
    ret
}

fn main() {
    unsafe {
        
        let result = syscall3(
            SYS_WRITE,
            STDOUT,
            MESSAGE.as_ptr() as usize,
            MESSAGE.len() as usize,
        );
        println!("After syscall (second print) - x8: {}, x0: {}, x1: {}, x2: {}", SYS_WRITE, STDOUT, MESSAGE.as_ptr() as usize, MESSAGE.len() as usize);
        println!("result is {}", result);
    };
}
