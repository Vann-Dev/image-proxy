use crate::utils::decoder;
use dotenv::dotenv;
use warp::Filter;
use warp::Rejection;

#[tokio::main]
pub async fn server() {
    dotenv().ok();

    let api = warp::get()
        .and(warp::path::param())
        .and_then(|params: String| async {
            if decoder::validate(params.clone()) {
                Result::<_, Rejection>::Ok(warp::reply::json(&decoder::decode(params)))
            } else {
                Result::<_, Rejection>::Ok(warp::reply::json(&"False"))
            }
        });

    warp::serve(api).run(([127, 0, 0, 1], 3000)).await;
}
