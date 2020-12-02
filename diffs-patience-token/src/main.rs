// same as diffs-myers-token, only replace "myers" with "patience"

use ansi_term::Colour;



pub struct D <'a> {
  a: &'a Vec<&'a str>,
  b: &'a Vec<&'a str>
}

impl <'a> diffs::Diff for D <'a> {

  type Error = ();

  fn equal(&mut self, pos1: usize, _pos2: usize, len: usize) -> std::result::Result<(), ()> {
    println!(
      "  {}", &self.a[pos1..(pos1+len)].join(" ")
    );
    Ok(())
  }

  fn replace(&mut self, pos1: usize, len1: usize, pos2: usize, len2: usize) -> Result<(), ()> {
    self.insert(pos1, pos2, len2).ok(); // .ok() = ignore errors
    self.delete(pos1, len1, pos2).ok();
    Ok(())
  }

  fn delete(&mut self, pos1: usize, len1: usize, _pos2: usize) -> std::result::Result<(), ()> {
    println!("{}", Colour::Red.paint(format!(
      "- {}", &self.a[pos1..(pos1+len1)].join(" ")
    )));
    Ok(())
  }

  fn insert(&mut self, _pos1: usize, pos2: usize, len2: usize) -> std::result::Result<(), ()> {
    println!("{}", Colour::Green.paint(format!(
      "+ {}", &self.b[pos2..(pos2+len2)].join(" ")
    )));
    Ok(())
  }

  /*
  fn finish(&mut self) -> std::result::Result<(), ()> {
    println!("finish");
    Ok(())
  }
  */

}



pub fn print_diff_diffs_patience_token(a_str: &str, b_str: &str) {

  // compare tokens, split by single space
  // collect: iterator -> vector
  let a: Vec<&str> = a_str.split(" ").collect();
  let b: Vec<&str> = b_str.split(" ").collect();

  //println!("a.len() = {}", a.len()); println!("b.len() = {}", b.len());

  let mut diff = diffs::Replace::new( D{ a: &a, b: &b } );

  diffs::patience::diff(&mut diff, &a, 0, a.len()-1, &b, 0, b.len()-1).unwrap();

}



// diff: (ERROR (tag_name)) (text)

static S1: &'static str =
"(fragment (ERROR (tag_name)) (text) (script_element (start_tag (tag_name)) (end_tag)))"
;

static S2: &'static str =
"(fragment (script_element (start_tag (tag_name)) (end_tag)))"
;



fn main() {

  println!("\ndiffs - patience diff algorithm - compare tokens\n");

  // diff header
  println!("{}\n{}",
    Colour::Green.paint("+++ expected"),
    Colour::Red.paint("--- actual"),
  );

  print_diff_diffs_patience_token(S1, S2);

}
