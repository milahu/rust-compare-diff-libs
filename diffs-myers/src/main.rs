// TODO
// https://nest.pijul.com/pijul/pijul/discussions/153

use ansi_term::Colour;



pub fn print_diff_diffs_myers(a_str: &str, b_str: &str) {

  // convert str to slice: s -> s.as_bytes()

  // based on diffs-0.4.0/src/test.rs

  //let a: &[usize] = &[0, 1, 2, 3, 4];
  //let b: &[usize] = &[0, 1, 2, 9, 4];

  let a = a_str.as_bytes();
  let b = b_str.as_bytes();

  // based on pijul/diff/diff.rs

  pub struct D <'a> {
    a: &'a str,
    b: &'a str
  };

  impl <'a> diffs::Diff for D <'a> {

    type Error = ();

    fn equal(&mut self, pos1: usize, pos2: usize, len: usize) -> std::result::Result<(), ()> {
      println!("  equal   {:3?} {:3?} {:3?} {:3?}", pos1, pos2, len, &self.a[pos1..(pos1+len)]);
      Ok(())
    }

    fn replace(&mut self, pos1: usize, len1: usize, pos2: usize, len2: usize) -> Result<(), ()> {
      // yellow: assert black background / dark mode
      // white background -> blue is better
      println!("{}", Colour::Yellow.paint(format!(
        "  replace {:3?} {:3?} {:3?} {:3?} {:3?} -> {:3?}",
        pos1, len1, pos2, len2,
        &self.a[pos1..(pos1+len1)], &self.b[pos2..(pos2+len2)]
      )));
      Ok(())
    }

    fn delete(&mut self, pos1: usize, len1: usize, pos2: usize) -> std::result::Result<(), ()> {
      println!("{}", Colour::Red.paint(format!(
        "- delete  {:3?} {:3?} {:3?} {:3?}",
        pos1, len1, pos2, &self.a[pos1..(pos1+len1)]
      )));
      Ok(())
    }

    fn insert(&mut self, pos1: usize, pos2: usize, len2: usize) -> std::result::Result<(), ()> {
      println!("{}", Colour::Green.paint(format!(
        "+ insert  {:3?} {:3?} {:3?} {:3?}",
        pos1, pos2, len2, &self.b[pos2..(pos2+len2)]
      )));
      Ok(())
    }

    fn finish(&mut self) -> std::result::Result<(), ()> {
      println!("  finish");
      Ok(())
    }

  }

  let mut diff = diffs::Replace::new(
    D{ a: a_str, b: b_str }
  );

  //println!("a.len() = {}", a.len()); println!("b.len() = {}", b.len());

  diffs::myers::diff(&mut diff, a, 0, a.len()-1, b, 0, b.len()-1).unwrap();

}



// diff: (ERROR (tag_name)) (text)

static S1: &'static str = "\
(fragment (ERROR (tag_name)) (text) (script_element (start_tag (tag_name)) (end_tag)))";

static S2: &'static str = "\
(fragment (script_element (start_tag (tag_name)) (end_tag)))";



fn main() {

  println!("\ndiffs - myers diff algorithm");

  // diff header
  println!("{}\n{}",
    Colour::Green.paint("+++ expected"),
    Colour::Red.paint("--- actual"),
  );

  print_diff_diffs_myers(S1, S2);

}
