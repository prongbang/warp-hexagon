use async_trait::async_trait;
use crate::core::error::Error;
use crate::customer::domain::model::Customer;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn fetch_all(&self) -> Result<Vec<Customer>, Error>;
    async fn fetch_one(&self, guid: String) -> Result<Customer, Error>;
    async fn create(&self, new_customer: Customer) -> Result<Customer, Error>;
    async fn update(&self, guid: String, new_customer: Customer) -> Result<Customer, Error>;
    async fn delete(&self, guid: String) -> Result<(), Error>;
}