mod day_2;
mod day_5;
mod day_9;
mod day_12;
mod day_16;
use std::sync::{ Arc, RwLock };
use rand::{ Rng, RngCore, SeedableRng };
use rocket::{ get, routes };
use crate::day_2::{ two_dest_one, two_dest_two, two_dest_three_one, two_dest_three_two };
use crate::day_5::day_5_task_one;
use crate::day_9::{ day_9_task_one, day_9_task_four, day_9_task_two };
use crate::day_12::{ day_12_task_one, day_12_task_one_two, day_12_task_two };
use crate::day_16::{ day_16_task_one_one, day_16_task_one_two };
use leaky_bucket::RateLimiter;
use tokio::time::Duration;

#[get("/")]
fn index() -> &'static str {
    "Hello, bird!"
}

pub struct MilkCookiesPack {
    value: String,
    is_winner: bool,
    winner: char,
    is_full: bool,
    // RNG: rand::rngs::StdRng::from_seed::<u64>(2024)
}
impl MilkCookiesPack {
    fn reset() -> Self {
        MilkCookiesPack {
            value: "⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬜⬜⬜⬜⬜\n".to_string(),
            is_winner: false,
            is_full: false,
            winner: ' ',
        }
    }

    fn random(rnd: &mut rand::rngs::StdRng) -> Self {
        let mut rng = rand::thread_rng();
        let return_val = "⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬜⬜⬜⬜⬜\n";

        let mut ret_vec = return_val.chars().collect::<Vec<char>>();
        for i in 0..ret_vec.len() {
            if ret_vec[i] == '⬜' || ret_vec[i] == '\n' {
                continue;
            }
            let random_num = rnd.gen::<bool>();
            match random_num {
                false => {
                    ret_vec[i] = '🥛';
                }
                true => {
                    ret_vec[i] = '🍪';
                }
            }
        }

        return MilkCookiesPack {
            value: ret_vec.iter().collect::<String>(),
            is_winner: false,
            is_full: false,
            winner: ' ',
        };
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let board = MilkCookiesPack::reset();
    let limiter = RateLimiter::builder()
        .initial(0)
        .interval(Duration::from_secs(1))
        .refill(1)
        .max(5)
        .build();
    let shared_limiter = Arc::new(RwLock::new(limiter));
    let shared_board = Arc::new(RwLock::new(board));
    let rocket = rocket
        ::build()
        .manage(shared_limiter)
        .manage(shared_board)
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
                day_9_task_four,
                day_12_task_one,
                day_12_task_one_two,
                day_12_task_two,
                day_16_task_one_one,
                day_16_task_one_two
            ]
        );

    Ok(rocket.into())
}
