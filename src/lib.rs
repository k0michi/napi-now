#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn now() -> u128 {
  use std::time::SystemTime;

  match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
      Ok(n) => n.as_nanos(),
      Err(_) => panic!("SystemTime before UNIX EPOCH!"),
  }
}