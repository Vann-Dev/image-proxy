use std::io::Cursor;

pub async fn fetch(uri: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(uri).await?.bytes().await?;

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

    Ok(result)
}
