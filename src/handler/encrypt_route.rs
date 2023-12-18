use crate::utils::{self, encrypter::encrypt, logger::log};
use serde_json::json;

#[derive(serde::Deserialize, serde::Serialize)]

pub struct RequestStruct {
    url: String,
}

pub async fn encrypt_route(request: RequestStruct) -> Result<impl warp::Reply, warp::Rejection> {
    let encrypt_url = encrypt(request.url.clone());

    log(
        utils::logger::Mode::Info,
        "Success encrypting: ".to_owned() + &request.url,
    );

    Ok(warp::reply::json(&json!({"data": encrypt_url})))
}
