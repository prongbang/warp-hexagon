use std::sync::Arc;
use async_trait::async_trait;
use crate::core::error::Error;
use crate::customer::data::db::Db;
use crate::customer::domain::model::Customer;
use crate::customer::repository::repository::Repository;

pub struct InLocalRepository {
    db: Db,
}

impl InLocalRepository {
    pub fn new(db: Db) -> Arc<dyn Repository> {
        Arc::new(Self { db })
    }
}

#[async_trait]
impl Repository for InLocalRepository {
    async fn fetch_all(&self) -> Result<Vec<Customer>, Error> {
        let customers = self.db.lock().await;
        let customers: Vec<Customer> = customers.clone();
        Ok(customers)
    }

    async fn fetch_one(&self, guid: String) -> Result<Customer, Error> {
        let customers = self.db.lock().await;

        // Find customer by guid
        for c in customers.iter() {
            if c.guid == guid {
                return Ok(c.clone());
            }
        }

        Err(Error::NotFound)
    }

    async fn create(&self, new_customer: Customer) -> Result<Customer, Error> {
        let mut customers = self.db.lock().await;

        // Validate data duplicate
        for c in customers.iter() {
            if c.guid == new_customer.guid {
                return Err(Error::Conflict);
            }
        }

        // Create
        customers.push(new_customer.clone());

        Ok(new_customer.clone())
    }

    async fn update(&self, guid: String, new_customer: Customer) -> Result<Customer, Error> {
        let mut customers = self.db.lock().await;

        for c in customers.iter_mut() {
            if c.guid == guid {
                *c = new_customer.clone();
                return Ok(new_customer.clone());
            }
        }

        Err(Error::NotFound)
    }

    async fn delete(&self, guid: String) -> Result<(), Error> {
        let mut customers = self.db.lock().await;

        let customer_count = customers.len();

        customers.retain(|customer| customer.guid != guid);

        let deleted = customers.len() != customer_count;
        if deleted {
            return Ok(());
        }

        Err(Error::NotFound)
    }
}