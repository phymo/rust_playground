// In languages like C, Go, and Rust, classes are not a feature.
// Instead, these languages use structs,
// which define only a group of properties

// derive stc::fmt::Debug for Structure;
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

// fn area(rect: &Rectangle) -> u32 {
//   rect.width * rect.height
// }

// implimentation of rectangle methods in struct
impl Rectangle {
  // method of instance with self
  fn area(self: &Rectangle) -> u32 {
  self.width * self.height
  }

  // no self => normal function => not method => Rectangle::aa()
  fn aa() {
    println!("aa");
  }

}

fn main() {
  let rect1 = Rectangle { width: 30, height: 50 };
  println!("rect1 is {:#?}", rect1);
  let rect2 = Rectangle { width: 10, height: 40 };
  println!("rect2 is {:#?}", rect2);
  println!("rect1 area is {}", &rect1.area());
  println!("rect2 area is {}", &rect2.area());
  println!("rect1 is {:#?}", rect1);
  println!("rect2 is {:#?}", rect2);
  Rectangle::aa();
}

