use std::convert::Infallible;
use warp::{self, Filter};

use crate::customer::db::Db;
use crate::customer::handler;
use crate::customer::model::Customer;

pub fn customer_routes(db: Db) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    get_customer(db.clone())
        .or(update_customer(db.clone()))
        .or(delete_customer(db.clone()))
        .or(create_customer(db.clone()))
        .or(list_customers(db.clone()))
}

/// GET /customer
pub fn list_customers(
    db: Db,
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("customer")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handler::list_customers)
}

/// POST /customer
pub fn create_customer(
    db: Db,
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("customer")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handler::create_customers)
}

/// GET /customer/{guid}
pub fn get_customer(
    db: Db,
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("customer" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handler::get_customer)
}

/// PUT /customer/{guid}
pub fn update_customer(
    db: Db,
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("customer" / String)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handler::update_customer)
}

/// DELETE /customer/{guid}
pub fn delete_customer(
    db: Db,
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("customer" / String)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handler::delete_customer)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Customer,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}