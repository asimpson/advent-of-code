use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn read() -> String {
    let current_dir = std::env::current_dir().expect("Can't get current dir");
    let path = Path::new(&current_dir).join("src/input.txt");
    let mut input_file = File::open(&path).expect("Unable to open the input file.");
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();
    return input;
}

fn main() {
  // let input = read();
  // loop over each digit
  // find every digit that is the same as the one ahead of it
  // add these special digits together

  let input = read();
  // let input = "91212129";
  let p: Vec<_> = input.split("").collect();

  let s = p
    .iter()
    .filter(|&&x| x != "" && x != "\n")
    .map(|x| x.parse::<i32>().expect("maps String to i32 integer."))
    .collect::<Vec<_>>();

  let mut prev = None;
  let end = s.iter().last();
  let start = s.iter().nth(0);
  let circle: Option<&i32> = if end == start { start } else { None };

  let result: i32 = s
    .iter()
    .filter_map(|x| if Some(x) == prev { Some(x) } else { prev = Some(x); None })
    .sum();

  // println!("{:?}", circle.unwrap())
  println!("The captcha is: {:?}", result + circle.unwrap());
}
