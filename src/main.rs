extern crate data_encoding;
extern crate reqwest;

use data_encoding::{HEXLOWER, DecodeError};
use data_encoding::BASE64;
use std::collections::HashMap;

/*
Convert lowercase hex to base64
 */
fn hextobase64(input: &str) -> Result<String, DecodeError>{
  Ok(BASE64.encode(&HEXLOWER.decode(input.as_bytes())?))
}

/*
XOR 2 equal length buffers together
 */
fn xor(left: &str, right: &str) ->Result<String, DecodeError>{
  assert_eq!(left.len(), right.len());
  let mut left_bytes = HEXLOWER.decode(left.as_bytes())?;
  let right_bytes = HEXLOWER.decode(right.as_bytes())?;

  left_bytes.iter_mut().
      zip(right_bytes.iter()).
      for_each(|(lb, rb)| *lb ^= *rb);
  Ok(HEXLOWER.encode(&left_bytes))
}

/*
Decrypt a string that has been XOR'd with a single character
 */
fn single_byte_xor_decrypt(input: &str) -> Result<String, DecodeError>{
  //create frequency map
  let _magic = "ETAOIN SHRDL";
  let mut counts: HashMap<u8, u32> = HashMap::with_capacity(_magic.len());
  for m in _magic.chars() {
    counts.insert(m as u8, 0);
  }

  //allocate space to track most likely valid plaintext
  let mut best_score = 0;
  let mut result = Vec::new();
  //convert hex input string to bytes
  let str_bytes = HEXLOWER.decode(input.as_bytes()).unwrap();

  //choose a 1 byte char from 'A' to 'Z'
  for c in 65..91{
    //allocate memory for this plaintext
    let mut plaintext = Vec::with_capacity(str_bytes.len());

    //reset frequency map to 0
    for (_, val) in counts.iter_mut() {
      *val = 0;
    }

    for e in str_bytes.iter(){
      let decrypted = *e ^ c;
      let mut key = decrypted;

      //try to capitalize if it's a possible lowercase letter
      if decrypted > 96 {
        key = decrypted - 32;
      }

      //Update frequency map with count of relevant chars
      if let Some(x) = counts.get_mut(&key){
        match x.checked_add(1) {
          Some(v) => {
            *x = v;
          }
          None => {}
        }
      }
      plaintext.push(decrypted);
    }

    let score = counts.values().sum();

    if score > best_score{
      best_score = score;
      result = plaintext.clone(); //should result be &mut Vec<u8> instead? TODO: learn more about ownership
    }
  }

  Ok(String::from_utf8(result).expect("invalid"))
}


fn fetch_url() -> Result<(), Box<dyn std::error::Error>> {
  let mut res = reqwest::blocking::get("https://cryptopals.com/static/challenge-data/4.txt")?;
  for s in res.text()?.lines(){
    println!("{}", s);
  }
  Ok(())
}

fn main() {
  fetch_url();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hextobase64_good() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    assert_eq!(hextobase64(input).unwrap(), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
  }

  #[test]
  fn test_xor_good(){
    let result = xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965");
    assert_eq!(result.unwrap(), "746865206b696420646f6e277420706c6179");
  }

  #[test]
  #[should_panic]
  fn test_xor_mismatched_str_length(){
    assert!(xor("1c0111001f010100061a024b53535009181", "686974207468652062756c6c277320657965").is_err(),
    "String length is mismatched");
  }

  #[test]
  fn test_single_byte_xor_decrypt(){
    assert_eq!(single_byte_xor_decrypt("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap(), "Cooking MC's like a pound of bacon");
  }
}