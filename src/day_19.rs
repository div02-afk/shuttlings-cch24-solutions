use std::sync::Arc;
use rocket::response::status::BadRequest;
use rocket::serde::{ Deserialize, json::Json };
use rocket::{ delete, get, http::Status, post, State, put };
use sqlx::{ types::chrono, Executor, Pool, Postgres };
use crate::day_5::RequestResponse;

#[derive(Clone, sqlx::FromRow, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
struct Quote {
    id: Option<sqlx::types::Uuid>,
    author: String,
    quote: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
    version: i32,
}

#[post("/19/reset")]
pub async fn day_19_task_one_a(pool: &State<Arc<Pool<Postgres>>>) -> RequestResponse {
    pool.execute("DELETE FROM quotes;").await.unwrap();
    RequestResponse::Success("".to_string())
}

#[get("/19/cite/<id>")]
pub async fn day_19_task_one_b(pool: &State<Arc<Pool<Postgres>>>, id: &str) -> RequestResponse {
    let query = "SELECT * FROM quotes WHERE id = $1";
    let id = sqlx::types::Uuid::parse_str(id).unwrap();
    let results = sqlx::query_as::<_, Quote>(query).bind(id).fetch_one(&***pool).await;
    match results {
        Ok(quote) => {
            return RequestResponse::JSON(Status::Ok, serde_json::to_string(&quote).unwrap());
        }
        Err(e) => {
            println!("{}", e);
            return RequestResponse::Error(Status::NotFound, "".to_string());
        }
    }
}

#[delete("/19/remove/<id>")]
pub async fn day_19_task_one_c(pool: &State<Arc<Pool<Postgres>>>, id: &str) -> RequestResponse {
    let query = "DELETE FROM quotes WHERE id = $1 RETURNING *";
    // let search_result = day_19_task_one_b(pool, id).await;
    let id = match sqlx::types::Uuid::parse_str(id) {
        Ok(id) => id,
        Err(_) => {
            return RequestResponse::Error(Status::BadRequest, "".to_string());
        }
    };
    let results = match sqlx::query_as::<_, Quote>(query).bind(id).fetch_optional(&***pool).await {
        Ok(results) =>
            match results {
                Some(quote) => {
                    return RequestResponse::JSON(
                        Status::Ok,
                        serde_json::to_string(&quote).unwrap()
                    );
                }
                None => {
                    return RequestResponse::Error(Status::NotFound, "".to_string());
                }
            }
        Err(_) => {
            return RequestResponse::Error(Status::NotFound, "".to_string());
        }
    };
}
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Update {
    pub author: String,
    pub quote: String,
}

#[put("/19/undo/<id>", data = "<update>")]
pub async fn day_19_task_one_d(
    pool: &State<Arc<Pool<Postgres>>>,
    id: &str,
    update: Json<Update>
) -> RequestResponse {
    let update = update.into_inner();
    let query =
        "UPDATE quotes SET author = $2, quote = $3, version = version+1 WHERE id = $1 RETURNING *";
    let id = match sqlx::types::Uuid::parse_str(id) {
        Ok(id) => id,
        Err(_) => {
            return RequestResponse::Error(Status::BadRequest, "".to_string());
        }
    };
    let results = sqlx
        ::query_as::<_, Quote>(query)
        .bind(id)
        .bind(update.author)
        .bind(update.quote.clone())
        .fetch_optional(&***pool).await;

    match results {
        Ok(results) => {
            match results {
                Some(quote) => {
                    return RequestResponse::JSON(
                        Status::Ok,
                        serde_json::to_string(&quote).unwrap()
                    );
                }
                None => {
                    return RequestResponse::Error(Status::NotFound, "".to_string());
                }
            }
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
                let query =
                    "INSERT INTO quotes (id, author, quote) VALUES ($1, $2, $3) RETURNING *";
                let exec = sqlx
                    ::query_as::<_, Quote>(query)
                    .bind(uuid)
                    .bind(author)
                    .bind(quote_text)
                    .fetch_one(&***pool).await;

                match exec {
                    Ok(quote) => {
                        return RequestResponse::JSON(
                            Status::Created,
                            serde_json::to_string(&quote).unwrap()
                        );
                    }
                    Err(_) => {
                        return RequestResponse::Error(
                            Status::BadRequest,
                            "Invalid JSON structure".to_string()
                        );
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
#[derive(Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
struct ListResponse {
    quotes: Vec<Quote>,
    page: i64,
    next_token: Option<String>,
}

#[get("/19/list?<token>")]
pub async fn day_19_task_two_a(pool: &State<Arc<Pool<Postgres>>>, token: &str) -> RequestResponse {
    let mut i_token = 0;

    if token != "" {
        i_token = match token.parse::<i64>() {
            Ok(token) => token,
            Err(_) => {
                return RequestResponse::Error(Status::BadRequest, "Invalid token".to_string());
            }
        };
        if token.len() != 16 {
            return RequestResponse::Error(Status::BadRequest, "Invalid token".to_string());
        }
    }
    println!("token {}", token);
    let token = i_token;
    let query = "SELECT * FROM quotes ORDER BY created_at LIMIT 4 OFFSET $1";
    let mut results = sqlx
        ::query_as::<_, Quote>(&query)
        .bind(token * 3)
        .fetch_all(&***pool).await
        .unwrap();

    let mut new_token = None;
    if results.len() == 4 {
        new_token = format!("{:0>16}", token + 1).into();
        results.pop();
    }
    let response = ListResponse {
        next_token: new_token,
        page: token + 1,
        quotes: results,
    };
    return RequestResponse::JSON(Status::Ok, serde_json::to_string(&response).unwrap());
}
#[get("/19/list", rank = 2)]
pub async fn day_19_task_two_b(pool: &State<Arc<Pool<Postgres>>>) -> RequestResponse {
    return day_19_task_two_a(pool, "").await;
}
