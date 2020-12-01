use ansi_term::Colour;
use difference::{Changeset, Difference};
use dissimilar;
use diffs;



pub fn print_diff_dissimilar(actual: &str, expected: &str) {
  let chunk_list = dissimilar::diff(actual, expected);
  for chunk in &chunk_list {
    match chunk {
      dissimilar::Chunk::Equal(part) => { println!("  {}", part) }
      dissimilar::Chunk::Insert(part) => { println!("{}", Colour::Green.paint(format!("+ {}", part))) }
      dissimilar::Chunk::Delete(part) => { println!("{}", Colour::Red.paint(format!("- {}", part))) }
    }
  }
}



pub fn print_diff_difference(actual: &String, expected: &String) {
  let changeset = Changeset::new(actual, expected, " ");
  for diff in &changeset.diffs {
    match diff {
      Difference::Same(part) => { println!("  {}", part) }
      Difference::Add(part) => { println!("{}", Colour::Green.paint(format!("+ {}", part))) }
      Difference::Rem(part) => { println!("{}", Colour::Red.paint(format!("- {}", part))) }
    }
  }
}



pub fn print_diff_diffs_myers(actual: &str, expected: &str) {

  // convert str to slice: s -> s.as_bytes()

  // based on diffs-0.4.0/src/test.rs

  //let a: &[usize] = &[0, 1, 2, 3, 4];
  //let b: &[usize] = &[0, 1, 2, 9, 4];

  let a = actual.as_bytes();
  let b = expected.as_bytes();

  // based on pijul/diff/diff.rs

  // D: tuple struct
  pub struct D <'a> (&'a str, &'a str);

  impl <'a> diffs::Diff for D <'a> {

    type Error = ();

    fn equal(&mut self, pos1: usize, pos2: usize, len: usize) -> std::result::Result<(), ()> {
      println!("equal   {:3?} {:3?} {:3?} {:3?}", pos1, pos2, len, &self.0[pos1..(pos1+len)]);
      Ok(())
    }

    fn replace(&mut self, pos1: usize, len1: usize, pos2: usize, len2: usize) -> Result<(), ()> {
      println!("replace {:3?} {:3?} {:3?} {:3?} {:3?} -> {:3?}", pos1, len1, pos2, len2,
        &self.0[pos1..(pos1+len1)], &self.1[pos2..(pos2+len2)]);
      Ok(())
    }

    fn delete(&mut self, pos1: usize, len1: usize, pos2: usize) -> std::result::Result<(), ()> {
      println!("delete  {:3?} {:3?} {:3?} {:3?}", pos1, len1, pos2, &self.0[pos1..(pos1+len1)]);
      Ok(())
    }

    fn insert(&mut self, pos1: usize, pos2: usize, len2: usize) -> std::result::Result<(), ()> {
      println!("insert  {:3?} {:3?} {:3?} {:3?}", pos1, pos2, len2, &self.1[pos2..(pos2+len2)]);
      Ok(())
    }
  }

  let mut diff = diffs::Replace::new(
    D(expected, actual) // init tuple struct
  );

  println!("a.len() = {}", a.len());
  println!("b.len() = {}", b.len());

  diffs::myers::diff(&mut diff, a, 0, a.len(), b, 0, b.len()).unwrap();

}



// diff: (ERROR (tag_name)) (text)

static S1: &'static str = "\
(fragment (ERROR (tag_name)) (text) (script_element (start_tag (tag_name)) (end_tag)))";

static S2: &'static str = "\
(fragment (script_element (start_tag (tag_name)) (end_tag)))";



fn main() {

  println!("\ndifference - Longest Common Subsequence (LCS) algorithm");
  print_diff_difference(&String::from(S1), &String::from(S2));

  println!("\ndissimilar - myers diff algorithm");
  print_diff_dissimilar(S1, S2);

  println!("\ndiffs - myers diff algorithm");
  print_diff_diffs_myers(S1, S2);

}
