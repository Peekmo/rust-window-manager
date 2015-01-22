extern crate xlib;
extern crate libc;
use libc::*;

fn main() {
    let bytes = std::string::String::from_str(":0").into_bytes();
    let mut cchars : Vec<c_char> = bytes.map_in_place(|w| w as c_char);
    let name: *mut c_char = cchars.as_mut_slice().as_mut_ptr(); 

    let dpy = unsafe { xlib::XOpenDisplay(name) };
    let root = unsafe { xlib::XDefaultRootWindow(dpy) };

    println!("Hello, world!");
}
