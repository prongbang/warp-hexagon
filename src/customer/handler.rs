use std::convert::Infallible;
use warp::{self, http::StatusCode};
use crate::customer::db::Db;
use crate::customer::model::Customer;

pub async fn list_customers(
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let customers = db.lock().await;
    let customers: Vec<Customer> = customers.clone();

    Ok(warp::reply::json(&customers))
}

pub async fn create_customers(
    new_customer: Customer,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    // Validate data duplicate
    for c in customers.iter() {
        if c.guid == new_customer.guid {
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    // Create
    customers.push(new_customer);

    Ok(StatusCode::CREATED)
}

pub async fn get_customer(
    guid: String,
    db: Db,
) -> Result<Box<dyn warp::Reply>, Infallible> {
    let customers = db.lock().await;

    // Find customer by guid
    for c in customers.iter() {
        if c.guid == guid {
            return Ok(Box::new(warp::reply::json(&c)));
        }
    }

    Ok(Box::new(StatusCode::NOT_FOUND))
}

pub async fn update_customer(
    guid: String,
    update_customer: Customer,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    for c in customers.iter_mut() {
        if c.guid == guid {
            *c = update_customer;
            return Ok(StatusCode::OK);
        }
    }

    Ok(StatusCode::NOT_FOUND)
}

pub async fn delete_customer(
    guid: String,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    let customer_count = customers.len();

    customers.retain(|customer| customer.guid != guid);

    let deleted = customers.len() != customer_count;
    if deleted {
        return Ok(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NOT_FOUND)
}