use dotenv::dotenv;
use warp::Filter;

use crate::utils::decoder::decode;

#[tokio::main]
pub async fn server() {
    dotenv().ok();

    let hi = warp::path("api")
        .and(warp::path::param())
        .map(|params: String| format!("{}", decode(params)));

    warp::serve(hi).run(([127, 0, 0, 1], 3000)).await;
}
