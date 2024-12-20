use std::sync::Arc;

use rocket::{ get, http::Status, post, State };
use sqlx::{ types::chrono, Executor, Pool, Postgres, Row };

use crate::day_5::RequestResponse;

#[derive(serde::Deserialize, serde::Serialize)]
struct Quote {
    id: Option<sqlx::types::Uuid>,
    author: String,
    quote: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[post("/19/reset")]
pub async fn day_19_task_one_a(pool: &State<Arc<Pool<Postgres>>>) -> RequestResponse {
    pool.execute("DELETE FROM quotes;").await.unwrap();
    RequestResponse::Success("".to_string())
}

#[get("/19/cite/<id>")]
pub async fn day_19_task_one_b(pool: &State<Arc<Pool<Postgres>>>, id: &str) -> RequestResponse {
    let query = "SELECT quote FROM quotes WHERE id = $1";

    let id = sqlx::types::Uuid::parse_str(id).unwrap();

    let results = sqlx::query(query).bind(id).fetch_one(&***pool).await;
    println!("{:?}", results);
    match results {
        Ok(result) => {
            println!("{:?}", result);
            let quote = result.get("quote");
            return RequestResponse::Success(quote);
        }
        Err(_) => {
            return RequestResponse::Error(Status::NotFound, "".to_string());
        }
    }
}

#[post("/19/draft", data = "<quote>")]
pub async fn day_19_task_one_e(pool: &State<Arc<Pool<Postgres>>>, quote: &str) -> RequestResponse {
    let parsed_quote = serde_json::from_str::<serde_json::Value>(quote);
    match parsed_quote {
        Ok(quote_info) => {
            let uuid = sqlx::types::Uuid::new_v4();
            println!("{:?}, {:?}", uuid, quote_info);

            if
                let (Some(author), Some(quote_text)) = (
                    quote_info["author"].as_str(),
                    quote_info["quote"].as_str(),
                )
            {
                let query = "INSERT INTO quotes (id, author, quote) VALUES ($1, $2, $3)";
                let exec = sqlx
                    ::query(query)
                    .bind(uuid)
                    .bind(author)
                    .bind(quote_text)
                    .execute(&***pool).await;
                match exec {
                    Ok(results) if results.rows_affected() != 0 => {
                        let fetch_query: &str = "SELECT * FROM quotes WHERE id = $1";
                        let results = sqlx::query(fetch_query).bind(uuid).fetch_one(&***pool).await;
                        match results {
                            Ok(result) => {
                                let quote = Quote {
                                    id: result.try_get("id").ok(),
                                    author: result.try_get("author").unwrap(),
                                    quote: result.try_get("quote").unwrap(),
                                    created_at: result.try_get("created_at").ok(),
                                };
                                let quote_json = serde_json::to_string(&quote).unwrap();
                                return RequestResponse::Error(Status::Created, quote_json);
                            }
                            Err(_) => {
                                return RequestResponse::Error(
                                    Status::InternalServerError,
                                    "".to_string()
                                );
                            }
                        }
                    }
                    Ok(_) => {
                        return RequestResponse::Error(Status::InternalServerError, "".to_string());
                    }
                    Err(err) => {
                        println!("Database error: {}", err);
                        return RequestResponse::Error(Status::InternalServerError, "".to_string());
                    }
                }
            } else {
                println!("Invalid JSON structure");
                return RequestResponse::Error(
                    Status::BadRequest,
                    "Invalid JSON structure".to_string()
                );
            }
        }
        Err(_) => {
            return RequestResponse::Error(Status::BadRequest, "".to_string());
        }
    }
}
