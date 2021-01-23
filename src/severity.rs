pub enum Severity {
    Critical,
    Error,
    Info,
    Debug,
}

impl Severity {
    pub (crate) fn to_int(&self) -> i32 {
        match self {
            Self::Critical => libc::LOG_CRIT,
            Self::Error => libc::LOG_ERR,
            Self::Info => libc::LOG_INFO,
            Self::Debug => libc::LOG_DEBUG,
        }
    }
}
