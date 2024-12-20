use crate::day_5::RequestResponse;
use rocket::{ get, http::{ Cookie, CookieJar, Status }, post };
use jsonwebtoken::{
    decode,
    decode_header,
    encode,
    Algorithm,
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};

static SANTA_PUB_KEY: &[u8] = include_bytes!("../day16_santa_public_key.pem");

// const KEY: &[u8] = b"secret";
#[post("/16/wrap", data = "<json_body>")]
pub fn day_16_task_one_one(json_body: &str, cookies: &CookieJar) -> RequestResponse {
    let json_body = serde_json::from_str::<serde_json::Value>(json_body);
    match json_body {
        Ok(json_body) => {
            let encode = encode::<serde_json::Value>(
                &Header::default(),
                &json_body,
                &EncodingKey::from_secret("secret".as_ref())
            );
            if encode.is_err() {
                return RequestResponse::Error(Status::BadRequest, "".to_string());
            }
            let encode = encode.unwrap();
            let mut cookie = Cookie::new("gift", encode);
            cookie.make_permanent();
            cookies.add(cookie);
            return RequestResponse::Success("".to_string());
        }
        Err(_err) => {
            return RequestResponse::Error(Status::BadRequest, "".to_string());
        }
    }
}

#[get("/16/unwrap")]
pub fn day_16_task_one_two(cookies: &CookieJar) -> RequestResponse {
    if cookies.get("gift").is_some() {
        let token = cookies.get("gift").unwrap().value();
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = false;
        validation.required_spec_claims.remove("exp");
        println!("{:?}", validation);
        let decoded_token = decode::<serde_json::Value>(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &validation
        );
        match decoded_token {
            Err(err) => {
                println!("Error {}", err);
                return RequestResponse::Error(Status::BadRequest, "".to_string());
            }
            Ok(token_data) => {
                let mut json_token = token_data.claims;
                if json_token.get("exp").is_some() {
                    json_token.as_object_mut().unwrap().remove("exp");
                }
                let original_json = json_token.to_string();
                return RequestResponse::Success(original_json);
            }
        }
    }
    return RequestResponse::Error(Status::BadRequest, "".to_string());
}

#[post("/16/decode", data = "<token>")]
pub fn day_16_task_two(token: &str) -> RequestResponse {
    let header = decode_header(&token);
    let mut token = token.to_string();
    match header {
        Ok(header) => {
            let algorithm = header.alg.into();
            let mut validation = Validation::new(algorithm);
            validation.validate_exp = false;
            validation.required_spec_claims.remove("exp");
            token = token.replace("=", "");

            println!("{}", token);
            let decode = decode::<serde_json::Value>(
                &token,
                &DecodingKey::from_rsa_pem(SANTA_PUB_KEY).expect(
                    "Unable to create DecodingKey from PEM"
                ),
                &validation
            );

            match decode {
                Ok(decoded) => {
                    let decoded = decoded.claims;
                    let original_json = decoded.to_string();
                    return RequestResponse::Success(original_json);
                }
                Err(err) => {
                    match jsonwebtoken::errors::Error::kind(&err) {
                        jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                            return RequestResponse::Error(Status::Unauthorized, "".to_string());
                        }
                        _ => {
                            println!("Error {}", err);
                            return RequestResponse::Error(Status::BadRequest, "".to_string());
                        }
                    }
                }
            }
        }
        Err(_err) => {
            return RequestResponse::Error(Status::BadRequest, "".to_string());
        }
    }
}
