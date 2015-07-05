fn reverse(s: String) -> String {
  s.chars().rev().collect()
}

pub extern fn reverse(s: *const libc::c_char) -> *const libc::c_char {
  // reverse string and return
}

#[test]
fn it_works() {
  let string = "Don't use palindrome".to_string();
  assert!(
    reverse(string) == "emordnilap esu t'noD"
  );
}
