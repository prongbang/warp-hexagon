use std::convert::Infallible;
use std::fs::File;
use std::sync::Arc;

use serde_json::from_reader;
use tokio::sync::Mutex;
use warp::Filter;

use crate::customer::domain::model::Customer;

pub type Db = Arc<Mutex<Vec<Customer>>>;

pub fn init_db() -> Db {
    let file = File::open("./data/customers.json");
    match file {
        Ok(json) => {
            let customers = from_reader(json).unwrap();
            Arc::new(Mutex::new(customers))
        }
        Err(_) => {
            Arc::new(Mutex::new(Vec::new()))
        }
    }
}

fn with_db(db: Db) -> impl Filter<Extract=(Db, ), Error=Infallible> + Clone {
    warp::any().map(move || db.clone())
}