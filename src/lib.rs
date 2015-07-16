pub struct TwoNumbers {
    first: i32,
    second: i32,
}

#[no_mangle]
pub extern fn add_struct_vals(numbers: TwoNumbers) -> i32 {
    numbers.first + numbers.second
}

#[test]
fn it_works() {
    let numbers = TwoNumbers { first: 10, second: 20 };
    assert!(add_struct_vals(numbers) == 30);
}
