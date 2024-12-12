use rocket::form::validate::Contains;
use rocket::post;
use rocket::http::Status;
use rocket::http::ContentType;
use rocket::response::{ Responder, Result };
use rocket::Request;
use rocket::Response;
use cargo_manifest::Manifest;

impl<'r> Responder<'r, 'static> for RequestResponse {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        match self {
            RequestResponse::Success(s) =>
                Response::build().sized_body(s.len(), std::io::Cursor::new(s)).ok(),
            RequestResponse::Error(status, msg) =>
                Response::build()
                    .status(status)
                    .sized_body(msg.len(), std::io::Cursor::new(msg.to_string()))
                    .ok(),
        }
    }
}
pub enum RequestResponse {
    Success(String),
    Error(Status, String),
}


const INVALID_MANIFEST: &str = "Invalid manifest";
const EMPTY_MSG: &str = "";
const MAGIC_KEYWORD: &str = "Christmas 2024";
const MAGIC_KEYWORD_ERROR: &str = "Magic keyword not provided";

fn handle_toml(package: String) -> RequestResponse {
    println!("Package: {}", package);
    match Manifest::from_slice(package.as_bytes()) {
        Ok(package) => {
            let package_info = package.package.unwrap();
            println!("Package: {:?}", package_info);
            if package_info.keywords.is_none() {
                return RequestResponse::Error(Status::BadRequest, MAGIC_KEYWORD_ERROR.to_string());
            }
            let keywords = package_info.keywords.unwrap();
            println!("Keywords: {:?}", keywords.clone().as_local());
            if !keywords.as_local().contains(MAGIC_KEYWORD.to_string()) {
                return RequestResponse::Error(Status::BadRequest, MAGIC_KEYWORD_ERROR.to_string());
            }

            if package_info.metadata.is_none() {
                // return RequestResponse::Error(Status::BadRequest, INVALID_MANIFEST.to_string());
                return RequestResponse::Error(Status::NoContent, EMPTY_MSG.to_string());
            }
            let metadata = package_info.metadata.unwrap();
            if metadata.get("orders").is_none() {
                return RequestResponse::Error(Status::NoContent, EMPTY_MSG.to_string());
            }
            let orders = metadata.get("orders").unwrap();
            let mut result = String::new();
            for order in orders.as_array().unwrap() {
                if order.get("item").is_none() || order.get("quantity").is_none() {
                    continue;
                }
                let item = order.get("item").unwrap();
                let quantity = order.get("quantity").unwrap();
                if !item.is_str() || !quantity.is_integer() {
                    continue;
                }
                result.push_str(
                    &format!("{}: {}\n", item.as_str().unwrap(), quantity.as_integer().unwrap())
                );
            }
            if result.is_empty() {
                return RequestResponse::Error(Status::NoContent, EMPTY_MSG.to_string());
            }
            result.pop();
            return RequestResponse::Success(result);
            // println!("Manifest: {:?}", metadata);
            // return RequestResponse::Error(Status::BadRequest, INVALID_MANIFEST.to_string());
        }
        Err(_) => {
            println!("Error parsing toml");
            return RequestResponse::Error(Status::BadRequest, INVALID_MANIFEST.to_string());
        }
    }
    // return RequestResponse::Error(Status::BadRequest, INVALID_MANIFEST.to_string());
}

fn handle_json(package: String) -> RequestResponse {
    match serde_json::from_str::<serde_json::Value>(&package) {
        Ok(package) => {
            if !package.is_object() {
                return RequestResponse::Error(Status::BadRequest, INVALID_MANIFEST.to_string());
            }
            let toml_string = toml::to_string(&package).unwrap();
            return handle_toml(toml_string);
        }
        Err(_) => {
            return RequestResponse::Error(Status::BadRequest, INVALID_MANIFEST.to_string());
        }
    }
}

fn handle_yaml(package: String) -> RequestResponse {
    match serde_yaml::from_str::<serde_yaml::Mapping>(&package) {
        Ok(package) => {
            if package.is_empty() {
                return RequestResponse::Error(Status::BadRequest, INVALID_MANIFEST.to_string());
            }
            let toml_string = toml::to_string(&package).unwrap();
            return handle_toml(toml_string);
        }
        Err(_) => {
            return RequestResponse::Error(Status::BadRequest, INVALID_MANIFEST.to_string());
        }
    }
}
#[post("/5/manifest", data = "<package>")]
pub fn day_5_task_one(content_type: &ContentType, package: String) -> RequestResponse {
    // let allowed_media_types = ["application/toml", "application/json", "application/yaml"];
    let media_type = content_type.to_string();

    match media_type.as_str() {
        "application/toml" => {
            return handle_toml(package);
        }
        "application/json" => {
            return handle_json(package);
        }
        "application/yaml" => {
            return handle_yaml(package);
        }
        _ => {
            return RequestResponse::Error(Status::UnsupportedMediaType, EMPTY_MSG.to_string());
        }
    }
}
