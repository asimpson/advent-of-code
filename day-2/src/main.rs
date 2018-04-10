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

fn main() {
  let input = "5 1 9 5
              7 5 3
              2 4 6 8";

  let collection: Vec<&str> = input
   .split('\n')
   .map(|x| x.trim())
   .collect();

  let map: u32 = collection
    .iter()
    .map(|x| {
      let row: Vec<u32> = x
        .split(" ")
        .map(|y| y.parse().unwrap())
        .collect();

      return row.iter().max().unwrap() - row.iter().min().unwrap();
    })
    .sum();

  println!("{:?}", map);
}

// For each row, determine the difference between the largest value and the smallest value;
// the checksum is the sum of all of these differences.
