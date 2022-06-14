use std::io::stdin;
use rand::Rng;

fn guess_number() {
  println!("Hello, world!");
    let target = rand::thread_rng().gen_range(1..101);
    println!("target is {}", target);
    loop {
        // you have to declare guess in the loop, otherwise it will be a multi-line string
        // and can not be parsed to integer?
        let mut guess = String::new();
        println!("guess a number");
        match stdin().read_line(&mut guess) {
            Ok(_) => {
                let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(e) => { println!("not a number: {}", e); continue; },
                };
                println!("you guessed {}", guess);
                if guess == target {
                    println!("You win!");
                    break;
                } else if guess > target {
                    println!("Too high!");
                } else {
                    println!("Too low!");
                }

            }
            Err(_) => { println!("error"); },
        };
      }
}
