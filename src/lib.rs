#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate libc;
extern crate lv2;

mod yassyui;
use std::mem;
use std::ffi::CStr;
use std::str;
use std::ptr;

// have to define new type. Otherwise error: "cannot define inherent impl for a type outside of
// the crate where the type is defined; define and implement a trait or new type instead"
#[repr(C)]
struct Descriptor(lv2::LV2UIDescriptor);

impl Descriptor {
    pub extern "C" fn instantiate(_descriptor: *const lv2::LV2UIDescriptor,
                                  plugin_uri: *const libc::c_char,
                                  bundle_path: *const libc::c_char,
                                  write_function: lv2::LV2UIWriteFunction,
                                  controller: lv2::LV2UIController,
                                  widget: *mut lv2::LV2UIWidget,
                                  features: *const (*const lv2::LV2Feature))
                                  -> lv2::LV2UIHandle {
        println!("host calls instantiate()");
        print_features(features);
        // unsafe {
        //     let hoit = gtk_box_new(1i8 as libc::c_int, 4i8 as libc::c_int);
        //     let hoit = gtk_button_box_new(1i8 as libc::c_int);
        //     *widget = hoit as  lv2::LV2UIWidget;
        //     mem::forget(hoit);
        // }
        let bx = Box::new(yassyui::yassyui::new());

        let ptr = (&*bx as *const yassyui::yassyui) as *mut libc::c_void;
        mem::forget(bx);
        ptr
    }

    pub extern "C" fn cleanup(handle: lv2::LV2UIHandle) {
        println!("host calls cleanup()");
    }

    pub extern "C" fn port_event(ui: lv2::LV2UIHandle,
                                 port_index: libc::c_uint,
                                 buffer_size: libc::c_uint,
                                 format: libc::c_uint,
                                 buffer: *const libc::c_void) {
        println!("host calls port_event()")
    }

    pub extern "C" fn extension_data(uri: *const libc::c_char) -> *const libc::c_void {
        unsafe {
            let buf = CStr::from_ptr(uri).to_bytes();
            let s: &str = str::from_utf8(buf).unwrap();
            println!("hoit: {}", s);
            if s == "http://lv2plug.in/ns/extensions/ui#idleInterface" {
                return &idleinterface as *const lv2::LV2UIIdleInterface as *const libc::c_void;
            } else if s == "http://lv2plug.in/ns/extensions/ui#showInterface" {
                return &nullinterface as *const lv2::LV2UIIdleInterface as *const libc::c_void;
            }

            &nullinterface as *const lv2::LV2UIIdleInterface as *const libc::c_void
        }
    }
}

static S: &'static [u8] = b"http://example.org/yassyui\0";

static mut desc: lv2::LV2UIDescriptor = lv2::LV2UIDescriptor {
    uri: 0 as *const libc::c_char, // ptr::null() isn't const fn (yet)
    instantiate: Descriptor::instantiate,
    cleanup: Descriptor::cleanup,
    port_event: Descriptor::port_event,
    extension_data: Descriptor::extension_data,
};

static mut idleinterface: lv2::LV2UIIdleInterface = lv2::LV2UIIdleInterface { idle: ui_idle };

static mut nullinterface: lv2::LV2UIIdleInterface =
    lv2::LV2UIIdleInterface { idle: ui_nullfunction };

#[no_mangle]
pub extern "C" fn lv2ui_descriptor(index: i32) -> *const lv2::LV2UIDescriptor {
    if index != 0 {
        return std::ptr::null();
    } else {
        // credits to ker on stackoverflow:
        // http://stackoverflow.com/questions/31334356/static-struct-with
        // -c-strings-for-lv2-plugin (duplicate) or http://stackoverflow.com/questions/
        // 25880043/creating-a-static-c-struct-containing-strings
        let ptr = S.as_ptr() as *const libc::c_char;
        unsafe {
            desc.uri = ptr;
            return &desc as *const lv2::LV2UIDescriptor;
        }
    }
}

fn print_features(features: *const (*const lv2::LV2Feature)) {
    // Print lv2 host features
    let mut x: isize = 0;
    unsafe {
        loop {

            let fptr: *const lv2::LV2Feature = *features.offset(x);
            if fptr.is_null() {
                println!("End of features");
                break;
            }
            let uriptr = (*fptr).uri;
            let buf = CStr::from_ptr(uriptr).to_bytes();
            let s: &str = str::from_utf8(buf).unwrap();
            println!("uri: {}", s);
            x = x + 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn ui_idle(handle: lv2::LV2UIHandle) -> libc::c_int {
    println!("host calls idle()");
    return 0i32 as libc::c_int;
}

#[no_mangle]
pub extern "C" fn ui_nullfunction(handle: lv2::LV2UIHandle) -> libc::c_int {
    println!("host calls ui_nullfunction()");
    return 0i32 as libc::c_int;
}

// #[link(name = "gtk-3")]
// extern {
//     // fn snappy_max_compressed_length(source_length: libc::size_t) -> libc::size_t;
//     fn gtk_box_new(orient: libc::c_int, spacing: libc::c_int) -> *const libc::c_void;
//     fn gtk_button_box_new(orient: libc::c_int) -> *const libc::c_void;
// }
