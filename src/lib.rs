#![feature(test)]
extern crate hyper;
extern crate test;
use hyper::Client;
use std::io::Read;
extern crate time;

#[no_mangle]
pub extern fn run_threads() {
  let start_time = time::now();
  for i in 0..5 {
    let client = Client::new();
    println!("Requesting {}", i.to_string());
    let mut response = client.get("http://wikipedia.com/").send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    println!("BodyLength: {}", body.len().to_string());
  }
  let end_time = time::now();
  println!("{:?}", (end_time - start_time));
}
