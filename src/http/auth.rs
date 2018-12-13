extern crate hmac;
extern crate sha1;
extern crate base64;

use self::hmac::{Hmac, Mac};
use self::sha1::Sha1;
use self::base64::encode_config;

type HmacSha1 = Hmac<Sha1>;

// HTTP Method 	POST
// Base Url https://api.twitter.com/1.1/statuses/update.json
// params raw

pub fn create_signature(key: &str, msg: &str) -> String {
    let mut hmac_sha1 = HmacSha1::new_varkey(key.as_bytes()).unwrap();
    hmac_sha1.input(msg.as_bytes());

    // move to static or singleton when know how!
    let config = base64::Config::new (
        base64::CharacterSet::Standard,
        true,
        true,
        base64::LineWrap::NoWrap
    );

    base64::encode_config(&hmac_sha1.result().code(), config)
}

pub fn percent_encode(src: &str) -> String {
    let reserved_chars = "-.,~";  
    let mut encoded = String::new();

    for letter in src.chars() {
        if letter.is_ascii_alphabetic() || 
           letter.is_ascii_digit() || 
           reserved_chars.find(letter) != None {
               encoded.push(letter);
        } else {

        }
    }

    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_hash() {
        let key = "key";
        let msg = "The quick brown fox jumps over the lazy dog";
        let expected = String::from("3nybhbi3iqa8ino29wqQcBydtNk=");
        let actual = create_signature(key, msg);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_percent_encode_src_string() {
        let src = "Cats + Dogs";
        let expected = String::from("Cats%20%2B%20Dogs");
        let actual = percent_encode(&src);        
        assert_eq!(actual, expected);
    }
}