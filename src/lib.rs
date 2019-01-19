use std::cell::RefCell;
use std::ptr::null_mut;
use std::os::raw::{c_int, c_char, c_void, c_float};
use std::ffi::CString;

#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern fn();

extern {
    pub fn emscripten_set_main_loop(func: em_callback_func, fps: c_int, simulate_infinite_loop: c_int);
    pub fn emscripten_cancel_main_loop();
    pub fn emscripten_get_now() -> c_float;

    // note: this function is not documented but is used by the ports of glfw, SDL and EGL
    // as well as glutin.
    pub fn emscripten_GetProcAddress(name: *const c_char) -> *const c_void;
}

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

pub fn set_main_loop_callback<F>(callback: F, fps: c_int, blocking: bool) where F: FnMut() {
    MAIN_LOOP_CALLBACK.with(|log| {
        *log.borrow_mut() = &callback as *const _ as *mut c_void;
    });

    unsafe { emscripten_set_main_loop(wrapper::<F>, fps, blocking as c_int); }

    unsafe extern "C" fn wrapper<F>() where F: FnMut() {
        MAIN_LOOP_CALLBACK.with(|z| {
            let closure = *z.borrow_mut() as *mut F;
            (*closure)();
        });
    }
}

pub fn get_proc_address(addr: &str) -> *const() {
        let addr = CString::new(addr).unwrap();

        unsafe {
            // FIXME: if `as_ptr()` is used, then wrong data is passed to emscripten
            emscripten_GetProcAddress(addr.into_raw() as *const _) as *const _
        }
}
