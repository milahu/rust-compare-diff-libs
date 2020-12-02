use ansi_term::Colour;

use difference::{Changeset, Difference};



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



// diff: (ERROR (tag_name)) (text)

const S1: &str =
"(fragment (ERROR (tag_name)) (text) (script_element (start_tag (tag_name)) (end_tag)))"
;

const S2: &str =
"(fragment (script_element (start_tag (tag_name)) (end_tag)))"
;



fn main() {

  println!("\ndifference - Longest Common Subsequence (LCS) algorithm\n");

  // diff header
  println!("{}\n{}",
    Colour::Green.paint("+++ expected"),
    Colour::Red.paint("--- actual"),
  );

  print_diff_difference(&String::from(S1), &String::from(S2));

}
