use lv2;
use std::ptr;

#[repr(C)]
pub struct yassyui {
    pub dummy: i8,
}

impl yassyui {
    pub fn new() -> yassyui {
        yassyui { dummy: 1i8 }
    }
    pub fn hello(&self) {
        println!("Hello", );
    }
}
