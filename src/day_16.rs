use std::time::SystemTime;

use crate::day_5::RequestResponse;
use rocket::{ get, http::{ Cookie, CookieJar, Status }, post };

use jsonwebtoken::{ decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation };

const KEY: &[u8] = b"secret";
#[post("/16/wrap", data = "<json_body>")]
pub fn day_16_task_one_one(json_body: &str, cookies: &CookieJar) -> RequestResponse {
    let json_body = serde_json::from_str::<serde_json::Value>(json_body);
    match json_body {
        Ok(mut json_body) => {
            let printable = json_body.to_string();
            println!("{}", printable);
            // let expiration =
            //     SystemTime::now()
            //         .duration_since(SystemTime::UNIX_EPOCH)
            //         .expect("Time went backwards")
            //         .as_secs() + 3600; // 1 hour

            // json_body["exp"] = serde_json::Value::from(expiration);
            let encode = encode::<serde_json::Value>(
                &Header::default(),
                &json_body,
                &EncodingKey::from_secret("secret".as_ref())
            );
            if encode.is_err() {
                return RequestResponse::Error(Status::BadRequest, "".to_string());
            }
            let encode = encode.unwrap();
            let mut cookie = Cookie::new("token", encode);
            cookie.make_permanent();
            cookies.add(cookie);
            return RequestResponse::Success("Token added to cookies".to_string());
        }
        Err(err) => {
            return RequestResponse::Error(Status::BadRequest, err.to_string());
        }
    }
}

#[get("/16/unwrap")]
pub fn day_16_task_one_two(cookies: &CookieJar) -> RequestResponse {
    let all_cookies: Vec<_> = cookies.iter().collect();
    println!("{:?}", all_cookies);
    if cookies.get("token").is_some() {
        let token = cookies.get("token").unwrap().value();
        println!("Token found, {}", token);
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = false;
        validation.required_spec_claims.remove("exp");

        let decoded_token = decode::<serde_json::Value>(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &validation
        );
        match decoded_token {
            Err(err) => {
                println!("Error {}", err);
                return RequestResponse::Error(Status::BadRequest, err.to_string());
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
    return RequestResponse::Error(Status::BadRequest, "No token found".to_string());
}
