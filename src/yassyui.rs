use lv2;
use std::ptr;

#[repr(C)]
pub struct yassyui {
    pub host: *const lv2::LV2UIExternalUIHost,
    pub dummy: i8,
}

impl yassyui {
    pub fn new() -> yassyui {
        yassyui { host: ptr::null(), dummy: 1i8 }
    }
    pub fn hello(&self) {
        println!("Hello", );
    }
}
