extern crate libc;

pub mod ffi {
    extern "C" {
        pub fn gtk_init(argc: *mut libc::int32_t, argv: *mut *mut *mut libc::uint8_t);
        pub fn gtk_window_new(win_type: libc::uint8_t) -> *mut libc::c_void;
        pub fn gtk_widget_show(widget: *mut libc::c_void);
        pub fn gtk_main();
    }
}

pub mod wrapper {

    use crate::ffi;
    use std::ptr;

    pub fn gtk_init() {
        unsafe {
            let argc: *mut libc::int32_t = ptr::null_mut();
            let argv: *mut *mut *mut libc::uint8_t = ptr::null_mut();
            ffi::gtk_init(argc, argv);
        }
    }

    pub fn gtk_window_new() -> *mut libc::c_void {
        unsafe { ffi::gtk_window_new(0) }
    }

    pub fn gtk_widget_show(widget: *mut libc::c_void) {
        unsafe { ffi::gtk_widget_show(widget) }
    }

    pub fn gtk_main() {
        unsafe { ffi::gtk_main() }
    }
}

#[cfg(test)]
mod tests {
    use crate::wrapper;

    #[test]
    fn show_window() {
        wrapper::gtk_init();

        wrapper::gtk_main();
    }
}
