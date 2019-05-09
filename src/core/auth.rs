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
//
//    let collected_params = collect_parameters(&twitter_oauth);
//
//
//    let request_method = "POST";
//    let base_url = percent_encode("https://stream.twitter.com/1.1/statuses/update.json");
//
//
//
//    let mut oauth_params = vec![oauth_consumer_key, oauth_nonce, oauth_signature_method, oauth_timestamp, oauth_token, oauth_version];
//
//    // 1. create oauth_signature
//    let parameters = format!("{0}&status={1}", &oauth_params.join("&"), percent_encode(&twitter_oauth.stream_track_params));
//    let oauth_signature = format!("oauth_signature={}", percent_encode(&sign(&request_method, &base_url, &parameters, &twitter_oauth)));
//    oauth_params.push(oauth_signature);
//
//    // 3. sort lexie
//    oauth_params.sort();
//
//    // 4. create OAuth header
//    let authorization_header = format!("OAuth {}", &oauth_params.join(", "));
//
//    authorization_header
    let r = String::from("test");
    r
}

//fn sign(request_method: &str, base_url: &str, parameters: &str, twitter_oauth: &TwitterConfig) -> String {
//    let base_signature = format!("{0}&{1}&{2}", &request_method, &base_url, &percent_encode(&parameters));
//
//    let signing_key = format!("{0}&{1}", &percent_encode(&twitter_oauth.oauth.consumer_secret_key),
//        &percent_encode(&twitter_oauth.oauth.token_secret));
//
//    let mut hmac_sha1 = HmacSha1::new_varkey(signing_key.as_bytes()).unwrap();
//    hmac_sha1.input(base_signature.as_bytes());
//
//    let config = base64::Config::new (
//        base64::CharacterSet::Standard,
//        true,
//        true,
//        base64::LineWrap::NoWrap
//    );
//
//    base64::encode_config(&hmac_sha1.result().code(), config)
//}

fn percent_encode(src: &str) -> String {  
    let mut encoded = String::new();
    for character in src.chars() {
        if character.is_alphabetic() || character.is_ascii_digit() || "-._~".contains(character) {
            encoded.push(character);
        } else {
            let mut bytes = vec![0; character.len_utf8()];
            character.encode_utf8(&mut bytes);
            for byte in bytes.iter() {
                encoded.push_str(&format!("%{:x}", byte));
            }
        }
    }
    encoded
}

fn collect_parameters(twitter_oauth: &TwitterConfig) -> String {
    let include_entities = format!("include_entities={}", percent_encode("true"));
    let oauth_consumer_key = format!("oauth_consumer_key={}", percent_encode(&twitter_oauth.oauth.consumer_key));
    let oauth_nonce = format!("oauth_nonce={}", percent_encode(&twitter_oauth.oauth.nonce));
    let oauth_signature_method = format!("oauth_signature_method={}", percent_encode(&twitter_oauth.oauth.signature_method));
    let oauth_timestamp = format!("oauth_timestamp={}", "1318622958"); //percent_encode(&Instant::now().elapsed().as_secs().to_string()));
    let oauth_token = format!("oauth_token={}", percent_encode(&twitter_oauth.oauth.token));
    let oauth_version = format!("oauth_version={}", percent_encode(&twitter_oauth.oauth.version));

    let mut oauth_params = vec![include_entities, oauth_consumer_key, oauth_nonce, oauth_signature_method, oauth_timestamp, oauth_token, oauth_version];
    let parameters = format!("{0}&status={1}", &oauth_params.join("&"), percent_encode(&twitter_oauth.stream_track_params));

    parameters
}

fn create_base_string(request_method: &str, base_uri: &str, parameters: String) {

}

//fn sign(twitter_config: &TwitterConfig) {
//    let base_uri =
//    let signing_key = format!("{0}&{1}", &twitter_config.oauth.consumer_key, &twitter_config.oauth.token_secret);
//
//}

#[cfg(test)]
mod tests {    
    use super::*;

    #[test]
    fn should_create_signature() {
        let twitter_config = twitter_config();
        let actual = sign(base_uri, twitter_config);
    }

    #[test]
    fn should_create_base_string() {
        let twitter_config = twitter_config();

        let request_method = "PoSt";
        let base_uri = "https://stream.twitter.com/1.1/statuses/update.json";
        let parameters = collect_parameters(&twitter_config);

        let expected = "POST&https%3A%2F%2Fapi.twitter.com%2F1.1%2Fstatuses%2Fupdate.json&include_entities%3Dtrue%26oauth_consumer_key%3Dxvz1evFS4wEEPTGEFPHBog%26oauth_nonce%3DkYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg%26oauth_signature_method%3DHMAC-SHA1%26oauth_timestamp%3D1318622958%26oauth_token%3D370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb%26oauth_version%3D1.0%26status%3DHello%2520Ladies%2520%252B%2520Gentlemen%252C%2520a%2520signed%2520OAuth%2520request%2521";
        let actual = create_base_string(request_method, base_uri, parameters);
    }

    #[test]
    fn should_collect_parameters() {
        let twitter_config = twitter_config();
        let expected = String::from("include_entities=true&oauth_consumer_key=xvz1evFS4wEEPTGEFPHBog&oauth_nonce=kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg&oauth_signature_method=HMAC-SHA1&oauth_timestamp=1318622958&oauth_token=370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb&oauth_version=1.0&status=Hello%20Ladies%20%2B%20Gentlemen%2C%20a%20signed%20OAuth%20request%21");
        let actual = collect_parameters(&twitter_config);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_percent_encode_twitter_example() {
        let request = "Hello Ladies + Gentlemen, a signed OAuth request!";
        let expected = String::from("Hello%20Ladies%20%2b%20Gentlemen%2c%20a%20signed%20OAuth%20request%21");
        let actual = percent_encode(request);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_percent_encode_src_containing_unicode() {
        let src = "Snowman☃©-._~ ";
        let expected = String::from("Snowman%e2%98%83%c2%a9-._~%20");
        let actual = percent_encode(&src);
        assert_eq!(actual, expected);
    }

    fn twitter_config() -> TwitterConfig {
        return TwitterConfig {
            stream_track_params: String::from("Hello Ladies + Gentlemen, a signed OAuth request!"),
            oauth: OAuth {
                consumer_key: String::from("xvz1evFS4wEEPTGEFPHBog"),
                consumer_secret: String::from("kAcSOqF21Fu85e7zjz7ZN2U4ZRhfV3WpwPAoE3Z7kBw"),
                nonce: String::from("kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg"),
                signature_method: String::from("HMAC-SHA1"),
                token: String::from("370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb"),
                token_secret: String::from("LswwdoUaIvS8ltyTt5jkRh4J50vUPVVHtR2YPi5kE"),
                version: String::from("1.0")
            }
        };
    }

//    #[test]
//     fn should_create_authorization_header() {
//         let twitter_config =
//             TwitterConfig {
//                 stream_track_params: String::from("Hello Ladies + Gentlemen, a signed OAuth request!"),
//                 oauth: OAuth {
//                     consumer_key: String::from("xvz1evFS4wEEPTGEFPHBog"),
//                     consumer_secret_key: String::from("kAcSOqF21Fu85e7zjz7ZN2U4ZRhfV3WpwPAoE3Z7kBw"),
//                     nonce: String::from("kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg"),
//                     signature_method: String::from("HMAC-SHA1"),
//                     token: String::from("370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb"),
//                     token_secret: String::from("LswwdoUaIvS8ltyTt5jkRh4J50vUPVVHtR2YPi5kE"),
//                     version: String::from("1.0")
//             }
//         };
//         let actual = create_authorization_header(&twitter_config);
//         let expected = String::from("OAuth oauth_consumer_key=\"xvz1evFS4wEEPTGEFPHBog\", oauth_nonce=\"kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg\", oauth_signature=\"tnnArxj06cWHq44gCs1OSKk%2FjLY%3D\", oauth_signature_method=\"HMAC-SHA1\", oauth_timestamp=\"1318622958\", oauth_token=\"370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb\", oauth_version=\"1.0\"");
//         assert_eq!(actual, expected);
//     }

}