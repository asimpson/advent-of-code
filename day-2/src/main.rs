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
  // let input = "5 1 9 5
  //             7 5 3
  //             2 4 6 8";
  let input = read();

  let collection: Vec<&str> = input
   .lines()
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

// For each row, determine the difference between the largest value and the smallest value;
// the checksum is the sum of all of these differences.
// <cjm> asimpson: if you ever have a map operation that generates sub-iterators
//       and you want to collect all the results, flat_map is the tool for the
//       job
//       https://play.rust-lang.org/?gist=6635e844624410c10d1f1ac0258d3f23&version=stable
//                                                                         [10:33]
// <cjm> asimpson: but newlines do also count as whitespace, so you can just do
//       this
//       https://play.rust-lang.org/?gist=96aac505b4e50a3ced6925a81bf237cb&version=stable
// <lucasem> asimpson, cjm: you can also use str::split_whitespace directly
//           https://play.rust-lang.org/?gist=99437fb0b82491ff23a161ae4dae51a4&version=stable
// <qoxncyha> you dont need the ::<_>:
//            https://play.rust-lang.org/?gist=9d5f7d1ac03fe57bea882295dd8bb9c8&version=stable
//                                                                         [10:38]
// <asimpson> qoxncyha: oh right!                                          [10:39]
// <lucasem> (you also don't need the &str)
//           https://play.rust-lang.org/?gist=f525ebce22590b594d7182292b6e8cb5&version=stable
//                                                                         [10:42]
// <asimpson> lucasem: that brings up another question I've had before. Is it
//            considered best practice to declare the type if you know it or use
//            the _ whenever possible?                                     [10:44]
// <lucasem> asimpson: it depends on how obvious it is
// <lucasem> in this case, it's obviously a vec of &str, so it's fine to leave it
//           out                                                           [10:45]
// <qoxncyha> i like to specify types even when theyre not needed sometimes
// <asimpson> at that point it's a "developer ergonomic" thing rather than what's
//            best for the compiler.
// <lucasem> asimpson: yeah
// <lucasem> use code style / software design judgement                    [10:46]
