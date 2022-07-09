use std::sync::Arc;
use crate::customer::model::Customer;
use crate::customer::repository::Repository;
use async_trait::async_trait;
use crate::core::error::Error;

#[async_trait]
pub trait GetCustomerListUseCase: Send + Sync {
    async fn execute(&self) -> Result<Vec<Customer>, Error>;
}

#[async_trait]
pub trait GetCustomerOneUseCase: Send + Sync {
    async fn execute(&self, guid: String) -> Result<Customer, Error>;
}

#[async_trait]
pub trait CreateCustomerUseCase: Send + Sync {
    async fn execute(&self, customer: Customer) -> Result<Customer, Error>;
}

#[async_trait]
pub trait UpdateCustomerUseCase: Send + Sync {
    async fn execute(&self, guid: String, customer: Customer) -> Result<Customer, Error>;
}

#[async_trait]
pub trait DeleteCustomerUseCase: Send + Sync {
    async fn execute(&self, guid: String) -> Result<(), Error>;
}

pub struct DefaultGetCustomerListUseCase {
    repo: Arc<dyn Repository>,
}

pub struct DefaultGetCustomerOneUseCase {
    repo: Arc<dyn Repository>,
}

pub struct DefaultCreateCustomerUseCase {
    repo: Arc<dyn Repository>,
}

pub struct DefaultUpdateCustomerUseCase {
    repo: Arc<dyn Repository>,
}

pub struct DefaultDeleteCustomerUseCase {
    repo: Arc<dyn Repository>,
}

impl DefaultGetCustomerListUseCase {
    pub fn new(repo: Arc<dyn Repository>) -> Arc<dyn GetCustomerListUseCase> {
        Arc::new(DefaultGetCustomerListUseCase { repo })
    }
}

impl DefaultGetCustomerOneUseCase {
    pub fn new(repo: Arc<dyn Repository>) -> Arc<dyn GetCustomerOneUseCase> {
        Arc::new(DefaultGetCustomerOneUseCase { repo })
    }
}

impl DefaultCreateCustomerUseCase {
    pub fn new(repo: Arc<dyn Repository>) -> Arc<dyn CreateCustomerUseCase> {
        Arc::new(DefaultCreateCustomerUseCase { repo })
    }
}

impl DefaultUpdateCustomerUseCase {
    pub fn new(repo: Arc<dyn Repository>) -> Arc<dyn UpdateCustomerUseCase> {
        Arc::new(DefaultUpdateCustomerUseCase { repo })
    }
}

impl DefaultDeleteCustomerUseCase {
    pub fn new(repo: Arc<dyn Repository>) -> Arc<dyn DeleteCustomerUseCase> {
        Arc::new(DefaultDeleteCustomerUseCase { repo })
    }
}

#[async_trait]
impl GetCustomerListUseCase for DefaultGetCustomerListUseCase {
    async fn execute(&self) -> Result<Vec<Customer>, Error> {
        self.repo.fetch_all().await
    }
}

#[async_trait]
impl GetCustomerOneUseCase for DefaultGetCustomerOneUseCase {
    async fn execute(&self, guid: String) -> Result<Customer, Error> {
        self.repo.fetch_one(guid).await
    }
}

#[async_trait]
impl CreateCustomerUseCase for DefaultCreateCustomerUseCase {
    async fn execute(&self, customer: Customer) -> Result<Customer, Error> {
        self.repo.create(customer).await
    }
}

#[async_trait]
impl UpdateCustomerUseCase for DefaultUpdateCustomerUseCase {
    async fn execute(&self, guid: String, customer: Customer) -> Result<Customer, Error> {
        self.repo.update(guid, customer).await
    }
}

#[async_trait]
impl DeleteCustomerUseCase for DefaultDeleteCustomerUseCase {
    async fn execute(&self, guid: String) -> Result<(), Error> {
        self.repo.delete(guid).await
    }
}