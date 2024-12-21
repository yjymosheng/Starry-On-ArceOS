use core::ffi::{c_char, c_int};

pub(crate) fn sys_dup(old_fd:c_int)->c_int{
    arceos_posix_api::sys_dup(old_fd)
}
#[cfg(target_arch = "x86_64")]
pub(crate) fn sys_dup2(old_fd: c_int, new_fd: c_int) -> c_int{
    arceos_posix_api::sys_dup2(old_fd, new_fd)
}
#[cfg(target_arch = "riscv64")]
pub(crate) fn sys_dup3(old_fd: c_int, new_fd: c_int) -> c_int{
    arceos_posix_api::sys_dup2(old_fd, new_fd)
}

pub(crate) fn sys_close(fd: c_int) -> c_int{
    arceos_posix_api::sys_close(fd)
}

pub(crate) fn sys_getcwd(buf: *mut c_char, size: usize) -> *mut c_char {
    arceos_posix_api::sys_getcwd(buf, size)
}


pub(crate) fn sys_openat(dfd: c_int, fname: *const c_char, flags: c_int, mode: arceos_posix_api::ctypes::mode_t) -> c_int {
    arceos_posix_api::sys_open(fname, flags, mode) 
    // sys_open(filename: *const c_char, flags: c_int, mode: ctypes::mode_t) -> c_int
}

pub(crate)  fn sys_fstat (fd: c_int, buf: *mut arceos_posix_api::ctypes::stat) -> c_int{
    unsafe { arceos_posix_api::sys_fstat(fd,buf)} 
}



// #[cfg(target_arch = "x86_64")]
// pub(crate) fn sys_open(filename: *const c_char, flags: c_int, mode: ctypes::mode_t) -> c_int {
//     arceos_posix_api::sys_open(filename, flags, mode)
// }