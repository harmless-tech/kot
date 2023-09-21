// TODO: Block Building on unsupported platforms?

pub static SUPPORTED_OSES: [&str; 3] = ["windows", "macos", "linux"];

pub static SUPPORTED_ARCHES: [&str; 3] = ["x86_64", "arm", "aarch64"];

pub static SUPPORTED_OS_FAMILIES: [&str; 2] = ["windows", "unix"];

pub mod current {
    pub static TRIPLET: &str = env!("TARGET");

    #[cfg(target_os = "windows")]
    pub static OS: &str = "windows";
    #[cfg(target_os = "macos")]
    pub static OS: &str = "macos";
    #[cfg(target_os = "linux")]
    pub static OS: &str = "linux";

    #[cfg(target_family = "windows")]
    pub static OS_FAMILY: &str = "windows";
    #[cfg(target_family = "unix")]
    pub static OS_FAMILY: &str = "unix";

    #[cfg(target_arch = "x86_64")]
    pub static ARCH: &str = "x86_64";
    #[cfg(target_arch = "arm")]
    pub static ARCH: &str = "arm";
    #[cfg(target_arch = "aarch64")]
    pub static ARCH: &str = "aarch64";
}
