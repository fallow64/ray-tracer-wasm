#[cfg(target_arch = "wasm32")]
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

// SAFETY: This is safe because WebAssembly is single-threaded.
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub mod console {
    mod js {
        unsafe extern "C" {
            pub unsafe fn console_log(ptr: *const u8, len: usize);
            pub unsafe fn console_error(ptr: *const u8, len: usize);
        }
    }

    pub fn log(s: &str) {
        unsafe {
            js::console_log(s.as_ptr(), s.len());
        }
    }

    pub fn error(s: &str) {
        unsafe {
            js::console_error(s.as_ptr(), s.len());
        }
    }
}

// Custom print macros for WASM
#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        $crate::glue::console::log(&s);
    }};
}

#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! println {
    () => {
        $crate::glue::console::log("\n");
    };
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        $crate::glue::console::log(&s);
    }};
}

#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! eprintln {
    () => {
        $crate::glue::console_error("\n");
    };
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        $crate::glue::console::error(&s);
    }};
}

#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! dbg {
    () => {
        {
            $crate::glue::console::log(
                concat!("[", file!(), ":", line!(), "]")
            );
        }
    };

    ($val:expr $(,)?) => {{
        let tmp = $val;
        let s = format!(
            "[{}:{}] {} = {:?}",
            file!(),
            line!(),
            stringify!($val),
            &tmp
        );
        $crate::glue::console::log(&s);
        tmp
    }};

    ($($val:expr),+ $(,)?) => {
        (
            $(
                $crate::dbg!($val)
            ),+
        )
    };
}
