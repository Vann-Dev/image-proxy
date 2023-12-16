use crate::utils::encrypter::encrypt;
use serde_json::json;

#[derive(serde::Deserialize, serde::Serialize)]

pub struct RequestStruct {
    url: String,
}

pub async fn encrypt_route(request: RequestStruct) -> Result<impl warp::Reply, warp::Rejection> {
    let encrypt_url = encrypt(request.url);

    Ok(warp::reply::json(&json!({"data": encrypt_url})))
}
