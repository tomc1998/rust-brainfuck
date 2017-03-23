use std::env;
use std::io::ErrorKind;
use std::char;

/// Load a file at the given path into a `String`
fn load_file(path: &str) -> Result<String, std::io::Error> {
  use std::io::Read;
  let mut data = String::new();
  try!(try!(std::fs::File::open(path)).read_to_string(&mut data));
  return Ok(data);
}

fn main() {
  let f = &String::from(env::args().skip(1).take(1).next().unwrap_or(String::from("")).trim());
  if f.is_empty() { println!("Please provide a valid filename."); return; }
  let src = load_file(f).map_err(|e| match e.kind() {
    ErrorKind::NotFound => String::from("No file with that name found."),
    _ => format!("Unknown error loading the file: {}", e),
  });
  if src.is_err() { println!("{}", src.unwrap_err()); return; }
  let res = interpret(&src.unwrap());
  if res.is_err() { println!("ERROR: {}", res.unwrap_err()); }
}

/// Interpret the given source.
fn interpret(src: &str) -> Result<(), String> {
  // Create memory
  let mut memory : [u32; 30000] = [0; 30000];
  let mut ptr: usize = 0; // Memory pointer
  let mut iptr: usize = 0; // Instruction pointer

  /// Function to set the iptr to the matching '[' or ']' character
  #[inline(always)]
  fn goto_matching_close_bracket(iptr: &mut usize, 
                                 ptr: usize, 
                                 src: &str,
                                 memory: [u32; 30000]) -> Result<(), String> {
    if memory[ptr] != 0 { return Ok(()); }
    let mut ii : i32 = 0;
    loop {
      *iptr += 1;
      while !src.is_char_boundary(*iptr) { *iptr += 1; } // Dealing with utf 8
      if *iptr > src.len() { return Err(String::from("Bracket not closed.")); }
      if src[*iptr..].chars().next().unwrap() == '[' { ii += 1; }
      if src[*iptr..].chars().next().unwrap() == ']' { ii -= 1; }
      if ii < 0 { return Ok(()); }
    }
  }

  /// Function to set the *iptr to the matching '[' or ']' character
  #[inline(always)]
  fn goto_matching_open_bracket(iptr: &mut usize, 
                                 ptr: usize, 
                                 src: &str,
                                 memory: [u32; 30000]) -> Result<(), String> {
    if memory[ptr] == 0 { return Ok(()); }
    let mut ii : i32 = 0;
    loop {
      *iptr -= 1;
      while !src.is_char_boundary(*iptr) { *iptr -= 1; } // Dealing with utf 8
      if *iptr > src.len() { return Err(String::from("Too many closing brackets.")); }
      if src[*iptr..].chars().next().unwrap() == ']' { ii += 1; }
      if src[*iptr..].chars().next().unwrap() == '[' { ii -= 1; }
      if ii < 0 { return Ok(()); }
    }
  }

  loop {
    if iptr >= src.len() { return Ok(()); }
    while !src.is_char_boundary(iptr) { iptr += 1; } // Dealing with utf 8
    let c = src[iptr..].chars().next().unwrap();
    match c {
      '>' => ptr += 1,
      '<' => ptr -= 1,
      '+' => memory[ptr] += 1,
      '-' => memory[ptr] -= 1,
      '.' => print!("{}", try!(char::from_u32(memory[ptr])
                               .ok_or(format!("Invalid UTF-8 char: {}", memory[ptr])))),
      ',' => (),
      '[' => try!(goto_matching_close_bracket(&mut iptr, ptr, src, memory)),
      ']' => try!(goto_matching_open_bracket(&mut iptr, ptr, src, memory)),
      _ => (),
    };
    iptr += 1;
  }
}
