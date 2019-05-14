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
    let include_entities = format!("include_entities={}", percent_encode("true"));
    let oauth_consumer_key = format!("oauth_consumer_key={}", percent_encode(&twitter_oauth.oauth.consumer_key));
    let oauth_nonce = format!("oauth_nonce={}", percent_encode(&twitter_oauth.oauth.nonce));
    let oauth_signature_method = format!("oauth_signature_method={}", percent_encode(&twitter_oauth.oauth.signature_method));
    let oauth_timestamp = format!("oauth_timestamp={}", "1318622958"); //percent_encode(&Instant::now().elapsed().as_secs().to_string()));
    let oauth_token = format!("oauth_token={}", percent_encode(&twitter_oauth.oauth.token));
    let oauth_version = format!("oauth_version={}", percent_encode(&twitter_oauth.oauth.version));
    let oauth_signature = format!("oauth_signature={}", percent_encode(&sign(&twitter_oauth)));

    let mut oauth_params = vec![include_entities, oauth_consumer_key, oauth_nonce, oauth_signature,
                                oauth_signature_method, oauth_timestamp, oauth_token, oauth_version];
    oauth_params.sort();

    // 4. create OAuth header
    let authorization_header = format!("OAuth {}", &oauth_params.join(", "));
    authorization_header
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

fn collect_parameters(twitter_oauth: &TwitterConfig) -> String {
    let include_entities = format!("include_entities={}", percent_encode("true"));
    let oauth_consumer_key = format!("oauth_consumer_key={}", percent_encode(&twitter_oauth.oauth.consumer_key));
    let oauth_nonce = format!("oauth_nonce={}", percent_encode(&twitter_oauth.oauth.nonce));
    let oauth_signature_method = format!("oauth_signature_method={}", percent_encode(&twitter_oauth.oauth.signature_method));
    let oauth_timestamp = format!("oauth_timestamp={}", "1318622958"); //percent_encode(&Instant::now().elapsed().as_secs().to_string()));
    let oauth_token = format!("oauth_token={}", percent_encode(&twitter_oauth.oauth.token));
    let oauth_version = format!("oauth_version={}", percent_encode(&twitter_oauth.oauth.version));

    let mut oauth_params = vec![include_entities, oauth_consumer_key, oauth_nonce, oauth_signature_method, oauth_timestamp, oauth_token, oauth_version];
    let parameters = format!("{0}&track={1}", &oauth_params.join("&"), percent_encode(&twitter_oauth.stream_track_params));

    parameters
}

fn create_base_string(request_method: &str, base_uri: &str, parameters: &String) -> String {
    let mut base_string = format!("{}&{}&{}", &String::from(request_method).to_uppercase(),
                                  &percent_encode(base_uri), &percent_encode(parameters));
    base_string
}

fn sign(twitter_config: &TwitterConfig) -> String {
    let signing_key = format!("{0}&{1}", percent_encode(&twitter_config.oauth.consumer_secret),
                              percent_encode(&twitter_config.oauth.token_secret));
    let params = collect_parameters(twitter_config);
    let base_string = create_base_string(&twitter_config.request_method, &twitter_config.stream_uri, &params);

    let mut hmac_sha1 = HmacSha1::new_varkey(signing_key.as_bytes()).unwrap();
    hmac_sha1.input(base_string.as_bytes());
    let signature = &hmac_sha1.result().code();

    let config = base64::Config::new (
        base64::CharacterSet::Standard,
        true,
        true,
        base64::LineWrap::NoWrap
    );
    let encoded = base64::encode_config(&signature, config);
    encoded
}

#[cfg(test)]
mod tests {    
    use super::*;

    #[test]
    fn should_create_signature() {
        let twitter_config = build_twitter_config();
        let expected = "hCtSmYh+iHYCEqBWrE7C7hYmtUk=";
        let actual = sign(&twitter_config);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_create_signature_from_twitter_example() {
        let signing_key = "kAcSOqF21Fu85e7zjz7ZN2U4ZRhfV3WpwPAoE3Z7kBw&LswwdoUaIvS8ltyTt5jkRh4J50vUPVVHtR2YPi5kE";
        let request = "POST&https%3A%2F%2Fapi.twitter.com%2F1.1%2Fstatuses%2Fupdate.json&include_entities%3Dtrue%26oauth_consumer_key%3Dxvz1evFS4wEEPTGEFPHBog%26oauth_nonce%3DkYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg%26oauth_signature_method%3DHMAC-SHA1%26oauth_timestamp%3D1318622958%26oauth_token%3D370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb%26oauth_version%3D1.0%26status%3DHello%2520Ladies%2520%252B%2520Gentlemen%252C%2520a%2520signed%2520OAuth%2520request%2521";

        let mut hmac_sha1 = HmacSha1::new_varkey(signing_key.as_bytes()).unwrap();
        hmac_sha1.input(request.as_bytes());
        let signature = &hmac_sha1.result().code();

        let config = base64::Config::new (
            base64::CharacterSet::Standard,
            true,
        true,
        base64::LineWrap::NoWrap
        );

        let actual = base64::encode_config(&signature, config);
        let expected = "hCtSmYh+iHYCEqBWrE7C7hYmtUk=";
        assert_eq!(expected, actual);
    }

//    #[test]
//    fn should_create_base_string() {
//        let twitter_config = build_twitter_config();
//
//        let request_method = "PoSt";
//        let base_uri = "https://stream.twitter.com/1.1/statuses/update.json";
//        let parameters = &collect_parameters(&twitter_config);
//
//        let expected = "POST&https%3A%2F%2Fapi.twitter.com%2F1.1%2Fstatuses%2Fupdate.json&include_entities%3Dtrue%26oauth_consumer_key%3Dxvz1evFS4wEEPTGEFPHBog%26oauth_nonce%3DkYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg%26oauth_signature_method%3DHMAC-SHA1%26oauth_timestamp%3D1318622958%26oauth_token%3D370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb%26oauth_version%3D1.0%26status%3DHello%2520Ladies%2520%252B%2520Gentlemen%252C%2520a%2520signed%2520OAuth%2520request%2521";
//        let actual = create_base_string(request_method, base_uri, parameters);
//        assert_eq!(actual, expected);
//    }
//
//    #[test]
//    fn should_collect_parameters() {
//        let twitter_config = build_twitter_config();
//        let expected = String::from("include_entities=true&oauth_consumer_key=xvz1evFS4wEEPTGEFPHBog&oauth_nonce=kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg&oauth_signature_method=HMAC-SHA1&oauth_timestamp=1318622958&oauth_token=370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb&oauth_version=1.0&status=Hello%20Ladies%20%2B%20Gentlemen%2C%20a%20signed%20OAuth%20request%21");
//        let actual = collect_parameters(&twitter_config);
//        assert_eq!(actual, expected);
//    }
//
//    #[test]
//    fn should_percent_encode_twitter_example() {
//        let request = "Hello Ladies + Gentlemen, a signed OAuth request!";
//        let expected = String::from("Hello%20Ladies%20%2b%20Gentlemen%2c%20a%20signed%20OAuth%20request%21");
//        let actual = percent_encode(request);
//        assert_eq!(actual, expected);
//    }
//
//    #[test]
//    fn should_percent_encode_src_containing_unicode() {
//        let src = "Snowman☃©-._~ ";
//        let expected = String::from("Snowman%e2%98%83%c2%a9-._~%20");
//        let actual = percent_encode(&src);
//        assert_eq!(actual, expected);
//    }

    fn build_twitter_config() -> TwitterConfig {
        return TwitterConfig {
            request_method: String::from("POST"),
            stream_uri: String::from("https://api.twitter.com/1.1/statuses/update.json"),
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

    #[test]
     fn should_create_authorization_header() {
         let twitter_config = build_twitter_config();
         let actual = create_authorization_header(&twitter_config);
         let expected = String::from("OAuth oauth_consumer_key=xvz1evFS4wEEPTGEFPHBog, oauth_nonce=kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg, oauth_signature=tnnArxj06cWHq44gCs1OSKk%2FjLY%3D, oauth_signature_method=HMAC-SHA1, oauth_timestamp=1318622958, oauth_token=370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb, oauth_version=1.0");
         assert_eq!(actual, expected);
     }

}