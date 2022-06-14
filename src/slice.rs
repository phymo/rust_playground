fn slice() {
  // &[0..3] means the first 3 elements of the array,
  // without the ownership, so the array is not changed
  let a = [1, 2, 3, 4, 5];
  let str = String::from("Hello world");
  let slice = &a[0..3];
  // {:?} is used to print the array
  // {:#?} is used to print the array pretty
  println!("{:?}", slice);
  slice_str(&str[..]);
}

fn slice_str(str: &str) {
  // &str is a slice of &u8, so it can be used as a slice of i8
  let slice = &str[0..3];
  println!("{}", slice);
}
