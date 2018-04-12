use std::fs::File;
use std::path::Path;
use std::io::Read;

fn read() -> String {
    let current_dir = std::env::current_dir().expect("Can't get current dir");
    let path = Path::new(&current_dir).join("src/input.txt");
    let mut input_file = File::open(&path).expect("Unable to open the input file.");
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();
    return input;
}

fn part_one() {
  let input = read();
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

  // this can be refactored to use `fold`.
  let result: i32 = s
    .iter()
    .filter_map(|x| if Some(x) == prev { Some(x) } else { prev = Some(x); None })
    .sum();

  println!("The captcha is: {:?}", result + circle.unwrap());
}

fn part_two() {
  let input = read();

  let numbers = input
  .trim_right()
  .chars()
  .map(|x| x.to_digit(10).unwrap())
  .collect::<Vec<_>>();

  let half = numbers.len() / 2;
  let num_iter = numbers.iter();

  let result = num_iter.enumerate()
    .fold(0, |acc, (i, &x)| {
      if numbers[(i + half) % numbers.len()] == x {
        acc + x
      } else {
        acc + 0
      }
    });

  println!("two: {:?}", result);
}

fn main() {
  // part_one();
  part_two();
}
