use libc;
use lv2;
use std::ptr;
// use websocket::Server;
use std::net::TcpListener;

#[repr(C)]
pub struct yassyui {
    pub host: *const lv2::LV2UIExternalUIHost,
    pub controller: lv2::LV2UIController,
    pub extwidget: lv2::LV2UIExternalUIWidget,
    pub showing: bool, // pub tcplistener: TcpListener,
}

impl yassyui {
    pub fn new() -> yassyui {
        let ui = yassyui {
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
            showing: false, // tcplistener: TcpListener::bind("127.0.0.1:2794").unwrap(),
        };
        ui
    }
    pub fn hello(&self) {
        println!("Hello", );
    }
}
