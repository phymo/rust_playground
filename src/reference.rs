// reference and borrow
fn reference() {
  let s1 = String::from("hello");

  // let s2 = s1; will cause ownership move;
  // let s2 = s1.clone(); also copy heap data, but not move ownership;
  // just borrow s1, so s1 is valid
  // & means reference, &s1 means the address of s1, without the ownership,
  // so the value of s1 is not changed
  let s2 = &s1;
  let len = calculate_length(&s1);
  println!("s1 is {}, s2 is {}, length is {}", s1, s2, len);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}


// mutable reference
// in one scope, only one mutable reference is allowed;
