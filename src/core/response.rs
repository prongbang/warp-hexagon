use serde::{Deserialize, Serialize};
use warp::{self};
use warp::reply::Json;

#[derive(Clone, Deserialize, Serialize)]
pub struct Response<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

pub fn success<T>(data: &T) -> Json
    where
        T: Serialize,
{
    warp::reply::json(&Response {
        code: 200,
        message: "success".to_string(),
        data,
    })
}

pub fn success_message() -> Json {
    warp::reply::json(&Response {
        code: 200,
        message: "success".to_string(),
        data: (),
    })
}

pub fn bad_request(message: &str) -> Json {
    warp::reply::json(&Response {
        code: 400,
        message: message.to_string(),
        data: (),
    })
}

pub fn not_found(message: &str) -> Json {
    warp::reply::json(&Response {
        code: 404,
        message: message.to_string(),
        data: (),
    })
}