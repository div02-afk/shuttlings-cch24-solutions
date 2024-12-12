mod day_2;
mod day_5;
mod day_9;
use std::sync::{ Arc, RwLock };

use rocket::{ get, routes };
use crate::day_2::{ two_dest_one, two_dest_two, two_dest_three_one, two_dest_three_two };
use crate::day_5::day_5_task_one;
use crate::day_9::{ day_9_task_one, day_9_task_four, day_9_task_two };
use leaky_bucket::RateLimiter;
use tokio::time::Duration;

#[get("/")]
fn index() -> &'static str {
    "Hello, bird!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let limiter = RateLimiter::builder()
        .initial(0)
        .interval(Duration::from_secs(1))
        .refill(1)
        .max(5)
        .build();
    let shared_limiter = Arc::new(RwLock::new(limiter));
    let rocket = rocket
        ::build()
        .manage(shared_limiter)
        .mount(
            "/",
            routes![
                index,
                two_dest_one,
                two_dest_two,
                two_dest_three_one,
                two_dest_three_two,
                day_5_task_one,
                day_9_task_two,
                day_9_task_one,
                day_9_task_four
            ]
        );

    Ok(rocket.into())
}
