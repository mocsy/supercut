use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

/// Apply a luminance mask on an image to alter the Alpha channel.
/// It does not remove or otherwise alter the other channels.
/// If the mask is of different size, it is resized to fit.
pub fn masked_cut(image: &DynamicImage, mask: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img = image.to_rgba8();
    let (width, height) = img.dimensions();
    let (mwidth, mheight) = mask.dimensions();
    let wrate = mwidth as f64 / width as f64;
    let hrate = mheight as f64 / height as f64;

    // println!("ss");
    for x in 0..width {
        for y in 0..height {
            let mut px = *img.get_pixel(x, y);
            let alpha = mask.get_pixel((x as f64 * wrate) as u32, (y as f64 * hrate) as u32);
            let avg = 255 - ((alpha.0[0] as u32 + alpha.0[1] as u32 + alpha.0[2] as u32) / 3) as u8;

            px.0[3] = avg;

            img.put_pixel(x, y, px);
        }
    }
    // println!("ee");

    img
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let image = image::open("cat.jpg").unwrap();
        let mask = image::open("mask_4k.png").unwrap();

        masked_cut(&image, &mask).save("test.png").unwrap();
    }
}
