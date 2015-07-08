#![feature(libc)]
#![feature(cstr_to_str)]
#![feature(cstr_memory)]

extern crate hyper;
extern crate libc;

use hyper::Client;
use hyper::header::Connection;
use std::io::Read;
use std::ffi::{CStr,CString};

#[no_mangle]
pub extern fn get_body(c_url: *const libc::c_char) -> *const libc::c_char {
  let url = ruby_string_to_ref_str(c_url);
  let client = Client::new();
  let temp_resp = client.get(url).header(Connection::close()).send();
  let mut resp = temp_resp.unwrap();
  let mut body = String::new();
  resp.read_to_string(&mut body).unwrap();
  string_to_ruby_string(body)
}

fn ruby_string_to_ref_str<'a>(r_string: *const libc::c_char) -> &'a str {
  unsafe { CStr::from_ptr(r_string) }.to_str().unwrap()
}

fn string_to_ruby_string(string: String) -> *const libc::c_char {
  CString::new(string).unwrap().into_ptr()
}
