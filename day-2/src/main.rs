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
  // let input = "5 1 9 5
  //             7 5 3
  //             2 4 6 8";
  let input = read();

  let collection: Vec<&str> = input
   .lines()
   // .map(|x| x.trim())
   .collect();

  let map: u32 = collection
    .iter()
    .map(|x| {
      let row: Vec<u32> = x
        // easier to just do this then split_whitespace
        .split("\t")
        .map(|y| y.parse().unwrap())
        .collect();

      return row.iter().max().unwrap() - row.iter().min().unwrap();
    })
    .sum();

  println!("{:?}", map);
}

fn part_two() {
  // let input = "5 9 2 8
  //              9 4 7 3
  //              3 8 6 5";
  let input = read();

  let collection: Vec<&str> = input
   .lines()
   // .map(|x| x.trim())
   .collect();

  let checksum: u32 = collection
    .iter()
    .map(|x| {
      let rows: Vec<u32> = x
        .split("\t")
        .map(|y| y.parse().unwrap())
        .collect();

      let row_sum: u32 = rows
        .iter()
        .fold(0, |acc, row| {
          let row_clone = rows.clone();

          let filter: u32 = row_clone
          .iter()
          .filter_map(|a| {
            if row.checked_rem(*a).unwrap() == 0 && *a != *row {
              return Some(*row / *a)
            } else {
              return None;
            }
          })
          .sum();

          return acc + filter;
        });

      return row_sum;
    })
    .sum();

  println!("checksum is: {:?}", checksum);
}

fn main() {
  part_two();
  part_one();
}
