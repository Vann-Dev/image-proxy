use crate::handler::encrypt_route::encrypt_route;
use crate::handler::main_route::main_route;
use dotenv::dotenv;
use warp::Filter;

#[tokio::main]
pub async fn server() {
    dotenv().ok();

    let main_route = warp::get().and(warp::path::param()).and_then(main_route);

    let encrypt_route = warp::post()
        .and(warp::path("encrypt"))
        .and(warp::body::json())
        .and_then(encrypt_route);

    let routes = main_route.or(encrypt_route);

    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
}
