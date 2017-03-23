use std::env;
use std::io::ErrorKind;
use std::char;
use std::collections::HashMap;

/// Load a file at the given path into a `String`
fn load_file(path: &str) -> Result<String, std::io::Error> {
  use std::io::Read;
  let mut data = String::new();
  try!(try!(std::fs::File::open(path)).read_to_string(&mut data));
  return Ok(data);
}

fn main() {
  // Parse arguments and load file.
  let f = &String::from(env::args().skip(1).take(1).next().unwrap_or(String::from("")).trim());
  if f.is_empty() { println!("Please provide a valid filename."); return; }
  let src = load_file(f).map_err(|e| match e.kind() {
    ErrorKind::NotFound => String::from("No file with that name found."),
    _ => format!("Unknown error loading the file: {}", e),
  });
  if src.is_err() { println!("{}", src.unwrap_err()); return; }

  // Interpret source.
  let res = interpret(&src.unwrap());
  if res.is_err() { println!("ERROR: {}", res.unwrap_err()); }
}

/// Interpret the given source.
fn interpret(src: &str) -> Result<(), String> {
  // Create memory
  let mut memory : [u32; 30000] = [0; 30000];
  let mut ptr: usize = 0; // Memory pointer
  let mut iptr: usize = 0; // Instruction pointer
  let mut goto_map = HashMap::new();

  // Get input buffer
  use std::io;
  let mut in_buf = String::new();

  // Calculate gotos and store in the goto_map
  let mut pos_stack : Vec<usize> = Vec::new();
  let mut ii : usize = 0;
  loop {
    while !src.is_char_boundary(ii) { ii += 1; }
    if ii >= src.len() { break; }
    let c = src[ii..].chars().next().unwrap();
    match c {
      '[' => pos_stack.push(ii),
      ']' => {
        let p = try!(pos_stack.pop().ok_or(String::from("Too many closing brackets.")));
        goto_map.insert(p, ii);
        goto_map.insert(ii, p);
      },
      _ => (),
    }
    ii += 1;
  }
  if pos_stack.len() > 0 { return Err(String::from("Unmatched bracket")); }

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
      ',' => {
        if in_buf.len() == 0 { 
          try!(io::stdin().read_line(&mut in_buf).map_err(|e|format!("{}", e))) ;
        }
        memory[ptr] = in_buf[0..1].chars().next().unwrap() as u32;
        in_buf = in_buf[1..].to_string();
      }


      '[' if memory[ptr] == 0 => iptr = *goto_map.get(&iptr).unwrap(),
      ']' if memory[ptr] != 0 => iptr = *goto_map.get(&iptr).unwrap(),
      _ => (),
    };
    iptr += 1;
  }
}
