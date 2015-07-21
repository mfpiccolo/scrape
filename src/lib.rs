#![feature(libc)]
#![feature(cstr_memory)]
extern crate libc;

use std::ffi::CString;
use std::mem;

pub struct TwoNumbers {
    first: i32,
    second: i32,
}

impl TwoNumbers {
    fn plus_one_to_each(self) -> TwoNumbers {
        let mut tn = self;
        tn.first += 1;
        tn.second += 1;
        tn
    }
}

#[no_mangle]
pub extern fn add_one_to_vals(numbers: TwoNumbers) -> TwoNumbers {
   numbers.plus_one_to_each()
}

#[no_mangle]
pub extern fn add_struct_vals(numbers: TwoNumbers) -> i32 {
    numbers.first + numbers.second
}

#[repr(C)]
pub struct RubyArray {
  len: libc::size_t,
  data: *const libc::c_void,
}

impl RubyArray {
  fn from_vec<T>(vec: Vec<T>) -> RubyArray {
    let array = RubyArray { data: vec.as_ptr() as *const libc::c_void, len: vec.len() as libc::size_t };
    mem::forget(vec);
    array
  }
}

#[no_mangle]
pub extern fn number_to_char_array() -> RubyArray {
  let mut utf_chars: Vec<*const libc::c_char> = vec![];

  for i in 33..126 {
    let maybe_char: Option<char> = std::char::from_u32(i);
    utf_chars.push(CString::new(maybe_char.unwrap().to_string()).unwrap().into_ptr());
  }

  RubyArray::from_vec(utf_chars)
}

#[no_mangle]
pub extern fn print_chars() {
  for i in 33..126 {
    println!("{:?}", std::char::from_u32(i).unwrap());
  }
}

#[test]
fn it_works() {
    let numbers = TwoNumbers { first: 10, second: 20 };
    assert!(add_struct_vals(numbers) == 30);
}
