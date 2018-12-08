extern crate hmac;
extern crate sha1;
extern crate base64;

use self::hmac::{Hmac, Mac};
use self::sha1::Sha1;
use self::base64::encode_config;

type HmacSha1 = Hmac<Sha1>;

//struct RequestParams {
//    oauth_consumer_key: String,
//    oauth_nonce: String,
//    oauth_signature
//    oauth_signature_method
//    oauth_timestamp
//    oauth_token
//    params
//
//}

pub fn create_signature(key: &str, msg: &str) -> String {
    let mut hmac_sha1 = HmacSha1::new_varkey(key.as_bytes()).unwrap();
    hmac_sha1.input(msg.as_bytes());

    // move to static or singleton when know how!
    let config = base64::Config::new(
        base64::CharacterSet::Standard,
        true,
        true,
        base64::LineWrap::NoWrap
    );

    let encoded = base64::encode_config(&hmac_sha1.result().code(), config);

    return encoded;
}

#[cfg(test)]
mod tests {
    use super::create_signature;

    #[test]
    fn should_return_correct_hash() {
        let key = "key";
        let msg = "The quick brown fox jumps over the lazy dog";
        let expected = String::from("3nybhbi3iqa8ino29wqQcBydtNk=");

        let actual = create_signature(key, msg);

        assert_eq!(actual, expected);
    }

}