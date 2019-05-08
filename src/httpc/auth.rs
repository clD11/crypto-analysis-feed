extern crate hmac;
extern crate sha1;
extern crate base64;

use self::hmac::{Hmac, Mac};
use self::sha1::Sha1;
use std::time::Instant;

use TwitterConfig;
use OAuth;

type HmacSha1 = Hmac<Sha1>;

pub fn create_authorization_header(twitter_oauth: &TwitterConfig) -> String {

    let request_method = "POST";
    let base_url = percent_encode("https://stream.twitter.com/1.1/statuses/filter.json");

    let oauth_consumer_key = format!("oauth_consumer_key={}", percent_encode(&twitter_oauth.oauth.consumer_key));
    let oauth_nonce = format!("oauth_nonce={}", percent_encode(&twitter_oauth.oauth.nonce));
    let oauth_signature_method = format!("oauth_signature_method={}", percent_encode(&twitter_oauth.oauth.signature_method));
    let oauth_timestamp = format!("oauth_timestamp={}", percent_encode(&Instant::now().elapsed().as_secs().to_string()));
    let oauth_token = format!("oauth_token={}", percent_encode(&twitter_oauth.oauth.token));
    let oauth_version = format!("oauth_version={}", percent_encode(&twitter_oauth.oauth.version));
    
    let mut oauth_params = vec![oauth_consumer_key, oauth_nonce, oauth_signature_method, oauth_timestamp, oauth_token, oauth_version];
    
    // 1. create oauth_signature
    let parameters = format!("{0}&track={1}", &oauth_params.join("&"), percent_encode(&twitter_oauth.stream_track_params));
    let oauth_signature = format!("oauth_signature={}", percent_encode(&sign(&request_method, &base_url, &parameters, &twitter_oauth)));    
    oauth_params.push(oauth_signature);

    // 3. create OAuth header
    let authorization_header = format!("OAuth {}", &oauth_params.join(", "));

    authorization_header
}

fn sign(request_method: &str, base_url: &str, parameters: &str, twitter_oauth: &TwitterConfig) -> String {
    let base_signature = format!("{0}&{1}&{2}", &request_method, &base_url, &percent_encode(&parameters));

    let signing_key = format!("{0}&{1}", &percent_encode(&twitter_oauth.oauth.consumer_secret_key),
        &percent_encode(&twitter_oauth.oauth.token_secret));

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

fn percent_encode(src: &str) -> String {  
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

    #[test]
    fn should_percent_encode_src_containing_unicode() {
        let src = "Snowman☃©-._~ ";
        let expected = String::from("Snowman%E2%98%83%C2%A9-._~%20");
        let actual = percent_encode(&src);
        assert_eq!(actual, expected);
    }

//     #[test]
//     fn should_create_authorization_header() {
//         let twitter_config =
//             TwitterConfig {
//                 stream_track_params: String::from("random"),
//                 oauth: OAuth {
//                     consumer_key: String::from("xvz1evFS4wEEPTGEFPHBog"),
//                     consumer_secret_key: String::from("test"),
//                     nonce: String::from("kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg"),
//                     signature_method: String::from("HMAC-SHA1"),
//                     token: String::from("370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb"),
//                     token_secret: String::from("test"),
//                     version: String::from("1.0")
//             }
//         };
//         let actual = create_authorization_header(&twitter_config);
//     }

}