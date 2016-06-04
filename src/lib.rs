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
    pub extern "C" fn instantiate(descriptor: *const lv2::LV2UIDescriptor,
                                  plugin_uri: *const libc::c_char,
                                  bundle_path: *const libc::c_char,
                                  write_function: lv2::LV2UIWriteFunction,
                                  controller: lv2::LV2UIController,
                                  widget: *mut lv2::LV2UIWidget,
                                  features: *const (*const lv2::LV2Feature))
                                  -> lv2::LV2UIHandle {
        println!("host calls instantiate()");
        print_features(features);
        let mut bx = Box::new(yassyui::yassyui::new());
        let uitype = unsafe { lv2::cstring((*descriptor).uri) };
        println!("UITYPE: {}", uitype);
        if uitype == "http://example.org/yassy#kx" {
            println!("MAPPING FEATURE FOR: {}", uitype);
            let featureptr = lv2::mapfeature(features,
                                             "http://kxstudio.sf.net/ns/lv2ext/external-ui#Host");
            match featureptr {
                Ok(fp) => bx.host = fp as *const lv2::LV2UIExternalUIHost,
                _ => return ptr::null_mut(),
            }
        }
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
                return &showinterface as *const lv2::LV2UIShowInterface as *const libc::c_void;
            }

            &nullinterface as *const lv2::LV2UIIdleInterface as *const libc::c_void
        }
    }
}

static SUI: &'static [u8] = b"http://example.org/yassy#ui\0";

static mut descUI: lv2::LV2UIDescriptor = lv2::LV2UIDescriptor {
    uri: 0 as *const libc::c_char, // ptr::null() isn't const fn (yet)
    instantiate: Descriptor::instantiate,
    cleanup: Descriptor::cleanup,
    port_event: Descriptor::port_event,
    extension_data: Descriptor::extension_data,
};

static SKX: &'static [u8] = b"http://example.org/yassy#kx\0";

pub extern "C" fn nullfunction(arg: *const i8) -> *const libc::c_void {
    ptr::null()
}

static mut descKX: lv2::LV2UIDescriptor = lv2::LV2UIDescriptor {
    uri: 0 as *const libc::c_char, // ptr::null() isn't const fn (yet)
    instantiate: Descriptor::instantiate,
    cleanup: Descriptor::cleanup,
    port_event: Descriptor::port_event,
    extension_data: nullfunction,
};

static mut idleinterface: lv2::LV2UIIdleInterface = lv2::LV2UIIdleInterface { idle: ui_idle };
static mut showinterface: lv2::LV2UIShowInterface = lv2::LV2UIShowInterface {
    show: ui_show,
    hide: ui_hide,
};

static mut nullinterface: lv2::LV2UIIdleInterface =
    lv2::LV2UIIdleInterface { idle: ui_nullfunction };

#[no_mangle]
pub extern "C" fn lv2ui_descriptor(index: i32) -> *const lv2::LV2UIDescriptor {
    // credits to ker on stackoverflow:
    // http://stackoverflow.com/questions/31334356/static-struct-with
    // -c-strings-for-lv2-plugin (duplicate) or http://stackoverflow.com/questions/
    // 25880043/creating-a-static-c-struct-containing-strings

    // Credits to Hanspeter Portner for explaining how to use ui:UI and kx:Widget:
    // http://lists.lv2plug.in/pipermail/devel-lv2plug.in/2016-May/001649.html
    let ptr: *const libc::c_char;
    let desc: lv2::LV2UIDescriptor;
    unsafe {
        match index {
            0 => {
                ptr = SUI.as_ptr() as *const libc::c_char;
                descUI.uri = ptr;
                return &descUI as *const lv2::LV2UIDescriptor;

            }
            1 => {
                ptr = SKX.as_ptr() as *const libc::c_char;
                descKX.uri = ptr;
                return &descKX as *const lv2::LV2UIDescriptor;
            }
            _ => return std::ptr::null(),
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
pub extern "C" fn ui_show(handle: lv2::LV2UIHandle) -> libc::c_int {
    println!("host calls show()");
    return 0i32 as libc::c_int;
}

#[no_mangle]
pub extern "C" fn ui_hide(handle: lv2::LV2UIHandle) -> libc::c_int {
    println!("host calls hide()");
    return 0i32 as libc::c_int;
}

#[no_mangle]
pub extern "C" fn ui_nullfunction(handle: lv2::LV2UIHandle) -> libc::c_int {
    println!("host calls ui_nullfunction()");
    return 0i32 as libc::c_int;
}
