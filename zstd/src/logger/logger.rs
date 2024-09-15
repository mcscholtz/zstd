use core::fmt::Write;
use alloc::string::String;

pub use super::term::TerminalColor;

#[cfg(not(test))]
extern "C" {

    pub fn zlog_set_level(level: u32);

    pub fn zlog_inst_dbg(inst: *const struct_logger_instance, text: *const u8);
    pub fn zlog_inst_inf(inst: *const struct_logger_instance, text: *const u8);
    pub fn zlog_inst_wrn(inst: *const struct_logger_instance, text: *const u8);
    pub fn zlog_inst_err(inst: *const struct_logger_instance, text: *const u8);
    pub fn zlog_inst_log(inst: *const struct_logger_instance, text: *const u8);
}

#[allow(non_camel_case_types)]
#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct struct_logger_instance {
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*const u8, core::marker::PhantomPinned)>,
}

pub struct ZephyrLogger;

impl ZephyrLogger {
    // Should logging be enabled at this level.  For now, it is safe to return true, and we'll just
    // log everything.
    //fn enabled(&self, _metadata: &log::Metadata) -> bool {
    //    true
    //}

    fn show(inst: *const struct_logger_instance, log_level: Level, c_str: &str) {
        unsafe {
            match log_level {
                Level::Always => zlog_inst_log(inst, c_str.as_bytes().as_ptr()),
                Level::Error => zlog_inst_err(inst, c_str.as_bytes().as_ptr()),
                Level::Warn => zlog_inst_wrn(inst, c_str.as_bytes().as_ptr()),
                Level::Info => zlog_inst_inf(inst, c_str.as_bytes().as_ptr()),
                Level::Debug => zlog_inst_dbg(inst, c_str.as_bytes().as_ptr()),
            }
        }
    }

    fn log(
        inst: *const struct_logger_instance,
        log_level: Level,
        color: Option<TerminalColor>,
        args: core::fmt::Arguments,
    ) {
        let mut c_str = String::new();
        if let Ok(_) = match color {
            Some(color) => write!(
                c_str,
                "{}{}{}\0",
                color.color(),
                format_args!("{}", args),
                TerminalColor::reset()
            ),
            None => write!(c_str, "{}\0", args),
        } {
            Self::show(inst, log_level, c_str.as_str());
        } else {
            c_str.clear();
            if let Some(args) = args.as_str() {
                let len = args.len() + 3;
                write!(c_str, "{}\0", format_args!("<--truncated--> ({})", len)).unwrap();
            } else {
                write!(c_str, "{}\0", format_args!("<--truncated-->")).unwrap();
            }
            Self::show(inst, log_level, c_str.as_str());
        }
    }

    pub fn info(
        inst: *const struct_logger_instance,
        color: TerminalColor,
        args: core::fmt::Arguments,
    ) {
        Self::log(inst, Level::Info, Some(color), args);
    }

    pub fn debug(inst: *const struct_logger_instance, args: core::fmt::Arguments) {
        Self::log(inst, Level::Debug, None, args);
    }

    pub fn warn(inst: *const struct_logger_instance, args: core::fmt::Arguments) {
        Self::log(inst, Level::Warn, None, args);
    }

    pub fn error(inst: *const struct_logger_instance, args: core::fmt::Arguments) {
        Self::log(inst, Level::Error, None, args);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum Level {
    Always,
    Error,
    Warn,
    Info,
    Debug,
}

pub trait Module {
    fn instance() -> *const struct_logger_instance;
    fn module_name() -> &'static str;
    fn color() -> TerminalColor {
        TerminalColor::Default
    }
}

#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => {{
		#[cfg(test)]
		println!("{}: {}", ScopedModule::module_name(), core::format_args!($($arg)*));
		#[cfg(not(test))]
		$crate::logger::logger::ZephyrLogger::error(ScopedModule::instance(), core::format_args!($($arg)*));
		panic!($($arg)*);
	}};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
		#[cfg(test)]
		println!("{}: {}", ScopedModule::module_name(), core::format_args!($($arg)*));
		#[cfg(not(test))]
		$crate::logger::logger::ZephyrLogger::error(ScopedModule::instance(), core::format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
		#[cfg(test)]
		println!("{}: {}", ScopedModule::module_name(), core::format_args!($($arg)*));
		#[cfg(not(test))]
		$crate::logger::logger::ZephyrLogger::warn(ScopedModule::instance(), core::format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
            //#[cfg(test)]
		    //println!("{}: {}", ScopedModule::module_name(), core::format_args!($($arg)*));
		    //#[cfg(not(test))]
		    $crate::logger::logger::ZephyrLogger::info(ScopedModule::instance(), ScopedModule::color(), core::format_args!($($arg)*));

    }}
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{

            #[cfg(test)]
            println!("{}: {}", ScopedModule::module_name(), core::format_args!($($arg)*));
            #[cfg(not(test))]
            $crate::logger::logger::ZephyrLogger::debug(ScopedModule::instance(), core::format_args!($($arg)*));

    }}
}

#[macro_export]
macro_rules! module {
    ($instance:ident, $color:expr) => {
        use $crate::logger::logger::struct_logger_instance;
        use $crate::logger::logger::Module;
        use $crate::logger::term::TerminalColor;

        struct ScopedModule;

        impl Module for ScopedModule {
            fn module_name() -> &'static str {
                stringify!($instance)
            }

            fn color() -> TerminalColor {
                $color
            }

            fn instance() -> *const struct_logger_instance {
                extern "C" {
                    static $instance: struct_logger_instance;
                }
                unsafe { &$instance as *const struct_logger_instance }
            }
        }
    };
}
