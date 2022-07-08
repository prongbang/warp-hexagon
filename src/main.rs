mod customer;

use warp;

#[tokio::main]
async fn main()  {
    // Initialize
    let db = customer::db::init_db();

    // Routes
    let customer_routes = customer::route::customer_routes(db);

    // Serve
    warp::serve(customer_routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}
