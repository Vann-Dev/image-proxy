use std::io::Cursor;

use super::logger::{self, log};

pub async fn fetch(uri: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(uri.clone()).await?.bytes().await?;

    let image = image::load_from_memory(&resp).unwrap().to_rgb8();
    let (width, height) = (image.clone().width(), image.clone().height());
    let image_byte_vec = image.clone().into_raw();

    let mut result = Vec::new();
    image::write_buffer_with_format(
        &mut Cursor::new(&mut result),
        &image_byte_vec,
        width,
        height,
        image::ColorType::Rgb8,
        image::ImageFormat::Jpeg,
    )
    .unwrap();

    log(
        logger::Mode::Info,
        "Success fetching real source from: ".to_string() + &uri,
    );

    Ok(result)
}
