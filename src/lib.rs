extern crate hyper;
use std::sync::{Arc, Mutex};
use std::thread;
use hyper::Client;
use std::io::Read;

#[no_mangle]
pub extern fn run_threads() {
  let client = Arc::new(Client::new());
  let threads: Vec<_> = (0..5).map(|i| {
    let client = client.clone();
    thread::spawn(move || {
      println!("Requesting {}", i.to_string());
      let mut response = client.get("http://google.com").send().unwrap();
      let mut body = String::new();
      response.read_to_string(&mut body).unwrap();
      body.len().to_string()
    })
  }).collect();

  let responses: Vec<_> = threads.into_iter().map(|thread| thread.join())
                                               .collect();
  println!("All threads joined. Full responses are:");
  for response in responses.into_iter() {
    println!("The response contains the following headers: {:?}", response.ok());
  }
}
