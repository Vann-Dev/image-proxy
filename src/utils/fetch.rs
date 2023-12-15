use image::{ImageBuffer, Rgb};

pub async fn fetch(
    uri: String,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(uri).await?.bytes().await?;

    let image = image::load_from_memory(&resp).unwrap().into_rgb8();

    Ok(image)
}
