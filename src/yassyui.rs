use libc;
use lv2;
use std::ptr;
use websocket::Server;
// use std::net::TcpListener;
use std::sync::mpsc;
// use websocket::Message;

#[repr(C)]
pub struct yassyui {
    pub host: *const lv2::LV2UIExternalUIHost,
    pub controller: lv2::LV2UIController,
    pub write: lv2::LV2UIWriteFunction,
    pub extwidget: lv2::LV2UIExternalUIWidget,
    pub showing: bool,
    // pub tcplistener: TcpListener,
    pub sender: mpsc::Sender<f32>,
    pub receiver: mpsc::Receiver<f32>,
}

impl yassyui {
    pub fn new() -> yassyui {
        // println!("address: {}", ipaddr);
        let (tx, rx) = mpsc::channel();
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
            write: None,
            showing: false, /* tcplistener: TcpListener::bind("127.0.0.1:2794").unwrap()
                             * sender: tx, */
            sender: tx,
            receiver: rx,
        };
        ui
    }
    // pub fn receive(&self, val: f32) {
    //     println!("Hello", );
    // }
}
