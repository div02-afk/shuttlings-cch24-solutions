use rocket::{
    fs::TempFile,
    form::{ Errors, Form },
    http::Status,
    routes,
    response::content::RawHtml,
    post,
    get,
    Route,
    FromForm,
};
use tokio::fs;
use std::borrow::BorrowMut;
use tera::escape_html;

#[get("/star")]
pub async fn day_23_task_two() -> Result<RawHtml<String>, rocket::http::Status> {
    let response = "<div id=\"star\" class=\"lit\"></div>".to_string();
    Ok(RawHtml(response))
}

#[get("/present/<color>")]
pub async fn day_23_task_three(color: &str) -> Result<RawHtml<String>, Status> {
    let sanitized_color = escape_html(color);
    let colors = vec!["red", "blue", "purple"];
    if colors.contains(&&color) {
        let index = colors
            .iter()
            .position(|&c| c == sanitized_color)
            .unwrap_or(0);
        let response = format!(
            "<div class=\"present {}\" hx-get=\"/23/present/{}\" hx-swap=\"outerHTML\"><div class=\"ribbon\"></div><div class=\"ribbon\"></div><div class=\"ribbon\"></div><div class=\"ribbon\"></div></div>",
            colors[index % 3],
            colors[(index + 1) % 3]
        );
        println!("{}", response);
        return Ok(RawHtml(response.to_string()));
    }
    return Err(Status::ImATeapot);
}

#[get("/ornament/<state>/<n>")]
pub async fn day_23_task_four(state: &str, n: &str) -> Result<RawHtml<String>, Status> {
    let sanitized_state = escape_html(state);
    let sanitized_n = escape_html(n);
    let n = sanitized_n;
    match sanitized_state.as_str() {
        "on" => {
            return Ok(
                RawHtml(
                    format!(
                        "<div class=\"ornament on\" id=\"ornament{n}\" \
                        hx-trigger=\"load delay:2s once\" \
                        hx-get=\"/23/ornament/off/{n}\" hx-swap=\"outerHTML\"></div>"
                    )
                )
            );
        }
        "off" => {
            return Ok(
                RawHtml(
                    format!(
                        "<div class=\"ornament\" id=\"ornament{n}\" \
                        hx-trigger=\"load delay:2s once\" \
                        hx-get=\"/23/ornament/on/{n}\" hx-swap=\"outerHTML\"></div>"
                    )
                )
            );
        }
        _ => {
            return Err(Status::ImATeapot);
        }
    };
}

#[derive(FromForm)]
pub struct Lockfile<'r> {
    pub lockfile: Option<TempFile<'r>>,
}

#[post("/lockfile", data = "<file>")]
pub async fn day_23_task_six(
    file: Result<Form<Lockfile<'_>>, Errors<'_>>
) -> Result<RawHtml<String>, Status> {
    match file {
        Ok(mut file) => {
            if file.lockfile.is_none() {
                println!("lockfile not found");
                return Err(Status::BadRequest);
            }

            let temp = &mut file.lockfile.as_mut().unwrap().borrow_mut();
            let path = temp.path().unwrap();

            match fs::read_to_string(&path).await {
                Ok(content) => {
                    println!("Content {content}");
                    match content.parse::<toml::Table>() {
                        Ok(content) => {
                            let packages = content
                                .get("package")
                                .and_then(|p| p.as_array())
                                .unwrap();
                            // println!("{:?}", packages);
                            let mut result = "".to_string();
                            for i in 0..packages.len() {
                                println!("Processing {i}");
                                match packages[i].get("checksum") {
                                    Some(checksum) => {
                                        if !checksum.is_str() {
                                            return Err(Status::BadRequest);
                                        }
                                        let checksum_str = checksum
                                            .as_str()
                                            .unwrap_or_else(|| { "" });
                                        if !checksum_str.chars().all(|c| c.is_digit(16)) {
                                            println!("checksum is not a valid hex string");
                                            return Err(Status::UnprocessableEntity);
                                        }
                                        if checksum_str.len() >= 10 {
                                            let color = &checksum_str[0..6];
                                            let top = i64
                                                ::from_str_radix(&checksum_str[6..8], 16)
                                                .unwrap();
                                            let left = i64
                                                ::from_str_radix(&checksum_str[8..10], 16)
                                                .unwrap();
                                            let response = format!(
                                                "<div style=\"background-color:#{color};top:{top}px;left:{left}px;\"></div>"
                                            );
                                            result += response.as_str();
                                        } else {
                                            println!("checksum short");
                                            return Err(Status::UnprocessableEntity);
                                        }
                                    }
                                    None => {
                                        println!("no checksum");
                                        // return Err(Status::BadRequest);
                                    }
                                }
                            }
                            return Ok(RawHtml(result));
                        }
                        Err(e) => {
                            println!("couldn't parse {e}");
                            return Err(Status::BadRequest);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read lockfile content: {}", e);
                    return Err(Status::BadRequest);
                }
            }
        }
        Err(e) => {
            eprintln!("Error in parsing form: {}", e);
            return Err(Status::BadRequest);
        }
    }
}

pub fn routes() -> Vec<Route> {
    return routes![day_23_task_two, day_23_task_three, day_23_task_four, day_23_task_six];
}
