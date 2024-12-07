mod day_2;
mod day_5;
use rocket::{ get, routes };
use crate::day_2::{ two_dest_one, two_dest_two, two_dest_three_one, two_dest_three_two };
use crate::day_5::task_one;
#[get("/")]
fn index() -> &'static str {
    "Hello, bird!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket
        ::build()
        .mount(
            "/",
            routes![
                index,
                two_dest_one,
                two_dest_two,
                two_dest_three_one,
                two_dest_three_two,
                task_one
            ]
        );

    Ok(rocket.into())
}
