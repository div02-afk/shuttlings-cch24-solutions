use std::{ borrow::BorrowMut, sync::{ Arc, RwLock } };
use leaky_bucket::RateLimiter;
use rocket::{ http::Status, post, State };
use serde_json::json;
use tokio::time::Duration;
use crate::day_5::RequestResponse;

#[post("/9/milk", rank = 2)]
pub async fn day_9_task_one(
    // content_type: &ContentType,
    bucket: &State<Arc<RwLock<RateLimiter>>>
) -> RequestResponse {
    // println!("Taking milk {}", bucket.balance());
    // let milk = data.open((2).kibibytes()).into_string().await.unwrap();

    if bucket.read().unwrap().try_acquire(1) {
        return RequestResponse::Success("Milk withdrawn\n".to_string());
    } else {
        return RequestResponse::Error(Status::TooManyRequests, "No milk available\n".to_string());
    }
}

#[post("/9/milk", format = "application/json", data = "<milk>")]
pub fn day_9_task_two(bucket: &State<Arc<RwLock<RateLimiter>>>, milk: String) -> RequestResponse {
    if !bucket.read().unwrap().try_acquire(1) {
        return RequestResponse::Error(Status::TooManyRequests, "No milk available\n".to_string());
    }
    println!("Milk: {}", milk);
    let parsed_json = match serde_json::from_str::<serde_json::Value>(&milk) {
        Ok(json) => json,
        Err(_) => {
            return RequestResponse::Error(Status::BadRequest, "".to_string());
        }
    };
    let milk_needed;
    let converted_to;
    let is_liter = parsed_json.get("liters").is_some();
    let is_gallon = parsed_json.get("gallons").is_some();
    let is_litre = parsed_json.get("litres").is_some();
    let is_pint = parsed_json.get("pints").is_some();
    if is_liter && !is_gallon && !is_litre && !is_pint {
        milk_needed = (parsed_json.get("liters").unwrap().as_f64().unwrap() as f32) * 0.26417206;
        converted_to = "gallons";
    } else if !is_liter && is_gallon && !is_litre && !is_pint {
        milk_needed = (parsed_json.get("gallons").unwrap().as_f64().unwrap() as f32) * 3.785412;
        converted_to = "liters";
    } else if is_litre && !is_gallon && !is_liter && !is_pint {
        milk_needed = (parsed_json.get("litres").unwrap().as_f64().unwrap() as f32) * 1.759754;
        converted_to = "pints";
    } else if !is_litre && !is_gallon && !is_liter && is_pint {
        milk_needed = (parsed_json.get("pints").unwrap().as_f64().unwrap() as f32) * 0.568261;
        converted_to = "litres";
    } else {
        return RequestResponse::Error(Status::BadRequest, "".to_string());
    }
    println!("Milk needed: {}", milk_needed as f32);
    // let temp = (milk_needed as f32).to_string().parse::<f32>().unwrap();
    let rounded_temp = (milk_needed * 10_000_000.0).round() / 10_000_000.0;
    let response = json!({
        converted_to:rounded_temp
    });

    RequestResponse::Success(response.to_string())
}

#[post("/9/refill")]
pub fn day_9_task_four(bucket: &State<Arc<RwLock<RateLimiter>>>) -> RequestResponse {
    // println!("Refilling milk {}", bucket.balance());
    let new_limiter = RateLimiter::builder()
        .initial(5)
        .interval(Duration::from_secs(1))
        .refill(1)
        .max(5)
        .build();
    let mut writable = bucket.write().unwrap();
    *writable = new_limiter;

    return RequestResponse::Success("".to_string());
}
