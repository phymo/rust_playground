fn ownership() {
  let s1 = String::from("hello");
  let s2 = s1;
  // the string's ownership is transfered to s2, so s1 is no longer valid
  println!("s2 is {}; ", s2);
}

fn ownership2() {
  let s1 = String::from("hello2");

  // s2 deep copies s1 from the heap, so s1 is still valid
  let s2 = s1.clone();
  println!("s1 is {}; s2 is {}", s1, s2);
}

fn ownership_function() {
  let s1 = String::from("hello");

  let s2 = takes_ownership(s1);
  // s1 in the function and out the function, then s1 is no longer valid
  // println!("s1 is {}", s1);

  // s2 is still valid, because it is a copy of s1 and returned by the function
  println!("s2 is {}", s2);
}

fn takes_ownership(some_string: String) -> String {
  println!("{}", some_string);

  // no ";" here, means return some_string
  some_string
}
