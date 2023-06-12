use std::io::Read;

fn main() {
  encoding();
  decoding();
}

#[allow(dead_code)]
fn encoding() {
  println!("\n\n==> Starting the encoding process:\n\n");
  let lines = pull_lines();
  for l in 0..lines.len() { // pull each line
    let line = &lines[l];
    println!("Processing: {}\n", line);
    let chars = pull_chars(line);

    for c in 0..chars.len() { // pull each char
      let binary = find_binary(chars[c]);
      let hamming_binary = find_hamming_binary(binary);
      let token = find_token(&hamming_binary);
      print!("Character = {}, token = {:>4}, Hamming Code = ", chars[c], token);
      for b in 0..hamming_binary.len() {
        print!("{}", hamming_binary[b]);
      }
      println!("");
    }
    print!("\n\n\n");
  }
}

fn pull_lines() -> Vec<String> {
  let mut file = std::fs::File::open("Encoding Text.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut lines = Vec::new();

  for i in contents.split('\n'){
    lines.push(i.to_string());
  }
  lines
}

fn pull_chars(line: &str) -> Vec<char> {
  
  let mut chars = Vec::new();

  for c in line.chars(){
    chars.push(c);
  }
  chars
}

fn find_binary(c: char) -> Vec<u16>{
  let char_string = c.to_string();
  let char_slice = char_string.as_bytes(); //&[u8]
  let ascii: u8 = char_slice[0];
  let mut temp = ascii;

  //print!("{} ", temp);
  
  let mut char_binary = vec![0; 7];
  
  let mut index = 6;
  for i in 0..7 {
    let power = u8::pow(2, index);
    if temp >= power {
      char_binary[i] = 1;
      temp -= power;
    }
    if index > 0 {
      index -= 1;
    }
  }
  //add 4 redundant bits & additional bits for 16 bits
  char_binary.push(0);
  char_binary.push(0);
  char_binary.insert(6, 0);
  char_binary.insert(3, 0);
  char_binary.insert(0,0);
  char_binary.insert(0,0);
  char_binary.insert(0,0);
  char_binary.insert(0,0);
  char_binary.insert(0,0);

  char_binary
}

fn find_hamming_binary(v: Vec<u16>) -> Vec<u16>{
  let mut vec = v;

  if (vec[15] + vec[13] + vec[11] + vec[9] + vec[7] + vec[5]) % 2 == 1 { // find R1
    vec[15] = 1;
  }
  if (vec[14] + vec[13] + vec[10] + vec[9] + vec[6] + vec[5]) % 2 == 1 { // find R2
    vec[14] = 1;
  }
  if (vec[12] + vec[11] + vec[10] + vec[9]) % 2 == 1 { // find R4
    vec[12] = 1;
  }
  if (vec[8] + vec[7] + vec[6] + vec[5]) % 2 == 1 { // find R8
    vec[8] = 1;
  }
  
  vec
}

fn find_token(v: &Vec<u16>) -> u16{
  let vec = v;
  let mut token: u16 = Default::default();

  let mut index: u32 = 10;
  for i in 5..16 {
    let power = u16::pow(2, index);
    if vec[i] == 1 {
      token += power;
    }
    if index > 0 {
      index -= 1;
    }
  }
  token
}

#[allow(dead_code)]
fn decoding() {
  println!("==> Starting the decoding process:\n\n");
  let mut file = std::fs::File::open("Decoding Text.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  
  let mut lines = Vec::new();
  for i in contents.split('='){
    lines.push(i.to_string());
  }

  let mut count = 1;
  for chunk in lines {
    if !chunk.is_empty() {
      println!("\nProcessing line #{}", count);
      let ascii = char_to_ascii(&chunk);
      let num = ascii_to_int(&ascii);
      for x in 0..num.len() {
  
        let binary = binary(num[x]);
        convert(&binary);
        if !check(&binary) {
          println!("Error: {}", x);
        }
      }
      count += 1;
      println!("");
    }
  } 
}


fn char_to_ascii(chunk: &str) -> Vec<u8> {
  
  let mut ascii = Vec::new();

  for line in chunk.split('\n') {
    for byte in line.as_bytes() {
      ascii.push(*byte);
    }
  }
  ascii
}

fn ascii_to_int(ascii: &Vec<u8>) -> Vec<u16> {
  let mut nums = Vec::new();
  let mut current_num: u16 = 0;
  let mut previous_space = true;

  for &x in ascii {
    if x == 32 {
      if !previous_space {
        nums.push(current_num);
        current_num = 0;
      }
      previous_space = true;
    }
    else {
      let number = (x - 48) as u16;
      current_num = current_num * 10 + number;
      previous_space = false;
    }
  }

  if !previous_space {
    nums.push(current_num)
  }
  nums
}

fn binary(x: u16) -> Vec<u16> {

  let mut temp = x;
  let mut binary = Vec::new();

  for index in (0..16).rev() {
    let power = u16::pow(2, index);
    if temp >= power {
      binary.push(1);
      temp -= power;
    }
    else {
      binary.push(0);
    }
  }
  binary
}

fn check(binary: &Vec<u16>) -> bool {
  let mut check = true;
  let mut check_vec = vec![0; 4];
  let vec = binary;

  if (vec[15] + vec[13] + vec[11] + vec[9] + vec[7] + vec[5]) % 2 == 1 { // find R1
    check_vec[0] = 1;
    check = false;
  }
  if (vec[14] + vec[13] + vec[10] + vec[9] + vec[6] + vec[5]) % 2 == 1 { // find R2
    check_vec[1] = 1;
    check = false;
  }
  if (vec[12] + vec[11] + vec[10] + vec[9]) % 2 == 1 { // find R4
    check_vec[2] = 1;
    check = false;
  }
  if (vec[8] + vec[7] + vec[6] + vec[5]) % 2 == 1 { // find R8
    check_vec[0] = 1;
    check = false;
  }
  check
}

fn convert(binary: &Vec<u16>) {

  let mut vec = Vec::new();
  vec.push(binary[5]);
  vec.push(binary[6]);
  vec.push(binary[7]);
  vec.push(binary[9]);
  vec.push(binary[10]);
  vec.push(binary[11]);
  vec.push(binary[13]);
  
  let mut char_ascii: u16 = Default::default();

  let mut index = 6;
  for i in 0..7 {
    let power = u16::pow(2, index);
    if vec[i] == 1 {
      char_ascii += power;
    }
    if index > 0 {
      index -= 1;
    }
  }
  let c = char_ascii as u8 as char;
  print!("{}", c);
}