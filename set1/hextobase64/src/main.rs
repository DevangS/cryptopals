/*
Convert hex to base64
The string:

49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
Should produce:

SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
*/

extern crate data_encoding;

use data_encoding::{HEXLOWER, DecodeError};
use data_encoding::BASE64;

fn hextobase64(input: &[u8;96]) -> Result<String, DecodeError>{

  
  let binary = HEXLOWER.decode(input)?;
  let result = BASE64.encode(&binary);

  println!("{}",result);

  Ok(result)
}

fn main() {
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_simple() {
    let input = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(hextobase64(input).unwrap(), expected);
  }
}