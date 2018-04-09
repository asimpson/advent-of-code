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

// <CognitiveRadiation> asimpson: there's nothign wrong with calling .iter()
//                  multiple times
// <CognitiveRadiation> asimpson: although now that I look more closely, you're
//                  calling .collect() on an interator
// <CognitiveRadiation> and then calling .iter() on it after it's been collected
// <CognitiveRadiation> which is probably unnecessary
// <sarnold> asimpson: putting an .iter() inside a .fold() like that feels a bit
//           like you're going to get a quadratic runtime out of the
//           solution. that's not hte worst thing in the world if the inputs are
//           always ~ten chars :) but I think you're right, there's probably
//           something simpler                                             [14:54]
// <asimpson> sarnold: yeah it felt weird, but I needed to refer to a point in
//            the list I'm folding on.
// <aghast> asimpson: You cloned s. So your closure could just check s[i + half]
// <aghast> (well, %n)                                                     [14:57]
// <asimpson> aghast: ah ok
// <sarnold> aghast: hah, ten points for spotting the easier solution :D

// https://www.reddit.com/r/rust/comments/8aw450/hey_rustaceans_got_an_easy_question_ask_here/dx2j421/?context=0
// Also, you probably shouldn't use Iterator::nth if you can avoid it. It can be implemented as a literal "do n steps, one at a time". Sometimes it is efficient, but don't assume that.
// Also also, you can't re-use iterators like that. Iterator combinators consume the iterator they're called on. If you want to use an iterator more than once, you need to either re-create it each time, or clone it from a saved checkpoint, assuming the iterator supports cloning.
