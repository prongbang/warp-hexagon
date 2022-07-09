use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;
use crate::customer;
use crate::customer::data::db::Db;

#[derive(Clone)]
pub struct ServiceLocator {
    pub db: Db,
    pub customer_repo: Arc<dyn customer::repository::repository::Repository>,
}

pub fn with_service_locator(locator: ServiceLocator) -> impl Filter<Extract=(ServiceLocator, ), Error=Infallible> + Clone {
    warp::any().map(move || locator.clone())
}