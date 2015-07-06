#![feature(libc)]
#![feature(cstr_to_str)]
#![feature(cstr_memory)]
extern crate libc;

use std::ffi::{CStr, CString};
#[no_mangle]
pub extern fn ruby_reverse(s: *const libc::c_char) -> *const libc::c_char {
  let str = ruby_string_to_ref_str(s);
  let string: String = str.chars().rev().collect();
  string_to_ruby_string(string)
}

#[no_mangle]
pub extern fn concat(s1: *const libc::c_char, s2: *const libc::c_char) -> *const libc::c_char {
    let s1_and_str = ruby_string_to_ref_str(s1);
    let s2_and_str = ruby_string_to_ref_str(s2);
    let s1_string = s1_and_str.to_string();
    let string: String = s1_string + s2_and_str;
    string_to_ruby_string(string)
}

fn ruby_string_to_ref_str<'a>(r_string: *const libc::c_char) -> &'a str {
  unsafe { CStr::from_ptr(r_string) }.to_str().unwrap()
}

fn string_to_ruby_string(string: String) -> *const libc::c_char {
  CString::new(string).unwrap().into_ptr()
}
