// fn main() {
//   let num = Some(4);
//   let split = 5;
//   match num {
//       Some(x) if x < split => assert!(x < split),
//       Some(x) => assert!(x >= split),
//       None => (),
//   }
// }

fn main() {
  let mut c: i32 = 0;
  let mut next: i32 = 1;

  {
     let rc = &mut c;
     next = *rc + 1;
  }
  println!("{}", next);
}
