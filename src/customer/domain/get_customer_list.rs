use std::sync::Arc;
use crate::core::error::Error;
use crate::customer::domain::model::Customer;
use crate::customer::repository::repository::Repository;

pub async fn execute(repo: Arc<dyn Repository>) -> Result<Vec<Customer>, Error> {
    repo.fetch_all().await
}