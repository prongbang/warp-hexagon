use std::convert::Infallible;
use warp;
use crate::core::response;
use crate::customer;
use crate::customer::domain::model::Customer;
use crate::locator::service_locator;

pub async fn fetch_all(
    locator: service_locator::ServiceLocator,
) -> Result<impl warp::Reply, Infallible> {
    match customer::domain::get_customer_list::execute(locator.customer_repo).await {
        Ok(customers) => {
            Ok(response::success(&customers))
        }
        Err(_) => {
            let v: Vec<Customer> = Vec::new();
            Ok(response::success(&v))
        }
    }
}

pub async fn create(
    new_customer: Customer,
    locator: service_locator::ServiceLocator,
) -> Result<impl warp::Reply, Infallible> {
    match customer::domain::create_customer::execute(locator.customer_repo, new_customer).await {
        Ok(customer) => Ok(response::success(&customer)),
        Err(_) => Ok(response::bad_request("bad-request"))
    }
}

pub async fn fetch_one(
    guid: String,
    locator: service_locator::ServiceLocator,
) -> Result<Box<dyn warp::Reply>, Infallible> {
    match customer::domain::get_customer::execute(locator.customer_repo, guid).await {
        Ok(customer) => Ok(Box::new(response::success(&customer))),
        Err(_) => Ok(Box::new(response::not_found("not-found")))
    }
}

pub async fn update(
    guid: String,
    update_customer: Customer,
    locator: service_locator::ServiceLocator,
) -> Result<impl warp::Reply, Infallible> {
    match customer::domain::update_customer::execute(locator.customer_repo, guid, update_customer).await {
        Ok(customer) => Ok(Box::new(response::success(&customer))),
        Err(_) => Ok(Box::new(response::not_found("not-found")))
    }
}

pub async fn delete(
    guid: String,
    locator: service_locator::ServiceLocator,
) -> Result<impl warp::Reply, Infallible> {
    match customer::domain::delete_customer::execute(locator.customer_repo, guid).await {
        Ok(_) => Ok(Box::new(response::success_message())),
        Err(_) => Ok(Box::new(response::not_found("not-found")))
    }
}