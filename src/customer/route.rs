use warp::{self, Filter};

use crate::customer::handler;
use crate::customer::domain::model::Customer;
use crate::locator::service_locator;
use crate::locator::service_locator::with_service_locator;

pub fn customer_routes(
    locator: service_locator::ServiceLocator,
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    get_customer(locator.clone())
        .or(update_customer(locator.clone()))
        .or(delete_customer(locator.clone()))
        .or(create_customer(locator.clone()))
        .or(list_customers(locator.clone()))
}

/// GET /customer
pub fn list_customers(
    locator: service_locator::ServiceLocator,
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("customer")
        .and(warp::get())
        .and(with_service_locator(locator))
        .and_then(handler::fetch_all)
}

/// POST /customer
pub fn create_customer(
    locator: service_locator::ServiceLocator,
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("customer")
        .and(warp::post())
        .and(json_body())
        .and(with_service_locator(locator))
        .and_then(handler::create)
}

/// GET /customer/{guid}
pub fn get_customer(
    locator: service_locator::ServiceLocator,
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("customer" / String)
        .and(warp::get())
        .and(with_service_locator(locator))
        .and_then(handler::fetch_one)
}

/// PUT /customer/{guid}
pub fn update_customer(
    locator: service_locator::ServiceLocator,
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("customer" / String)
        .and(warp::put())
        .and(json_body())
        .and(with_service_locator(locator))
        .and_then(handler::update)
}

/// DELETE /customer/{guid}
pub fn delete_customer(
    locator: service_locator::ServiceLocator,
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("customer" / String)
        .and(warp::delete())
        .and(with_service_locator(locator))
        .and_then(handler::delete)
}

fn json_body() -> impl Filter<Extract=(Customer, ), Error=warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}