fn loops() {
  let i: i32 = 0;
  // loop
  _loop(i);
  // while
  _while(i);
  // for
  _for();
  // for array
  let arr = [1, 2, 3, 4, 5];
  _for_array(arr);
}

fn _loop(mut i:i32) {
  loop {
    i += 1;
    println!("{}", i);
    if i >= 9 {
      break;
    }
  }
}

fn _while(mut i:i32) {
  while i < 10 {
    println!("{}", i);
    i += 1;
  }
}

fn _for() {
  for i in 0..10 {
    println!("{}", i);
  }
}

fn _for_array(arr: [i32; 5]) {
  for i in arr.iter() {
    println!("{}", i);
  }
}

