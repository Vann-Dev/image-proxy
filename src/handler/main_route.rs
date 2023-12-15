use crate::utils::decoder::{decode, validate};
use crate::utils::fetch::fetch;

pub async fn main_route(params: String) -> Result<impl warp::Reply, warp::Rejection> {
    if validate(params.clone()) {
        let real_uri = decode(params);
        let result = fetch(real_uri).await.unwrap().into_raw();

        Ok(warp::reply::with_header(
            result,
            "Content-Type",
            "image/jpeg",
        ))
    } else {
        Err(warp::reject::not_found())
    }
}
