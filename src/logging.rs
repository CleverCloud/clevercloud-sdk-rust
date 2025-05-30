#![allow(unused_macros, dead_code)]

macro_rules! trace {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        if ::log::log_enabled!(log::Level::Trace) {
            ::tracing::trace!($($arg)*);
        }
    };
    () => ()
}

macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        if ::log::log_enabled!(log::Level::Debug) {
            ::tracing::debug!($($arg)*);
        }
    };
    () => ()
}

macro_rules! info {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        if ::log::log_enabled!(log::Level::Info) {
            ::tracing::info!($($arg)*);
        }
    };
    () => ()
}

macro_rules! warn {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        if ::log::log_enabled!(log::Level::Warn) {
            ::tracing::warn!($($arg)*);
        }
    };
    () => ()
}

macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        if ::log::log_enabled!(log::Level::Error) {
            ::tracing::error!($($arg)*);
        }
    };
    () => ()
}
