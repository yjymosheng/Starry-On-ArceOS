use core::ffi::c_int;

use crate::syscall_body;

/// sys_uname 中指定的结构体类型
#[repr(C)]
pub struct UtsName {
    /// 系统名称
    pub sysname: [u8; 65],
    /// 网络上的主机名称
    pub nodename: [u8; 65],
    /// 发行编号
    pub release: [u8; 65],
    /// 版本
    pub version: [u8; 65],
    /// 硬件类型
    pub machine: [u8; 65],
    /// 域名
    pub domainname: [u8; 65],
}

impl Default for UtsName {
    fn default() -> Self {
        Self {
            sysname: Self::from_str("Starry-Next"),
            nodename: Self::from_str("Starry - Next yjymosheng"),
            release: Self::from_str("10.0.0"),
            version: Self::from_str("10.0.0"),
            #[cfg(target_arch = "riscv64")]
            machine: Self::from_str("RISC-V 64 on qemu"),
            #[cfg(target_arch = "x86_64")]
            machine: Self::from_str("X86 64 on qemu"),
            domainname: Self::from_str("https://github.com/yjymosheng/Starry-On-ArceOS"),
        }
    }
}

impl UtsName {
    fn from_str(info: &str) -> [u8; 65] {
        let mut data: [u8; 65] = [0; 65];
        data[..info.len()].copy_from_slice(info.as_bytes());
        data
    }
}

pub(crate) fn sys_uname(utsname: *mut UtsName) -> c_int {
    syscall_body!(sys_uname, {
        if utsname.is_null() {
            return Err(axerrno::LinuxError::EFAULT);
        }
        unsafe {
            *utsname = UtsName::default();
        }

        // 查资料说,正确返回0 错误返回-1 ,这里默认正确,我想不到除了指针错误的其他情况
        Ok(0)
    })
}
