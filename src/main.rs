mod customer;
mod core;
mod locator;

use warp;
use crate::locator::service_locator::ServiceLocator;

#[tokio::main]
async fn main() {
    // Initialize
    let db = customer::db::init_db();
    let customer_repo = customer::repository::InLocalRepository::new(db.clone());
    let get_customer_list_use_case = customer::domain::DefaultGetCustomerListUseCase::new(customer_repo.clone());
    let get_customer_one_use_cae = customer::domain::DefaultGetCustomerOneUseCase::new(customer_repo.clone());
    let create_customer_use_case = customer::domain::DefaultCreateCustomerUseCase::new(customer_repo.clone());
    let update_customer_use_case = customer::domain::DefaultUpdateCustomerUseCase::new(customer_repo.clone());
    let delete_customer_use_case = customer::domain::DefaultDeleteCustomerUseCase::new(customer_repo.clone());

    // Service Locator
    let service_locator = ServiceLocator {
        db,
        customer_repo,
        get_customer_list_use_case,
        get_customer_one_use_cae,
        create_customer_use_case,
        update_customer_use_case,
        delete_customer_use_case,
    };

    // Routes
    let customer_routes = customer::route::customer_routes(service_locator);

    // Serve
    warp::serve(customer_routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}
