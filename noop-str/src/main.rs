// TODO read input from files
// TODO split by line / whitespace / character

use ansi_term::Colour;



//pub fn print_diff_noop_str(actual: &String, expected: &String) {
pub fn print_diff_noop_str(actual: &str, expected: &str) {
  println!("s1 = {}", Colour::Green.paint(actual));
  println!("s2 = {}", Colour::Red.paint(expected));
}



const S1: &str =
"(fragment (ERROR (tag_name)) (text) (script_element (start_tag (tag_name)) (end_tag)))"
;

const S2: &str =
"(fragment (script_element (start_tag (tag_name)) (end_tag)))"
;



fn main() {

  println!("\nno operation - str type");
  //print_diff_noop_str(&String::from(S1), &String::from(S2));
  print_diff_noop_str(&S1, &S2);

}
