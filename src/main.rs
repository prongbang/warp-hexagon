mod customer;
mod core;
mod locator;

use warp;
use crate::locator::service_locator::ServiceLocator;

#[tokio::main]
async fn main() {
    // Initialize
    let db = customer::db::init_db();
    let customer_repo = customer::repository::in_local_repository::InLocalRepository::new(db.clone());

    // Service Locator
    let service_locator = ServiceLocator {
        db,
        customer_repo,
    };

    // Routes
    let customer_routes = customer::route::customer_routes(service_locator);

    // Serve
    warp::serve(customer_routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}
