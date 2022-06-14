use std::env;
use std::fs;
use std::process;
use std::error::Error;
fn grep_demo() {
  let args: Vec<String>  = env::args().collect();
  let config: Config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });
  if let Err(e) = run(config) {
    println!("Application error: {}", e);
    process::exit(1);
  };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(config.filename).expect("Something goes wrong");

  // println!("{:?}", content);

  for line in search(&config.query, &content) {
    println!("line: {}", line);
  };
  Ok(())
}

struct Config {
  query: String,
  filename: String,
}

impl Config {
  fn new(args: &Vec<String>) -> Result<Config, &'static str> {
    if args.len() <3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config {
      query,
      filename
    })
  }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in content.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}

