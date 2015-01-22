extern crate xlib;
extern crate libc;
use libc::*;

fn main() {
    let bytes = std::string::String::from_str(":0").into_bytes();
    let mut cchars : Vec<c_char> = bytes.map_in_place(|w| w as c_char);
    let name: *mut c_char = cchars.as_mut_slice().as_mut_ptr(); 

    let dpy = unsafe { xlib::XOpenDisplay(name) };
    let screen = unsafe { xlib::XDefaultScreen(dpy) };
    let root = unsafe { xlib::XRootWindow(dpy, screen) };
    let white_pixel = unsafe { xlib::XWhitePixel(dpy, screen) };
    let black_pixel = unsafe { xlib::XBlackPixel(dpy, screen) };

    let bytes_store = std::string::String::from_str("Test xlib").into_bytes();
    let mut cchars_store : Vec<c_char> = bytes_store.map_in_place(|w| w as c_char);
    let store_name: *mut c_char = cchars_store.as_mut_slice().as_mut_ptr(); 

    let win = unsafe { xlib::XCreateSimpleWindow(dpy, root, 50, 50, 400, 290, 10, black_pixel, white_pixel) };
    unsafe {
        xlib::XStoreName(dpy, win, store_name);
        xlib::XMapWindow(dpy, win); 
    }

    loop {
        let ev: *mut xlib::XEvent;
        unsafe {
            xlib::XNextEvent(dpy, ev);
        }
    };
}
