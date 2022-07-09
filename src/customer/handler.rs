use std::convert::Infallible;
use warp;
use crate::core::response;
use crate::customer::model::Customer;
use crate::locator::service_locator;

pub async fn fetch_all(
    locator: service_locator::ServiceLocator,
) -> Result<impl warp::Reply, Infallible> {
    let use_case = locator.get_customer_list_use_case.clone();
    match use_case.execute().await {
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
    let use_case = locator.create_customer_use_case.clone();
    match use_case.execute(new_customer).await {
        Ok(customer) => Ok(response::success(&customer)),
        Err(_) => Ok(response::bad_request("bad-request"))
    }
}

pub async fn fetch_one(
    guid: String,
    locator: service_locator::ServiceLocator,
) -> Result<Box<dyn warp::Reply>, Infallible> {
    let use_case = locator.get_customer_one_use_cae.clone();
    match use_case.execute(guid).await {
        Ok(customer) => Ok(Box::new(response::success(&customer))),
        Err(_) => Ok(Box::new(response::not_found("not-found")))
    }
}

pub async fn update(
    guid: String,
    update_customer: Customer,
    locator: service_locator::ServiceLocator,
) -> Result<impl warp::Reply, Infallible> {
    let use_case = locator.update_customer_use_case.clone();
    match use_case.execute(guid, update_customer).await {
        Ok(customer) => Ok(Box::new(response::success(&customer))),
        Err(_) => Ok(Box::new(response::not_found("not-found")))
    }
}

pub async fn delete(
    guid: String,
    locator: service_locator::ServiceLocator,
) -> Result<impl warp::Reply, Infallible> {
    let use_case = locator.delete_customer_use_case.clone();
    match use_case.execute(guid).await {
        Ok(_) => Ok(Box::new(response::success_message())),
        Err(_) => Ok(Box::new(response::not_found("not-found")))
    }
}