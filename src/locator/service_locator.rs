use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;
use crate::customer;
use crate::customer::db::Db;
use crate::customer::domain::{CreateCustomerUseCase, DeleteCustomerUseCase, GetCustomerListUseCase, GetCustomerOneUseCase, UpdateCustomerUseCase};

#[derive(Clone)]
pub struct ServiceLocator {
    pub db: Db,
    pub customer_repo: Arc<dyn customer::repository::Repository>,
    pub get_customer_list_use_case: Arc<dyn GetCustomerListUseCase>,
    pub get_customer_one_use_cae: Arc<dyn GetCustomerOneUseCase>,
    pub create_customer_use_case: Arc<dyn CreateCustomerUseCase>,
    pub update_customer_use_case: Arc<dyn UpdateCustomerUseCase>,
    pub delete_customer_use_case: Arc<dyn DeleteCustomerUseCase>,
}

pub fn with_service_locator(locator: ServiceLocator) -> impl Filter<Extract=(ServiceLocator, ), Error=Infallible> + Clone {
    warp::any().map(move || locator.clone())
}