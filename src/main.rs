use libc;
use std::ffi::CString;
fn main() {
    let path = CString::new("/foo.txt").expect("CString failed");
    let fd = unsafe { libc::open(path.as_ptr(), libc::O_RDONLY, 0o666) };
    // Check for errors
    if fd < 0 {
        println!("[RUST + WASM] Error opening file: {}", fd);
        // Handle the error
    } else {
        println!(
            "[RUST + WASM] File opened successfully with file descriptor: {}",
            fd
        );

        // Read file content
        let mut buffer = [0u8; 256]; // Buffer to store file data
        let bytes_read =
            unsafe { libc::read(fd, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len()) };

        if bytes_read > 0 {
            let content = String::from_utf8_lossy(&buffer[..bytes_read as usize]);
            println!("[RUST + WASM] File content: {}", content);
        }

        unsafe { libc::close(fd) };
    }
}
