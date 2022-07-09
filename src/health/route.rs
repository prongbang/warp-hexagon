use warp::{Filter};
use crate::database::postgres;
use crate::health;

pub fn health_routes(db_pool: &postgres::DBPool) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("health")
        .and(postgres::with_db(db_pool.clone()))
        .and_then(health::handler::health_handler)
}