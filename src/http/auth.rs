extern crate hmac;
extern crate sha1;
extern crate base64;

use self::hmac::{Hmac, Mac};
use self::sha1::Sha1;
use std::time::Instant;

use TwitterConfig;

type HmacSha1 = Hmac<Sha1>;

pub fn create_signature(twitter_oauth: &TwitterConfig) -> String {
    // 1. HTTP Method POST
    // 2. Base Url https://api.twitter.com/1.1/statuses/update.json
    // 3. params raw

    let request_method = "POST";
    let base_url = percent_encode("https://stream.twitter.com/1.1/statuses/filter.json");    
    let mut parameters = String::new();   
    parameters.push_str("oauth_consumer_key=");
    parameters.push_str(&percent_encode(&twitter_oauth.oauth.consumer_key));    
    parameters.push_str("&oauth_nonce=");
    parameters.push_str(&percent_encode(&twitter_oauth.oauth.nonce));    
    parameters.push_str("&oauth_signature_method=");
    parameters.push_str(&percent_encode(&twitter_oauth.oauth.signature_method));    
    parameters.push_str("&oauth_timestamp=");
    parameters.push_str(&percent_encode(&Instant::now().elapsed().as_secs().to_string()));
    parameters.push_str("&oauth_token=");
    parameters.push_str(&percent_encode(&twitter_oauth.oauth.token));
    parameters.push_str("&oauth_version=");
    parameters.push_str(&percent_encode(&twitter_oauth.oauth.version));
    parameters.push_str("&track=");
    parameters.push_str(&percent_encode(&twitter_oauth.stream_track_params));

    let mut base_signature = String::new();
    base_signature.push_str(&request_method);
    base_signature.push_str("&");
    base_signature.push_str(&base_url);
    base_signature.push_str("&");
    base_signature.push_str(&percent_encode(&parameters));

    let mut signing_key = String::new();
    signing_key.push_str(&percent_encode(&twitter_oauth.oauth.consumer_secret_key));
    signing_key.push_str("&");
    signing_key.push_str(&percent_encode(&twitter_oauth.oauth.token_secret));

    let mut hmac_sha1 = HmacSha1::new_varkey(signing_key.as_bytes()).unwrap();
    hmac_sha1.input(base_signature.as_bytes());

    let config = base64::Config::new (
        base64::CharacterSet::Standard,
        true,
        true,
        base64::LineWrap::NoWrap
    );

    base64::encode_config(&hmac_sha1.result().code(), config)
}

pub fn percent_encode(src: &str) -> String {  
    let mut encoded = String::new();
    for character in src.chars() {
        if character.is_alphabetic() || character.is_ascii_digit() || "-._~".contains(character) {
            encoded.push(character);
        } else {
            let mut bytes = vec![0; character.len_utf8()];
            character.encode_utf8(&mut bytes);
            for byte in bytes.iter() {
                encoded.push_str(&format!("%{:X}", byte));
            }
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn should_return_correct_hash() {
    //     let key = "key";
    //     let msg = "The quick brown fox jumps over the lazy dog";
    //     let expected = String::from("3nybhbi3iqa8ino29wqQcBydtNk=");
    //     let actual = create_signature(key, msg);
    //     assert_eq!(actual, expected);
    // }

    #[test]
    fn should_percent_encode_src_containing_ascii() {
        let src = "Cats + Dogs-.,~";
        let expected = String::from("Cats%20%2B%20Dogs-.,~");
        let actual = percent_encode(&src);        
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_percent_encode_src_containing_unicode() {
        let src = "Snowman☃©";
        let expected = String::from("Snowman%E2%98%83%C2%A9");
        let actual = percent_encode(&src);
        assert_eq!(actual, expected);
    }

}