use ansi_term::Colour;

//use dissimilar;



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



// diff: (ERROR (tag_name)) (text)

static S1: &'static str = "\
(fragment (ERROR (tag_name)) (text) (script_element (start_tag (tag_name)) (end_tag)))";

static S2: &'static str = "\
(fragment (script_element (start_tag (tag_name)) (end_tag)))";



fn main() {

  println!("\ndissimilar - myers diff algorithm\n");

  // diff header
  println!("{}\n{}",
    Colour::Green.paint("+++ expected"),
    Colour::Red.paint("--- actual"),
  );

  print_diff_dissimilar(S1, S2);

}
