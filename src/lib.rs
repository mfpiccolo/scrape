#[no_mangle]
pub extern fn sum(x: i32, y: i32) -> i32 {
  x + y
}

#[test]
fn it_works() {
  assert!(sum(1, 2) == 3);
}
