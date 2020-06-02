extern crate data_encoding;

use data_encoding::{HEXLOWER, DecodeError};
use data_encoding::BASE64;

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

fn main() {
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
}