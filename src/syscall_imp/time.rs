use core::ffi::c_int;

use arceos_posix_api as api;

use crate::syscall_body;

pub(crate) fn sys_clock_gettime(clock_id: i32, tp: *mut api::ctypes::timespec) -> i32 {
    unsafe { api::sys_clock_gettime(clock_id, tp) }
}


fn gettimeofday(ts : *mut arceos_posix_api::ctypes::timeval) -> c_int {
    let current_us =  axhal::time::monotonic_time_nanos() as usize / 1000;
    unsafe {
        *ts = arceos_posix_api::ctypes::timeval {
            tv_sec: (current_us / 1_000_000) as i64, 
            tv_usec:( current_us % 1_000_000) as i64,
        }
    }
    0
}

//这玩意儿就是rcore ch3的获取时间 ,grep查一下timeval在哪,然后引用就行了,为了简洁写一个函数直接被调用吧
pub(crate) fn sys_get_time_of_day(ts : *mut arceos_posix_api::ctypes::timeval) -> c_int { 
    syscall_body!( get_time_of_day,
        {unsafe {
            gettimeofday(ts);
        }
    }
    Ok(0)
    )
}