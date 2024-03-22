use libc::EXIT_SUCCESS;

fn main() {
    let exit_status: libc::c_int = EXIT_SUCCESS;

    unsafe {
        libc::exit(exit_status);
    };
}
