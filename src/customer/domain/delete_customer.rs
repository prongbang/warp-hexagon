use std::sync::Arc;
use crate::core::error::Error;
use crate::customer::repository::repository::Repository;

pub async fn execute(repo: Arc<dyn Repository>, guid: String) -> Result<(), Error> {
    repo.delete(guid).await
}