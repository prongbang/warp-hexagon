use warp;
use warp::Filter;
use warp_hexagon::{customer, health};
use warp_hexagon::core::error_postgres;
use warp_hexagon::database::postgres;
use warp_hexagon::locator::service_locator::ServiceLocator;

#[tokio::main]
async fn main() {
    // Initialize
    let db = customer::data::db::init_db();

    // Postgres
    let db_pool = postgres::create_pool().expect("database pool can be created");
    postgres::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    // Service Locator
    let customer_repo = customer::repository::in_local_repository::InLocalRepository::new(db.clone());
    let service_locator = ServiceLocator {
        db,
        customer_repo,
    };

    // Routes
    let health_routes = health::route::health_routes(&db_pool);
    let customer_routes = customer::route::customer_routes(service_locator);

    let routes = health_routes
        .or(customer_routes)
        .with(warp::cors().allow_any_origin())
        .recover(error_postgres::handle_rejection);

    // Serve
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}
