extern crate data_encoding;

use data_encoding::{HEXLOWER, DecodeError};
use data_encoding::BASE64;

/*
Convert lowercase hex to base64
 */
fn hextobase64(input: &[u8;96]) -> Result<String, DecodeError>{
  Ok(BASE64.encode(&HEXLOWER.decode(input)?))
}

fn main() {
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple() {
    let input = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    assert_eq!(hextobase64(input).unwrap(), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";);
  }
}