use std::sync::{ Arc, RwLock };
use crate::{ day_5::RequestResponse, MilkCookiesPack };
use rocket::{ get, http::Status, post, State };

#[get("/12/board")]
pub fn day_12_task_one(board: &State<Arc<RwLock<MilkCookiesPack>>>) -> RequestResponse {
    let response = board.read().unwrap().value.clone();
    let winner = board.read().unwrap().winner.clone();
    let is_full = board.read().unwrap().is_full.clone();
    if winner != ' ' {
        return RequestResponse::Success(format!("{}{} wins!\n", response, winner));
    }
    if is_full {
        return RequestResponse::Success(format!("{}No winner.\n", response));
    }
    RequestResponse::Success(response.to_string())
}

#[post("/12/reset")]
pub fn day_12_task_one_two(board: &State<Arc<RwLock<MilkCookiesPack>>>) -> RequestResponse {
    let mut board = board.write().unwrap();
    *board = MilkCookiesPack::reset();
    RequestResponse::Success(board.value.to_string())
}

#[get("/12/random-board")]
pub fn day_12_task_three(board: &State<Arc<RwLock<MilkCookiesPack>>>) -> RequestResponse {
    let mut board = board.write().unwrap();
    let random_board = MilkCookiesPack::random(&mut board.rng);
    let board_vec = random_board.value.chars().collect::<Vec<char>>();
    let a = check_winner(&board_vec);
    let b = is_board_full(&board_vec);
    if a != ' ' {
        return RequestResponse::Success(format!("{}{} wins!\n", random_board.value.to_string(), a));
    } else if b {
        return RequestResponse::Success(format!("{}No winner.\n", random_board.value.to_string()));
    }
    return RequestResponse::Success(format!("{}", random_board.value.to_string()));
}
fn check_winner_horizontal(board_vec: &Vec<char>) -> char {
    for row in 0..board_vec.len() - 7 {
        if row % 7 != 1 {
            continue;
        }
        println!("row {}", row);
        if
            board_vec[row] != '⬛' &&
            board_vec[row] == board_vec[row + 1] &&
            board_vec[row] == board_vec[row + 2] &&
            board_vec[row] == board_vec[row + 3]
        {
            return board_vec[row];
        }
    }
    ' '
}

fn check_winner_vertical(board_vec: &Vec<char>) -> char {
    for col in 1..5 {
        if
            board_vec[col] != '⬛' &&
            board_vec[col] == board_vec[col + 7] &&
            board_vec[col] == board_vec[col + 14] &&
            board_vec[col] == board_vec[col + 21]
        {
            return board_vec[col];
        }
    }
    ' '
}

fn check_winner_diagonal(board_vec: &Vec<char>) -> char {
    if
        board_vec[1] != '⬛' &&
        board_vec[1] == board_vec[9] &&
        board_vec[1] == board_vec[17] &&
        board_vec[1] == board_vec[25]
    {
        return board_vec[1];
    }
    if
        board_vec[4] != '⬛' &&
        board_vec[4] == board_vec[10] &&
        board_vec[4] == board_vec[16] &&
        board_vec[4] == board_vec[22]
    {
        return board_vec[4];
    }

    ' '
}

fn check_winner(board_vec: &Vec<char>) -> char {
    let horizontal = check_winner_horizontal(board_vec);
    println!("horizontal {}", horizontal);
    if horizontal != ' ' {
        return horizontal;
    }

    let vertical = check_winner_vertical(board_vec);
    println!("vertical {}", vertical);
    if vertical != ' ' {
        return vertical;
    }
    let diagonal = check_winner_diagonal(board_vec);
    println!("diagonal {}", diagonal);
    diagonal
}

fn is_board_full(board_vec: &Vec<char>) -> bool {
    board_vec.iter().all(|&c| c != '⬛')
}

#[post("/12/place/<team>/<column>")]
pub fn day_12_task_two(
    board: &State<Arc<RwLock<MilkCookiesPack>>>,
    team: &str,
    column: &str
) -> RequestResponse {
    if let Some(x) = column.parse::<i8>().ok() {
        if x <= 0 || x > 4 {
            return RequestResponse::Error(Status::BadRequest, "".to_string());
        }
    } else {
        return RequestResponse::Error(Status::BadRequest, "".to_string());
    }

    let col_num = column.parse::<i8>().unwrap();

    if (team != "cookie" && team != "milk") || col_num <= 0 || col_num > 4 {
        return RequestResponse::Error(Status::BadRequest, "".to_string());
    }

    println!("col_num {}", col_num);
    let mut board = board.write().unwrap();
    if board.is_winner {
        return RequestResponse::Error(
            Status::ServiceUnavailable,
            // board.value.to_string()
            format!("{}{} wins!\n", board.value.to_string(), board.winner)
        );
    } else if board.is_full {
        return RequestResponse::Error(
            Status::ServiceUnavailable,
            // board.value.to_string()
            format!("{}No winner.\n", board.value.to_string())
        );
    }
    let mut board_vec = board.value.chars().collect::<Vec<char>>();

    let mut placed = false;
    for i in 0..5 {
        let index = (col_num + (4 - i) * 7) as usize;
        println!("Index {}", index);
        match team.to_string().as_str() {
            "cookie" => {
                if board_vec[index] == '⬛' {
                    println!("added cookie at {}", col_num);
                    board_vec[index] = '🍪';
                    placed = true;
                    break;
                }
            }
            "milk" => {
                if board_vec[index] == '⬛' {
                    board_vec[index] = '🥛';
                    println!("added milk at {}", col_num);
                    placed = true;
                    break;
                }
            }
            _ => {
                break;
            }
        }

        println!("{:?}", board_vec);
    }
    if !placed {
        return RequestResponse::Error(Status::ServiceUnavailable, board.value.to_string());
    }
    let is_winner = check_winner(&board_vec);
    let is_full = is_board_full(&board_vec);
    board.winner = is_winner;

    println!("result {}", is_winner);
    board.value = board_vec.iter().collect::<String>();
    if is_winner != ' ' {
        board.is_winner = true;
        return RequestResponse::Success(
            format!("{}{} wins!\n", board.value.to_string(), is_winner)
        );
    } else if is_full {
        board.is_full = true;
        return RequestResponse::Success(format!("{}No winner.\n", board.value.to_string()));
    }
    return RequestResponse::Success(format!("{}", board.value.to_string()));
}
