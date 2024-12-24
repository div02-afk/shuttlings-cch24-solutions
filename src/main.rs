mod day_2;
mod day_5;
mod day_9;
mod day_12;
mod day_16;
mod day_19;

use std::sync::{ Arc, RwLock };
use rand::rngs::StdRng;
use rand::{ Rng, SeedableRng };
use rocket::routes;
use shuttle_runtime::CustomError;
use crate::day_2::{ two_dest_one, two_dest_two, two_dest_three_one, two_dest_three_two };
use crate::day_5::day_5_task_one;
use crate::day_9::{ day_9_task_one, day_9_task_four, day_9_task_two };
use crate::day_12::{ day_12_task_one, day_12_task_one_two, day_12_task_two, day_12_task_three };
use crate::day_16::{ day_16_task_one_one, day_16_task_one_two, day_16_task_two };
use crate::day_19::{
    day_19_task_one_a,
    day_19_task_one_b,
    day_19_task_one_c,
    day_19_task_one_d,
    day_19_task_one_e,
    day_19_task_two_a,
    day_19_task_two_b,
};
use leaky_bucket::RateLimiter;
use tokio::time::Duration;
use sqlx::{ Executor, PgPool };
pub struct MilkCookiesPack {
    value: String,
    is_winner: bool,
    winner: char,
    is_full: bool,
    rng: rand::rngs::StdRng,
}
impl MilkCookiesPack {
    fn reset() -> Self {
        MilkCookiesPack {
            value: "⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬛⬛⬛⬛⬜\n⬜⬜⬜⬜⬜⬜\n".to_string(),
            is_winner: false,
            is_full: false,
            winner: ' ',
            rng: StdRng::seed_from_u64(2024),
        }
    }

    fn random(rnd: &mut rand::rngs::StdRng) -> Self {
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
            rng: rnd.to_owned(),
        };
    }
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    pool.execute(include_str!("./schema.sql")).await.map_err(CustomError::new)?;

    let shared_pool = Arc::new(pool);
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
        .manage(shared_pool)
        .mount(
            "/",
            routes![
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
                day_12_task_three,
                day_16_task_one_one,
                day_16_task_one_two,
                day_16_task_two,
                day_19_task_one_a,
                day_19_task_one_b,
                day_19_task_one_c,
                day_19_task_one_d,
                day_19_task_one_e,
                day_19_task_two_a,
                day_19_task_two_b
            ]
        );

    Ok(rocket.into())
}
