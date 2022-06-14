use crate::List::*;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
  fn new() -> List {
    Nil
  }

  fn prepend(self, elem: i32) -> List {
    Cons(elem, Box::new(self))
  }

  fn len(&self) -> u32 {
    // &self
    // *self
    // self
    println!("self{:?}", self);
    println!("*self{:?}", *self);
    println!("&self{:?}", &self);
    match *self {
      Cons(_, ref tail) => 1 + tail.len(),
      Nil => 0,
    }
  }

  fn stringify(&self) -> String{
    match *self {
      Cons(head, ref tail) => {
        // `format!` is similar to `print!`, but returns a heap
        // allocated string instead of printing to the console
        format!("{}, {}", head, tail.stringify())
      }
      Nil => {
        format!("Nil")
      }
    }
  }
}

fn main () {
  let mut list = List::new();
  list = list.prepend(1);
  list = list.prepend(2);
  list = list.prepend(3);
  println!("{}", list.len());
  println!("{}", list.stringify());
}
