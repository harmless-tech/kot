pub static ARCHES: &[&str] = &["x86_64", "arm", "aarch64", "riscv64", "s390x"];

pub static OSES: &[&str] = &["windows", "macos", "linux", "freebsd"];

pub static OS_FAMILIES: &[&str] = &["windows", "unix"];

pub mod current {
    pub static TRIPLET: &str = env!("TARGET");

    #[cfg(target_arch = "x86_64")]
    pub static ARCH: &str = "x86_64";
    #[cfg(target_arch = "arm")]
    pub static ARCH: &str = "arm";
    #[cfg(target_arch = "aarch64")]
    pub static ARCH: &str = "aarch64";
    #[cfg(target_arch = "riscv64")]
    pub static ARCH: &str = "riscv64";
    #[cfg(target_arch = "s390x")]
    pub static ARCH: &str = "s390x";

    #[cfg(target_os = "windows")]
    pub static OS: &str = "windows";
    #[cfg(target_os = "macos")]
    pub static OS: &str = "macos";
    #[cfg(target_os = "linux")]
    pub static OS: &str = "linux";
    #[cfg(target_os = "freebsd")]
    pub static OS: &str = "freebsd";

    #[cfg(target_family = "windows")]
    pub static OS_FAMILY: &str = "windows";
    #[cfg(target_family = "unix")]
    pub static OS_FAMILY: &str = "unix";
}
