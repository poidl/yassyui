use libc;
use lv2;
use std::ptr;

#[repr(C)]
pub struct yassyui {
    pub extwidget: lv2::LV2UIExternalUIWidget,
    pub host: *const lv2::LV2UIExternalUIHost,
    pub controller: lv2::LV2UIController,
    pub done: i32,
}

impl yassyui {
    pub fn new() -> yassyui {
        yassyui {
            extwidget: lv2::LV2UIExternalUIWidget {
                // Why "None"? Nullable function pointers. See
                // https://doc.rust-lang.org/book/ffi.html
                // https://mail.mozilla.org/pipermail/rust-dev/2014-September/011200.html
                run: None,
                show: None,
                hide: None,
            },
            host: ptr::null(),
            controller: ptr::null(),
            done: 0i32,
        }
    }
    pub fn hello(&self) {
        println!("Hello", );
    }
}
