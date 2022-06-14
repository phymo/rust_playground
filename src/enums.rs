#![allow(dead_code)]

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y:i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// impliment the method of Message
impl Message {
  fn call(&self) {
    println!("{:?}", self);
  }
}

enum Status {
  Rich,
  Poor,
}

enum Work {
  Civilian,
  Soldier,
}

fn main () {
  let m = Message::Write(String::from("hello"));
  m.call();

  // explicit use each name of enum, so they are available without manual scoping;
  use Status::{Poor, Rich};
  use crate::Work::*;
  // equivalent to: `Status::Poor` and `Work::Civilian`
  let status = Poor;
  let work = Civilian;

  match status {
    Rich => println!("The rich have lots of money!"),
    Poor => println!("The poor have no money..."),
  }

  test();
}

// Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.
// This enum is Option<T>, and it is defined by the standard library
// you can use Some and None directly without the Option:: prefix.
// enum Option<T> {
//   None,
//   Some(T),
// }

fn test() {
  let x: i8 = 5;
  let y: Option<i8> = Some(5i8);
  // no implementation for `i8 + Option<i32>
  // let sum = x + y;
  let z: i8 = y.unwrap();
  let sum = x + z;
  println!("z: {}", sum);
}
